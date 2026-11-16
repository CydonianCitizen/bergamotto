import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtemp, mkdir, readFile, rm, writeFile, copyFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  parseCargoDenyAllowList,
  renderThirdPartyLicenses,
} from './generate-third-party-licenses.mjs';
import { collectOptionalToolLicenses } from './optional-tools-license-data.mjs';

const root = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const mitText = 'MIT License\n\nCopyright (c) Example Authors\n\nPermission is hereby granted.\n';

function hash(value) {
  return createHash('sha256').update(value).digest('hex');
}

function inputs() {
  return {
    rust: {
      schemaVersion: 1,
      tool: { name: 'cargo-about', version: '0.9.3' },
      scope: { targets: ['x86_64-unknown-linux-gnu'] },
      aboutMarkdownTargets: ['x86_64-unknown-linux-gnu'],
      aboutMarkdown: '# Third-party Rust licenses\n\n## MIT License (`MIT`)\n\nUsed by:\n- [serde 1.0.0](https://github.com/serde-rs/serde)\n\n~~~~text\n' + mitText + '\n~~~~\n',
      aboutTemplateLicenseTexts: [{ spdxId: 'MIT', text: mitText, sha256: hash(mitText) }],
      unresolved: [],
      packages: [{
        name: 'serde',
        version: '1.0.0',
        repository: 'https://github.com/serde-rs/serde',
        source: 'registry+https://github.com/rust-lang/crates.io-index#serde@1.0.0',
        isWorkspace: false,
        declaredLicense: 'MIT OR Apache-2.0',
        selectedLicenseIds: ['MIT'],
        targets: ['x86_64-unknown-linux-gnu'],
        licenseTextSha256: [hash(mitText)],
      }],
      licenseTexts: [{
        spdxId: 'MIT',
        name: 'MIT License',
        text: mitText,
        sha256: hash(mitText),
        sourcePath: '/tmp/cargo/registry/src/serde-1.0.0/LICENSE',
        crates: [{
          name: 'serde',
          version: '1.0.0',
          repository: 'https://github.com/serde-rs/serde',
          targets: ['x86_64-unknown-linux-gnu'],
        }],
      }],
    },
    frontend: {
      packages: [{
        name: 'react',
        version: '19.0.0',
        repository: 'https://github.com/facebook/react',
        license: 'MIT',
        licenseChoice: 'MIT',
        licenseText: mitText,
        licenseFiles: [{
          path: 'LICENSE',
          sha256: hash(mitText),
          text: mitText,
          spdxId: 'MIT',
          spdxIds: ['MIT'],
        }],
      }],
      bundledNotProduction: [],
      productionNotBundled: [],
      fonts: [],
      fallbacks: [],
      bundle: { modules: [], assets: [] },
    },
    optionalTools: {
      tools: [{
        name: 'FFmpeg',
        licenseUrl: 'https://www.gnu.org/licenses/gpl-3.0.html',
        sourceUrl: 'https://ffmpeg.org/download.html',
        assets: [{
          platform: 'linux-x64',
          version: '7.0.2',
          license: 'GPL-3.0-or-later',
        }],
      }],
    },
  };
}

test('deny.toml allow list is parsed and excludes unmanaged license identifiers', async () => {
  const denyConfig = await readFile(path.join(root, 'deny.toml'), 'utf8');
  const allowList = parseCargoDenyAllowList(denyConfig);
  assert.equal(allowList.has('MIT'), true);
  assert.equal(allowList.has('MPL-2.0'), true);
  assert.equal(allowList.has('GPL-3.0-only'), false);
});

test('output is deterministic and includes package provenance plus actual license text', async () => {
  const testInputs = inputs();
  const allowList = new Set(['MIT', 'Apache-2.0']);
  const first = renderThirdPartyLicenses({ ...testInputs, allowList });
  const second = renderThirdPartyLicenses({ ...testInputs, allowList });
  assert.equal(first, second);
  assert.match(first, /serde-rs\/serde/);
  assert.match(first, /api\/v1\/crates\/serde\/1\.0\.0\/download/);
  assert.match(first, /Copyright \(c\) Example Authors/);
  assert.match(first, /GPL-3\.0-or-later/);
  assert.equal(first.includes('/tmp/cargo/registry'), false);
});

test('generation verifies cargo-about template output and target coverage', () => {
  const allowList = new Set(['MIT', 'Apache-2.0']);
  const wrongText = inputs();
  wrongText.rust.aboutMarkdown = wrongText.rust.aboutMarkdown.replace(
    'Permission is hereby granted.',
    'Permission is not hereby granted.',
  );
  assert.throws(
    () => renderThirdPartyLicenses({ ...wrongText, allowList }),
    /about\.hbs rendered unverified cargo-about union text MIT\|/,
  );

  const wrongTargets = inputs();
  wrongTargets.rust.aboutMarkdownTargets = [];
  assert.throws(
    () => renderThirdPartyLicenses({ ...wrongTargets, allowList }),
    /about\.hbs render did not cover the collected target set/,
  );
});

test('template validation uses cargo-about union JSON while final notices retain package source bytes', () => {
  const testInputs = inputs();
  const templateText = mitText.replace('Permission is hereby granted.', 'Cargo-about rendered permission text.');
  testInputs.rust.aboutTemplateLicenseTexts = [{
    spdxId: 'MIT',
    text: templateText,
    sha256: hash(templateText),
  }];
  testInputs.rust.aboutMarkdown = `# Third-party Rust licenses\n\n## MIT License (\`MIT\`)\n\nUsed by:\n- [serde 1.0.0](https://github.com/serde-rs/serde)\n\n~~~~text\n${templateText}\n~~~~\n`;

  const rendered = renderThirdPartyLicenses({
    ...testInputs,
    allowList: new Set(['MIT', 'Apache-2.0']),
  });
  assert.match(rendered, /Permission is hereby granted\./);
  assert.doesNotMatch(rendered, /Cargo-about rendered permission text/);
});

test('template matching normalizes line endings but keeps the raw source hash', () => {
  const testInputs = inputs();
  const crlfText = mitText.replaceAll('\n', '\r\n');
  const digest = hash(crlfText);
  testInputs.rust.packages[0].licenseTextSha256 = [digest];
  testInputs.rust.licenseTexts[0].text = crlfText;
  testInputs.rust.licenseTexts[0].sha256 = digest;
  const output = renderThirdPartyLicenses({ ...testInputs, allowList: new Set(['MIT', 'Apache-2.0']) });
  assert.equal(output.includes('Source text SHA-256 (raw file bytes): `' + digest + '`.'), true);
  assert.equal(output.includes('\r'), false);
});

test('union template JSON merges LF and CRLF copies only when normalized bodies match', () => {
  const testInputs = inputs();
  const crlfText = mitText.replaceAll('\n', '\r\n');
  testInputs.rust.aboutTemplateLicenseTexts = [
    { spdxId: 'MIT', text: mitText, sha256: hash(mitText) },
    { spdxId: 'MIT', text: crlfText, sha256: hash(crlfText) },
  ];
  const rendered = renderThirdPartyLicenses({
    ...testInputs,
    allowList: new Set(['MIT', 'Apache-2.0']),
  });
  assert.match(rendered, /Permission is hereby granted\./);
});

test('legacy slash-separated Rust license alternatives appear in the selection table', () => {
  const testInputs = inputs();
  const bsdText = 'BSD 3-Clause License\n\nCopyright (c) Example Authors\n\nRedistribution and use are permitted.\n';
  const bsdHash = hash(bsdText);
  testInputs.rust.aboutMarkdown = `# Third-party Rust licenses\n\n## BSD 3-Clause License (\`BSD-3-Clause\`)\n\nUsed by:\n- [brotli-decompressor 6.3.1](https://github.com/dropbox/rust-brotli-decompressor)\n\n~~~~text\n${bsdText}\n~~~~\n`;
  testInputs.rust.aboutTemplateLicenseTexts = [{ spdxId: 'BSD-3-Clause', text: bsdText, sha256: bsdHash }];
  testInputs.rust.packages = [{
    name: 'brotli-decompressor',
    version: '6.3.1',
    repository: 'https://github.com/dropbox/rust-brotli-decompressor',
    source: 'registry+https://github.com/rust-lang/crates.io-index#brotli-decompressor@6.3.1',
    isWorkspace: false,
    declaredLicense: 'BSD-3-Clause/MIT',
    selectedLicenseIds: ['BSD-3-Clause'],
    targets: ['x86_64-unknown-linux-gnu'],
    licenseTextSha256: [bsdHash],
  }];
  testInputs.rust.licenseTexts = [{
    spdxId: 'BSD-3-Clause',
    name: 'BSD 3-Clause License',
    text: bsdText,
    sha256: bsdHash,
    crates: [{
      name: 'brotli-decompressor',
      version: '6.3.1',
      repository: 'https://github.com/dropbox/rust-brotli-decompressor',
      targets: ['x86_64-unknown-linux-gnu'],
    }],
  }];

  const rendered = renderThirdPartyLicenses({
    ...testInputs,
    allowList: new Set(['MIT', 'BSD-3-Clause']),
  });
  assert.match(rendered, /brotli-decompressor@6\.3\.1.*declared `BSD-3-Clause\/MIT`; selected `BSD-3-Clause`/);
});

test('generation fails closed when a Rust package is disallowed or its text is missing', async t => {
  const allowList = new Set(['MIT', 'Apache-2.0']);
  const disallowed = inputs();
  disallowed.rust.packages[0].selectedLicenseIds = ['GPL-3.0-only'];
  assert.throws(
    () => renderThirdPartyLicenses({ ...disallowed, allowList }),
    /uses disallowed license GPL-3.0-only/,
  );

  const missingText = inputs();
  missingText.rust.licenseTexts[0].text = '';
  assert.throws(
    () => renderThirdPartyLicenses({ ...missingText, allowList }),
    /missing actual license text/,
  );
});

test('generation fails closed when an npm package omits its license text', () => {
  const testInputs = inputs();
  testInputs.frontend.packages[0].licenseFiles[0].text = '';
  assert.throws(
    () => renderThirdPartyLicenses({ ...testInputs, allowList: new Set(['MIT']) }),
    /npm react@19\.0\.0 LICENSE is missing actual license text/,
  );
});

test('generation rejects missing license files and missing file hashes', () => {
  const allowList = new Set(['MIT']);
  const missingFiles = inputs();
  missingFiles.frontend.packages[0].licenseFiles = [];
  assert.throws(
    () => renderThirdPartyLicenses({ ...missingFiles, allowList }),
    /npm package react@19\.0\.0 has no actual license text files/,
  );

  const missingHash = inputs();
  delete missingHash.frontend.packages[0].licenseFiles[0].sha256;
  assert.throws(
    () => renderThirdPartyLicenses({ ...missingHash, allowList }),
    /npm package react@19\.0\.0 license file hash mismatch/,
  );
});

test('actual Tauri alternatives and Vite composite notice keep all permitted text', async () => {
  const readPackage = async (packageName, choice, licensePaths, spdxIdsForPath = {}) => {
    const packageRoot = path.join(root, 'node_modules', packageName);
    const metadata = JSON.parse(await readFile(path.join(packageRoot, 'package.json'), 'utf8'));
    const repositoryValue = metadata.repository?.url ?? metadata.repository;
    const repository = String(repositoryValue).replace(/^git\+/, '').replace(/\.git$/, '');
    const licenseFiles = [];
    for (const licensePath of licensePaths) {
      const sourceRoot = licensePath.startsWith('third-party/') ? root : packageRoot;
      const text = await readFile(path.join(sourceRoot, licensePath), 'utf8');
      const ids = spdxIdsForPath[licensePath] ?? [licensePath.includes('APACHE') ? 'Apache-2.0' : 'MIT'];
      licenseFiles.push({
        path: licensePath,
        text,
        sha256: hash(text),
        spdxId: ids[0],
        spdxIds: ids,
      });
    }
    return {
      name: metadata.name,
      version: metadata.version,
      repository,
      license: metadata.license,
      licenseChoice: choice,
      licenseText: licenseFiles[0].text,
      licenseFiles,
    };
  };

  const api = await readPackage('@tauri-apps/api', 'Apache-2.0', [
    'LICENSE-APACHE-2.0',
    'LICENSE-MIT',
  ]);
  const dialogFiles = [
    'third-party/license-texts/tauri-apps-plugin-dialog-2.8.4/LICENSE_MIT',
    'third-party/license-texts/tauri-apps-plugin-dialog-2.8.4/LICENSE_APACHE-2.0',
  ];
  const dialog = await readPackage('@tauri-apps/plugin-dialog', 'MIT', dialogFiles);
  const vite = await readPackage('vite', 'MIT', ['LICENSE.md'], { 'LICENSE.md': ['ISC', 'MIT'] });
  const testInputs = inputs();
  testInputs.frontend.packages = [api, dialog, vite];
  const rendered = renderThirdPartyLicenses({
    ...testInputs,
    allowList: new Set(['MIT', 'Apache-2.0', 'ISC']),
  });

  assert.match(rendered, /@tauri-apps\/api@/);
  assert.match(rendered, /@tauri-apps\/plugin-dialog@/);
  assert.match(rendered, /### ISC \/ MIT/);
  assert.match(rendered, /VoidZero Inc\. and Vite contributors/);
  assert.match(rendered, /Tauri Apps Contributors/);
  assert.match(rendered, /Permission is hereby granted/);
});

test('optional-tool versions and source URLs are read from the managed-tools manifest', async () => {
  const result = await collectOptionalToolLicenses({ root });
  const manifest = JSON.parse(await readFile(path.join(root, 'shared/managed-tools.json'), 'utf8'));
  const ffmpeg = result.tools.find(tool => tool.id === 'ffmpeg');
  const linux = ffmpeg.assets.find(asset => asset.platform === 'linux-x64');
  assert.equal(linux.version, manifest.tools.ffmpeg.platforms['linux-x64'].version);
  assert.equal(linux.downloadUrl, manifest.tools.ffmpeg.platforms['linux-x64'].url);
  assert.equal(ffmpeg.sourceUrl, manifest.tools.ffmpeg.license.sourceUrl);
  assert.equal(linux.license, 'GPL-3.0-or-later');
});

test('optional-tool evidence stops generation when the manifest drifts', async () => {
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), 'bergamotto-license-test-'));
  try {
    await mkdir(path.join(tempRoot, 'shared'), { recursive: true });
    await mkdir(path.join(tempRoot, 'third-party'), { recursive: true });
    await copyFile(
      path.join(root, 'third-party/optional-tools-verification.json'),
      path.join(tempRoot, 'third-party/optional-tools-verification.json'),
    );
    await writeFile(path.join(tempRoot, 'shared/managed-tools.json'), '{"tools":{}}\n', 'utf8');
    await assert.rejects(
      collectOptionalToolLicenses({ root: tempRoot }),
      /shared\/managed-tools.json changed/,
    );
  } finally {
    await rm(tempRoot, { recursive: true, force: true });
  }
});
