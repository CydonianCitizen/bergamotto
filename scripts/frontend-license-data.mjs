import { createHash } from 'node:crypto';
import { mkdtemp, readFile, readdir, rm, stat } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';

const ALLOWED_LICENSE_IDS = new Set([
  'Apache-2.0',
  'BSD-3-Clause',
  'ISC',
  'MIT',
  'OFL-1.1',
]);

const LICENSE_FILE_NAME = /^(?:license|licence|copying|unlicense|notice)(?:[._ -].*)?$/i;
const DIALOG_FALLBACK = {
  name: '@tauri-apps/plugin-dialog',
  version: '2.8.4',
  directory: 'tauri-apps-plugin-dialog-2.8.4',
  tag: 'dialog-js-v2.8.4',
  commit: '128190689dbe59b59d344e01bf07b00862aa3791',
  repository: 'https://github.com/tauri-apps/plugins-workspace',
  npmTarball: {
    url: 'https://registry.npmjs.org/@tauri-apps/plugin-dialog/-/plugin-dialog-2.8.4.tgz',
    sha256: '3eff363f336b8d62ee321bbf78b39988fd162e597c961f3ea1c00cf3de9ed791',
    integrity: 'sha512-+6ZOiXenOaxXwb8qCbBPdT2jX2cFPaVVHGIskrpmJLpmWj+qKRTD6X/z9x7hTdSQn7NedcoXCkWGej4a7dC4lw==',
    size: 6802,
    spdxFileSha256: 'eb8a6c84630461b352badcab1dbe5d0168c56d377358b2b8c86b51003272d5ef',
    packageJsonSha256: 'af8a8f5263c57545433dc83e79b88fad6f18c200657f7c6cc7ed6328eb7e8997',
  },
  files: [
    {
      name: 'LICENSE_MIT',
      license: 'MIT',
      url: 'https://github.com/tauri-apps/plugins-workspace/blob/dialog-js-v2.8.4/plugins/dialog/LICENSE_MIT',
      sha256: '89ff9689dcf9dd53968785d05a26f7898bb169dbfcada8d032b3e68cf0d55607',
    },
    {
      name: 'LICENSE_APACHE-2.0',
      license: 'Apache-2.0',
      url: 'https://github.com/tauri-apps/plugins-workspace/blob/dialog-js-v2.8.4/plugins/dialog/LICENSE_APACHE-2.0',
      sha256: '0cec06e0e55fbc3dc5cee4fca9b607f66cb8f4e4dbcf3b3c013594dd156732e9',
    },
  ],
};

const VERIFIED_DM_SANS_BINARY_NAMES = new Map([
  ['9fea608a947e67020c33cad9a6fe3d60c54119dfb8cff87768a8117a15ed7543', {
    family: 'DM Sans 9pt',
    fontVersion: 262406,
  }],
  ['a5d38fe99f930275684999b462c7123faa063d9e44e73b4b241723d884aa0f49', {
    family: 'DM Sans 9pt',
    fontVersion: 262406,
  }],
]);

function sha256(data) {
  return createHash('sha256').update(data).digest('hex');
}

function stableJson(value) {
  return JSON.stringify(value, null, 2);
}

function normalizeRepository(repository) {
  if (typeof repository === 'string') return repository;
  if (repository && typeof repository.url === 'string') return repository.url;
  return null;
}

function normalizeAuthors(authors, author) {
  const values = authors ?? author;
  if (values === undefined || values === null) return [];
  return (Array.isArray(values) ? values : [values]).map(value => {
    if (typeof value === 'string') return value;
    if (value && typeof value.name === 'string') return value.name;
    throw new Error(`Unsupported package author metadata: ${stableJson(value)}`);
  });
}

function parseLicenseExpression(value, name, version) {
  if (typeof value !== 'string' || value.trim() === '') {
    throw new Error(`${name}@${version} has no declared npm license expression`);
  }
  const tokens = value.split(/\s+OR\s+/i).map(token => token.trim());
  if (tokens.some(token => !ALLOWED_LICENSE_IDS.has(token))) {
    throw new Error(`${name}@${version} has unsupported or disallowed license expression: ${value}`);
  }
  if (tokens.length === 0 || new Set(tokens).size !== tokens.length) {
    throw new Error(`${name}@${version} has an invalid license expression: ${value}`);
  }
  return tokens;
}

async function findLicenseFiles(packageDirectory) {
  const candidates = [];
  const pending = [packageDirectory];

  while (pending.length > 0) {
    const directory = pending.pop();
    const entries = await readdir(directory, { withFileTypes: true });
    for (const entry of entries) {
      if (entry.name === '.git') continue;
      const entryPath = path.join(directory, entry.name);
      if (entry.isDirectory()) {
        pending.push(entryPath);
        continue;
      }
      if (!entry.isFile() || !LICENSE_FILE_NAME.test(entry.name) || /\.spdx$/i.test(entry.name)) continue;
      const bytes = await readFile(entryPath);
      const text = bytes.toString('utf8');
      if (text.trim() === '') throw new Error(`Empty license or notice file: ${entryPath}`);
      candidates.push({
        path: path.relative(packageDirectory, entryPath).split(path.sep).join('/'),
        sha256: sha256(bytes),
        text,
      });
    }
  }

  return candidates.sort((left, right) => left.path.localeCompare(right.path));
}

function licenseIdsInText(file) {
  const filename = path.basename(file.path).toLowerCase();
  const text = file.text;
  const result = new Set();

  if (/apache[-_ ]?2(?:\.0|_0)?/.test(filename) || /Apache License\s*Version 2\.0/i.test(text)) {
    result.add('Apache-2.0');
  }
  if (/\bmit\b/.test(filename) || /\bMIT License\b|The MIT License/i.test(text)) result.add('MIT');
  if (/\bisc\b/.test(filename) || /ISC License/i.test(text)) result.add('ISC');
  if (/\bofl\b/.test(filename) || /SIL OPEN FONT LICENSE Version 1\.1/i.test(text)) result.add('OFL-1.1');
  if (/bsd[-_ ]?3/.test(filename) || (/Redistribution and use in source and binary forms/i.test(text)
    && /(?:Neither the name of the copyright holder nor the names of its contributors|names of any contributors may not be used to endorse or promote)/i.test(text))) {
    result.add('BSD-3-Clause');
  }

  return result;
}

function annotateLicenseFiles(licenseFiles) {
  return licenseFiles.map(file => {
    const spdxIds = [...licenseIdsInText(file)].sort();
    return { ...file, spdxId: spdxIds[0] ?? null, spdxIds };
  });
}

function copyrightLines(licenseFiles) {
  const lines = new Set();
  for (const file of licenseFiles) {
    for (const line of file.text.split(/\r?\n/)) {
      if (/^\s*(?:Copyright\b|\(c\)\s*\d|©\s*\d)/i.test(line)) lines.add(line.trim());
    }
  }
  return [...lines].sort((left, right) => left.localeCompare(right));
}

function validateLicenseFiles(name, version, expressionTokens, licenseFiles) {
  if (licenseFiles.length === 0) {
    throw new Error(`${name}@${version} has license metadata but no actual license text`);
  }
  const identified = new Set(licenseFiles.flatMap(file => [...licenseIdsInText(file)]));
  const missing = expressionTokens.filter(token => !identified.has(token));
  if (missing.length > 0) {
    throw new Error(`${name}@${version} is missing actual license text for ${missing.join(', ')}`);
  }
}

async function verifyDialogFallback(root, packageJson, expressionTokens) {
  if (packageJson.name !== DIALOG_FALLBACK.name || packageJson.version !== DIALOG_FALLBACK.version) {
    throw new Error(`No verified license-text fallback is available for ${packageJson.name}@${packageJson.version}`);
  }
  if (expressionTokens.length !== 2 || !expressionTokens.includes('MIT') || !expressionTokens.includes('Apache-2.0')) {
    throw new Error(`${DIALOG_FALLBACK.name}@${DIALOG_FALLBACK.version} changed its declared license expression`);
  }

  const directory = path.join(root, 'third-party', 'license-texts', DIALOG_FALLBACK.directory);
  const provenancePath = path.join(directory, 'PROVENANCE.md');
  const provenance = await readFile(provenancePath, 'utf8');
  for (const marker of [
    DIALOG_FALLBACK.tag,
    DIALOG_FALLBACK.commit,
    DIALOG_FALLBACK.npmTarball.url,
    DIALOG_FALLBACK.npmTarball.sha256,
    DIALOG_FALLBACK.npmTarball.integrity,
    DIALOG_FALLBACK.npmTarball.size.toLocaleString('en-US'),
    DIALOG_FALLBACK.npmTarball.spdxFileSha256,
    DIALOG_FALLBACK.npmTarball.packageJsonSha256,
    'Fallback method:** 2',
    'Verification status:** verified',
  ]) {
    if (!provenance.includes(marker)) throw new Error(`Missing verified plugin-dialog fallback provenance marker: ${marker}`);
  }

  const licenseFiles = [];
  for (const expected of DIALOG_FALLBACK.files) {
    const bytes = await readFile(path.join(directory, expected.name));
    const digest = sha256(bytes);
    if (digest !== expected.sha256) {
      throw new Error(`${DIALOG_FALLBACK.name}@${DIALOG_FALLBACK.version} fallback text hash mismatch for ${expected.name}: ${digest}`);
    }
    licenseFiles.push({
      path: `third-party/license-texts/${DIALOG_FALLBACK.directory}/${expected.name}`,
      sha256: digest,
      text: bytes.toString('utf8'),
      license: expected.license,
      spdxId: expected.license,
      spdxIds: [expected.license],
      sourceUrl: expected.url,
      method: 'verified-exact-version-repository-tag',
    });
  }
  validateLicenseFiles(packageJson.name, packageJson.version, expressionTokens, licenseFiles);

  return {
    licenseFiles,
    record: {
      name: DIALOG_FALLBACK.name,
      version: DIALOG_FALLBACK.version,
      method: 'verified-exact-version-repository-tag',
      verificationStatus: 'verified',
      repository: DIALOG_FALLBACK.repository,
      tag: DIALOG_FALLBACK.tag,
      commit: DIALOG_FALLBACK.commit,
      checkedDate: '2026-11-16',
      licenseChoice: 'MIT',
      npmTarball: DIALOG_FALLBACK.npmTarball,
      licenseFiles: licenseFiles.map(({ path: licensePath, sha256: digest, license, sourceUrl }) => ({
        path: licensePath,
        sha256: digest,
        license,
        spdxId: license,
        sourceUrl,
      })),
      spdxReconstructionAnnotation: null,
      provenancePath: path.relative(root, provenancePath).split(path.sep).join('/'),
    },
  };
}

/**
 * Audit one installed package directory against its npm metadata and full legal texts.
 * SPDX-only templates are intentionally excluded and cannot satisfy this audit.
 */
export async function auditPackageDirectory(packageDirectory, { root = process.cwd() } = {}) {
  const packageJson = JSON.parse(await readFile(path.join(packageDirectory, 'package.json'), 'utf8'));
  const { name, version } = packageJson;
  if (typeof name !== 'string' || typeof version !== 'string') {
    throw new Error(`Invalid package metadata under ${packageDirectory}`);
  }
  const expressionTokens = parseLicenseExpression(packageJson.license, name, version);
  let licenseFiles = await findLicenseFiles(packageDirectory);
  let fallback = null;

  if (licenseFiles.length === 0) {
    if (name !== DIALOG_FALLBACK.name) {
      throw new Error(`${name}@${version} has license metadata but no actual license text`);
    }
    const verified = await verifyDialogFallback(root, packageJson, expressionTokens);
    licenseFiles = verified.licenseFiles;
    fallback = verified.record;
  } else {
    validateLicenseFiles(name, version, expressionTokens, licenseFiles);
    licenseFiles = annotateLicenseFiles(licenseFiles);
  }

  const licenseText = licenseFiles
    .map(file => file.text.trimEnd())
    .join('\n\n---\n\n');
  const chosenLicense = expressionTokens[0];
  const fallbackCopyright = fallback
    ? [
        ...copyrightLines(licenseFiles),
        '2019-2022, The Tauri Programme in the Commons Conservancy',
      ]
    : copyrightLines(licenseFiles);

  return {
    name,
    version,
    license: packageJson.license,
    repository: normalizeRepository(packageJson.repository),
    authors: normalizeAuthors(packageJson.authors, packageJson.author),
    copyright: fallbackCopyright,
    licenseChoice: chosenLicense,
    licenseText,
    licenseFiles,
    fallback,
  };
}

function lockPackageName(packagePath) {
  const segments = packagePath.split('/');
  const nodeModulesIndex = segments.lastIndexOf('node_modules');
  if (nodeModulesIndex < 0 || nodeModulesIndex + 1 >= segments.length) return null;
  const first = segments[nodeModulesIndex + 1];
  if (first.startsWith('@')) {
    const second = segments[nodeModulesIndex + 2];
    return second ? `${first}/${second}` : null;
  }
  return first;
}

async function productionPackagesFromLock(root) {
  const lock = JSON.parse(await readFile(path.join(root, 'package-lock.json'), 'utf8'));
  if (!lock.packages || typeof lock.packages !== 'object') {
    throw new Error('package-lock.json has no package map; cannot audit the production npm graph');
  }

  const records = [];
  for (const [packagePath, lockEntry] of Object.entries(lock.packages)) {
    if (!packagePath.startsWith('node_modules/') || lockEntry.dev === true) continue;
    const packageDirectory = path.join(root, packagePath);
    try {
      await stat(packageDirectory);
    } catch {
      if (lockEntry.optional === true) continue;
      throw new Error(`Production package is missing from node_modules: ${packagePath}`);
    }
    const packageJson = JSON.parse(await readFile(path.join(packageDirectory, 'package.json'), 'utf8'));
    if (packageJson.version !== lockEntry.version) {
      throw new Error(`Installed version mismatch for ${packagePath}: package-lock has ${lockEntry.version}, package has ${packageJson.version}`);
    }
    records.push({
      name: packageJson.name ?? lockPackageName(packagePath),
      version: packageJson.version,
      packagePath,
      packageDirectory,
      optional: lockEntry.optional === true,
    });
  }
  return records.sort((left, right) => `${left.name}@${left.version}`.localeCompare(`${right.name}@${right.version}`));
}

function packageFromModuleId(moduleId, root, packageRecords) {
  let modulePath = moduleId.replace(/^\0+/, '').split(/[?#]/, 1)[0];
  if (modulePath.startsWith('file://')) {
    modulePath = new URL(modulePath).pathname;
  }
  modulePath = modulePath.replaceAll('\\', '/');
  const segments = modulePath.split('/');
  const nodeModulesIndex = segments.lastIndexOf('node_modules');
  if (nodeModulesIndex < 0 || nodeModulesIndex + 1 >= segments.length) return null;

  const first = segments[nodeModulesIndex + 1];
  const packageSegments = first.startsWith('@')
    ? segments.slice(nodeModulesIndex + 1, nodeModulesIndex + 3)
    : [first];
  if (packageSegments.length !== (first.startsWith('@') ? 2 : 1)) return null;
  const packageName = packageSegments.join('/');
  const absolutePath = path.resolve(modulePath);
  const candidate = packageRecords
    .filter(record => record.name === packageName)
    .filter(record => {
      const directory = path.resolve(record.packageDirectory);
      return absolutePath === directory || absolutePath.startsWith(directory + path.sep);
    })
    .sort((left, right) => right.packageDirectory.length - left.packageDirectory.length)[0];
  return candidate ?? null;
}

function bufferForAsset(source) {
  return Buffer.isBuffer(source) ? source : Buffer.from(source);
}

async function runTemporaryProductionBuild(root) {
  const { build, loadConfigFromFile } = await import('vite');
  const configFile = path.join(root, 'vite.config.ts');
  const loaded = await loadConfigFromFile({ command: 'build', mode: 'production' }, configFile, root);
  if (!loaded) throw new Error(`Could not load Vite config at ${configFile}`);

  const tempOutputDirectory = await mkdtemp(path.join(os.tmpdir(), 'bergamotto-frontend-license-'));
  let generatedMetadata = null;
  const auditPlugin = {
    name: 'bergamotto-frontend-license-audit',
    enforce: 'post',
    generateBundle(_options, outputBundle) {
      const files = [];
      const modules = new Set();
      for (const item of Object.values(outputBundle)) {
        if (item.type === 'chunk') {
          for (const moduleId of Object.keys(item.modules)) modules.add(moduleId);
          files.push({
            fileName: item.fileName,
            type: 'chunk',
            isEntry: item.isEntry,
            isDynamicEntry: item.isDynamicEntry,
            imports: [...item.imports],
            dynamicImports: [...item.dynamicImports],
            modules: Object.keys(item.modules).sort(),
          });
        } else {
          files.push({
            fileName: item.fileName,
            type: item.fileName === 'index.html' ? 'html' : 'asset',
          });
        }
      }
      generatedMetadata = { files, modules: [...modules].sort() };
    },
  };

  try {
    await build({
      ...loaded.config,
      configFile: false,
      root,
      logLevel: 'silent',
      plugins: [...(loaded.config.plugins ?? []), auditPlugin],
      build: {
        ...loaded.config.build,
        outDir: tempOutputDirectory,
        write: true,
        emptyOutDir: true,
      },
    });
    if (!generatedMetadata) throw new Error('Vite completed without emitting a production bundle');
    const expectedNames = generatedMetadata.files.map(file => file.fileName).sort();
    const outputNames = await filesUnder(tempOutputDirectory);
    if (stableJson(expectedNames) !== stableJson(outputNames)) {
      throw new Error(`Temporary Vite output differs from its Rollup bundle graph (expected: ${expectedNames.join(', ')}; actual: ${outputNames.join(', ')})`);
    }
    const files = [];
    for (const metadata of generatedMetadata.files) {
      files.push({ ...metadata, bytes: await readFile(path.join(tempOutputDirectory, metadata.fileName)) });
    }
    return { files, modules: generatedMetadata.modules };
  } finally {
    await rm(tempOutputDirectory, { recursive: true, force: true });
  }
}

async function filesUnder(directory, relative = '') {
  const result = [];
  for (const entry of await readdir(path.join(directory, relative), { withFileTypes: true })) {
    const entryRelative = path.join(relative, entry.name);
    if (entry.isDirectory()) result.push(...await filesUnder(directory, entryRelative));
    else if (entry.isFile()) result.push(entryRelative.split(path.sep).join('/'));
  }
  return result.sort();
}

async function compareDistFiles(root, generated) {
  const distDirectory = path.join(root, 'dist');
  const expectedNames = generated.files.map(file => file.fileName).sort();
  const actualNames = await filesUnder(distDirectory).catch(error => {
    throw new Error(`Could not inspect the latest dist output at ${distDirectory}: ${error.message}`);
  });
  if (stableJson(expectedNames) !== stableJson(actualNames)) {
    const missing = expectedNames.filter(file => !actualNames.includes(file));
    const extra = actualNames.filter(file => !expectedNames.includes(file));
    throw new Error(`dist does not match the current temporary Vite build (missing: ${missing.join(', ') || 'none'}; extra: ${extra.join(', ') || 'none'}). Run npm run build before regenerating licenses.`);
  }

  const files = [];
  for (const file of generated.files) {
    const distPath = path.join(distDirectory, file.fileName);
    const diskBytes = await readFile(distPath);
    const generatedSha256 = sha256(file.bytes);
    const distSha256 = sha256(diskBytes);
    if (generatedSha256 !== distSha256) {
      throw new Error(`dist asset differs from the current temporary Vite build: ${file.fileName}`);
    }
    files.push({
      fileName: file.fileName,
      type: file.type,
      size: diskBytes.length,
      sha256: distSha256,
      inMemorySha256: generatedSha256,
      identical: true,
      modules: file.modules ?? [],
      imports: file.imports ?? [],
      dynamicImports: file.dynamicImports ?? [],
      isEntry: file.isEntry ?? false,
    });
  }

  return { files, outputDifferences: [] };
}

function findPackageLockEntry(root, packageName, records) {
  const matching = records.filter(record => record.name === packageName);
  if (matching.length === 1) return matching[0];
  if (matching.length > 1) {
    throw new Error(`Ambiguous installed package versions for emitted module ${packageName}: ${matching.map(record => record.version).join(', ')}`);
  }
  const packageDirectory = path.join(root, 'node_modules', packageName);
  return { name: packageName, packageDirectory, packagePath: path.relative(root, packageDirectory).split(path.sep).join('/') };
}

function tailwindCssEvidence(cssText) {
  const markers = [];
  const banner = cssText.match(/tailwindcss\s+v?([\d.]+)/i)?.[0] ?? null;
  if (banner) markers.push(`Tailwind version banner: ${banner}`);
  if (/@layer\s+properties/.test(cssText)) markers.push('generated Tailwind @layer properties block');
  if (/--tw-[\w-]+\s*:/.test(cssText)) markers.push('generated Tailwind --tw-* theme or utility properties');
  return { present: markers.length > 0, banner, markers };
}

function helperEvidence(moduleIds) {
  const helpers = [];
  for (const moduleId of moduleIds) {
    if (moduleId.includes('vite/modulepreload-polyfill')) helpers.push({ id: moduleId, type: 'Vite modulepreload polyfill' });
    else if (moduleId.includes('vite/preload-helper')) helpers.push({ id: moduleId, type: 'Vite dynamic-import preload helper' });
    else if (moduleId.includes('commonjsHelpers.js')) helpers.push({ id: moduleId, type: 'Vite/Rollup CommonJS helper' });
  }
  return helpers.sort((left, right) => left.id.localeCompare(right.id));
}

function extractCssFamily(cssText) {
  return cssText.match(/@font-face\s*\{[^}]*font-family:\s*([^;}]+)/i)?.[1]?.trim().replace(/^['"]|['"]$/g, '') ?? null;
}

function reservedNamesFromMetadata(metadata, licenseText) {
  const explicit = metadata.reservedFontNames ?? metadata.reservedNames;
  if (Array.isArray(explicit)) return explicit;
  const declaration = licenseText.match(/^Reserved Font Names?:\s*(.+)$/im);
  return declaration ? declaration[1].split(',').map(name => name.trim()).filter(Boolean) : [];
}

async function fontRecords(root, outputFiles, packageRecord, cssText) {
  const emittedFonts = outputFiles.filter(file => file.fileName.toLowerCase().endsWith('.woff2'));
  if (emittedFonts.length === 0) return [];

  const fontPackageDirectory = path.join(root, 'node_modules', '@fontsource-variable', 'dm-sans');
  const metadata = JSON.parse(await readFile(path.join(fontPackageDirectory, 'metadata.json'), 'utf8'));
  const sourceFiles = [];
  const pending = [path.join(fontPackageDirectory, 'files')];
  while (pending.length > 0) {
    const directory = pending.pop();
    for (const entry of await readdir(directory, { withFileTypes: true })) {
      const entryPath = path.join(directory, entry.name);
      if (entry.isDirectory()) pending.push(entryPath);
      else if (entry.isFile() && entry.name.toLowerCase().endsWith('.woff2')) {
        const bytes = await readFile(entryPath);
        sourceFiles.push({
          path: path.relative(root, entryPath).split(path.sep).join('/'),
          name: entry.name,
          bytes,
          sha256: sha256(bytes),
        });
      }
    }
  }

  const licenseSource = packageRecord.licenseFiles.find(file => /license/i.test(path.basename(file.path)));
  if (!licenseSource) throw new Error('DM Sans license file missing from package audit record');
  const cssFamily = extractCssFamily(cssText);
  const reservedFontNames = reservedNamesFromMetadata(metadata, packageRecord.licenseText);
  const records = [];
  for (const emitted of emittedFonts) {
    const emittedSha256 = sha256(emitted.bytes);
    const matches = sourceFiles.filter(source => source.sha256 === emittedSha256);
    if (matches.length !== 1) {
      throw new Error(`Emitted font ${emitted.fileName} matches ${matches.length} source WOFF2 files by SHA-256; expected exactly one`);
    }
    const source = matches[0];
    const binaryNameInspection = VERIFIED_DM_SANS_BINARY_NAMES.get(source.sha256);
    if (!binaryNameInspection) {
      throw new Error(`DM Sans source font ${source.name} has no checked binary name metadata for SHA-256 ${source.sha256}`);
    }
    records.push({
      package: packageRecord.name,
      version: packageRecord.version,
      filename: emitted.fileName,
      sourceFilename: source.name,
      sourcePath: source.path,
      sourceSha256: source.sha256,
      emittedSha256,
      sourceMatchesEmitted: true,
      license: packageRecord.license,
      licenseChoice: packageRecord.licenseChoice,
      licenseText: packageRecord.licenseText,
      licenseSha256: licenseSource.sha256,
      licenseSourcePath: `node_modules/@fontsource-variable/dm-sans/${licenseSource.path}`,
      attribution: metadata.license?.attribution ?? null,
      metadata: {
        family: metadata.family,
        familyId: metadata.id,
        version: metadata.version,
        source: metadata.source,
        cssFamily,
        binaryNameInspection: {
          tool: 'fontconfig 2.18.3 fc-query',
          inspectedSha256: source.sha256,
          ...binaryNameInspection,
        },
        reservedFontNames,
        reservedNamesEvidence: reservedFontNames.length > 0
          ? 'Found in font package metadata or a Reserved Font Names declaration in the license text.'
          : 'No specific Reserved Font Name declaration was found in font package metadata or license text.',
      },
    });
  }
  return records.sort((left, right) => left.filename.localeCompare(right.filename));
}

function byNameVersion(records) {
  return new Map(records.map(record => [`${record.name}@${record.version}`, record]));
}

function displayLicenseSources(record) {
  return record.licenseFiles.map(file => `[${file.spdxId ?? 'license text'}] ${file.path} (${file.sha256})`).join('<br>');
}

function markdownTable(headers, rows) {
  return [
    `| ${headers.join(' | ')} |`,
    `| ${headers.map(() => '---').join(' | ')} |`,
    ...rows.map(row => `| ${row.map(value => String(value ?? '').replaceAll('|', '\\|')).join(' | ')} |`),
  ].join('\n');
}

export function markdownCode(value) {
  const displayValue = String(value).replace(/[\u0000-\u001f\u007f-\u009f]/g, character =>
    `\\u${character.charCodeAt(0).toString(16).padStart(4, '0')}`,
  );
  return '\u0060' + displayValue.replaceAll('\u0060', '\\u0060') + '\u0060';
}

/**
 * Build a fresh Vite production graph into a temporary directory, verify it byte-for-byte against dist,
 * then audit all emitted npm packages, generated runtime helpers, CSS, and font assets.
 */
export async function collectFrontendLicenses({ root = process.cwd() } = {}) {
  root = path.resolve(root);
  const lockProduction = await productionPackagesFromLock(root);
  const allAudited = [];
  for (const record of lockProduction) {
    allAudited.push(await auditPackageDirectory(record.packageDirectory, { root }));
  }
  const allAuditedByKey = byNameVersion(allAudited);
  const generated = await runTemporaryProductionBuild(root);
  const { files: distFiles, outputDifferences } = await compareDistFiles(root, generated);
  const modulePackageKeys = new Set();
  const modulePackageNames = new Set();
  for (const moduleId of generated.modules) {
    const packageRecord = packageFromModuleId(moduleId, root, lockProduction);
    if (packageRecord) {
      modulePackageKeys.add(`${packageRecord.name}@${packageRecord.version}`);
      modulePackageNames.add(packageRecord.name);
    } else if (moduleId.includes('/node_modules/')) {
      const name = moduleId.match(/node_modules\/(?:((?:@[^/]+\/[^/]+))|([^/]+))/)?.slice(1).find(Boolean);
      if (name) {
        const matching = await findPackageLockEntry(root, name, lockProduction);
        if (!matching.packageDirectory) throw new Error(`Could not identify emitted npm package module: ${moduleId}`);
        const packageAudit = await auditPackageDirectory(matching.packageDirectory, { root });
        modulePackageKeys.add(`${packageAudit.name}@${packageAudit.version}`);
        modulePackageNames.add(packageAudit.name);
        allAuditedByKey.set(`${packageAudit.name}@${packageAudit.version}`, packageAudit);
      }
    }
  }

  const cssFiles = generated.files.filter(file => file.fileName.toLowerCase().endsWith('.css'));
  const cssEvidence = cssFiles.map(file => ({
    fileName: file.fileName,
    ...tailwindCssEvidence(file.bytes.toString('utf8')),
  }));
  const tailwindPresent = cssEvidence.some(item => item.present);
  const helpers = helperEvidence(generated.modules);
  const viteHelperPresent = helpers.length > 0;

  const devPackagesToInclude = [];
  if (tailwindPresent) {
    const tailwind = await auditPackageDirectory(path.join(root, 'node_modules', 'tailwindcss'), { root });
    allAuditedByKey.set(`${tailwind.name}@${tailwind.version}`, tailwind);
    devPackagesToInclude.push({
      key: `${tailwind.name}@${tailwind.version}`,
      reason: `The emitted CSS contains Tailwind-generated styles (${cssEvidence.flatMap(item => item.markers).join('; ')}).`,
    });
  }
  if (viteHelperPresent) {
    const vite = await auditPackageDirectory(path.join(root, 'node_modules', 'vite'), { root });
    allAuditedByKey.set(`${vite.name}@${vite.version}`, vite);
    devPackagesToInclude.push({
      key: `${vite.name}@${vite.version}`,
      reason: `The emitted JavaScript contains Vite/Rollup runtime helpers: ${helpers.map(item => item.type).join(', ')}.`,
    });
  }

  for (const key of modulePackageKeys) {
    if (!allAuditedByKey.has(key)) {
      const [name, version] = key.split('@').filter(Boolean);
      throw new Error(`No audited npm license record for emitted module package ${name}@${version}`);
    }
  }
  const bundledKeys = new Set([...modulePackageKeys, ...devPackagesToInclude.map(record => record.key)]);
  const productionKeys = new Set(lockProduction.map(record => `${record.name}@${record.version}`));
  const packages = [...bundledKeys]
    .map(key => allAuditedByKey.get(key))
    .filter(Boolean)
    .sort((left, right) => `${left.name}@${left.version}`.localeCompare(`${right.name}@${right.version}`));

  const bundledNotProduction = devPackagesToInclude
    .filter(record => !productionKeys.has(record.key))
    .map(record => {
      const packageRecord = allAuditedByKey.get(record.key);
      return { name: packageRecord.name, version: packageRecord.version, reason: record.reason };
    })
    .sort((left, right) => `${left.name}@${left.version}`.localeCompare(`${right.name}@${right.version}`));
  const productionNotBundled = lockProduction
    .filter(record => !modulePackageKeys.has(`${record.name}@${record.version}`))
    .map(record => {
      const audited = allAuditedByKey.get(`${record.name}@${record.version}`);
      return {
        name: record.name,
        version: record.version,
        license: audited.license,
        licenseChoice: audited.licenseChoice,
        licenseFiles: audited.licenseFiles.map(({ path: filePath, sha256: digest, spdxId }) => ({ path: filePath, sha256: digest, spdxId })),
        reason: 'Installed in the production npm graph but no module from this package appears in the emitted Vite bundle.',
      };
    })
    .sort((left, right) => `${left.name}@${left.version}`.localeCompare(`${right.name}@${right.version}`));

  const fontPackageEntry = lockProduction.find(record => record.name === '@fontsource-variable/dm-sans');
  const fontPackage = fontPackageEntry
    ? allAuditedByKey.get(`${fontPackageEntry.name}@${fontPackageEntry.version}`)
    : null;
  if (!fontPackage && generated.files.some(file => file.fileName.toLowerCase().endsWith('.woff2'))) {
    throw new Error('Emitted fonts have no matching @fontsource-variable/dm-sans production package record');
  }
  const allCssText = cssFiles.map(file => file.bytes.toString('utf8')).join('\n');
  const fonts = fontPackage ? await fontRecords(root, generated.files, fontPackage, allCssText) : [];
  const fallbacks = packages.map(record => record.fallback).filter(Boolean);
  const npmProductionPackages = lockProduction.map(record => ({ name: record.name, version: record.version }));
  const distOutputByName = new Map(distFiles.map(file => [file.fileName, file]));
  const assets = generated.files.map(file => ({ ...distOutputByName.get(file.fileName), modules: file.modules ?? [] }));
  const moduleIds = generated.modules;

  return {
    packages,
    bundledNotProduction,
    productionNotBundled,
    fonts,
    fallbacks,
    bundle: {
      source: 'Vite production build loaded from vite.config.ts and emitted only into a temporary directory',
      verifiedAgainstDist: true,
      moduleIds,
      modulePackageKeys: [...modulePackageKeys].sort(),
      assets,
      outputDifferences,
      cssEvidence,
      runtimeHelpers: helpers,
      npmProductionPackages,
      npmProductionPackageCount: npmProductionPackages.length,
      bundledPackageCount: packages.length,
      emittedFontCount: fonts.length,
    },
  };
}

/** Render the reproducible audit report; the caller owns writing the generated file. */
export function renderFrontendLicenseAudit(data) {
  const lines = [
    '# Frontend and font license audit',
    '',
    `Generated from a fresh programmatic Vite production build in a temporary directory. The configured Vite plugins were retained, and every emitted output file was compared byte-for-byte with ${markdownCode('dist')}: ${data.bundle.verifiedAgainstDist ? 'passed' : 'failed'} (${data.bundle.assets.length} files).`,
    '',
    `The package-lock production graph contains ${data.bundle.npmProductionPackageCount} installed packages. The emitted npm module graph contains ${data.bundle.modulePackageKeys.length} package versions, plus ${data.bundle.runtimeHelpers.length} bundler/runtime helpers and ${data.bundle.cssEvidence.filter(item => item.present).length} CSS asset(s) with Tailwind-generated content.`,
    '',
    '## License texts included in the shipped frontend',
    '',
    markdownTable(
      ['Package', 'Version', 'Declared license', 'Selected branch', 'Actual license files'],
      data.packages.map(record => [
        record.name,
        record.version,
        record.license,
        record.licenseChoice,
        displayLicenseSources(record),
      ]),
    ),
    '',
    'Every listed license file is copied in full from the installed npm package or the verified source fallback, hashed on each generation, and checked against its declared license expression. SPDX templates are not used as license text. Lucide React keeps its complete bundled license file, including the Feather MIT portion.',
    '',
    '## Bundle and production npm graph differences',
    '',
    '### Bundled code from development dependencies',
    '',
    data.bundledNotProduction.length === 0
      ? 'None.'
      : markdownTable(['Package', 'Version', 'Bundle evidence'], data.bundledNotProduction.map(record => [record.name, record.version, record.reason])),
    '',
    '### Production dependencies not present in the bundle module graph',
    '',
    data.productionNotBundled.length === 0
      ? 'None.'
      : markdownTable(
        ['Package', 'Version', 'Declared license', 'License text found', 'Reason'],
        data.productionNotBundled.map(record => [record.name, record.version, record.license, record.licenseFiles.map(file => `${file.path} (${file.sha256})`).join('<br>'), record.reason]),
      ),
    '',
    'Production-but-not-bundled packages were audited for a supported license expression and actual license text. Their license terms are not copied into the shipped notice because their code is not in the frontend output.',
    '',
    '## Output and CSS inspection',
    '',
    markdownTable(['Output file', 'Type', 'Bytes', 'SHA-256', 'Verification'], data.bundle.assets.map(file => [file.fileName, file.type, file.size, file.sha256, file.identical ? 'byte-for-byte match' : 'mismatch'])),
    '',
    ...(data.bundle.outputDifferences.length > 0
      ? data.bundle.outputDifferences.map(item => `- ${markdownCode(item.fileName)}: ${item.reason}; dist SHA-256 ${markdownCode(item.distSha256)}, in-memory SHA-256 ${markdownCode(item.inMemorySha256)}.`)
      : ['No output differences; all dist files matched the temporary build byte-for-byte.']),
    '',
    ...data.bundle.cssEvidence.flatMap(item => [
      `- ${markdownCode(item.fileName)}: ${item.present ? item.markers.join('; ') : 'no Tailwind-generated style markers found'}`,
    ]),
    '',
    `Runtime helpers observed in emitted module IDs: ${data.bundle.runtimeHelpers.length === 0 ? 'none' : data.bundle.runtimeHelpers.map(item => `${markdownCode(item.type)} (${markdownCode(item.id)})`).join(', ')}.`,
    '',
    '## DM Sans font assets',
    '',
    data.fonts.length === 0
      ? 'No WOFF2 font assets were emitted.'
      : markdownTable(
        ['Emitted font', 'Source file', 'Emitted SHA-256', 'Source SHA-256', 'Family / version / reserved-name check'],
        data.fonts.map(font => [
          font.filename,
          font.sourcePath,
          font.emittedSha256,
          font.sourceSha256,
          `${font.metadata.cssFamily ?? 'unknown CSS family'} / ${font.metadata.family} ${font.metadata.version}; binary ${font.metadata.binaryNameInspection.family} (${font.metadata.binaryNameInspection.fontVersion}) / ${font.metadata.reservedNamesEvidence}`,
        ]),
      ),
    '',
    data.fonts.length > 0
      ? `The emitted font files match the source WOFF2 files byte-for-byte. The Vite filename suffix is a content fingerprint, not a font-family rename: package metadata names ${data.fonts[0].metadata.family} (${data.fonts[0].metadata.version}), fc-query reports ${data.fonts[0].metadata.binaryNameInspection.family}, and CSS uses ${markdownCode(data.fonts[0].metadata.cssFamily)}. No specific Reserved Font Name declaration appears in the package metadata or OFL text. The complete OFL 1.1 text is from ${markdownCode(data.fonts[0].licenseSourcePath)} (SHA-256 ${markdownCode(data.fonts[0].licenseSha256)}). Copyright attribution: ${data.fonts[0].attribution}.`
      : '',
    '',
    '## Verified source fallback',
    '',
    data.fallbacks.length === 0
      ? 'No source fallback was required.'
      : data.fallbacks.map(fallback => [
        `### ${fallback.name}@${fallback.version}`,
        '',
        `- Method: ${fallback.method}; status: ${fallback.verificationStatus}.`,
        `- Stage 1 package tarball: [${fallback.npmTarball.url}](${fallback.npmTarball.url}), SHA-256 ${markdownCode(fallback.npmTarball.sha256)}, npm integrity ${markdownCode(fallback.npmTarball.integrity)}, ${fallback.npmTarball.size} bytes. Its only license file was SPDX metadata (SHA-256 ${markdownCode(fallback.npmTarball.spdxFileSha256)}); package.json SHA-256 ${markdownCode(fallback.npmTarball.packageJsonSha256)}.`,
        `- Exact source: ${fallback.repository}, tag ${markdownCode(fallback.tag)}, commit ${markdownCode(fallback.commit)}.`,
        `- Chosen branch: ${fallback.licenseChoice}. Both permitted license texts were recovered.`,
        `- Provenance record: ${markdownCode(fallback.provenancePath)}.`,
        ...fallback.licenseFiles.map(file => `- ${markdownCode(file.license)} text: ${markdownCode(file.path)} (SHA-256 ${markdownCode(file.sha256)}; [upstream source](${file.sourceUrl})).`),
        '- SPDX reconstruction was not used.',
      ].join('\n')).join('\n\n'),
    '',
    '## Production package graph evidence',
    '',
    markdownTable(['Package', 'Version'], data.bundle.npmProductionPackages.map(record => [record.name, record.version])),
    '',
    'This report audits source license files for the complete installed production package graph and emits legal texts only for packages or generated assets present in the production bundle.',
    '',
  ];
  return lines.join('\n');
}
