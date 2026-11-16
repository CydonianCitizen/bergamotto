import { createHash } from 'node:crypto';
import { execFile as execFileCallback } from 'node:child_process';
import { readFile, rename, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { promisify } from 'node:util';
import { fileURLToPath } from 'node:url';

import { collectFrontendLicenses, renderFrontendLicenseAudit } from './frontend-license-data.mjs';
import { collectOptionalToolLicenses } from './optional-tools-license-data.mjs';

const execFile = promisify(execFileCallback);
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const EXPECTED_CARGO_ABOUT_VERSION = '0.9.3';
const FRONTEND_LICENSE_EXCEPTIONS = ['OFL-1.1'];
const REQUIRED_SPDX_RECONSTRUCTION_ANNOTATION = 'license text reconstructed from the SPDX template; no copyright notice found in the distributed package';

function sha256(text) {
  return createHash('sha256').update(text).digest('hex');
}

function fail(message) {
  throw new Error(`Third-party license generation failed: ${message}`);
}

function text(value, label) {
  if (typeof value !== 'string' || value.trim() === '') fail(`${label} is missing actual license text`);
  return value;
}

function parseLicenseIds(expression) {
  if (Array.isArray(expression)) return expression.flatMap(parseLicenseIds);
  if (typeof expression !== 'string' || !expression.trim()) return [];
  return expression
    .replace(/[()]/g, ' ')
    .split(/\s+(?:AND|OR)\s+/)
    .map(value => value.trim())
    .filter(Boolean);
}

export function parseCargoDenyAllowList(denyConfig) {
  const licensesSection = denyConfig.match(/\[licenses\]([\s\S]*?)(?=\n\[[^\]]+\]|$)/)?.[1];
  const allowList = licensesSection?.match(/\ballow\s*=\s*\[([\s\S]*?)\]/)?.[1];
  if (!allowList) fail('deny.toml [licenses].allow list could not be parsed');
  const licenses = [...allowList.matchAll(/"([^"]+)"/g)].map(match => match[1]);
  if (!licenses.length) fail('deny.toml [licenses].allow list is empty');
  return new Set(licenses);
}

function validateRustData(rust, allowList) {
  if (rust.schemaVersion !== 1) fail('unsupported Rust license-data schema');
  if (rust.tool?.name !== 'cargo-about' || rust.tool?.version !== EXPECTED_CARGO_ABOUT_VERSION) {
    fail(`expected cargo-about ${EXPECTED_CARGO_ABOUT_VERSION}; received ${rust.tool?.name ?? 'unknown'} ${rust.tool?.version ?? 'unknown'}`);
  }
  if (rust.unresolved?.length) fail(`Rust license metadata is unresolved: ${JSON.stringify(rust.unresolved)}`);
  if (!Array.isArray(rust.packages) || !rust.packages.length) fail('Rust license collector returned no third-party packages');
  if (!Array.isArray(rust.licenseTexts) || !rust.licenseTexts.length) fail('Rust license collector returned no license texts');

  const licenseTexts = new Map();
  for (const notice of rust.licenseTexts) {
    const actualText = text(notice.text, `Rust ${notice.spdxId} notice`);
    const actualHash = sha256(actualText);
    if (actualHash !== notice.sha256) fail(`Rust ${notice.spdxId} license text hash mismatch`);
    if (licenseTexts.has(actualHash)) fail(`duplicate Rust license text entry ${actualHash}`);
    licenseTexts.set(actualHash, notice);
  }

  for (const packageInfo of rust.packages) {
    if (packageInfo.isWorkspace !== false) fail(`Rust package ${packageInfo.name}@${packageInfo.version} is not explicitly identified as third-party`);
    if (!packageInfo.name || !packageInfo.version || !packageInfo.source) {
      fail(`Rust package is missing name/version/source provenance: ${JSON.stringify(packageInfo)}`);
    }
    if (!Array.isArray(packageInfo.targets) || packageInfo.targets.length === 0) {
      fail(`Rust package ${packageInfo.name}@${packageInfo.version} has no target provenance`);
    }
    if (!Array.isArray(packageInfo.selectedLicenseIds) || packageInfo.selectedLicenseIds.length === 0) {
      fail(`Rust package ${packageInfo.name}@${packageInfo.version} has no selected SPDX license`);
    }
    for (const id of packageInfo.selectedLicenseIds) {
      if (!allowList.has(id)) fail(`Rust package ${packageInfo.name}@${packageInfo.version} uses disallowed license ${id}`);
    }
    if (!Array.isArray(packageInfo.licenseTextSha256) || packageInfo.licenseTextSha256.length === 0) {
      fail(`Rust package ${packageInfo.name}@${packageInfo.version} has no pinned license text`);
    }
    for (const hash of packageInfo.licenseTextSha256) {
      const notice = licenseTexts.get(hash);
      if (!notice) fail(`Rust package ${packageInfo.name}@${packageInfo.version} references missing license text ${hash}`);
      if (!packageInfo.selectedLicenseIds.includes(notice.spdxId)) {
        fail(`Rust package ${packageInfo.name}@${packageInfo.version} text ${hash} does not match its selected SPDX licenses`);
      }
      if (!notice.crates?.some(crate => crate.name === packageInfo.name && crate.version === packageInfo.version)) {
        fail(`Rust license text ${hash} does not include ${packageInfo.name}@${packageInfo.version} in its provenance list`);
      }
    }
    for (const id of packageInfo.selectedLicenseIds) {
      if (!packageInfo.licenseTextSha256.some(hash => licenseTexts.get(hash)?.spdxId === id)) {
        fail(`Rust package ${packageInfo.name}@${packageInfo.version} has no complete ${id} license text`);
      }
    }
  }

  const targetList = rust.aboutMarkdownTargets ?? rust.scope.targets;
  if (!Array.isArray(targetList) || JSON.stringify(targetList) !== JSON.stringify(rust.scope.targets)) {
    fail('cargo-about about.hbs render did not cover the collected target set');
  }
  if (typeof rust.aboutMarkdown !== 'string' || !rust.aboutMarkdown.trim()) {
    fail('cargo-about did not render the checked-in about.hbs template');
  }
  if (!Array.isArray(rust.aboutTemplateLicenseTexts) || rust.aboutTemplateLicenseTexts.length === 0) {
    fail('cargo-about did not provide the raw union JSON license texts for validating about.hbs');
  }
  const templateLicenseTexts = new Map();
  for (const templateNotice of rust.aboutTemplateLicenseTexts) {
    const templateText = text(templateNotice.text, `cargo-about union ${templateNotice.spdxId} notice`);
    if (!templateNotice.spdxId || sha256(templateText) !== templateNotice.sha256) {
      fail(`cargo-about union ${templateNotice.spdxId ?? 'unknown'} text hash mismatch`);
    }
    const normalizedText = templateText.replace(/\r\n?/g, '\n');
    const key = `${templateNotice.spdxId}|${sha256(normalizedText)}`;
    const existingText = templateLicenseTexts.get(key);
    if (existingText !== undefined && existingText !== normalizedText) {
      fail(`conflicting cargo-about union license text ${key}`);
    }
    templateLicenseTexts.set(key, normalizedText);
  }
  const renderedTexts = parseAboutMarkdownLicenseTexts(rust.aboutMarkdown);
  if (renderedTexts.size === 0) fail('cargo-about about.hbs rendered no complete license texts');
  for (const [key, renderedText] of renderedTexts) {
    const expectedText = templateLicenseTexts.get(key);
    if (expectedText === undefined) fail(`about.hbs rendered unverified cargo-about union text ${key}`);
    if (renderedText !== expectedText) fail(`about.hbs changed cargo-about union text ${key}`);
  }
  const renderedIds = new Set([...renderedTexts.keys()].map(key => key.slice(0, key.indexOf('|'))));
  for (const notice of licenseTexts.values()) {
    if (!renderedIds.has(notice.spdxId)) {
      fail(`about.hbs output is missing the selected ${notice.spdxId} license section`);
    }
  }
  return licenseTexts;
}

function parseAboutMarkdownLicenseTexts(markdown) {
  const lines = markdown.replace(/\r\n?/g, '\n').split('\n');
  const rendered = new Map();
  for (let index = 0; index < lines.length; index += 1) {
    const header = lines[index].match(/^## .+ \(`([^`]+)`\)$/);
    if (!header) continue;
    const open = lines.indexOf('~~~~text', index + 1);
    if (open === -1) continue;
    const close = lines.indexOf('~~~~', open + 1);
    if (close === -1) fail(`about.hbs output has an unterminated ${header[1]} text block`);
    const body = lines.slice(open + 1, close).join('\n');
    rendered.set(`${header[1]}|${sha256(body)}`, body);
    index = close;
  }
  return rendered;
}

async function loadRustFallbackData(repoRoot, allowList) {
  const ledgerPath = path.join(repoRoot, 'third-party/rust-license-fallbacks.json');
  let ledger;
  try {
    ledger = JSON.parse(await readFile(ledgerPath, 'utf8'));
  } catch (error) {
    fail(`cannot read Rust recovery ledger: ${error.message}`);
  }
  if (ledger.schemaVersion !== 1 || !Array.isArray(ledger.packages)) fail('unsupported Rust recovery ledger schema');

  const seen = new Set();
  const supplementalByHash = new Map();
  for (const record of ledger.packages) {
    const packageKey = `${record.name}@${record.version}`;
    if (!record.name || !record.version || seen.has(packageKey)) fail(`duplicate or incomplete Rust recovery record ${packageKey}`);
    seen.add(packageKey);
    if (record.verified !== true || ![2, 3].includes(record.method) || !record.repository || !record.commit) {
      fail(`Rust recovery record ${packageKey} is missing verified source/method metadata`);
    }
    if (!Array.isArray(record.files) || record.files.length === 0) fail(`Rust recovery record ${packageKey} has no license files`);
    const artifactDirectory = path.resolve(repoRoot, 'third-party/license-texts', `${record.name}-${record.version}`);
    const provenancePath = path.join(artifactDirectory, 'PROVENANCE.md');
    let provenance;
    try {
      provenance = await readFile(provenancePath, 'utf8');
    } catch {
      fail(`Rust recovery record ${packageKey} has no PROVENANCE.md`);
    }
    if (!provenance.includes(`Recovery method: ${record.method}`) || !provenance.includes(record.commit)) {
      fail(`Rust recovery provenance does not record the method and package commit for ${packageKey}`);
    }
    if (record.method === 3 && !provenance.includes(REQUIRED_SPDX_RECONSTRUCTION_ANNOTATION)) {
      fail(`Rust recovery provenance omits the required SPDX reconstruction annotation for ${packageKey}`);
    }

    for (const file of record.files) {
      if (typeof file.path !== 'string' || !file.path || path.isAbsolute(file.path)) fail(`Rust recovery file path is invalid for ${packageKey}`);
      const resolvedPath = path.resolve(artifactDirectory, file.path);
      if (!resolvedPath.startsWith(`${artifactDirectory}${path.sep}`)) fail(`Rust recovery file escapes its package directory: ${packageKey}/${file.path}`);
      if (!/^[a-f0-9]{64}$/i.test(file.sha256 ?? '') || !file.spdxId || !file.sourceUrl) {
        fail(`Rust recovery file lacks hash/license/source metadata: ${packageKey}/${file.path}`);
      }
      if (!allowList.has(file.spdxId)) fail(`Rust recovery record ${packageKey} uses disallowed license ${file.spdxId}`);
      let bytes;
      try {
        bytes = await readFile(resolvedPath);
      } catch {
        fail(`Rust recovery text is missing: ${packageKey}/${file.path}`);
      }
      if (sha256(bytes) !== file.sha256) fail(`Rust recovery text hash mismatch: ${packageKey}/${file.path}`);
      if (!provenance.includes(file.sha256) || !provenance.includes(file.sourceUrl)) {
        fail(`Rust recovery provenance does not pin ${packageKey}/${file.path}`);
      }
      if (record.method === 3) {
        const source = record.repositoryEvidence?.canonicalSpdxSource;
        if (!source?.repository || !source.commit || !source.path || !source.sha256 || source.reconstructedSha256 !== file.sha256) {
          fail(`Rust recovery record ${packageKey} does not separate canonical SPDX and reconstructed text hashes`);
        }
      }
    }

    const policyNotice = record.repositoryEvidence?.upstreamPolicyNotice;
    if (policyNotice) {
      const noticePath = path.resolve(artifactDirectory, policyNotice.path ?? '');
      if (!noticePath.startsWith(`${artifactDirectory}${path.sep}`)) fail(`Rust NOTICE escapes its package directory: ${packageKey}`);
      const bytes = await readFile(noticePath);
      if (sha256(bytes) !== policyNotice.sha256) fail(`Rust upstream NOTICE hash mismatch: ${packageKey}`);
      const notice = supplementalByHash.get(policyNotice.sha256) ?? {
        text: bytes.toString('utf8'),
        sha256: policyNotice.sha256,
        sources: [],
      };
      notice.sources.push({
        label: `${packageKey} upstream license policy notice`,
        url: policyNotice.sourceUrl,
        path: policyNotice.path,
      });
      supplementalByHash.set(policyNotice.sha256, notice);
    }
  }
  return {
    records: ledger.packages,
    supplementalNotices: [...supplementalByHash.values()],
  };
}

function validateFrontendData(frontend, allowList) {
  if (!frontend || !Array.isArray(frontend.packages) || frontend.packages.length === 0) {
    fail('frontend license collector returned no npm packages');
  }
  const allowed = new Set([...allowList, ...FRONTEND_LICENSE_EXCEPTIONS]);
  const noticeSources = [];
  for (const packageInfo of frontend.packages) {
    if (!packageInfo.name || !packageInfo.version || !packageInfo.repository || !packageInfo.licenseChoice || !packageInfo.license) {
      fail(`npm package is missing name/version/license/repository provenance: ${JSON.stringify(packageInfo)}`);
    }
    const declaredIds = parseLicenseIds(packageInfo.license);
    const selectedIds = parseLicenseIds(packageInfo.licenseChoice);
    if (declaredIds.length === 0) fail(`npm package ${packageInfo.name}@${packageInfo.version} has an invalid declared license expression`);
    if (selectedIds.length === 0) fail(`npm package ${packageInfo.name}@${packageInfo.version} has an invalid license choice`);
    if (selectedIds.some(id => !declaredIds.includes(id))) {
      fail(`npm package ${packageInfo.name}@${packageInfo.version} selected a license outside its declared expression`);
    }
    for (const id of selectedIds) {
      if (!allowed.has(id)) fail(`npm package ${packageInfo.name}@${packageInfo.version} uses disallowed license ${id}`);
    }
    const files = packageInfo.licenseFiles ?? [];
    if (files.length === 0) {
      fail(`npm package ${packageInfo.name}@${packageInfo.version} has no actual license text files`);
    }
    const coveredIds = new Set();
    for (const licenseFile of files) {
      const actualText = text(licenseFile.text, `npm ${packageInfo.name}@${packageInfo.version} ${licenseFile.path ?? 'license file'}`);
      if (typeof licenseFile.path !== 'string' || !licenseFile.path.trim()) {
        fail(`npm package ${packageInfo.name}@${packageInfo.version} license file has no source path`);
      }
      if (!/^[a-f0-9]{64}$/i.test(licenseFile.sha256 ?? '') || sha256(actualText) !== licenseFile.sha256) {
        fail(`npm package ${packageInfo.name}@${packageInfo.version} license file hash mismatch: ${licenseFile.path ?? 'unknown path'}`);
      }
      const candidateIds = parseLicenseIds(licenseFile.spdxIds?.length
        ? licenseFile.spdxIds
        : licenseFile.spdxId ?? licenseFile.license);
      if (candidateIds.length === 0) {
        fail(`npm package ${packageInfo.name}@${packageInfo.version} license file has no recognized SPDX identifier: ${licenseFile.path}`);
      }
      for (const id of candidateIds) {
        if (!allowed.has(id)) fail(`npm package ${packageInfo.name}@${packageInfo.version} has disallowed license text ${id}`);
        coveredIds.add(id);
      }
      noticeSources.push({
        text: actualText,
        spdxId: candidateIds.join(' / '),
        packageInfo,
        path: licenseFile.path,
        sourceUrl: licenseFile.sourceUrl,
        sha256: licenseFile.sha256,
      });
    }
    for (const id of selectedIds) {
      if (!coveredIds.has(id)) {
        fail(`npm package ${packageInfo.name}@${packageInfo.version} is missing full text for selected license ${id}`);
      }
    }
  }

  for (const font of frontend.fonts ?? []) {
    const fontLicense = font.license ?? 'OFL-1.1';
    for (const id of parseLicenseIds(fontLicense)) {
      if (!allowed.has(id)) fail(`font ${font.package ?? font.packageName ?? font.fontFile} uses disallowed license ${id}`);
    }
    const fontText = text(font.licenseText, `font ${font.package ?? font.packageName ?? font.fontFile}`);
    const fontPackage = frontend.packages.find(packageInfo => packageInfo.name === (font.package ?? font.packageName) && packageInfo.version === font.version);
    if (!fontPackage) fail(`font asset has no matching npm package audit record: ${font.package ?? font.packageName}@${font.version}`);
    const fontLicenseSource = fontPackage.licenseFiles.find(licenseFile =>
      path.posix.join('node_modules', font.package ?? font.packageName, licenseFile.path) === font.licenseSourcePath,
    );
    if (!font.licenseSha256 || fontLicenseSource?.sha256 !== font.licenseSha256 || fontPackage.licenseText !== fontText) {
      fail(`font license source/hash mismatch for ${font.filename ?? font.fontFile ?? font.package}`);
    }
    if (!font.version || !(font.filename ?? font.fontFile) || !font.sourcePath || !font.sourceSha256 || !(font.emittedSha256 ?? font.assetSha256)) {
      fail(`font ${font.package ?? font.packageName ?? font.fontFile} is missing version, asset path, or emitted/source hash provenance`);
    }
  }

  for (const fallback of frontend.fallbacks ?? []) {
    if (!Array.isArray(fallback.licenseFiles) || fallback.licenseFiles.length === 0) {
      fail(`fallback ${fallback.name ?? fallback.package ?? 'UI component'} has no full license text`);
    }
    const fallbackPackage = frontend.packages.find(packageInfo => packageInfo.name === fallback.name && packageInfo.version === fallback.version);
    if (!fallbackPackage) fail(`fallback ${fallback.name ?? 'UI component'} has no matching frontend package record`);
    for (const licenseFile of fallback.licenseFiles ?? []) {
      const installedText = fallbackPackage.licenseFiles.find(file => file.path === licenseFile.path && file.sha256 === licenseFile.sha256);
      if (!installedText) fail(`fallback license file is not backed by exact text in the package audit: ${licenseFile.path ?? 'unknown path'}`);
      const fallbackLicense = licenseFile.spdxId ?? licenseFile.license ?? fallbackPackage.licenseChoice;
      const fallbackIds = parseLicenseIds(licenseFile.spdxIds?.length ? licenseFile.spdxIds : fallbackLicense);
      if (fallbackIds.length === 0 || fallbackIds.some(id => !allowed.has(id))) {
        fail(`fallback ${fallback.name ?? 'UI component'} uses a missing or disallowed license`);
      }
    }
  }

  return noticeSources;
}

function addNotice(notices, { text: noticeText, spdxId, source }) {
  const normalized = text(noticeText, `${spdxId} license`);
  const hash = sha256(normalized);
  const notice = notices.get(hash) ?? { text: normalized, sources: [] };
  notice.sources.push({ spdxId, ...source });
  notices.set(hash, notice);
}

function markdownTableCell(value) {
  return String(value ?? '').replaceAll('|', '\\|').replaceAll('\n', ' ');
}

function portableLicensePath(sourcePath) {
  if (typeof sourcePath !== 'string' || !sourcePath) return null;
  const resolved = path.resolve(sourcePath);
  const relative = path.relative(root, resolved);
  if (relative === '' || relative === '..' || relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative)) return null;
  return relative.split(path.sep).join('/');
}

function renderLicenseText(notice) {
  const labels = [...new Set(notice.sources.map(source => source.spdxId))].sort();
  const fullText = notice.text.replace(/\r\n?/g, '\n').replace(/\n*$/, '\n');
  const rustGroups = new Map();
  const otherSources = new Map();
  for (const source of notice.sources) {
    const crate = source.label.match(/^Rust crate (.+)@([^@]+)$/);
    if (crate) {
      const key = `${crate[1]}|${source.url ?? ''}|${source.licensePath ?? ''}`;
      const group = rustGroups.get(key) ?? {
        name: crate[1],
        url: source.url,
        licensePath: source.licensePath,
        versions: new Map(),
      };
      group.versions.set(crate[2], source.archiveUrl);
      rustGroups.set(key, group);
    } else {
      otherSources.set(`${source.label}|${source.url}|${source.licensePath}`, source);
    }
  }
  const sourceLines = [
    ...[...rustGroups.values()].map(group => ({
      sortKey: group.name,
      render: () => {
        const packageSource = group.url ? `[Rust crate ${group.name}](${group.url})` : `Rust crate ${group.name}`;
        const versions = [...group.versions].sort(([left], [right]) => left.localeCompare(right))
          .map(([version, archiveUrl]) => archiveUrl ? `[${version}](${archiveUrl})` : version)
          .join(', ');
        const licenseFile = group.licensePath ? `; license text file: \`${group.licensePath}\`` : '';
        return `- ${packageSource}; versions: ${versions}${licenseFile}`;
      },
    })),
    ...[...otherSources.values()].map(source => ({
      sortKey: source.label,
      render: () => {
        const packageSource = /^https?:\/\//.test(source.url ?? '')
          ? `[${source.label}](${source.url})`
          : `\`${source.label}\``;
        const licenseFile = source.licensePath ? `; license text file: \`${source.licensePath}\`` : '';
        const archive = source.archiveUrl ? `; versioned crate archive: [${source.label}](${source.archiveUrl})` : '';
        return `- ${packageSource}${licenseFile}${archive}`;
      },
    })),
  ].sort((left, right) => left.sortKey.localeCompare(right.sortKey)).map(item => item.render());
  return [
    `### ${labels.join(' / ')}`,
    '',
    `Source text SHA-256 (raw file bytes): \`${sha256(notice.text)}\`.`,
    '',
    'Applies to:',
    ...sourceLines,
    '',
    '````text',
    fullText,
    '````',
    '',
  ].join('\n');
}

export function renderThirdPartyLicenses({ rust, frontend, optionalTools, allowList, rustFallbacks = { records: [], supplementalNotices: [] } }) {
  const rustNotices = validateRustData(rust, allowList);
  const frontendNotices = validateFrontendData(frontend, allowList);
  const notices = new Map();

  for (const [hash, licenseNotice] of rustNotices) {
    for (const crate of licenseNotice.crates ?? []) {
      const label = `Rust crate ${crate.name}@${crate.version}`;
      addNotice(notices, {
        text: licenseNotice.text,
        spdxId: licenseNotice.spdxId,
        source: {
          label,
          url: crate.repository,
          licensePath: portableLicensePath(licenseNotice.sourcePath),
          archiveUrl: `https://crates.io/api/v1/crates/${encodeURIComponent(crate.name)}/${encodeURIComponent(crate.version)}/download`,
        },
      });
    }
  }

  for (const noticeSource of frontendNotices) {
    const packageInfo = noticeSource.packageInfo;
    addNotice(notices, {
      text: noticeSource.text,
      spdxId: noticeSource.spdxId,
      source: {
        label: `${packageInfo.name}@${packageInfo.version}`,
        url: noticeSource.sourceUrl ?? packageInfo.repository,
        licensePath: noticeSource.path,
      },
    });
  }

  const rustPackages = [...rust.packages].sort((a, b) => `${a.name}@${a.version}`.localeCompare(`${b.name}@${b.version}`));
  const npmPackages = [...frontend.packages].sort((a, b) => `${a.name}@${a.version}`.localeCompare(`${b.name}@${b.version}`));
  const lines = [
    '# Third-party license notices',
    '',
    'This file contains the license texts and package provenance for Bergamotto’s Rust and frontend dependencies, fonts, and shipped fallback UI files. Regenerate it with `node scripts/generate-third-party-licenses.mjs` after reviewing dependency changes.',
    '',
    '## Rust dependencies',
    '',
    `Collected with cargo-about ${rust.tool.version} for: ${rust.scope.targets.join(', ')}. The collector includes workspace dependency graphs with all features, development/build dependencies, and exact locked source metadata.`,
    `The checked-in about.hbs template is rendered by cargo-about for: ${(rust.aboutMarkdownTargets ?? rust.scope.targets).join(', ')}; its text blocks are verified against cargo-about’s raw union JSON output. The full license bodies below come from independently hash-verified package or pinned upstream source files, so cargo-about’s reflowed template text never replaces source-backed license bytes.`,
    '',
    '| Crate | Version | Selected SPDX license IDs | Targets | Authors / copyright metadata | Source |',
    '| --- | --- | --- | --- | --- | --- |',
  ];

  for (const packageInfo of rustPackages) {
    const licenseIds = [...packageInfo.selectedLicenseIds].sort().join(' AND ');
    const repository = packageInfo.repository;
    const authors = packageInfo.authors?.length ? packageInfo.authors.join('; ') : 'Not listed in Cargo metadata; copyright notices remain verbatim in the license text.';
    lines.push(`| ${markdownTableCell(packageInfo.name)} | ${markdownTableCell(packageInfo.version)} | ${markdownTableCell(licenseIds)} | ${markdownTableCell(packageInfo.targets.join(', '))} | ${markdownTableCell(authors)} | ${repository ? `[source](${repository})` : markdownTableCell(packageInfo.source)} |`);
  }

  const rustOrChoices = rustPackages.filter(packageInfo => /\bOR\b|[A-Za-z0-9.+-]+\/[A-Za-z0-9.+-]+/.test(packageInfo.declaredLicense ?? ''));
  lines.push('', '### Resolved Rust license alternatives', '');
  if (rustOrChoices.length === 0) {
    lines.push('No third-party crate declared an SPDX `OR` license expression in the collected target graphs.');
  } else {
    lines.push('For crates declaring alternatives, the listed SPDX IDs are the branch or branches cargo-about resolved from actual source license files for the listed target set:');
    for (const packageInfo of rustOrChoices) {
      lines.push(`- \`${packageInfo.name}@${packageInfo.version}\`: declared \`${packageInfo.declaredLicense}\`; selected \`${[...packageInfo.selectedLicenseIds].sort().join(', ')}\` for ${packageInfo.targets.join(', ')}.`);
    }
  }

  if (rustFallbacks.records?.length) {
    const methodCounts = new Map();
    for (const record of rustFallbacks.records) methodCounts.set(record.method, (methodCounts.get(record.method) ?? 0) + 1);
    lines.push('', '### Verified Rust source-text recovery records', '');
    lines.push(`The ledger covers ${rustFallbacks.records.length} pinned recovery records (${methodCounts.get(2) ?? 0} verbatim exact-commit texts; ${methodCounts.get(3) ?? 0} canonical SPDX reconstructions). Every text hash and provenance document is revalidated during generation.`, '');
    lines.push('| Crate | Version | Method | Verified | Selected branch | Upstream source commit | License text sources | Provenance | Required method-3 annotation |', '| --- | --- | --- | --- | --- | --- | --- | --- | --- |');
    const rustPackageByKey = new Map(rust.packages.map(packageInfo => [`${packageInfo.name}@${packageInfo.version}`, packageInfo]));
    for (const record of [...rustFallbacks.records].sort((a, b) => `${a.name}@${a.version}`.localeCompare(`${b.name}@${b.version}`))) {
      const rustPackage = rustPackageByKey.get(`${record.name}@${record.version}`);
      const selected = rustPackage?.selectedLicenseIds?.join(', ') ?? record.selectedLicenseIds.join(', ');
      const sources = record.files.map(file => `[${file.spdxId}](${file.sourceUrl})`).join('<br>');
      const provenancePath = `third-party/license-texts/${record.name}-${record.version}/PROVENANCE.md`;
      const archiveUrl = `https://crates.io/api/v1/crates/${encodeURIComponent(record.name)}/${encodeURIComponent(record.version)}/download`;
      const sourceCommit = `[${record.repository}@${record.commit}](${record.repository.replace(/\.git$/, '')}/commit/${record.commit})`;
      const annotation = record.method === 3
        ? REQUIRED_SPDX_RECONSTRUCTION_ANNOTATION
        : 'Full license text copied verbatim from the pinned upstream source commit.';
      lines.push(`| ${markdownTableCell(record.name)} | ${markdownTableCell(record.version)} | ${record.method} | ${record.verified ? 'yes' : 'no'} | ${markdownTableCell(selected)} | ${sourceCommit}<br>[crates.io archive](${archiveUrl}) | ${sources} | [PROVENANCE.md](${provenancePath}) | ${markdownTableCell(annotation)} |`);
    }
  }

  if (rustFallbacks.supplementalNotices?.length) {
    lines.push('', '### Supplemental upstream policy notices', '', 'These notices accompany the selected license texts. They are included verbatim as notices and are not treated as license bodies.', '');
    for (const notice of rustFallbacks.supplementalNotices) {
      const appliesTo = notice.sources.map(source => {
        const origin = source.url ? `[${source.label}](${source.url})` : source.label;
        return `- ${origin}; local copy: \`${source.path}\``;
      });
      lines.push(`#### Upstream notice — SHA-256 \`${notice.sha256}\``, '', 'Applies to:', ...appliesTo, '', '````markdown', notice.text.replace(/\r\n?/g, '\n').replace(/\n*$/, '\n'), '````', '');
    }
  }

  const featuredLicenseIds = [
    { id: 'MPL-2.0', title: 'MPL-2.0 — selectors' },
    { id: 'CDLA-Permissive-2.0', title: 'CDLA-Permissive-2.0 — webpki-roots' },
    { id: 'Unicode-3.0', title: 'Unicode-3.0' },
  ];
  const featuredTextHashes = new Set();
  const featuredMatches = featuredLicenseIds.map(feature => ({
    ...feature,
    packages: rustPackages.filter(packageInfo => packageInfo.selectedLicenseIds.includes(feature.id)),
    notices: [...notices.values()].filter(notice => notice.sources.some(source =>
      source.label.startsWith('Rust crate ') && parseLicenseIds(source.spdxId).includes(feature.id),
    )),
  }));
  if (featuredMatches.some(feature => feature.packages.length || feature.notices.length)) {
    lines.push('', '## Required Rust license text sections', '', 'These higher visibility license texts are shown once here with their exact crate versions and versioned crates.io archive links. They are omitted from the generic text list below to avoid duplicate license bodies.', '');
    for (const feature of featuredMatches) {
      if (!feature.packages.length && !feature.notices.length) continue;
      lines.push(`### ${feature.title}`, '');
      if (feature.packages.length) {
        lines.push('| Crate | Versions and versioned source archives | Selected SPDX branch | Repository |', '| --- | --- | --- | --- |');
        const groups = new Map();
        for (const packageInfo of feature.packages) {
          const key = `${packageInfo.name}|${packageInfo.repository ?? packageInfo.source ?? ''}`;
          const group = groups.get(key) ?? { name: packageInfo.name, repository: packageInfo.repository, source: packageInfo.source, versions: [] };
          group.versions.push(packageInfo.version);
          groups.set(key, group);
        }
        for (const group of [...groups.values()].sort((left, right) => left.name.localeCompare(right.name) || String(left.repository).localeCompare(String(right.repository)))) {
          const versions = [...new Set(group.versions)].sort((left, right) => left.localeCompare(right)).map(version => {
            const archiveUrl = `https://crates.io/api/v1/crates/${encodeURIComponent(group.name)}/${encodeURIComponent(version)}/download`;
            return `[${version}](${archiveUrl})`;
          }).join(', ');
          const repository = group.repository ? `[repository](${group.repository})` : markdownTableCell(group.source);
          lines.push(`| ${markdownTableCell(group.name)} | ${versions} | ${markdownTableCell(feature.id)} | ${repository} |`);
        }
        lines.push('');
      }
      for (const notice of feature.notices) {
        const hash = sha256(notice.text);
        if (featuredTextHashes.has(hash)) {
          lines.push(`License text SHA-256 \`${hash}\` is included in the preceding featured section.`, '');
          continue;
        }
        featuredTextHashes.add(hash);
        lines.push(renderLicenseText(notice));
      }
    }
  }

  lines.push('', '## Frontend dependencies', '', 'npm package records include the package version, full declared license expression, selected branch, repository and author/copyright metadata, and complete source-backed license files below.', '', '| Package | Version | Declared license expression | Selected branch | Authors / copyright metadata | Repository |', '| --- | --- | --- | --- | --- | --- |');
  for (const packageInfo of npmPackages) {
    const authors = packageInfo.copyright ?? packageInfo.authors ?? 'Not listed in package metadata; copyright notices remain verbatim in the license text.';
    lines.push(`| ${markdownTableCell(packageInfo.name)} | ${markdownTableCell(packageInfo.version)} | ${markdownTableCell(packageInfo.license)} | ${markdownTableCell(packageInfo.licenseChoice)} | ${markdownTableCell(Array.isArray(authors) ? authors.join('; ') : authors)} | [source](${packageInfo.repository}) |`);
  }

  lines.push('', '### Vite bundle coverage evidence', '');
  lines.push(`The frontend collector records ${frontend.bundle?.moduleIds?.length ?? 0} modules and ${frontend.bundle?.assets?.length ?? 0} output assets from the production bundle. Bundled-but-not-production records: ${frontend.bundledNotProduction?.length ?? 0}; production-but-not-bundled records: ${frontend.productionNotBundled?.length ?? 0}. Full byte-for-byte dist evidence is in [FRONTEND_LICENSE_AUDIT.md](third-party/FRONTEND_LICENSE_AUDIT.md).`);
  for (const record of frontend.bundledNotProduction ?? []) {
    lines.push(`- Bundled but not marked production: \`${record.name}@${record.version}\` — ${record.reason}.`);
  }
  for (const record of frontend.productionNotBundled ?? []) {
    lines.push(`- Production dependency not present in bundle: \`${record.name}@${record.version}\` — ${record.reason}.`);
  }

  if (frontend.fonts?.length) {
    lines.push('', '## Font assets', '');
    for (const font of frontend.fonts) {
      lines.push(`- **${font.package ?? font.packageName} ${font.version}:** ${font.filename ?? font.fontFile}; emitted/source SHA-256 \`${font.emittedSha256 ?? font.assetSha256 ?? 'not supplied'}\` / \`${font.sourceSha256 ?? 'not supplied'}\`; source \`${font.sourcePath ?? 'not supplied'}\`; ${font.license ?? 'OFL-1.1'} text from \`${font.licenseSourcePath ?? 'not supplied'}\` (SHA-256 \`${font.licenseSha256 ?? 'not supplied'}\`); attribution: ${font.attribution ?? 'not supplied'}.`);
    }
  }

  if (frontend.fallbacks?.length) {
    lines.push('', '## Shipped fallback UI files', '');
    for (const fallback of frontend.fallbacks) {
      const annotation = fallback.requiredAnnotation ?? fallback.annotation;
      lines.push(`- **${fallback.name ?? fallback.package}:** ${fallback.method ?? 'shipped fallback'}; status ${fallback.verificationStatus ?? fallback.status ?? 'not supplied'}; exact source ${fallback.repository ?? fallback.url ?? 'not supplied'}, tag \`${fallback.tag ?? 'not supplied'}\`, commit \`${fallback.commit ?? 'not supplied'}\`; files ${(fallback.licenseFiles ?? fallback.files ?? []).map(file => `\`${file.path ?? file.name}\` (${file.sha256 ?? 'hash not supplied'}; ${file.license ?? file.spdxId ?? 'license not supplied'})`).join(', ')}; SPDX reconstruction: ${fallback.spdxReconstructionAnnotation ?? (annotation ?? 'not used')}.${fallback.provenancePath ? ` Provenance record: \`${fallback.provenancePath}\`.` : ''}`);
    }
  }

  lines.push('', '## Optional tools downloaded on demand', '', 'These tools are downloaded only on demand after user consent. Their archives and executables are not included in Bergamotto installers. The listed FFmpeg downloads are GPL builds verified with `--enable-gpl` and `--enable-version3`. Python archives carry additional third-party notices in their `licenses/` folder. Versions, effective license findings, and license/source URLs below come directly from `shared/managed-tools.json`; evidence and scope notes are in [OPTIONAL_TOOLS_LICENSE_AUDIT.md](third-party/OPTIONAL_TOOLS_LICENSE_AUDIT.md).', '', '| Tool | Platform | Version | Effective license | License / source |', '| --- | --- | --- | --- | --- |');
  for (const tool of optionalTools.tools) {
    for (const asset of tool.assets) {
      lines.push(`| ${markdownTableCell(tool.name)} | ${markdownTableCell(asset.platform)} | ${markdownTableCell(asset.version)} | ${markdownTableCell(asset.license)} | [license](${tool.licenseUrl}) · [source](${tool.sourceUrl}) |`);
    }
  }
  lines.push('', '## Full license texts', '');
  for (const notice of [...notices.values()].filter(notice => !featuredTextHashes.has(sha256(notice.text))).sort((a, b) => {
    const labelA = [...new Set(a.sources.map(source => source.spdxId))].sort().join('/');
    const labelB = [...new Set(b.sources.map(source => source.spdxId))].sort().join('/');
    return labelA.localeCompare(labelB) || sha256(a.text).localeCompare(sha256(b.text));
  })) {
    lines.push(renderLicenseText(notice));
  }

  return lines.join('\n');
}

async function writeAtomically(filePath, content) {
  const temporaryPath = `${filePath}.tmp-${process.pid}`;
  try {
    await writeFile(temporaryPath, content, 'utf8');
    await rename(temporaryPath, filePath);
  } catch (error) {
    await rm(temporaryPath, { force: true });
    throw error;
  }
}

async function rustLicenseData() {
  try {
    const { stdout } = await execFile('python3', ['scripts/rust-license-data.py'], {
      cwd: root,
      maxBuffer: 64 * 1024 * 1024,
      encoding: 'utf8',
    });
    return JSON.parse(stdout);
  } catch (error) {
    fail(`Rust license collector failed. Install cargo-about ${EXPECTED_CARGO_ABOUT_VERSION} with its CLI feature, then run this command again. ${error.message}`);
  }
}

export async function generateThirdPartyLicenses({ repoRoot = root } = {}) {
  const denyConfig = await readFile(path.join(repoRoot, 'deny.toml'), 'utf8');
  const allowList = parseCargoDenyAllowList(denyConfig);
  const [rust, frontend, optionalTools, rustFallbacks] = await Promise.all([
    rustLicenseData(),
    collectFrontendLicenses({ root: repoRoot }),
    collectOptionalToolLicenses({ root: repoRoot }),
    loadRustFallbackData(repoRoot, allowList),
  ]);
  const contents = renderThirdPartyLicenses({ rust, frontend, optionalTools, allowList, rustFallbacks });
  const frontendAudit = renderFrontendLicenseAudit(frontend);
  if (typeof frontendAudit !== 'string' || !frontendAudit.trim()) fail('frontend license auditor returned an empty audit report');
  await writeAtomically(path.join(repoRoot, 'THIRD_PARTY_LICENSES'), contents);
  await writeAtomically(path.join(repoRoot, 'third-party/OPTIONAL_TOOLS_LICENSE_AUDIT.md'), optionalTools.auditMarkdown);
  await writeAtomically(path.join(repoRoot, 'third-party/FRONTEND_LICENSE_AUDIT.md'), frontendAudit);
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  generateThirdPartyLicenses().catch(error => {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  });
}
