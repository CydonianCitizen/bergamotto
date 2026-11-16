# Managed tools manifest verification

Date: 2026-11-15. The manifest's SHA-256 value is download-integrity digest. Each entry's `hashSource` states source and verification method. `sizeBytes` means uncompressed executable size, used for download-size cap, not archive length.

## Final asset pins and digest evidence

| Tool / platform | Version | Archive SHA-256 | Upstream digest verification |
| --- | --- | --- | --- |
| FFmpeg / Windows x64 | 9.0.2 | `60f467265b1e312373dbcd92200c2618a74850f98d3d078e94296bb3fa2047ba` | Gyan official SHA-256 sidecar matched local SHA-256 |
| FFmpeg / Linux x64 | 7.0.2 | `abda8d77ce8309141f83ab8edf0596834087c52467f6badf376a6a2a4c87cf67` | John Van Sickle MD5 sidecar matched local MD5; local SHA-256 also verified |
| FFmpeg / Linux arm64 | 7.0.2 | `f4149bb2b0784e30e99bdda85471c9b5930d3402014e934a5098b41d0f7201b1` | John Van Sickle MD5 sidecar matched local MD5; local SHA-256 also verified |
| FFmpeg / macOS arm64 | 9.0.2 | `c8ed4c4e6978a03c485edbfe4e0a5dc2380f8a30bba5150531b31b094492d924` | Martin Riedl official SHA-256 sidecar matched local SHA-256 |
| FFmpeg / macOS x64 | 9.0.2 | `4acc0be580f9b2788029eb7bd4d645ff87968911b0a62aeeb3940d42d54558d5` | Local SHA-256 calculated; no upstream digest found at checked `.sha256`, `.sha256sum`, `.md5`, `.md5sum` endpoints |
| Python / Windows x64 | 3.12.16 | `e23fe5b16926de1d81661b1a97352e89fb167549c55a6825796ccd0f32ec7f1b` | GitHub release API asset SHA-256 digest matched local SHA-256 |
| Python / Linux x64 | 3.12.16 | `c2f87d4b4e8db3abddb0f116c6ccfc503d0cbdbb9ee8faacddad1b679ec12e99` | GitHub release API asset SHA-256 digest matched local SHA-256 |
| Python / Linux arm64 | 3.12.16 | `4792f81750282597fb2a3389fe18b945ed2ba6aad5e45ad406d122c58cb33081` | GitHub release API asset SHA-256 digest matched local SHA-256 |
| Python / macOS arm64 | 3.12.16 | `e956e60bc9465d577ef8062059aa05cd3a2665b4b8678a2ed24e432368330b8f` | GitHub release API asset SHA-256 digest matched local SHA-256 |
| Python / macOS x64 | 3.12.16 | `83f2922437e6d546d0fcdb8c11f150a8e52787e8d9ea8651b1974e20908eb1bc` | GitHub release API asset SHA-256 digest matched local SHA-256 |
| ImageMagick / Windows x64 | 7.1.2-35 | `34db4364da26c88d6b8cc5bbc1fff22b85d00d3b399c82fe85ee4790efacbbf1` | Local SHA-256 calculated; no upstream digest found at checked `.sha256`, `.sha256sum`, `.md5`, `.md5sum` endpoints |
| ImageMagick / Linux x64 | 7.1.2-35 | `f111044ed06b68cfb478df724c5a32d35145afa21073242f888d234a2f8cc002` | Local SHA-256 calculated; no upstream digest found at checked `.sha256`, `.sha256sum`, `.md5`, `.md5sum` endpoints |

Downloaded every existing manifest archive and compared local SHA-256 with the pinned value; all matched before the explicitly noted FFmpeg Windows/macOS arm64 pin updates. Python and ImageMagick upstream values came from their GitHub release API records (`https://api.github.com/repos/astral-sh/python-build-standalone/releases/tags/20261110`, `https://api.github.com/repos/ImageMagick/ImageMagick/releases/tags/7.1.2-35`). John Van Sickle sidecars: `https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz.md5` reported `7fa72b652e19bf84c9461e332ea1cdf3`; `https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-arm64-static.tar.xz.md5` reported `807afe21601db0a73e426121c7d636ea`. Each matched the downloaded archive's `md5sum`.

## FFmpeg stable versions and build checks

FFmpeg lists 9.0.2 as latest stable, released 2026-09-18: <https://ffmpeg.org/download.html>. Gyan's release essentials are 9.0.2 and include `libx264` and `libmp3lame`: <https://www.gyan.dev/ffmpeg/builds/>. Windows and macOS arm64 now use 9.0.2. Linux remains 7.0.2 because the tested Riedl 9.0.2 x64 and arm64 build configurations contain `--enable-nonfree`; retained John Van Sickle 7.0.2 configurations do not.

Native Linux x64 John Van Sickle 7.0.2 `ffmpeg -version` output:

```text
ffmpeg version 7.0.2-static https://johnvansickle.com/ffmpeg/ Copyright (c) 2000-2024 the FFmpeg developers
configuration: --enable-gpl --enable-version3 --enable-static --disable-debug --disable-ffplay --disable-indev=sndio --disable-outdev=sndio --cc=gcc --enable-fontconfig --enable-frei0r --enable-gnutls --enable-gmp --enable-libgme --enable-gray --enable-libaom --enable-libfribidi --enable-libass --enable-libvmaf --enable-libfreetype --enable-libmp3lame --enable-libopencore-amrnb --enable-libopencore-amrwb --enable-libopenjpeg --enable-librubberband --enable-libsoxr --enable-libspeex --enable-libsrt --enable-libvorbis --enable-libopus --enable-libtheora --enable-libvidstab --enable-libvo-amrwbenc --enable-libvpx --enable-libwebp --enable-libx264 --enable-libx265 --enable-libxml2 --enable-libdav1d --enable-libxvid --enable-libzvbi --enable-libzimg
```

Its `ffmpeg -encoders` rows:

```text
V....D libx264              libx264 H.264 / AVC / MPEG-4 AVC / MPEG-4 part 10 (codec h264)
A....D libmp3lame           libmp3lame MP3 (MPEG audio layer 3) (codec mp3)
```

Downloaded Martin Riedl Linux x64 9.0.2 archive SHA-256 was `fa8ecf4abbd290d98f7d188b8649cc6b391ae209a98452be955a15aab1909d7f`. Native `ffmpeg -version` configuration included `--enable-nonfree`; native `ffmpeg -encoders` showed both required encoders. Its configuration (complete string):

```text
--prefix=/home/ffmpegBuild/out --pkg-config-flags=--static --extra-version='https://www.martin-riedl.de' --enable-gray --enable-libxml2 --enable-nonfree --enable-version3 --enable-gpl --enable-openssl --enable-libfreetype --enable-fontconfig --enable-libharfbuzz --enable-libbluray --enable-libsnappy --enable-libsrt --enable-libvmaf --enable-libass --enable-libklvanc --enable-libzimg --enable-libzvbi --enable-libaom --enable-libdav1d --enable-libopenh264 --enable-libopenjpeg --enable-librav1e --enable-libsvtav1 --enable-libvpx --enable-libvvenc --enable-libwebp --enable-libx264 --enable-libx265 --enable-libmp3lame --enable-libopus --enable-libvorbis --enable-libtheora --enable-decklink
```

Downloaded Riedl Linux arm64 9.0.2 configuration was inspected statically with `strings`; it contains `--enable-nonfree`, `--enable-libx264`, and `--enable-libmp3lame`. Retained arm64 7.0.2 configuration was inspected statically; it contains `--enable-gpl`, `--enable-libx264`, and `--enable-libmp3lame`, with no `--enable-nonfree`. arm64 runtime was not executed on this x64 host.

Downloaded Riedl macOS arm64 9.0.2 archive SHA-256 `c8ed4c4e6978a03c485edbfe4e0a5dc2380f8a30bba5150531b31b094492d924` matched its `.sha256` sidecar. Archive binary is root `ffmpeg`, 66,332,880 bytes. Static config string:

```text
--prefix=/Volumes/ffmpeg_arm64/out --pkg-config-flags=--static --extra-version='https://www.martin-riedl.de' --enable-gray --enable-libxml2 --enable-version3 --enable-gpl --enable-openssl --enable-libfreetype --enable-fontconfig --enable-libharfbuzz --enable-libbluray --enable-libsnappy --enable-libsrt --enable-libvmaf --enable-libass --enable-libklvanc --enable-libzimg --enable-libzvbi --enable-libaom --enable-libdav1d --enable-libopenh264 --enable-libopenjpeg --enable-librav1e --enable-libsvtav1 --enable-libvpx --enable-libvvenc --enable-libwebp --enable-libx264 --enable-libx265 --enable-libmp3lame --enable-libopus --enable-libvorbis --enable-libtheora
```

No `--enable-nonfree` appears; both requested encoders appear. Runtime `-version`/`-encoders` was not run because host is Linux x64.

Gyan Windows 9.0.2 official SHA-256 sidecar and local archive digest both report `60f467265b1e312373dbcd92200c2618a74850f98d3d078e94296bb3fa2047ba`. Static strings from extracted PE configuration include `--enable-gpl`, `--enable-libx264`, and `--enable-libmp3lame`; no `--enable-nonfree` appears. Windows runtime was not run on Linux.

Evermeet macOS x64 binary static config contains GPL, `--enable-libx264`, and `--enable-libmp3lame`, with no `--enable-nonfree`. Runtime was not run on Linux.

The Evermeet macOS x64 archive has no digest at the checked `.sha256`, `.sha256sum`, `.md5`, or `.md5sum` URL suffixes; its SHA-256 in the manifest was recomputed locally. macOS x64 static binary strings include `--enable-libx264`, `--enable-libmp3lame`, and `--enable-gpl`, with no `--enable-nonfree`; runtime was not run on Linux. ImageMagick API release asset digests matched both downloaded assets.
