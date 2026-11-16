# Optional managed-tool license audit

This audit is regenerated from `shared/managed-tools.json` and the checked evidence ledger. Managed tools are offered only on demand after explicit user consent; their archives are not included in Bergamotto installers.

Manifest SHA-256: `b648ce7a31178032484d6bba86d69b6f8b4afd3cf7f641a52f0e6900010e77ef`.

## Pinned assets and license evidence

| Tool | Platform | Manifest version | License evidence | Asset and provenance |
| --- | --- | --- | --- | --- |
| FFmpeg | linux-arm64 | 7.0.2 | GPL-3.0-or-later; checked 2026-11-16 via [configuration strings](../third-party/ffmpeg-linux-arm64-configuration.json) | [archive](https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-arm64-static.tar.xz) · SHA-256 `f4149bb2b0784e30e99bdda85471c9b5930d3402014e934a5098b41d0f7201b1` · [upstream source](https://ffmpeg.org/download.html) |
| FFmpeg | linux-x64 | 7.0.2 | GPL-3.0-or-later; checked 2026-11-15 via [line 6](../docs/audits/managed-tools/linux-x64-live-install.txt) | [archive](https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz) · SHA-256 `abda8d77ce8309141f83ab8edf0596834087c52467f6badf376a6a2a4c87cf67` · [upstream source](https://ffmpeg.org/download.html) |
| FFmpeg | macos-arm64 | 9.0.2 | GPL-3.0-or-later; checked 2026-11-15 via [lines 50-56](../docs/managed-tools-manifest-audit.md) | [archive](https://ffmpeg.martin-riedl.de/download/macos/arm64/1789931890_9.0.2/ffmpeg.zip) · SHA-256 `c8ed4c4e6978a03c485edbfe4e0a5dc2380f8a30bba5150531b31b094492d924` · [upstream source](https://ffmpeg.org/download.html) |
| FFmpeg | macos-x64 | 9.0.2 | GPL-3.0-or-later; checked 2026-11-16 via [configuration strings](../third-party/ffmpeg-macos-x64-configuration.json) | [archive](https://evermeet.cx/ffmpeg/ffmpeg-9.0.2.zip) · SHA-256 `4acc0be580f9b2788029eb7bd4d645ff87968911b0a62aeeb3940d42d54558d5` · [upstream source](https://ffmpeg.org/download.html) |
| FFmpeg | windows-x64 | 9.0.2 | GPL-3.0-or-later; checked 2026-11-16 via [configuration strings](../third-party/ffmpeg-windows-x64-configuration.json) | [archive](https://www.gyan.dev/ffmpeg/builds/packages/ffmpeg-9.0.2-essentials_build.zip) · SHA-256 `60f467265b1e312373dbcd92200c2618a74850f98d3d078e94296bb3fa2047ba` · [upstream source](https://ffmpeg.org/download.html) |
| ImageMagick | linux-x64 | 7.1.2-35 | ImageMagick (Apache-2.0 compatible); checked 2026-11-15 via [line 8](../docs/audits/managed-tools/linux-x64-live-install.txt) | [archive](https://github.com/ImageMagick/ImageMagick/releases/download/7.1.2-35/ImageMagick-7.1.2-35-gcc-x86_64.AppImage) · SHA-256 `f111044ed06b68cfb478df724c5a32d35145afa21073242f888d234a2f8cc002` · [upstream source](https://github.com/ImageMagick/ImageMagick) |
| ImageMagick | windows-x64 | 7.1.2-35 | ImageMagick (Apache-2.0 compatible); checked 2026-11-15 via [lines 19-20](../docs/managed-tools-manifest-audit.md) | [archive](https://github.com/ImageMagick/ImageMagick/releases/download/7.1.2-35/ImageMagick-7.1.2-35-portable-Q16-HDRI-x64.7z) · SHA-256 `34db4364da26c88d6b8cc5bbc1fff22b85d00d3b399c82fe85ee4790efacbbf1` · [upstream source](https://github.com/ImageMagick/ImageMagick) |
| Python 3 | linux-arm64 | 3.12.16 | PSF-2.0 (with bundled third-party licenses); checked 2026-11-15 via [lines 14-18](../docs/managed-tools-manifest-audit.md) | [archive](https://github.com/astral-sh/python-build-standalone/releases/download/20261110/cpython-3.12.16+20261110-aarch64-unknown-linux-gnu-install_only.tar.gz) · SHA-256 `4792f81750282597fb2a3389fe18b945ed2ba6aad5e45ad406d122c58cb33081` · [upstream source](https://github.com/python/cpython) |
| Python 3 | linux-x64 | 3.12.16 | PSF-2.0 (with bundled third-party licenses); checked 2026-11-15 via [line 10](../docs/audits/managed-tools/linux-x64-live-install.txt) | [archive](https://github.com/astral-sh/python-build-standalone/releases/download/20261110/cpython-3.12.16+20261110-x86_64-unknown-linux-gnu-install_only.tar.gz) · SHA-256 `c2f87d4b4e8db3abddb0f116c6ccfc503d0cbdbb9ee8faacddad1b679ec12e99` · [upstream source](https://github.com/python/cpython) |
| Python 3 | macos-arm64 | 3.12.16 | PSF-2.0 (with bundled third-party licenses); checked 2026-11-15 via [lines 14-18](../docs/managed-tools-manifest-audit.md) | [archive](https://github.com/astral-sh/python-build-standalone/releases/download/20261110/cpython-3.12.16+20261110-aarch64-apple-darwin-install_only.tar.gz) · SHA-256 `e956e60bc9465d577ef8062059aa05cd3a2665b4b8678a2ed24e432368330b8f` · [upstream source](https://github.com/python/cpython) |
| Python 3 | macos-x64 | 3.12.16 | PSF-2.0 (with bundled third-party licenses); checked 2026-11-15 via [lines 14-18](../docs/managed-tools-manifest-audit.md) | [archive](https://github.com/astral-sh/python-build-standalone/releases/download/20261110/cpython-3.12.16+20261110-x86_64-apple-darwin-install_only.tar.gz) · SHA-256 `83f2922437e6d546d0fcdb8c11f150a8e52787e8d9ea8651b1974e20908eb1bc` · [upstream source](https://github.com/python/cpython) |
| Python 3 | windows-x64 | 3.12.16 | PSF-2.0 (with bundled third-party licenses); checked 2026-11-15 via [lines 14-18](../docs/managed-tools-manifest-audit.md) | [archive](https://github.com/astral-sh/python-build-standalone/releases/download/20261110/cpython-3.12.16+20261110-x86_64-pc-windows-msvc-install_only.tar.gz) · SHA-256 `e23fe5b16926de1d81661b1a97352e89fb167549c55a6825796ccd0f32ec7f1b` · [upstream source](https://github.com/python/cpython) |

## FFmpeg

- License metadata: [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html).
- Upstream source: https://ffmpeg.org/download.html.
- FFmpeg's general license is LGPL-2.1-or-later; builds with GPL components are GPL-2.0-or-later, and `--enable-version3` upgrades that build to GPL-3.0-or-later. All five pinned assets have configuration evidence containing both `--enable-gpl` and `--enable-version3`, with no `--enable-nonfree` recorded.
- Each FFmpeg archive is offered only on demand after explicit user consent. It is not bundled in Bergamotto installers.
- **linux-arm64, 7.0.2:** GPL-3.0-or-later. Configuration was extracted from the verified arm64 binary; it was not executed on arm64. Evidence: third-party/ffmpeg-linux-arm64-configuration.json (configuration strings; SHA-256 `2ab89fee32086563d55c71523dc0bbe1f55186f3dbdc2c52da472ca690a2ae0a`).
- **linux-x64, 7.0.2:** GPL-3.0-or-later. Runtime evidence is for Linux x64 only. Evidence: docs/audits/managed-tools/linux-x64-live-install.txt (line 6; SHA-256 `58114cf59627397112b9100417cf2deb43209ec00039ea21535c0716e723f03f`).
- **macos-arm64, 9.0.2:** GPL-3.0-or-later. Runtime was not run on macOS arm64; evidence is static strings only. Evidence: docs/managed-tools-manifest-audit.md (lines 50-56; SHA-256 `8de9155106c06383bfdc065e1629de47a560207a90dfb7339ea4e770d5e5045c`).
- **macos-x64, 9.0.2:** GPL-3.0-or-later. Configuration was extracted from the verified macOS binary; it was not executed on macOS. Evidence: third-party/ffmpeg-macos-x64-configuration.json (configuration strings; SHA-256 `b2f14c38bb2c079721f2b43e9e31608a22369e7f40fff331cd96b09406bf093d`).
- **windows-x64, 9.0.2:** GPL-3.0-or-later. Configuration was extracted from the verified PE binary; it was not executed on Windows. Evidence: third-party/ffmpeg-windows-x64-configuration.json (configuration strings; SHA-256 `60602091db994b4b766574efdb474e6e421d0178f0eb5f32c55c3162b1ca8306`).

## ImageMagick

- License metadata: [ImageMagick (Apache-2.0 compatible)](https://imagemagick.org/script/license.php).
- Upstream source: https://github.com/ImageMagick/ImageMagick.
- The ImageMagick license permits use under the ImageMagick License, described by the project as compatible with Apache-2.0. Managed archives are only offered after explicit user consent and are not bundled in Bergamotto installers.
- **linux-x64, 7.1.2-35:** ImageMagick (Apache-2.0 compatible). Runtime evidence is for Linux x64 only. Evidence: docs/audits/managed-tools/linux-x64-live-install.txt (line 8; SHA-256 `58114cf59627397112b9100417cf2deb43209ec00039ea21535c0716e723f03f`).
- **windows-x64, 7.1.2-35:** ImageMagick (Apache-2.0 compatible). The asset digest is recorded in the manifest audit; no native Windows execution was performed. Evidence: docs/managed-tools-manifest-audit.md (lines 19-20; SHA-256 `8de9155106c06383bfdc065e1629de47a560207a90dfb7339ea4e770d5e5045c`).

## Python 3

- License metadata: [PSF-2.0 (with bundled third-party licenses)](https://docs.python.org/3/license.html).
- Upstream source: https://github.com/python/cpython.
- The standalone CPython distributions include licenses/notices for bundled third-party components. If a Python distribution is redistributed separately, include those files in its `licenses/` folder.
- Bergamotto offers these archives only on demand after explicit user consent; they are not bundled in Bergamotto installers.
- **linux-arm64, 3.12.16:** PSF-2.0 (with bundled third-party licenses). License applies to CPython; the archive's bundled third-party notices have not been inventoried here. Evidence: docs/managed-tools-manifest-audit.md (lines 14-18; SHA-256 `8de9155106c06383bfdc065e1629de47a560207a90dfb7339ea4e770d5e5045c`).
- **linux-x64, 3.12.16:** PSF-2.0 (with bundled third-party licenses). The archive's bundled third-party notices have not been inventoried here. Evidence: docs/audits/managed-tools/linux-x64-live-install.txt (line 10; SHA-256 `58114cf59627397112b9100417cf2deb43209ec00039ea21535c0716e723f03f`).
- **macos-arm64, 3.12.16:** PSF-2.0 (with bundled third-party licenses). License applies to CPython; the archive's bundled third-party notices have not been inventoried here. Evidence: docs/managed-tools-manifest-audit.md (lines 14-18; SHA-256 `8de9155106c06383bfdc065e1629de47a560207a90dfb7339ea4e770d5e5045c`).
- **macos-x64, 3.12.16:** PSF-2.0 (with bundled third-party licenses). License applies to CPython; the archive's bundled third-party notices have not been inventoried here. Evidence: docs/managed-tools-manifest-audit.md (lines 14-18; SHA-256 `8de9155106c06383bfdc065e1629de47a560207a90dfb7339ea4e770d5e5045c`).
- **windows-x64, 3.12.16:** PSF-2.0 (with bundled third-party licenses). License applies to CPython; the archive's bundled third-party notices have not been inventoried here. Evidence: docs/managed-tools-manifest-audit.md (lines 14-18; SHA-256 `8de9155106c06383bfdc065e1629de47a560207a90dfb7339ea4e770d5e5045c`).

When redistributing a managed Python build separately, include that build’s bundled third-party notices and license files in its `licenses/` folder. Bergamotto does not bundle these Python archives in its installers.

