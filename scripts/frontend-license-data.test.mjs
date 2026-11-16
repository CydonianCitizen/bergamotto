import assert from 'node:assert/strict';
import { mkdtemp, mkdir, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { auditPackageDirectory, markdownCode } from './frontend-license-data.mjs';

async function createPackage(t, { name = 'sample-package', license = 'MIT', files = {} } = {}) {
  const root = await mkdtemp(path.join(os.tmpdir(), 'bergamotto-license-test-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  await writeFile(path.join(root, 'package.json'), JSON.stringify({ name, version: '1.2.3', license }));
  for (const [filename, contents] of Object.entries(files)) {
    const filePath = path.join(root, filename);
    await mkdir(path.dirname(filePath), { recursive: true });
    await writeFile(filePath, contents);
  }
  return root;
}

test('fails closed when metadata exists but the package has no actual license text', async t => {
  const packageRoot = await createPackage(t, {
    license: 'MIT',
    files: { 'LICENSE.spdx': 'SPDX-License-Identifier: MIT\n' },
  });

  await assert.rejects(auditPackageDirectory(packageRoot), /actual license text/i);
});

test('returns every actual license file without trimming or rewriting its text', async t => {
  const mitText = 'MIT License\n\nCopyright (c) Sample\n\nFull terms.\n';
  const apacheText = 'Apache License\nVersion 2.0\n\nFull Apache terms.\n';
  const packageRoot = await createPackage(t, {
    license: 'MIT OR Apache-2.0',
    files: {
      'licenses/LICENSE-MIT': mitText,
      'licenses/LICENSE-APACHE-2.0': apacheText,
    },
  });

  const result = await auditPackageDirectory(packageRoot);

  assert.deepEqual(result.licenseFiles.map(file => file.text), [apacheText, mitText]);
  assert.match(result.licenseText, /Full Apache terms\./);
  assert.match(result.licenseText, /Full terms\./);
  assert.equal(result.licenseFiles.length, 2);
});

test('rejects an unrecognized SPDX expression instead of treating it as a filename', async t => {
  const packageRoot = await createPackage(t, {
    license: 'LicenseRef-Internal',
    files: { LICENSE: 'Some text.\n' },
  });

  await assert.rejects(auditPackageDirectory(packageRoot), /unsupported|unrecognized|disallowed/i);
});

test('rejects a modified persisted fallback license text by its expected source hash', async t => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'bergamotto-license-fallback-test-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  const packageRoot = path.join(root, 'node_modules', '@tauri-apps', 'plugin-dialog');
  await mkdir(packageRoot, { recursive: true });
  await writeFile(path.join(packageRoot, 'package.json'), JSON.stringify({
    name: '@tauri-apps/plugin-dialog',
    version: '2.8.4',
    license: 'MIT OR Apache-2.0',
    repository: 'https://github.com/tauri-apps/plugins-workspace',
    authors: ['Tauri Programme within The Commons Conservancy'],
  }));

  const fallbackDirectory = path.join(root, 'third-party', 'license-texts', 'tauri-apps-plugin-dialog-2.8.4');
  await mkdir(fallbackDirectory, { recursive: true });
  await writeFile(path.join(fallbackDirectory, 'PROVENANCE.md'), [
    'Fallback method:** 2',
    'Verification status:** verified',
    'dialog-js-v2.8.4',
    '128190689dbe59b59d344e01bf07b00862aa3791',
    'https://registry.npmjs.org/@tauri-apps/plugin-dialog/-/plugin-dialog-2.8.4.tgz',
    'sha512-+6ZOiXenOaxXwb8qCbBPdT2jX2cFPaVVHGIskrpmJLpmWj+qKRTD6X/z9x7hTdSQn7NedcoXCkWGej4a7dC4lw==',
    '6,802',
    '3eff363f336b8d62ee321bbf78b39988fd162e597c961f3ea1c00cf3de9ed791',
    'eb8a6c84630461b352badcab1dbe5d0168c56d377358b2b8c86b51003272d5ef',
    'af8a8f5263c57545433dc83e79b88fad6f18c200657f7c6cc7ed6328eb7e8997',
  ].join('\n'));
  await writeFile(path.join(fallbackDirectory, 'LICENSE_MIT'), 'Edited terms that must not pass.\n');
  await writeFile(path.join(fallbackDirectory, 'LICENSE_APACHE-2.0'), 'Apache License Version 2.0\n');

  await assert.rejects(
    auditPackageDirectory(packageRoot, { root }),
    /fallback text hash mismatch for LICENSE_MIT/i,
  );
});

test('Markdown display escapes virtual-module control bytes without mutating the raw ID', () => {
  const rawModuleId = '\u0000commonjsHelpers.js';
  const helper = { id: rawModuleId };
  const displayed = markdownCode(helper.id);

  assert.equal(helper.id, '\u0000commonjsHelpers.js');
  assert.equal(displayed, '`\\u0000commonjsHelpers.js`');
  assert.equal(displayed.includes('\u0000'), false);
});
