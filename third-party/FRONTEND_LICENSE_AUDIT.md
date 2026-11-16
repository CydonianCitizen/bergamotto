# Frontend and font license audit

Generated from a fresh programmatic Vite production build in a temporary directory. The configured Vite plugins were retained, and every emitted output file was compared byte-for-byte with `dist`: passed (11 files).

The package-lock production graph contains 13 installed packages. The emitted npm module graph contains 10 package versions, plus 3 bundler/runtime helpers and 1 CSS asset(s) with Tailwind-generated content.

## License texts included in the shipped frontend

| Package | Version | Declared license | Selected branch | Actual license files |
| --- | --- | --- | --- | --- |
| @fontsource-variable/dm-sans | 5.3.0 | OFL-1.1 | OFL-1.1 | [OFL-1.1] LICENSE (6fbd040a29c2037a765dfb9f2561e9965b5c95c6dcb5dce516089d63d5f17af7) |
| @tauri-apps/api | 2.12.4 | Apache-2.0 OR MIT | Apache-2.0 | [Apache-2.0] LICENSE-APACHE-2.0 (0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594)<br>[MIT] LICENSE-MIT (9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d) |
| @tauri-apps/plugin-dialog | 2.8.4 | MIT OR Apache-2.0 | MIT | [MIT] third-party/license-texts/tauri-apps-plugin-dialog-2.8.4/LICENSE_MIT (89ff9689dcf9dd53968785d05a26f7898bb169dbfcada8d032b3e68cf0d55607)<br>[Apache-2.0] third-party/license-texts/tauri-apps-plugin-dialog-2.8.4/LICENSE_APACHE-2.0 (0cec06e0e55fbc3dc5cee4fca9b607f66cb8f4e4dbcf3b3c013594dd156732e9) |
| ajv-formats | 3.0.1 | MIT | MIT | [MIT] LICENSE (9df3bb69929a3b650ed73b3bfa1756725aaff0ac296461605753547004eafeaf) |
| ajv | 8.20.0 | MIT | MIT | [MIT] LICENSE (a05350a88e318e4f5f2c2a1ff1e2e88daa4dd38e6e78b71cccae422bdc762cc3) |
| fast-deep-equal | 3.1.3 | MIT | MIT | [MIT] LICENSE (7bf9b2de73a6b356761c948d0e9eeb4be6c1270bd04c79cd489c1e400ffdfc1a) |
| lucide-react | 0.577.0 | ISC | ISC | [ISC] LICENSE (668dcc52803480e0a026b31140a4cae668772663cd764e5991d252eef03f98db) |
| react-dom | 19.3.1 | MIT | MIT | [MIT] LICENSE (da6d3703ed11cbe42bd212c725957c98da23cbff1998c05fa4b3d976d1a58e93) |
| react | 19.3.1 | MIT | MIT | [MIT] LICENSE (da6d3703ed11cbe42bd212c725957c98da23cbff1998c05fa4b3d976d1a58e93) |
| scheduler | 0.28.0 | MIT | MIT | [MIT] LICENSE (da6d3703ed11cbe42bd212c725957c98da23cbff1998c05fa4b3d976d1a58e93) |
| tailwindcss | 4.3.5 | MIT | MIT | [MIT] LICENSE (60e0b68c0f35c078eef3a5d29419d0b03ff84ec1df9c3f9d6e39a519a5ae7985) |
| vite | 7.3.8 | MIT | MIT | [ISC] LICENSE.md (a77a1c089806b39ad339535bdf3677f636c91d96693e8ad7b11fe733f650ea64) |

Every listed license file is copied in full from the installed npm package or the verified source fallback, hashed on each generation, and checked against its declared license expression. SPDX templates are not used as license text. Lucide React keeps its complete bundled license file, including the Feather MIT portion.

## Bundle and production npm graph differences

### Bundled code from development dependencies

| Package | Version | Bundle evidence |
| --- | --- | --- |
| tailwindcss | 4.3.5 | The emitted CSS contains Tailwind-generated styles (generated Tailwind @layer properties block; generated Tailwind --tw-* theme or utility properties). |
| vite | 7.3.8 | The emitted JavaScript contains Vite/Rollup runtime helpers: Vite/Rollup CommonJS helper, Vite modulepreload polyfill, Vite dynamic-import preload helper. |

### Production dependencies not present in the bundle module graph

| Package | Version | Declared license | License text found | Reason |
| --- | --- | --- | --- | --- |
| fast-uri | 3.1.9 | BSD-3-Clause | LICENSE (b010b0dfdfdb23d7396e03b82cd4621fc9bb8f95d6b0aea70b9c24e12074c786) | Installed in the production npm graph but no module from this package appears in the emitted Vite bundle. |
| json-schema-traverse | 1.0.0 | MIT | LICENSE (7bf9b2de73a6b356761c948d0e9eeb4be6c1270bd04c79cd489c1e400ffdfc1a) | Installed in the production npm graph but no module from this package appears in the emitted Vite bundle. |
| require-from-string | 2.0.2 | MIT | license (6ee0feb1f6ef996ff5a68600f8cf98909cf412d39ef3cdceaefd87d636fa1b7f) | Installed in the production npm graph but no module from this package appears in the emitted Vite bundle. |

Production-but-not-bundled packages were audited for a supported license expression and actual license text. Their license terms are not copied into the shipped notice because their code is not in the frontend output.

## Output and CSS inspection

| Output file | Type | Bytes | SHA-256 | Verification |
| --- | --- | --- | --- | --- |
| assets/dm-sans-latin-ext-wght-normal-BOFOeGcA.woff2 | asset | 18228 | a5d38fe99f930275684999b462c7123faa063d9e44e73b4b241723d884aa0f49 | byte-for-byte match |
| assets/dm-sans-latin-wght-normal-Xz1IZZA0.woff2 | asset | 36932 | 9fea608a947e67020c33cad9a6fe3d60c54119dfb8cff87768a8117a15ed7543 | byte-for-byte match |
| assets/index-DuMoVqcN.css | asset | 41015 | f03a87247bb1e5258518673f3e8e36d39fdf2635892430efb5a3e9b660625c23 | byte-for-byte match |
| assets/index-DIZysm_H.js | chunk | 224347 | 93b796819da8e2aa2cdb6b58dccdbe884e28b97ff4f2093f266b5b9abdc09d8f | byte-for-byte match |
| assets/RadialWheel-B-OOrM5N.js | chunk | 6742 | e1521beecb595aee150c0ea05d82193764e5a1afe076312cb6efc2497b11941a | byte-for-byte match |
| assets/ProgressHud-D4tF1Rz1.js | chunk | 1529 | 00c91383baa3a6bd8d16cc1707c3bd6f314562f7d4f7b3598cbf849c3d7446d7 | byte-for-byte match |
| assets/ActionStudio-B-Binxcf.js | chunk | 103803 | 70ada94fa2e65a1457ddbc8a5d844d1f6c580535b3c151d0a68a4fcc07de5c80 | byte-for-byte match |
| assets/wheelGeometry-DAxlNIC1.js | chunk | 27435 | baf6b9e2c234f39d76c4a0905a93c2b32c5c6922450c499b8cc3e63cefb1cdcf | byte-for-byte match |
| assets/loader-circle-DhYY9EIY.js | chunk | 138 | b3fafa00a581138610100a38751ce1e4d9fef196969181ba3162f28f334c16a0 | byte-for-byte match |
| assets/ipc-DiAFy2nM.js | chunk | 5511 | 0c08602c15be130b4d2ad24f3c8bd51ea17746eb92c28ca1fcd43c63b3bf1ca4 | byte-for-byte match |
| index.html | html | 395 | adb5d3465bd1066593faa0a57c39d99e0c9c808123ee73f2ce7b0b1d07ceb511 | byte-for-byte match |

No output differences; all dist files matched the temporary build byte-for-byte.

- `assets/index-DuMoVqcN.css`: generated Tailwind @layer properties block; generated Tailwind --tw-* theme or utility properties

Runtime helpers observed in emitted module IDs: `Vite/Rollup CommonJS helper` (`\u0000commonjsHelpers.js`), `Vite modulepreload polyfill` (`\u0000vite/modulepreload-polyfill.js`), `Vite dynamic-import preload helper` (`\u0000vite/preload-helper.js`).

## DM Sans font assets

| Emitted font | Source file | Emitted SHA-256 | Source SHA-256 | Family / version / reserved-name check |
| --- | --- | --- | --- | --- |
| assets/dm-sans-latin-ext-wght-normal-BOFOeGcA.woff2 | node_modules/@fontsource-variable/dm-sans/files/dm-sans-latin-ext-wght-normal.woff2 | a5d38fe99f930275684999b462c7123faa063d9e44e73b4b241723d884aa0f49 | a5d38fe99f930275684999b462c7123faa063d9e44e73b4b241723d884aa0f49 | DM Sans Variable / DM Sans v17; binary DM Sans 9pt (262406) / No specific Reserved Font Name declaration was found in font package metadata or license text. |
| assets/dm-sans-latin-wght-normal-Xz1IZZA0.woff2 | node_modules/@fontsource-variable/dm-sans/files/dm-sans-latin-wght-normal.woff2 | 9fea608a947e67020c33cad9a6fe3d60c54119dfb8cff87768a8117a15ed7543 | 9fea608a947e67020c33cad9a6fe3d60c54119dfb8cff87768a8117a15ed7543 | DM Sans Variable / DM Sans v17; binary DM Sans 9pt (262406) / No specific Reserved Font Name declaration was found in font package metadata or license text. |

The emitted font files match the source WOFF2 files byte-for-byte. The Vite filename suffix is a content fingerprint, not a font-family rename: package metadata names DM Sans (v17), fc-query reports DM Sans 9pt, and CSS uses `DM Sans Variable`. No specific Reserved Font Name declaration appears in the package metadata or OFL text. The complete OFL 1.1 text is from `node_modules/@fontsource-variable/dm-sans/LICENSE` (SHA-256 `6fbd040a29c2037a765dfb9f2561e9965b5c95c6dcb5dce516089d63d5f17af7`). Copyright attribution: Copyright 2014 The DM Sans Project Authors (https://github.com/googlefonts/dm-fonts) DMSans-Italic[opsz,wght].ttf: Copyright 2014 The DM Sans Project Authors (https://github.com/googlefonts/dm-fonts).

## Verified source fallback

### @tauri-apps/plugin-dialog@2.8.4

- Method: verified-exact-version-repository-tag; status: verified.
- Stage 1 package tarball: [https://registry.npmjs.org/@tauri-apps/plugin-dialog/-/plugin-dialog-2.8.4.tgz](https://registry.npmjs.org/@tauri-apps/plugin-dialog/-/plugin-dialog-2.8.4.tgz), SHA-256 `3eff363f336b8d62ee321bbf78b39988fd162e597c961f3ea1c00cf3de9ed791`, npm integrity `sha512-+6ZOiXenOaxXwb8qCbBPdT2jX2cFPaVVHGIskrpmJLpmWj+qKRTD6X/z9x7hTdSQn7NedcoXCkWGej4a7dC4lw==`, 6802 bytes. Its only license file was SPDX metadata (SHA-256 `eb8a6c84630461b352badcab1dbe5d0168c56d377358b2b8c86b51003272d5ef`); package.json SHA-256 `af8a8f5263c57545433dc83e79b88fad6f18c200657f7c6cc7ed6328eb7e8997`.
- Exact source: https://github.com/tauri-apps/plugins-workspace, tag `dialog-js-v2.8.4`, commit `128190689dbe59b59d344e01bf07b00862aa3791`.
- Chosen branch: MIT. Both permitted license texts were recovered.
- Provenance record: `third-party/license-texts/tauri-apps-plugin-dialog-2.8.4/PROVENANCE.md`.
- `MIT` text: `third-party/license-texts/tauri-apps-plugin-dialog-2.8.4/LICENSE_MIT` (SHA-256 `89ff9689dcf9dd53968785d05a26f7898bb169dbfcada8d032b3e68cf0d55607`; [upstream source](https://github.com/tauri-apps/plugins-workspace/blob/dialog-js-v2.8.4/plugins/dialog/LICENSE_MIT)).
- `Apache-2.0` text: `third-party/license-texts/tauri-apps-plugin-dialog-2.8.4/LICENSE_APACHE-2.0` (SHA-256 `0cec06e0e55fbc3dc5cee4fca9b607f66cb8f4e4dbcf3b3c013594dd156732e9`; [upstream source](https://github.com/tauri-apps/plugins-workspace/blob/dialog-js-v2.8.4/plugins/dialog/LICENSE_APACHE-2.0)).
- SPDX reconstruction was not used.

## Production package graph evidence

| Package | Version |
| --- | --- |
| @fontsource-variable/dm-sans | 5.3.0 |
| @tauri-apps/api | 2.12.4 |
| @tauri-apps/plugin-dialog | 2.8.4 |
| ajv-formats | 3.0.1 |
| ajv | 8.20.0 |
| fast-deep-equal | 3.1.3 |
| fast-uri | 3.1.9 |
| json-schema-traverse | 1.0.0 |
| lucide-react | 0.577.0 |
| react-dom | 19.3.1 |
| react | 19.3.1 |
| require-from-string | 2.0.2 |
| scheduler | 0.28.0 |

This report audits source license files for the complete installed production package graph and emits legal texts only for packages or generated assets present in the production bundle.
