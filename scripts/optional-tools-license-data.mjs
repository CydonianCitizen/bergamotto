import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import path from 'node:path';

const VERIFICATION_FILE = 'third-party/optional-tools-verification.json';

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function fail(message) {
  throw new Error(`Optional-tool license evidence: ${message}`);
}

function escapeCell(value) {
  return String(value).replaceAll('|', '\\|').replaceAll('\n', ' ');
}

function isHttpsUrl(value) {
  try {
    return new URL(value).protocol === 'https:';
  } catch {
    return false;
  }
}

/**
 * Read the live managed-tools manifest and the version-independent evidence
 * ledger. Versions, download URLs, and license/source URLs are always taken
 * from shared/managed-tools.json; the evidence ledger only records what was
 * actually checked and where the supporting report is pinned.
 */
export async function collectOptionalToolLicenses({ root = process.cwd() } = {}) {
  const manifestPath = path.join(root, 'shared/managed-tools.json');
  const verificationPath = path.join(root, VERIFICATION_FILE);
  const manifestBytes = await readFile(manifestPath);
  const verificationBytes = await readFile(verificationPath);
  const manifest = JSON.parse(manifestBytes.toString('utf8'));
  const verification = JSON.parse(verificationBytes.toString('utf8'));

  if (verification.schemaVersion !== 1) fail('unsupported verification schema version');
  if (verification.sources?.['shared/managed-tools.json'] !== sha256(manifestBytes)) {
    fail('shared/managed-tools.json changed; review and refresh the verification ledger before regenerating');
  }

  const sourceContents = new Map();
  for (const [relativePath, expectedHash] of Object.entries(verification.sources ?? {})) {
    const bytes = relativePath === 'shared/managed-tools.json'
      ? manifestBytes
      : await readFile(path.join(root, relativePath));
    const actualHash = sha256(bytes);
    if (actualHash !== expectedHash) {
      fail(`${relativePath} changed (expected ${expectedHash}, found ${actualHash}); review and refresh the verification ledger`);
    }
    sourceContents.set(relativePath, bytes.toString('utf8'));
  }

  const manifestTools = manifest.tools ?? {};
  const evidenceTools = verification.tools ?? {};
  const manifestIds = Object.keys(manifestTools).sort();
  const evidenceIds = Object.keys(evidenceTools).sort();
  if (JSON.stringify(manifestIds) !== JSON.stringify(evidenceIds)) {
    fail(`tool inventory differs from ${VERIFICATION_FILE}; manifest has [${manifestIds.join(', ')}], ledger has [${evidenceIds.join(', ')}]`);
  }

  const tools = [];
  for (const id of manifestIds) {
    const definition = manifestTools[id];
    const evidence = evidenceTools[id];
    const platforms = definition.platforms ?? {};
    const platformKeys = Object.keys(platforms).sort();
    const evidencePlatformKeys = Object.keys(evidence.platforms ?? {}).sort();
    if (JSON.stringify(platformKeys) !== JSON.stringify(evidencePlatformKeys)) {
      fail(`${id} platform inventory differs from ${VERIFICATION_FILE}`);
    }

    const assets = [];
    for (const platform of platformKeys) {
      const asset = platforms[platform];
      const proof = evidence.platforms[platform];
      const source = sourceContents.get(proof.source);
      if (!source) fail(`${id}/${platform} references unpinned source ${proof.source}`);
      if (!proof.evidence || !proof.checkedOn || !proof.licenseBasis) {
        fail(`${id}/${platform} is missing evidence, checkedOn, or licenseBasis`);
      }
      if (!/^\d{4}-\d{2}-\d{2}$/.test(proof.checkedOn)) fail(`${id}/${platform} has an invalid evidence date`);
      if (id === 'ffmpeg' && proof.license !== 'GPL-3.0-or-later') {
        fail(`${id}/${platform} does not have evidence for the expected GPL-3.0-or-later build`);
      }
      if (id !== 'ffmpeg' && proof.license !== definition.license?.name) {
        fail(`${id}/${platform} license evidence disagrees with shared/managed-tools.json`);
      }
      for (const marker of proof.requiredMarkers ?? []) {
        if (!source.includes(marker)) fail(`${id}/${platform} evidence marker ${JSON.stringify(marker)} is absent from ${proof.source}`);
      }
      for (const marker of proof.forbiddenMarkers ?? []) {
        if (source.includes(marker)) fail(`${id}/${platform} forbidden evidence marker ${JSON.stringify(marker)} is present in ${proof.source}`);
      }
      if (!asset.version || !isHttpsUrl(asset.url) || !/^[a-f0-9]{64}$/i.test(asset.sha256 ?? '')) {
        fail(`${id}/${platform} manifest asset is missing its version, URL, or SHA-256`);
      }
      if (id === 'ffmpeg' && proof.source.endsWith('-configuration.json')) {
        const report = JSON.parse(source);
        if (
          report.platform !== platform
          || report.version !== asset.version
          || report.url !== asset.url
          || report.archiveSha256 !== asset.sha256
          || report.method !== 'configuration string extracted from verified archive; not executed'
          || !/^[a-f0-9]{64}$/i.test(report.binarySha256 ?? '')
          || !Array.isArray(report.configurations)
          || report.configurations.length === 0
        ) {
          fail(`${id}/${platform} extracted-configuration provenance does not match shared/managed-tools.json`);
        }
        for (const configuration of report.configurations) {
          if (!configuration.includes('--enable-gpl') || !configuration.includes('--enable-version3')) {
            fail(`${id}/${platform} extracted configuration does not establish GPLv3`);
          }
          if (configuration.includes('--enable-nonfree')) {
            fail(`${id}/${platform} extracted configuration enables nonfree components`);
          }
          for (const encoder of definition.verification?.requiredEncoders ?? []) {
            if (!configuration.includes(`--enable-${encoder}`)) {
              fail(`${id}/${platform} extracted configuration is missing required encoder ${encoder}`);
            }
          }
        }
      }
      assets.push({
        platform,
        version: asset.version,
        downloadUrl: asset.url,
        archiveSha256: asset.sha256,
        license: proof.license,
        evidence: proof.evidence,
        checkedOn: proof.checkedOn,
        source: proof.source,
        sourceSha256: verification.sources[proof.source],
        citation: proof.citation,
        licenseBasis: proof.licenseBasis,
        limits: proof.limits ?? null,
      });
    }

    if (!definition.name || !definition.license?.name || !isHttpsUrl(definition.license?.url) || !isHttpsUrl(definition.license?.sourceUrl)) {
      fail(`${id} manifest is missing its name or license/source URL metadata`);
    }
    tools.push({
      id,
      name: definition.name,
      description: definition.description,
      licenseName: definition.license.name,
      licenseUrl: definition.license.url,
      sourceUrl: definition.license.sourceUrl,
      assets,
      notes: evidence.notes ?? [],
    });
  }

  return {
    manifestSha256: sha256(manifestBytes),
    tools,
    auditMarkdown: renderOptionalToolsAudit({ manifestSha256: sha256(manifestBytes), tools }),
  };
}

export function renderOptionalToolsAudit({ manifestSha256, tools }) {
  const lines = [
    '# Optional managed-tool license audit',
    '',
    'This audit is regenerated from `shared/managed-tools.json` and the checked evidence ledger. Managed tools are offered only on demand after explicit user consent; their archives are not included in Bergamotto installers.',
    '',
    `Manifest SHA-256: \`${manifestSha256}\`.`,
    '',
    '## Pinned assets and license evidence',
    '',
    '| Tool | Platform | Manifest version | License evidence | Asset and provenance |',
    '| --- | --- | --- | --- | --- |',
  ];

  for (const tool of tools) {
    for (const asset of tool.assets) {
      const evidenceLink = `[${escapeCell(asset.citation)}](../${asset.source})`;
      const provenance = `[archive](${asset.downloadUrl}) · SHA-256 \`${asset.archiveSha256}\` · [upstream source](${tool.sourceUrl})`;
      lines.push(`| ${escapeCell(tool.name)} | ${escapeCell(asset.platform)} | ${escapeCell(asset.version)} | ${escapeCell(asset.license)}; checked ${escapeCell(asset.checkedOn)} via ${evidenceLink} | ${provenance} |`);
    }
  }

  for (const tool of tools) {
    lines.push('', `## ${tool.name}`, '', `- License metadata: [${tool.licenseName}](${tool.licenseUrl}).`, `- Upstream source: ${tool.sourceUrl}.`);
    for (const note of tool.notes) lines.push(`- ${note}`);
    for (const asset of tool.assets) {
      lines.push(`- **${asset.platform}, ${asset.version}:** ${asset.license}. ${asset.limits ?? ''} Evidence: ${asset.source} (${asset.citation}; SHA-256 \`${asset.sourceSha256}\`).`);
    }
  }

  const python = tools.find(tool => tool.id === 'python3');
  if (python) {
    lines.push('', 'When redistributing a managed Python build separately, include that build’s bundled third-party notices and license files in its `licenses/` folder. Bergamotto does not bundle these Python archives in its installers.');
  }

  lines.push('', '');
  return lines.join('\n');
}
