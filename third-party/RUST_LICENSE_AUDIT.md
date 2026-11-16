# Rust Third-Party License Audit

Collected with cargo-about 0.9.3 using `--locked --workspace --all-features --fail` for the release targets: x86_64-unknown-linux-gnu, x86_64-apple-darwin, aarch64-apple-darwin, x86_64-pc-windows-msvc.

The target closure contains 515 distinct third-party crate versions. The three unpublished workspace crates (bergamot-core, bergamot-tauri, os-adapter) are excluded by the `private = { ignore = true }` cargo-about policy.

Build and development dependencies are retained (`ignore-build-dependencies = false`, `ignore-dev-dependencies = false`). Cargo-about target filtering supplies the reachable graph for each triple; `cargo metadata --locked --all-features` supplies authors, repository/source identifiers, and package paths. The helper does not treat the flat metadata package list as dependency reachability.

Every emitted license body is read byte-for-byte from a package license file or a checksum-verified upstream recovery file under `third-party/license-texts/`. cargo-about's synthesized SPDX fallback text is never emitted as package text. SHA-256 values pin the exact UTF-8 file bytes.

## Scope by target

| Target | Third-party crate versions |
| --- | ---: |
| `x86_64-unknown-linux-gnu` | 453 |
| `x86_64-apple-darwin` | 345 |
| `aarch64-apple-darwin` | 345 |
| `x86_64-pc-windows-msvc` | 338 |

## Selected license branches

`AND` expressions retain every required branch. Cargo-about branch choices are matched to complete package or checksum-verified upstream files. If cargo-about selected a synthesized SPDX body absent from the package while another declared `OR` branch has complete source text, the collector records that source-backed branch and explains the choice below. The audit output preserves the package's selected SPDX branch and source evidence.

- `adler2@2.0.1` declares `0BSD OR MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `aes@0.8.4` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `ahash@0.8.12` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `aho-corasick@1.1.5` declares `Unlicense OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `anyhow@1.0.104` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `arboard@3.7.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-broadcast@0.7.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-channel@2.5.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-executor@1.14.1` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-fs@2.2.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-io@2.6.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-lock@3.5.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-process@2.5.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-recursion@1.2.0` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-signal@0.2.14` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-task@4.7.1` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `async-trait@0.1.92` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `atomic-waker@1.1.2` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `autocfg@1.5.1` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `base64@0.21.7` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `base64@0.22.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `base64@0.23.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `bit-set@0.8.0` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `bit-vec@0.8.0` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `bitflags@2.13.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `block-buffer@0.10.4` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `block-padding@0.3.3` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `blocking@1.7.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `bumpalo@3.20.4` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `bytemuck@1.25.2` declares `Zlib OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `byteorder@1.5.0` declares `Unlicense OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `byteorder-lite@0.1.0` declares `Unlicense OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `camino@1.2.6` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `cargo-platform@0.1.9` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `cargo_toml@1.0.1` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `cbc@0.1.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `cc@1.11.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `cfg-expr@0.15.8` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `cfg-if@1.0.5` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `cipher@0.4.4` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `command-group@5.0.1` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `concurrent-queue@2.5.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `constant_time_eq@0.6.1` declares `CC0-1.0 OR MIT-0 OR Apache-2.0`; selected `Apache-2.0` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `cookie@0.18.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `core-foundation@0.10.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `core-foundation-sys@0.8.7` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `core-graphics@0.25.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `core-graphics-types@0.2.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `cpufeatures@0.2.17` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `crc@3.4.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `crc-catalog@2.5.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `crc32fast@1.5.5` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `crossbeam-channel@0.5.17` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `crossbeam-utils@0.8.23` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `crypto-common@0.1.7` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `ctor@1.0.14` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `deranged@0.5.9` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `digest@0.10.7` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `directories@6.0.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `dirs@7.0.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `dirs-sys@0.5.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `dispatch2@0.3.1` declares `Zlib OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `displaydoc@0.2.7` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `dtoa@1.0.11` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `dunce@1.0.5` declares `CC0-1.0 OR MIT-0 OR Apache-2.0`; selected `CC0-1.0` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about selected Apache-2.0 but those branches had no complete package text; selected CC0-1.0 from the declared OR expression because full source-backed text was available.
- `dyn-clone@1.0.20` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `embed_plist@1.2.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `enumflags2@0.7.12` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `enumflags2_derive@0.7.12` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `equivalent@1.0.3` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `erased-serde@0.4.11` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `errno@0.3.15` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `event-listener@5.4.2` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `event-listener-strategy@0.5.4` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `fastrand@2.5.0` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `fdeflate@0.3.7` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `field-offset@0.3.6` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `find-msvc-tools@0.1.15` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `fixedbitset@0.5.9` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `flate2@1.1.11` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `form_urlencoded@1.2.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `fraction@0.15.4` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `futures-channel@0.3.34` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `futures-core@0.3.34` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `futures-executor@0.3.34` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `futures-io@0.3.34` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `futures-lite@2.6.1` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `futures-macro@0.3.34` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `futures-sink@0.3.34` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `futures-task@0.3.34` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `futures-util@0.3.34` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `getrandom@0.2.17` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `getrandom@0.3.4` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `getrandom@0.4.3` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `glob@0.3.4` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `global-hotkey@0.8.0` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `half@2.7.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `hashbrown@0.12.3` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `hashbrown@0.15.5` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `hashbrown@0.17.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `heck@0.4.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `heck@0.5.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `hex@0.4.3` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `hkdf@0.12.4` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `hmac@0.12.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `html5ever@0.39.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `http@1.5.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `httparse@1.10.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `hyper-rustls@0.27.10` declares `Apache-2.0 OR ISC OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `idna@1.1.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `idna_adapter@1.2.2` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `image@0.25.10` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `indexmap@1.9.3` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `indexmap@2.14.2` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `inout@0.1.4` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `ipnet@2.12.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `itoa@1.0.19` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `jsonptr@0.7.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `keyboard-types@0.7.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `keyboard-types@0.8.3` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `keyring@3.6.3` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `lazy_static@1.5.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `libappindicator@0.9.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `libappindicator-sys@0.9.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `libc@0.2.191` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `linux-raw-sys@0.12.1` declares `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `lock_api@0.4.14` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `log@0.4.35` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `markup5ever@0.39.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `memchr@2.8.3` declares `Unlicense OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `mime@0.3.18` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `miniz_oxide@0.8.9` declares `MIT OR Zlib OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `miniz_oxide@0.9.1` declares `MIT OR Zlib OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `moxcms@0.8.1` declares `BSD-3-Clause OR Apache-2.0`; selected `Apache-2.0` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `muda@0.20.0` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `notify-types@2.3.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `num@0.4.3` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `num-bigint@0.4.8` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `num-complex@0.4.6` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `num-conv@0.2.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `num-integer@0.1.47` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `num-iter@0.1.46` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `num-rational@0.4.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `num-traits@0.2.19` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `objc2-app-kit@0.3.2` declares `Zlib OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `objc2-core-foundation@0.3.2` declares `Zlib OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `objc2-core-graphics@0.3.2` declares `Zlib OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `objc2-core-services@0.3.2` declares `Zlib OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `objc2-exception-helper@0.1.1` declares `Zlib OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `objc2-quartz-core@0.3.2` declares `Zlib OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `objc2-web-kit@0.3.2` declares `Zlib OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `once_cell@1.21.4` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `ordered-stream@0.2.0` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `parking@2.2.1` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `parking_lot@0.12.5` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `parking_lot_core@0.9.12` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `percent-encoding@2.3.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `petgraph@0.8.3` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `pin-project-lite@0.2.17` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `piper@0.2.5` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `pkg-config@0.3.34` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `png@0.18.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `polling@3.11.0` declares `Apache-2.0 OR MIT`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `powerfmt@0.2.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `ppv-lite86@0.2.21` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `proc-macro-crate@1.3.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `proc-macro-crate@2.0.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `proc-macro-crate@3.5.0` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `proc-macro-error@1.0.4` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `proc-macro-error-attr@1.0.4` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `proc-macro2@1.0.107` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `pxfm@0.1.30` declares `BSD-3-Clause OR Apache-2.0`; selected `Apache-2.0` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `quote@1.0.47` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `rand@0.8.8` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `rand_chacha@0.3.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `rand_core@0.6.4` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `raw-window-handle@0.6.2` declares `MIT OR Apache-2.0 OR Zlib`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `ref-cast@1.0.27` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `ref-cast-impl@1.0.27` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `regex@1.13.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `regex-automata@0.4.18` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `regex-syntax@0.8.11` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `reqwest@0.12.28` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `rustc-hash@2.1.3` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `rustc_version@0.4.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `rustix@1.1.6` declares `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `rustls@0.23.46` declares `Apache-2.0 OR ISC OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `rustls-pki-types@1.16.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `rustversion@1.0.23` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `ryu@1.0.23` declares `Apache-2.0 OR BSL-1.0`; selected `Apache-2.0` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `scopeguard@1.2.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `secret-service@4.0.0` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `semver@1.0.28` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde@1.0.229` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde-untagged@0.1.9` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde_core@1.0.229` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde_derive@1.0.229` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde_derive_internals@0.29.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde_json@1.0.154` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde_repr@0.1.21` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde_spanned@0.6.9` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde_spanned@1.1.3` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde_with@3.24.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serde_with_macros@3.24.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serialize-to-javascript@0.1.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `serialize-to-javascript-impl@0.1.2` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `servo_arc@0.4.3` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `sha1@0.10.7` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `sha2@0.10.9` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `shlex@2.0.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `signal-hook-registry@1.4.8` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `siphasher@1.0.4` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `smallvec@1.16.5` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `socket2@0.6.5` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `softbuffer@0.4.8` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `stable_deref_trait@1.2.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `static_assertions@1.1.0` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `static_vcruntime@3.0.0` declares `MIT OR Apache-2.0 OR Zlib`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `string_cache@0.9.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `string_cache_codegen@0.6.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `swift-rs@1.0.9` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `syn@1.0.109` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `syn@2.0.119` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `syn@3.0.10` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `system-deps@6.2.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tar@0.4.46` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri@2.12.5` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-build@2.7.4` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-codegen@2.7.4` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-macros@2.7.4` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-plugin@2.7.2` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-plugin-clipboard-manager@2.5.0` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-plugin-dialog@2.8.4` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-plugin-fs@2.7.1` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-plugin-global-shortcut@2.4.1` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-plugin-opener@2.7.3` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-plugin-single-instance@2.5.5` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-runtime@2.12.2` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-runtime-wry@2.12.3` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tauri-utils@2.10.2` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tempfile@3.27.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tendril@0.5.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `thiserror@1.0.69` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `thiserror@2.0.21` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `thiserror-impl@1.0.69` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `thiserror-impl@2.0.21` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `time@0.3.55` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `time-core@0.1.9` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `time-macros@0.2.32` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tokio-rustls@0.26.6` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `toml@0.8.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `toml@1.2.1+spec-1.1.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `toml_datetime@0.6.3` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `toml_datetime@1.1.3+spec-1.1.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `toml_edit@0.19.15` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `toml_edit@0.20.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `toml_edit@0.25.19+spec-1.1.0` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `toml_parser@1.1.5+spec-1.1.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `toml_writer@1.2.0+spec-1.1.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `tray-icon@0.25.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `typed-path@0.13.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `typeid@1.0.3` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `typenum@1.20.1` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `unicase@2.10.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `unicode-ident@1.0.27` declares `(MIT OR Apache-2.0) AND Unicode-3.0`; selected `MIT, Unicode-3.0` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu.
- `unicode-segmentation@1.13.3` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `url@2.5.8` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `utf8_iter@1.1.0` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `utf_types@1.0.0` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `uuid@1.29.2` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `web-time@1.1.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `web_atoms@0.2.6` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `weezl@0.1.12` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `winapi-util@0.1.11` declares `Unlicense OR MIT`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `window-vibrancy@0.8.1` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows@0.62.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-collections@0.3.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-core@0.62.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-future@0.3.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-implement@0.60.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-interface@0.59.3` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-link@0.2.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-numerics@0.3.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-result@0.4.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-strings@0.5.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-sys@0.59.0` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-sys@0.60.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-sys@0.61.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-targets@0.52.6` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-targets@0.53.5` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-threading@0.2.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows-version@0.1.7` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows_x86_64_msvc@0.52.6` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `windows_x86_64_msvc@0.53.1` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-pc-windows-msvc; cargo-about branch matched a complete package or checksum-verified license file.
- `wl-clipboard-rs@0.9.5` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `wry@0.57.2` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `x11rb@0.13.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `x11rb-protocol@0.13.2` declares `MIT OR Apache-2.0`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `xattr@1.7.0` declares `MIT OR Apache-2.0`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `xkeysym@0.2.1` declares `MIT OR Apache-2.0 OR Zlib`; selected `MIT` for x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `zerocopy@0.8.63` declares `BSD-2-Clause OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `zerocopy-derive@0.8.63` declares `BSD-2-Clause OR Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `zeroize@1.9.1` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `zeroize_derive@1.5.0` declares `Apache-2.0 OR MIT`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu; cargo-about branch matched a complete package or checksum-verified license file.
- `zune-core@0.5.3` declares `MIT OR Apache-2.0 OR Zlib`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.
- `zune-jpeg@0.5.15` declares `MIT OR Apache-2.0 OR Zlib`; selected `MIT` for aarch64-apple-darwin, x86_64-apple-darwin; cargo-about branch matched a complete package or checksum-verified license file.

## Provenance and text hashes

| Crate | Declared expression | Selected SPDX IDs | License SHA-256 | Source |
| --- | --- | --- | --- | --- |
| `adler2@2.0.1` | `0BSD OR MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/oyvindln/adler2 |
| `aes@0.8.4` | `MIT OR Apache-2.0` | `MIT` | `f7e8ab639afef15573680c97f796166835cbeb3865175882fea41c60d106b733` | https://github.com/RustCrypto/block-ciphers |
| `ahash@0.8.12` | `MIT OR Apache-2.0` | `MIT` | `0444c6991eead6822f7b9102e654448d51624431119546492e8b231db42c48bb` | https://github.com/tkaitchuck/ahash |
| `aho-corasick@1.1.5` | `Unlicense OR MIT` | `MIT` | `0f96a83840e146e43c0ec96a22ec1f392e0680e6c1226e6f3ba87e0740af850f` | https://github.com/BurntSushi/aho-corasick |
| `alloc-no-stdlib@3.0.0` | `BSD-3-Clause` | `BSD-3-Clause` | `c0c56f26d9c051cac4d200c34c84e7ae9aaa853e01a982a1df08b09931e518ae` | https://github.com/dropbox/rust-alloc-no-stdlib |
| `alloc-stdlib@0.3.0` | `BSD-3-Clause` | `BSD-3-Clause` | `c0c56f26d9c051cac4d200c34c84e7ae9aaa853e01a982a1df08b09931e518ae` | https://github.com/dropbox/rust-alloc-no-stdlib |
| `anyhow@1.0.104` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/anyhow |
| `arboard@3.7.0` | `MIT OR Apache-2.0` | `MIT` | `f35468024534e0f4f16bb94cd0e88c87fbd1314f919c2645a07d350fb77b40c0` | https://github.com/1Password/arboard |
| `async-broadcast@0.7.2` | `MIT OR Apache-2.0` | `MIT` | `24e5860bf589d8501643e6ea51ffb3df66db2867492b09033d486183efbfa970` | https://github.com/smol-rs/async-broadcast |
| `async-channel@2.5.0` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/async-channel |
| `async-executor@1.14.1` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/async-executor |
| `async-fs@2.2.0` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/async-fs |
| `async-io@2.6.0` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/async-io |
| `async-lock@3.5.0` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/async-lock |
| `async-process@2.5.0` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/async-process |
| `async-recursion@1.2.0` | `MIT OR Apache-2.0` | `MIT` | `cdbd06e25b8c9c5d6019949d2de8123f56d29cee4166396423d2ba8c81700845` | https://github.com/dcchut/async-recursion |
| `async-signal@0.2.14` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/async-signal |
| `async-task@4.7.1` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/async-task |
| `async-trait@0.1.92` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/async-trait |
| `atk@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `atk-sys@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `atomic-waker@1.1.2` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3`, `6226d0632e2e1a80c23597e964da9812ae193c535fe058154afb034e94167aa5` | https://github.com/smol-rs/atomic-waker |
| `autocfg@1.5.1` | `Apache-2.0 OR MIT` | `MIT` | `27995d58ad5c1145c1a8cd86244ce844886958a35eb2b78c6b772748669999ac` | https://github.com/cuviper/autocfg |
| `base64@0.21.7` | `MIT OR Apache-2.0` | `MIT` | `0dd882e53de11566d50f8e8e2d5a651bcf3fabee4987d70f306233cf39094ba7` | https://github.com/marshallpierce/rust-base64 |
| `base64@0.22.1` | `MIT OR Apache-2.0` | `MIT` | `0dd882e53de11566d50f8e8e2d5a651bcf3fabee4987d70f306233cf39094ba7` | https://github.com/marshallpierce/rust-base64 |
| `base64@0.23.1` | `MIT OR Apache-2.0` | `MIT` | `ab499c75a0f0da8e0fe83bf9ba3a27e5c39705310f07107c6c66b69958d2401c` | https://github.com/marshallpierce/rust-base64 |
| `bit-set@0.8.0` | `Apache-2.0 OR MIT` | `MIT` | `f51ac2c59a222f7476ce507ca879960e2b64ea64bb2786eefdbeb7b0b538d1b7` | https://github.com/contain-rs/bit-set |
| `bit-vec@0.8.0` | `Apache-2.0 OR MIT` | `MIT` | `f51ac2c59a222f7476ce507ca879960e2b64ea64bb2786eefdbeb7b0b538d1b7` | https://github.com/contain-rs/bit-vec |
| `bitflags@1.3.2` | `MIT/Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/bitflags/bitflags |
| `bitflags@2.13.2` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/bitflags/bitflags |
| `block-buffer@0.10.4` | `MIT OR Apache-2.0` | `MIT` | `d5c22aa3118d240e877ad41c5d9fa232f9c77d757d4aac0c2f943afc0a95e0ef` | https://github.com/RustCrypto/utils |
| `block-padding@0.3.3` | `MIT OR Apache-2.0` | `MIT` | `d5c22aa3118d240e877ad41c5d9fa232f9c77d757d4aac0c2f943afc0a95e0ef` | https://github.com/RustCrypto/utils |
| `block2@0.6.2` | `MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `blocking@1.7.0` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/blocking |
| `borrow-or-share@0.2.4` | `MIT-0` | `MIT-0` | `010492e490b92ec85db01daad2a290ea5959f29314a1c4ff25e7a172e223abf9` | https://github.com/yescallop/borrow-or-share |
| `brotli@9.1.1` | `BSD-3-Clause AND MIT` | `BSD-3-Clause, MIT` | `3d180008e36922a4e8daec11c34c7af264fed5962d07924aea928c38e8663c94`, `c0c56f26d9c051cac4d200c34c84e7ae9aaa853e01a982a1df08b09931e518ae` | https://github.com/dropbox/rust-brotli |
| `brotli-decompressor@6.3.1` | `BSD-3-Clause/MIT` | `BSD-3-Clause` | `c0c56f26d9c051cac4d200c34c84e7ae9aaa853e01a982a1df08b09931e518ae` | https://github.com/dropbox/rust-brotli-decompressor |
| `bumpalo@3.20.4` | `MIT OR Apache-2.0` | `MIT` | `65f94e99ddaf4f5d1782a6dae23f35d4293a9a01444a13135a6887017d353cee` | https://github.com/fitzgen/bumpalo |
| `bytecount@0.6.9` | `Apache-2.0/MIT` | `MIT` | `a5dea80c1f383cb5f80a6bb0da5e55a2beb9f24adb123ce6300af2cbaaa3bf65` | https://github.com/llogiq/bytecount |
| `bytemuck@1.25.2` | `Zlib OR Apache-2.0 OR MIT` | `MIT` | `9df9ba60a11af705f2e451b53762686e615d86f76b169cf075c3237730dbd7e2` | https://github.com/Lokathor/bytemuck |
| `byteorder@1.5.0` | `Unlicense OR MIT` | `MIT` | `0f96a83840e146e43c0ec96a22ec1f392e0680e6c1226e6f3ba87e0740af850f` | https://github.com/BurntSushi/byteorder |
| `byteorder-lite@0.1.0` | `Unlicense OR MIT` | `MIT` | `0f96a83840e146e43c0ec96a22ec1f392e0680e6c1226e6f3ba87e0740af850f` | https://github.com/image-rs/byteorder-lite |
| `bytes@1.12.1` | `MIT` | `MIT` | `45f522cacecb1023856e46df79ca625dfc550c94910078bd8aec6e02880b3d42` | https://github.com/tokio-rs/bytes |
| `cairo-rs@0.18.5` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `cairo-sys-rs@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `camino@1.2.6` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/camino-rs/camino |
| `cargo-platform@0.1.9` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/rust-lang/cargo |
| `cargo_metadata@0.19.2` | `MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/oli-obk/cargo_metadata |
| `cargo_toml@1.0.1` | `Apache-2.0 OR MIT` | `MIT` | `ee874098519e956c23a3c5c7e20581f8e9b1d70aa05eb178f983be9775090a2f` | https://gitlab.com/lib.rs/cargo_toml |
| `cbc@0.1.2` | `MIT OR Apache-2.0` | `MIT` | `63af4bea227c94d021e99427f6ca3a4b8efddadcca93ab6130f708cf6138cf68` | https://github.com/RustCrypto/block-modes |
| `cc@1.11.2` | `MIT OR Apache-2.0` | `MIT` | `378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` | https://github.com/rust-lang/cc-rs |
| `cfb@0.14.0` | `MIT` | `MIT` | `ebaac853b53fec0ed475796f533ce676912a4df9ad9d8a621fa76d5060591e10` | https://github.com/mdsteele/rust-cfb |
| `cfg-expr@0.15.8` | `MIT OR Apache-2.0` | `MIT` | `090a294a492ab2f41388252312a65cf2f0e423330b721a68c6665ac64766753b` | https://github.com/EmbarkStudios/cfg-expr |
| `cfg-if@1.0.5` | `MIT OR Apache-2.0` | `MIT` | `378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` | https://github.com/rust-lang/cfg-if |
| `cfg_aliases@0.2.2` | `MIT` | `MIT` | `1e2b7ade3fb228130408b9990cae6a7618eb314c75aa0b164bfe485d9d9756ee`, `31b94860253d8ec7b4529f51901044d3b459d6292d996504a36b1bae3a36a812` | https://github.com/katharostech/cfg_aliases |
| `cipher@0.4.4` | `MIT OR Apache-2.0` | `MIT` | `5c7bd92d1f096f12203dc1b601e3cb17484fa1b023e30b6b8edcc80e416237ec` | https://github.com/RustCrypto/traits |
| `clipboard-win@5.4.3` | `BSL-1.0` | `BSL-1.0` | `c9bff75738922193e67fa726fa225535870d2aa1059f91452c411736284ad566` | https://github.com/DoumanAsh/clipboard-win |
| `command-group@5.0.1` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/watchexec/command-group |
| `concurrent-queue@2.5.0` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/concurrent-queue |
| `constant_time_eq@0.6.1` | `CC0-1.0 OR MIT-0 OR Apache-2.0` | `Apache-2.0` | `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a` | https://codeberg.org/cesarb/constant_time_eq |
| `cookie@0.18.2` | `MIT OR Apache-2.0` | `MIT` | `f28420c1906af38be726cd7842798f0194b27c41d6051c0d1e9ee348b8a4a0ea` | https://github.com/SergioBenitez/cookie-rs |
| `core-foundation@0.10.1` | `MIT OR Apache-2.0` | `MIT` | `62065228e42caebca7e7d7db1204cbb867033de5982ca4009928915e4095f3a3` | https://github.com/servo/core-foundation-rs |
| `core-foundation-sys@0.8.7` | `MIT OR Apache-2.0` | `MIT` | `62065228e42caebca7e7d7db1204cbb867033de5982ca4009928915e4095f3a3` | https://github.com/servo/core-foundation-rs |
| `core-graphics@0.25.0` | `MIT OR Apache-2.0` | `MIT` | `62065228e42caebca7e7d7db1204cbb867033de5982ca4009928915e4095f3a3` | https://github.com/servo/core-foundation-rs |
| `core-graphics-types@0.2.0` | `MIT OR Apache-2.0` | `MIT` | `62065228e42caebca7e7d7db1204cbb867033de5982ca4009928915e4095f3a3` | https://github.com/servo/core-foundation-rs |
| `cpufeatures@0.2.17` | `MIT OR Apache-2.0` | `MIT` | `ae9baa7beea910273c2f384c2a6b721fb7bd02bda3436074a1072e4ee689f985` | https://github.com/RustCrypto/utils |
| `crc@3.4.0` | `MIT OR Apache-2.0` | `MIT` | `3488679340a49ecc34d342c4009d2dabf76f4a21f12aec2ca99b15805d656544` | https://github.com/mrhooray/crc-rs.git |
| `crc-catalog@2.5.0` | `MIT OR Apache-2.0` | `MIT` | `5ef8fcfb6cccec8fcae043c834099a60c8b7406408db576e026d2b7e67dc5cf5` | https://github.com/akhilles/crc-catalog.git |
| `crc32fast@1.5.5` | `MIT OR Apache-2.0` | `MIT` | `61d383b05b87d78f94d2937e2580cce47226d17823c0430fbcad09596537efcf` | https://github.com/srijs/rust-crc32fast |
| `crossbeam-channel@0.5.17` | `MIT OR Apache-2.0` | `MIT` | `5734ed989dfca1f625b40281ee9f4530f91b2411ec01cb748223e7eb87e201ab`, `b16db96b93b1d7cf7bea533f572091ec6bca3234fbe0a83038be772ff391a44c` | https://github.com/crossbeam-rs/crossbeam |
| `crossbeam-utils@0.8.23` | `MIT OR Apache-2.0` | `MIT` | `5734ed989dfca1f625b40281ee9f4530f91b2411ec01cb748223e7eb87e201ab` | https://github.com/crossbeam-rs/crossbeam |
| `crypto-common@0.1.7` | `MIT OR Apache-2.0` | `MIT` | `3521672491a3479422d5fe1aca6645dd2984090f85da6e5205abfb18fb7a6897` | https://github.com/RustCrypto/traits |
| `cssparser@0.37.0` | `MPL-2.0` | `MPL-2.0` | `fab3dd6bdab226f1c08630b1dd917e11fcb4ec5e1e020e2c16f83a0a13863e85` | https://github.com/servo/rust-cssparser |
| `cssparser-macros@0.7.1` | `MPL-2.0` | `MPL-2.0` | `fab3dd6bdab226f1c08630b1dd917e11fcb4ec5e1e020e2c16f83a0a13863e85` | https://github.com/servo/rust-cssparser |
| `ctor@1.0.14` | `Apache-2.0 OR MIT` | `MIT` | `bccaa8b6c09f94e81f06696e179dbe058464bbdfbc823b6d49cada1d71e84ac3` | https://github.com/mmastrac/linktime |
| `darling@0.24.1` | `MIT` | `MIT` | `8ea93490d74a5a1b1af3ff71d786271b3f1e5f0bea79ac16e02ec533cef040d6` | https://github.com/TedDriggs/darling |
| `darling_core@0.24.1` | `MIT` | `MIT` | `8ea93490d74a5a1b1af3ff71d786271b3f1e5f0bea79ac16e02ec533cef040d6` | https://github.com/TedDriggs/darling |
| `darling_macro@0.24.1` | `MIT` | `MIT` | `8ea93490d74a5a1b1af3ff71d786271b3f1e5f0bea79ac16e02ec533cef040d6` | https://github.com/TedDriggs/darling |
| `dbus@0.9.12` | `Apache-2.0/MIT` | `MIT` | `de3911c2d98c8bd2d701ee721347053d9b55995a11f9a8c955e44d3ca1b376bf` | https://github.com/diwic/dbus-rs |
| `deranged@0.5.9` | `MIT OR Apache-2.0` | `MIT` | `231c837c45eb53f108fb48929e488965bc4fcc14e9ea21d35f50e6b99d98685b` | https://github.com/jhpratt/deranged |
| `derive_more@2.1.1` | `MIT` | `MIT` | `8a35369f3ca263b3c62fbb5032947e53b6bfebc6c8a4d1bb982de1c069f6fba5` | https://github.com/JelteF/derive_more |
| `derive_more-impl@2.1.1` | `MIT` | `MIT` | `8a35369f3ca263b3c62fbb5032947e53b6bfebc6c8a4d1bb982de1c069f6fba5` | https://github.com/JelteF/derive_more |
| `digest@0.10.7` | `MIT OR Apache-2.0` | `MIT` | `9e0dfd2dd4173a530e238cb6adb37aa78c34c6bc7444e0e10c1ab5d8881f63ba` | https://github.com/RustCrypto/traits |
| `directories@6.0.0` | `MIT OR Apache-2.0` | `MIT` | `11955c617c19899b5c36d78c916739d1686f2e7ad8e3cec684c2f0f73b0db950` | https://github.com/soc/directories-rs |
| `dirs@7.0.0` | `MIT OR Apache-2.0` | `MIT` | `6a2e0ade09a7d5f816f11566fee2b151b32235a7fad52b41d49cce96f833c1a9` | https://codeberg.org/dirs/dirs-rs |
| `dirs-sys@0.5.0` | `MIT OR Apache-2.0` | `MIT` | `6a2e0ade09a7d5f816f11566fee2b151b32235a7fad52b41d49cce96f833c1a9` | https://github.com/dirs-dev/dirs-sys-rs |
| `dispatch2@0.3.1` | `Zlib OR Apache-2.0 OR MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `displaydoc@0.2.7` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/yaahc/displaydoc |
| `dlopen2@0.8.2` | `MIT` | `MIT` | `39fa265207450e77c62e90c5594a06c085b655d8374c7ced4bf7894b6bd95dd2` | https://github.com/OpenByteDev/dlopen2 |
| `dlopen2_derive@0.4.3` | `MIT` | `MIT` | `39fa265207450e77c62e90c5594a06c085b655d8374c7ced4bf7894b6bd95dd2` | https://github.com/OpenByteDev/dlopen2 |
| `dom_query@0.28.0` | `MIT` | `MIT` | `7554701d1e74fc434f68c522b1dd743c8c16d816a4b761b5213f367b898884c8` | https://github.com/niklak/dom_query |
| `downcast-rs@1.2.1` | `MIT/Apache-2.0` | `MIT` | `fb252fecf98d9af8f1d9201c5860319a514f5a8a24700dc84fc6ea287a7a4958` | https://github.com/marcianx/downcast-rs |
| `dpi@0.1.2` | `Apache-2.0 AND MIT` | `Apache-2.0, MIT` | `6dc0e068dcf3a5bc8e054205b85b7720e1d49265bbc64bf515d2cf79197df69a`, `9eff396a99bafd208e7126602420d6864f6066c08b84485a5b032320a9c01997` | https://github.com/rust-windowing/winit |
| `dtoa@1.0.11` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/dtoa |
| `dtoa-short@0.3.5` | `MPL-2.0` | `MPL-2.0` | `1f256ecad192880510e84ad60474eab7589218784b9a50bc7ceee34c2b91f1d5` | https://github.com/upsuper/dtoa-short |
| `dunce@1.0.5` | `CC0-1.0 OR MIT-0 OR Apache-2.0` | `CC0-1.0` | `a2010f343487d3f7618affe54f789f5487602331c0a8d03f49e9a7c547cf0499` | https://gitlab.com/kornelski/dunce |
| `dyn-clone@1.0.20` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/dyn-clone |
| `email_address@0.2.9` | `MIT` | `MIT` | `39712155836e5c10ef1b07037b606e4ad2d633e72205d162fdd7e2290aa54f15` | https://github.com/johnstonskj/rust-email_address.git |
| `embed-resource@3.0.12` | `MIT` | `MIT` | `1abdbbd4e3f95e56c79d4a8af190254c23ddb14db34aadadfafef9c3af041113` | https://github.com/nabijaczleweli/rust-embed-resource |
| `embed_plist@1.2.2` | `MIT OR Apache-2.0` | `MIT` | `daacc37f680c7dd0a758ac99a109f2f57d9f21d3683df9e9245055922842a695` | https://github.com/nvzqz/embed-plist-rs |
| `endi@1.1.1` | `MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/zeenix/endi |
| `enumflags2@0.7.12` | `MIT OR Apache-2.0` | `MIT` | `2e3edd2716fb15616e867bd53f724f0906732242e9ae76bb1faad1d88f7eeebd` | https://github.com/meithecatte/enumflags2 |
| `enumflags2_derive@0.7.12` | `MIT OR Apache-2.0` | `MIT` | `d13669f0eaff9b7015bff0d8637297213b53d9f239826636c07a15633acf7052` | https://github.com/meithecatte/enumflags2 |
| `equivalent@1.0.3` | `Apache-2.0 OR MIT` | `MIT` | `7365cc8878a1d7ce155a58c4ca09c3d7a6be413efa5334a80ea842912b669349` | https://github.com/indexmap-rs/equivalent |
| `erased-serde@0.4.11` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/erased-serde |
| `errno@0.3.15` | `MIT OR Apache-2.0` | `MIT` | `8764a597675778ddfd4e25f81b08a05dbcf089ac05662df7613fe67f150e3aa2` | https://github.com/lambda-fairy/rust-errno |
| `error-code@3.4.0` | `BSL-1.0` | `BSL-1.0` | `c9bff75738922193e67fa726fa225535870d2aa1059f91452c411736284ad566` | https://github.com/DoumanAsh/error-code |
| `event-listener@5.4.2` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/event-listener |
| `event-listener-strategy@0.5.4` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/event-listener-strategy |
| `fancy-regex@0.16.2` | `MIT` | `MIT` | `3bc70e239e91272782006c638fc0452a714d384224f11f0923036b7be07cf9b5` | https://github.com/fancy-regex/fancy-regex |
| `fastrand@2.5.0` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/fastrand |
| `fax@0.2.7` | `MIT` | `MIT` | `7101390b4c8edd752f4c0de47f35d61b44d9cf0ee8dd032115e25644e277cfff` | https://github.com/pdf-rs/fax |
| `fdeflate@0.3.7` | `MIT OR Apache-2.0` | `MIT` | `c77a4cf9da729987d0fe7ccd811e3bd27393914ddf3d23467c18cc22954513b3` | https://github.com/image-rs/fdeflate |
| `field-offset@0.3.6` | `MIT OR Apache-2.0` | `MIT` | `9dd44491b4cf95dcfa2963943b3ae91bff9a78d13049a9e0b53e0d0b0a9c8f4e` | https://github.com/Diggsey/rust-field-offset |
| `filetime@0.2.29` | `MIT/Apache-2.0` | `MIT` | `378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` | https://github.com/alexcrichton/filetime |
| `find-msvc-tools@0.1.15` | `MIT OR Apache-2.0` | `MIT` | `378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` | https://github.com/rust-lang/cc-rs |
| `fixedbitset@0.5.9` | `MIT OR Apache-2.0` | `MIT` | `ce592787ff2321feab698a4c612237f4378cc658ebb1d472913e5802cc47afb4` | https://github.com/petgraph/fixedbitset |
| `flate2@1.1.11` | `MIT OR Apache-2.0` | `MIT` | `025436edff4cfcdde17a5811fdea78892d8482efd1abdec5a17872d07a4f2112` | https://github.com/rust-lang/flate2-rs |
| `fluent-uri@0.3.2` | `MIT` | `MIT` | `9fa38f610a00115dd1f612bdd3559e3b381ea579538b6ff25a5899da250a5fce` | https://github.com/yescallop/fluent-uri-rs |
| `fnv@1.0.7` | `Apache-2.0 / MIT` | `MIT` | `65fdb6c76cd61612070c066eec9ecdb30ee74fb27859d0d9af58b9f499fd0c3e` | https://github.com/servo/rust-fnv |
| `foldhash@0.1.5` | `Zlib` | `Zlib` | `b1181a40b2a7b25cf66fd01481713bc1005df082c53ef73e851e55071b102744` | https://github.com/orlp/foldhash |
| `foldhash@0.2.0` | `Zlib` | `Zlib` | `b1181a40b2a7b25cf66fd01481713bc1005df082c53ef73e851e55071b102744` | https://github.com/orlp/foldhash |
| `foreign-types@0.5.0` | `MIT/Apache-2.0` | `MIT` | `333ea3aaa3cadb819f4acd9f9153f9feee060a995ca8710f32bc5bd9a4b91734` | https://github.com/sfackler/foreign-types |
| `foreign-types-macros@0.2.4` | `MIT/Apache-2.0` | `MIT` | `333ea3aaa3cadb819f4acd9f9153f9feee060a995ca8710f32bc5bd9a4b91734` | https://github.com/sfackler/foreign-types |
| `foreign-types-shared@0.3.1` | `MIT/Apache-2.0` | `MIT` | `333ea3aaa3cadb819f4acd9f9153f9feee060a995ca8710f32bc5bd9a4b91734` | https://github.com/sfackler/foreign-types |
| `form_urlencoded@1.2.2` | `MIT OR Apache-2.0` | `MIT` | `20c7855c364d57ea4c97889a5e8d98470a9952dade37bd9248b9a54431670e5e` | https://github.com/servo/rust-url |
| `fraction@0.15.4` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/dnsl48/fraction.git |
| `futures-channel@0.3.34` | `MIT OR Apache-2.0` | `MIT` | `6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd` | https://github.com/rust-lang/futures-rs |
| `futures-core@0.3.34` | `MIT OR Apache-2.0` | `MIT` | `6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd` | https://github.com/rust-lang/futures-rs |
| `futures-executor@0.3.34` | `MIT OR Apache-2.0` | `MIT` | `6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd` | https://github.com/rust-lang/futures-rs |
| `futures-io@0.3.34` | `MIT OR Apache-2.0` | `MIT` | `6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd` | https://github.com/rust-lang/futures-rs |
| `futures-lite@2.6.1` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3`, `6226d0632e2e1a80c23597e964da9812ae193c535fe058154afb034e94167aa5` | https://github.com/smol-rs/futures-lite |
| `futures-macro@0.3.34` | `MIT OR Apache-2.0` | `MIT` | `6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd` | https://github.com/rust-lang/futures-rs |
| `futures-sink@0.3.34` | `MIT OR Apache-2.0` | `MIT` | `6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd` | https://github.com/rust-lang/futures-rs |
| `futures-task@0.3.34` | `MIT OR Apache-2.0` | `MIT` | `6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd` | https://github.com/rust-lang/futures-rs |
| `futures-util@0.3.34` | `MIT OR Apache-2.0` | `MIT` | `6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd` | https://github.com/rust-lang/futures-rs |
| `gdk@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `gdk-pixbuf@0.18.5` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `gdk-pixbuf-sys@0.18.0` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `gdk-sys@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `gdkwayland-sys@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `gdkx11@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `gdkx11-sys@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `generic-array@0.14.7` | `MIT` | `MIT` | `c09aae9d3c77b531f56351a9947bc7446511d6b025b3255312d3e3442a9a7583` | https://github.com/fizyk20/generic-array.git |
| `gethostname@1.1.0` | `Apache-2.0` | `Apache-2.0` | `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2` | https://codeberg.org/swsnr/gethostname.rs.git |
| `getrandom@0.2.17` | `MIT OR Apache-2.0` | `MIT` | `42fa16951ce7f24b5a467a40e5b449a1d41e662f97ca779864f053f39e097737` | https://github.com/rust-random/getrandom |
| `getrandom@0.3.4` | `MIT OR Apache-2.0` | `MIT` | `29e9fe5074bd27e0e5d5d110394fbbcd841baee2651a3c4b4560a632702cede4` | https://github.com/rust-random/getrandom |
| `getrandom@0.4.3` | `MIT OR Apache-2.0` | `MIT` | `523a42c25d245dde9c015f882cec7f4555aad883382a6cf19b4b7d9b2cd5419b` | https://github.com/rust-random/getrandom |
| `gio@0.18.4` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `gio-sys@0.18.1` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `glib@0.18.5` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `glib-macros@0.18.5` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `glib-sys@0.18.1` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `glob@0.3.4` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-lang/glob |
| `global-hotkey@0.8.0` | `Apache-2.0 OR MIT` | `MIT` | `2ab5537b8c0cb1d475e2145b6a7994b04e7c71e6e5bd836f7f9c47c221a5ad9a` | https://github.com/tauri-apps/global-hotkey |
| `gobject-sys@0.18.0` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `gtk@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `gtk-sys@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `gtk3-macros@0.18.2` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk3-rs |
| `half@2.7.1` | `MIT OR Apache-2.0` | `MIT` | `508a77d2e7b51d98adeed32648ad124b7b30241a8e70b2e72c99f92d8e5874d1` | https://github.com/VoidStarKat/half-rs |
| `hashbrown@0.12.3` | `MIT OR Apache-2.0` | `MIT` | `ff8f68cb076caf8cefe7a6430d4ac086ce6af2ca8ce2c4e5a2004d4552ef52a2` | https://github.com/rust-lang/hashbrown |
| `hashbrown@0.15.5` | `MIT OR Apache-2.0` | `MIT` | `ff8f68cb076caf8cefe7a6430d4ac086ce6af2ca8ce2c4e5a2004d4552ef52a2` | https://github.com/rust-lang/hashbrown |
| `hashbrown@0.17.1` | `MIT OR Apache-2.0` | `MIT` | `ff8f68cb076caf8cefe7a6430d4ac086ce6af2ca8ce2c4e5a2004d4552ef52a2` | https://github.com/rust-lang/hashbrown |
| `heck@0.4.1` | `MIT OR Apache-2.0` | `MIT` | `7b63ecd5f1902af1b63729947373683c32745c16a10e8e6292e2e2dcd7e90ae0` | https://github.com/withoutboats/heck |
| `heck@0.5.0` | `MIT OR Apache-2.0` | `MIT` | `7b63ecd5f1902af1b63729947373683c32745c16a10e8e6292e2e2dcd7e90ae0` | https://github.com/withoutboats/heck |
| `hex@0.4.3` | `MIT OR Apache-2.0` | `MIT` | `f7bdb3426d045cd50efd4953026e3eb5a83d0199f458a075602611b9344da5b9` | https://github.com/KokaKiwi/rust-hex |
| `hkdf@0.12.4` | `MIT OR Apache-2.0` | `MIT` | `d288f9c9b4590446ec18c22ead8f8b5a12a3d4025b68f62dc9015063eb9cca69` | https://github.com/RustCrypto/KDFs/ |
| `hmac@0.12.1` | `MIT OR Apache-2.0` | `MIT` | `9e0dfd2dd4173a530e238cb6adb37aa78c34c6bc7444e0e10c1ab5d8881f63ba` | https://github.com/RustCrypto/MACs |
| `html5ever@0.39.0` | `MIT OR Apache-2.0` | `MIT` | `86dd7f026f916daf7511e39951ad8ea8cf55a8db67ae64060dacf829761c18f3` | https://github.com/servo/html5ever |
| `http@1.5.0` | `MIT OR Apache-2.0` | `MIT` | `dc91f8200e4b2a1f9261035d4c18c33c246911a6c0f7b543d75347e61b249cff` | https://github.com/hyperium/http |
| `http-body@1.1.0` | `MIT` | `MIT` | `248378d0a3383c173fb925f17141b88e71580b3ba17ddc6ac3d2a344683232ab` | https://github.com/hyperium/http-body |
| `http-body-util@0.1.5` | `MIT` | `MIT` | `248378d0a3383c173fb925f17141b88e71580b3ba17ddc6ac3d2a344683232ab` | https://github.com/hyperium/http-body |
| `httparse@1.10.1` | `MIT OR Apache-2.0` | `MIT` | `391a5396cec6230bfabd4ef4eb2350eb895bc5efce377a2218f5702ed020d3e3` | https://github.com/seanmonstar/httparse |
| `hyper@1.12.0` | `MIT` | `MIT` | `2d01890414494742ba4a509fcec8efa40f6d8be22cbd72be7cff08d6fda4ec89` | https://github.com/hyperium/hyper |
| `hyper-rustls@0.27.10` | `Apache-2.0 OR ISC OR MIT` | `MIT` | `709e3175b4212f7b13aa93971c9f62ff8c69ec45ad8c6532a7e0c41d7a7d6f8c` | https://github.com/rustls/hyper-rustls |
| `hyper-util@0.1.21` | `MIT` | `MIT` | `9e0a97848ea543aef745c98e84fde696a9a3e0735538f6daefdd3cb1942effc1` | https://github.com/hyperium/hyper-util |
| `ico@0.6.0` | `MIT` | `MIT` | `41ca90213441b294e7517d5858f0cc85bc12e19ec044001b45bc213de461d33b` | https://github.com/mdsteele/rust-ico |
| `icu_collections@2.4.0` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `icu_locale_core@2.4.0` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `icu_normalizer@2.4.0` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `icu_normalizer_data@2.4.0` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `icu_properties@2.4.0` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `icu_properties_data@2.4.0` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `icu_provider@2.4.0` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `ident_case@1.0.1` | `MIT/Apache-2.0` | `MIT` | `508a77d2e7b51d98adeed32648ad124b7b30241a8e70b2e72c99f92d8e5874d1` | https://github.com/TedDriggs/ident_case |
| `idna@1.1.0` | `MIT OR Apache-2.0` | `MIT` | `b38f11f6096706e6de553dabe2a7ed142d59b6fa8c97e290c67496154745cdd5` | https://github.com/servo/rust-url/ |
| `idna_adapter@1.2.2` | `Apache-2.0 OR MIT` | `MIT` | `8b43ce8accd61e9d370b5ca9e9c4f953279b5c239926c62315b40e24df51b726` | https://github.com/hsivonen/idna_adapter |
| `image@0.25.10` | `MIT OR Apache-2.0` | `MIT` | `c77a4cf9da729987d0fe7ccd811e3bd27393914ddf3d23467c18cc22954513b3` | https://github.com/image-rs/image |
| `indexmap@1.9.3` | `Apache-2.0 OR MIT` | `MIT` | `ecc269ef87fd38a1d98e30bfac9ba964a9dbd9315c3770fed98d4d7cb5882055` | https://github.com/bluss/indexmap |
| `indexmap@2.14.2` | `Apache-2.0 OR MIT` | `MIT` | `ecc269ef87fd38a1d98e30bfac9ba964a9dbd9315c3770fed98d4d7cb5882055` | https://github.com/indexmap-rs/indexmap |
| `infer@0.22.0` | `MIT` | `MIT` | `270b0d81891aa44c7e8726cf9432dade9556a01c848bacd2f3ef839c9c1a1859` | https://github.com/bojand/infer |
| `inotify@0.11.5` | `ISC` | `ISC` | `7f78b31de4e7a3e2ac8de40e2b409eda313f03637aa62b8416fbde9bdc5ce4c9` | https://github.com/hannobraun/inotify-rs |
| `inotify-sys@0.1.8` | `ISC` | `ISC` | `f68878c9ca61b25d4b2c470de571cb85ce97804a62bb153607f7e0e0d2684c8a` | https://github.com/hannobraun/inotify-sys |
| `inout@0.1.4` | `MIT OR Apache-2.0` | `MIT` | `304b898acad7f02e03d6d8832d682a3b43ee729ab5d3a069af5a36979083b6b4` | https://github.com/RustCrypto/utils |
| `ipnet@2.12.2` | `MIT OR Apache-2.0` | `MIT` | `47dc9ff29128ddfb4d6a0435383c9f89120bc374dbcc1dd00b933a0b28aa7865` | https://github.com/krisprice/ipnet |
| `is-docker@0.2.0` | `MIT` | `MIT` | `7377e48a67377f0baa9ccee6425e125eb7b96d5b8e706f4c99479ca4ffa21b77` | https://github.com/TheLarkInn/is-docker |
| `is-wsl@0.4.0` | `MIT` | `MIT` | `7377e48a67377f0baa9ccee6425e125eb7b96d5b8e706f4c99479ca4ffa21b77` | https://github.com/TheLarkInn/is-wsl |
| `itoa@1.0.19` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/itoa |
| `javascriptcore-rs@1.1.2` | `MIT` | `MIT` | `5444779e62a6f136892955bab2727694509b2cc1c0cd0f394d5edc7ff9203cd1` | https://github.com/tauri-apps/javascriptcore-rs |
| `javascriptcore-rs-sys@1.1.1` | `MIT` | `MIT` | `7ade43ec5e1a5e43000aa871d290dff4c0e9e2e16b7f6bea2ecf091e3d056d31` | https://github.com/tauri-apps/javascriptcore-rs |
| `json-patch@4.2.0` | `MIT/Apache-2.0` | `MIT` | `87f7527b1066337751f7f4d717089bf514ff8f7ee7d66c4c5b891b09e744ec11` | https://github.com/idubrov/json-patch |
| `jsonptr@0.7.1` | `MIT OR Apache-2.0` | `MIT` | `2bc107bdc18a74263d20bdefdb8073e17bc1a01b6b374dc5835683a33ee5ba7d` | https://github.com/chanced/jsonptr |
| `jsonschema@0.33.0` | `MIT` | `MIT` | `a573f030c2ae7eab837ab7d66440954ae54c5eabe6848c1a1fe9187215f50591` | https://github.com/Stranger6667/jsonschema |
| `keyboard-types@0.7.0` | `MIT OR Apache-2.0` | `MIT` | `31dbbab009f1b2e59a1622d5955dfafc38ebe834000215b49d954c6f85fe927c` | https://github.com/pyfisch/keyboard-types |
| `keyboard-types@0.8.3` | `MIT OR Apache-2.0` | `MIT` | `31dbbab009f1b2e59a1622d5955dfafc38ebe834000215b49d954c6f85fe927c` | https://github.com/rust-windowing/keyboard-types |
| `keyring@3.6.3` | `MIT OR Apache-2.0` | `MIT` | `157cc607717b1b372a69f6b4526f6ad18c1279af9f3267835cca2943f37bc07d` | https://github.com/hwchen/keyring-rs.git |
| `lazy_static@1.5.1` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/rust-lang-nursery/lazy-static.rs |
| `libappindicator@0.9.0` | `Apache-2.0 OR MIT` | `MIT` | `eb227437252b2a7a9c1fc342c93ade1f3d7ce38cc6dd754f613db07d53ceff0b` | registry+https://github.com/rust-lang/crates.io-index |
| `libappindicator-sys@0.9.0` | `Apache-2.0 OR MIT` | `MIT` | `eb227437252b2a7a9c1fc342c93ade1f3d7ce38cc6dd754f613db07d53ceff0b` | registry+https://github.com/rust-lang/crates.io-index |
| `libc@0.2.191` | `MIT OR Apache-2.0` | `MIT` | `123a331b5dbf04c30097fa43b8f858bc85df671fe776de498d01f3d6b7c1f69e` | https://github.com/rust-lang/libc |
| `libdbus-sys@0.2.7` | `Apache-2.0/MIT` | `MIT` | `200791e43dd83f50c535582da6efa3357f47e61a46b026308b13dd0f7f492b81`, `de3911c2d98c8bd2d701ee721347053d9b55995a11f9a8c955e44d3ca1b376bf` | https://github.com/diwic/dbus-rs |
| `libloading@0.7.4` | `ISC` | `ISC` | `b29f8b01452350c20dd1af16ef83b598fea3053578ccc1c7a0ef40e57be2620f` | https://github.com/nagisa/rust_libloading/ |
| `linux-raw-sys@0.12.1` | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/sunfishcode/linux-raw-sys |
| `litemap@0.8.3` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `lock_api@0.4.14` | `MIT OR Apache-2.0` | `MIT` | `c9a75f18b9ab2927829a208fc6aa2cf4e63b8420887ba29cdb265d6619ae82d5` | https://github.com/Amanieu/parking_lot |
| `log@0.4.35` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-lang/log |
| `lzma-rs@0.3.0` | `MIT` | `MIT` | `ad9f07faa5f6f671df93523293cf8c1ce24f14886fa101013c21536c7890c59b` | https://github.com/gendx/lzma-rs |
| `lzma-rust2@0.25.0` | `Apache-2.0` | `Apache-2.0` | `43070e2d4e532684de521b885f385d0841030efa2b1a20bafb76133a5e1379c1` | https://github.com/hasenbanck/lzma-rust2/ |
| `markup5ever@0.39.0` | `MIT OR Apache-2.0` | `MIT` | `86dd7f026f916daf7511e39951ad8ea8cf55a8db67ae64060dacf829761c18f3` | https://github.com/servo/html5ever |
| `memchr@2.8.3` | `Unlicense OR MIT` | `MIT` | `0f96a83840e146e43c0ec96a22ec1f392e0680e6c1226e6f3ba87e0740af850f` | https://github.com/BurntSushi/memchr |
| `memoffset@0.9.1` | `MIT` | `MIT` | `3234ac55816264ee7b6c7ee27efd61cf0a1fe775806870e3d9b4c41ea73c5cb1` | https://github.com/Gilnaa/memoffset |
| `mime@0.3.18` | `MIT OR Apache-2.0` | `MIT` | `df9cfd06d8a44d9a671eadd39ffd97f166481da015a30f45dfd27886209c5922` | https://github.com/hyperium/mime |
| `mime_guess@2.0.5` | `MIT` | `MIT` | `816000f9c28f7005d931c77636eb79fa7020071eb0f6e6a995515dfccd3559cb` | https://github.com/abonander/mime_guess |
| `miniz_oxide@0.8.9` | `MIT OR Zlib OR Apache-2.0` | `MIT` | `4108245a1f2df9d4e94df8abed5b4ba0759bb2f9b40a6b939f1be141077ae50b`, `799e9ca9d179295ef372f25d3769cdda7d25bb2668add6a6a1e22d1e4c678b8d` | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide |
| `miniz_oxide@0.9.1` | `MIT OR Zlib OR Apache-2.0` | `MIT` | `4108245a1f2df9d4e94df8abed5b4ba0759bb2f9b40a6b939f1be141077ae50b`, `799e9ca9d179295ef372f25d3769cdda7d25bb2668add6a6a1e22d1e4c678b8d` | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide |
| `mio@1.2.5` | `MIT` | `MIT` | `07919255c7e04793d8ea760d6c2ce32d19f9ff02bdbdde3ce90b1e1880929a9b` | https://github.com/tokio-rs/mio |
| `moxcms@0.8.1` | `BSD-3-Clause OR Apache-2.0` | `Apache-2.0` | `90bf2d659c43045111b65c733ab2a6d4cbcb422a098368c8c58a9ba3db4ed0c5` | https://github.com/awxkee/moxcms.git |
| `muda@0.20.0` | `Apache-2.0 OR MIT` | `MIT` | `2ab5537b8c0cb1d475e2145b6a7994b04e7c71e6e5bd836f7f9c47c221a5ad9a` | https://github.com/tauri-apps/muda |
| `new_debug_unreachable@1.0.6` | `MIT` | `MIT` | `f7715d38a3fa1b4ac97c5729740752505a39cb92ee83ab5b102aeb5eaa7cdea4` | https://github.com/mbrubeck/rust-debug-unreachable |
| `nix@0.27.1` | `MIT` | `MIT` | `66e3ee1fa7f909ad3c612d556f2a0cdabcd809ad6e66f3b0605015ac64841b70` | https://github.com/nix-rust/nix |
| `nix@0.29.0` | `MIT` | `MIT` | `66e3ee1fa7f909ad3c612d556f2a0cdabcd809ad6e66f3b0605015ac64841b70` | https://github.com/nix-rust/nix |
| `nix@0.32.0` | `MIT` | `MIT` | `66e3ee1fa7f909ad3c612d556f2a0cdabcd809ad6e66f3b0605015ac64841b70` | https://github.com/nix-rust/nix |
| `nom@8.0.0` | `MIT` | `MIT` | `4dbda04344456f09a7a588140455413a9ac59b6b26a1ef7cdf9c800c012d87f0` | https://github.com/rust-bakery/nom |
| `notify@8.2.2` | `CC0-1.0` | `CC0-1.0` | `e0456cbab4f7b5e7203e678faff37eadbdfef02e822db3b7a52a1dd9d3dc514a` | https://github.com/notify-rs/notify.git |
| `notify-types@2.3.0` | `MIT OR Apache-2.0` | `MIT` | `b63e167d61ced1d872a588c5ac764b0dda477d62209dc4f6b6bfea5517d0a24c` | https://github.com/notify-rs/notify.git |
| `num@0.4.3` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-num/num |
| `num-bigint@0.4.8` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-num/num-bigint |
| `num-cmp@0.1.0` | `MIT/Apache-2.0` | `MIT` | `3bb1e548250b3acc43febde65a5cdba9f2e0a3288018aa41f009a3c59ffab51a` | https://github.com/lifthrasiir/num-cmp |
| `num-complex@0.4.6` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-num/num-complex |
| `num-conv@0.2.2` | `MIT OR Apache-2.0` | `MIT` | `e2e245f2b566d0bfafb0f6a04c16b82d710c4e6e7d799e39bb4ef6e541b85b98` | https://github.com/jhpratt/num-conv |
| `num-integer@0.1.47` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-num/num-integer |
| `num-iter@0.1.46` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-num/num-iter |
| `num-rational@0.4.2` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-num/num-rational |
| `num-traits@0.2.19` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-num/num-traits |
| `objc2@0.6.5` | `MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `objc2-app-kit@0.3.2` | `Zlib OR Apache-2.0 OR MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `objc2-core-foundation@0.3.2` | `Zlib OR Apache-2.0 OR MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `objc2-core-graphics@0.3.2` | `Zlib OR Apache-2.0 OR MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `objc2-core-services@0.3.2` | `Zlib OR Apache-2.0 OR MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `objc2-encode@4.1.0` | `MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `objc2-exception-helper@0.1.1` | `Zlib OR Apache-2.0 OR MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `objc2-foundation@0.3.2` | `MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `objc2-quartz-core@0.3.2` | `Zlib OR Apache-2.0 OR MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `objc2-web-kit@0.3.2` | `Zlib OR Apache-2.0 OR MIT` | `MIT` | `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69` | https://github.com/madsmtm/objc2 |
| `once_cell@1.21.4` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/matklad/once_cell |
| `open@5.4.5` | `MIT` | `MIT` | `e63690624d604aa626c3c62b6ead1a9f4a916d2260ac8132022acbafe2556045` | https://github.com/Byron/open-rs |
| `option-ext@0.2.0` | `MPL-2.0` | `MPL-2.0` | `66a3107d5ad6a058aab753eaac2047ccb2ed0e39465dd0fe5844da3e300d5172` | https://github.com/soc/option-ext.git |
| `ordered-stream@0.2.0` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/danieldg/ordered-stream |
| `os_pipe@1.2.3` | `MIT` | `MIT` | `8f71659370c5268d9a1dc962a46232540e8fca63462586d8efaa95aab492a208` | https://github.com/oconnor663/os_pipe.rs |
| `outref@0.5.2` | `MIT` | `MIT` | `a00511478c83ebb464ef19d840f061c7444adc4636bb561900e75c5808cfc6cc` | https://github.com/Nugine/outref |
| `pango@0.18.3` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `pango-sys@0.18.0` | `MIT` | `MIT` | `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb` | https://github.com/gtk-rs/gtk-rs-core |
| `parking@2.2.1` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/parking |
| `parking_lot@0.12.5` | `MIT OR Apache-2.0` | `MIT` | `c9a75f18b9ab2927829a208fc6aa2cf4e63b8420887ba29cdb265d6619ae82d5` | https://github.com/Amanieu/parking_lot |
| `parking_lot_core@0.9.12` | `MIT OR Apache-2.0` | `MIT` | `c9a75f18b9ab2927829a208fc6aa2cf4e63b8420887ba29cdb265d6619ae82d5` | https://github.com/Amanieu/parking_lot |
| `percent-encoding@2.3.2` | `MIT OR Apache-2.0` | `MIT` | `b38f11f6096706e6de553dabe2a7ed142d59b6fa8c97e290c67496154745cdd5` | https://github.com/servo/rust-url/ |
| `petgraph@0.8.3` | `MIT OR Apache-2.0` | `MIT` | `7576269ea71f767b99297934c0b2367532690f8c4badc695edf8e04ab6a1e545` | https://github.com/petgraph/petgraph |
| `phf@0.13.1` | `MIT` | `MIT` | `0ab4d106b6faac07fb6a051815fd1b4d862d730895e2d7d7358c2f13565e7a38` | https://github.com/rust-phf/rust-phf |
| `phf_codegen@0.13.1` | `MIT` | `MIT` | `0ab4d106b6faac07fb6a051815fd1b4d862d730895e2d7d7358c2f13565e7a38` | https://github.com/rust-phf/rust-phf |
| `phf_generator@0.13.1` | `MIT` | `MIT` | `0ab4d106b6faac07fb6a051815fd1b4d862d730895e2d7d7358c2f13565e7a38` | https://github.com/rust-phf/rust-phf |
| `phf_macros@0.13.1` | `MIT` | `MIT` | `0ab4d106b6faac07fb6a051815fd1b4d862d730895e2d7d7358c2f13565e7a38` | https://github.com/rust-phf/rust-phf |
| `phf_shared@0.13.1` | `MIT` | `MIT` | `0ab4d106b6faac07fb6a051815fd1b4d862d730895e2d7d7358c2f13565e7a38` | https://github.com/rust-phf/rust-phf |
| `pin-project-lite@0.2.17` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/taiki-e/pin-project-lite |
| `piper@0.2.5` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/piper |
| `pkg-config@0.3.34` | `MIT OR Apache-2.0` | `MIT` | `378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` | https://github.com/rust-lang/pkg-config-rs |
| `plist@1.10.3` | `MIT` | `MIT` | `5b0ae40d1a35f7ae6591a28e44771240e6a88cb03a66c9189a45b9681639b466` | https://github.com/ebarnard/rust-plist/ |
| `png@0.18.1` | `MIT OR Apache-2.0` | `MIT` | `eaf40297c75da471f7cda1f3458e8d91b4b2ec866e609527a13acfa93b638652` | https://github.com/image-rs/image-png |
| `polling@3.11.0` | `Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/smol-rs/polling |
| `potential_utf@0.1.6` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `powerfmt@0.2.1` | `MIT OR Apache-2.0` | `MIT` | `070dbc7dda03a29296f2d58bdb9b7331af90f2abc9f31df22875d1eabaf29852` | https://github.com/jhpratt/powerfmt |
| `ppv-lite86@0.2.21` | `MIT OR Apache-2.0` | `MIT` | `4cada0bd02ea3692eee6f16400d86c6508bbd3bafb2b65fed0419f36d4f83e8f` | https://github.com/cryptocorrosion/cryptocorrosion |
| `precomputed-hash@0.1.1` | `MIT` | `MIT` | `7ca6700600dfa9c9497bf5556365067daa802c871ea78239f129309c7a2048f7` | https://github.com/emilio/precomputed-hash |
| `proc-macro-crate@1.3.1` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/bkchr/proc-macro-crate |
| `proc-macro-crate@2.0.2` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/bkchr/proc-macro-crate |
| `proc-macro-crate@3.5.0` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/bkchr/proc-macro-crate |
| `proc-macro-error@1.0.4` | `MIT OR Apache-2.0` | `MIT` | `544b3aed1fd723d0cadea567affdcfe0431e43e18d997a718f9d67256b814fde` | https://gitlab.com/CreepySkeleton/proc-macro-error |
| `proc-macro-error-attr@1.0.4` | `MIT OR Apache-2.0` | `MIT` | `544b3aed1fd723d0cadea567affdcfe0431e43e18d997a718f9d67256b814fde` | https://gitlab.com/CreepySkeleton/proc-macro-error |
| `proc-macro2@1.0.107` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/proc-macro2 |
| `pxfm@0.1.30` | `BSD-3-Clause OR Apache-2.0` | `Apache-2.0` | `90bf2d659c43045111b65c733ab2a6d4cbcb422a098368c8c58a9ba3db4ed0c5` | https://github.com/awxkee/pxfm |
| `quick-error@2.0.1` | `MIT/Apache-2.0` | `MIT` | `058f01fe181608d027fcde7e528fc03ea3cf90f30903c407644b0a9bbc54f500` | http://github.com/tailhook/quick-error |
| `quick-xml@0.41.0` | `MIT` | `MIT` | `5b2c207dcf571267ebe0bbdd5bf321f561ddc22ff597e7b363e96f4539507a29` | https://github.com/tafia/quick-xml |
| `quick-xml@0.42.0` | `MIT` | `MIT` | `f0cf9b1c62bbe3bd3a69f5f79c7158f513f612b4940a0a812d1db39d605318bc` | https://github.com/tafia/quick-xml |
| `quote@1.0.47` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/quote |
| `rand@0.8.8` | `MIT OR Apache-2.0` | `MIT` | `209fbbe0ad52d9235e37badf9cadfe4dbdc87203179c0899e738b39ade42177b` | https://github.com/rust-random/rand |
| `rand_chacha@0.3.1` | `MIT OR Apache-2.0` | `MIT` | `209fbbe0ad52d9235e37badf9cadfe4dbdc87203179c0899e738b39ade42177b` | https://github.com/rust-random/rand |
| `rand_core@0.6.4` | `MIT OR Apache-2.0` | `MIT` | `209fbbe0ad52d9235e37badf9cadfe4dbdc87203179c0899e738b39ade42177b` | https://github.com/rust-random/rand |
| `raw-window-handle@0.6.2` | `MIT OR Apache-2.0 OR Zlib` | `MIT` | `9c5a80639a57c1c945570e7ebbca0706305849ce3c098021325cca9db2f7acc4` | https://github.com/rust-windowing/raw-window-handle |
| `ref-cast@1.0.27` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/ref-cast |
| `ref-cast-impl@1.0.27` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/ref-cast |
| `referencing@0.33.0` | `MIT` | `MIT` | `a573f030c2ae7eab837ab7d66440954ae54c5eabe6848c1a1fe9187215f50591` | https://github.com/Stranger6667/jsonschema |
| `regex@1.13.1` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-lang/regex |
| `regex-automata@0.4.18` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-lang/regex |
| `regex-syntax@0.8.11` | `MIT OR Apache-2.0` | `MIT` | `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb` | https://github.com/rust-lang/regex |
| `reqwest@0.12.28` | `MIT OR Apache-2.0` | `MIT` | `47d4e1803702728e03d30f8a848ae2249ba274bbd9f8443e803f7924d83cd371` | https://github.com/seanmonstar/reqwest |
| `rfd@0.17.2` | `MIT` | `MIT` | `65fc947f4bac882a2cd104ef73c246b1b037059df9f2b3a58b7f77f0b9e56071`, `ae26bc7b1bc91fb8648f6a298decbb1a77090a56c9d04e2c23b8365e5f54acb5` | https://github.com/PolyMeilex/rfd |
| `ring@0.17.14` | `Apache-2.0 AND ISC` | `Apache-2.0, ISC` | `005fc765ddc5115da796cca915baa9557abae13ff35e0a47c47affc56f6c414d`, `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`, `f025ccfb7dfb6bdfedc75ca0f67acc69e6fb4998143d834f7c2f38a29989680f` | https://github.com/briansmith/ring |
| `rustc-hash@2.1.3` | `Apache-2.0 OR MIT` | `MIT` | `30fefc3a7d6a0041541858293bcbea2dde4caa4c0a5802f996a7f7e8c0085652` | https://github.com/rust-lang/rustc-hash |
| `rustc_version@0.4.1` | `MIT OR Apache-2.0` | `MIT` | `c9a75f18b9ab2927829a208fc6aa2cf4e63b8420887ba29cdb265d6619ae82d5` | https://github.com/djc/rustc-version-rs |
| `rustix@1.1.6` | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/bytecodealliance/rustix |
| `rustls@0.23.46` | `Apache-2.0 OR ISC OR MIT` | `MIT` | `709e3175b4212f7b13aa93971c9f62ff8c69ec45ad8c6532a7e0c41d7a7d6f8c` | https://github.com/rustls/rustls |
| `rustls-pki-types@1.16.0` | `MIT OR Apache-2.0` | `MIT` | `9117d922e667125508dde62b02c1f57ed22f5ad21eb536aa2e2d99e1c796e639` | https://github.com/rustls/pki-types |
| `rustls-webpki@0.103.17` | `ISC` | `ISC` | `5b698ca13897be3afdb7174256fa1574f8c6892b8bea1a66dd6469d3fe27885a` | https://github.com/rustls/webpki |
| `rustversion@1.0.23` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/rustversion |
| `ryu@1.0.23` | `Apache-2.0 OR BSL-1.0` | `Apache-2.0` | `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a` | https://github.com/dtolnay/ryu |
| `same-file@1.0.6` | `Unlicense/MIT` | `MIT` | `cb3c929a05e6cbc9de9ab06a4c57eeb60ca8c724bef6c138c87d3a577e27aa14` | https://github.com/BurntSushi/same-file |
| `schemars@0.8.22` | `MIT` | `MIT` | `1954992a2b32e8a2af24a4c11b726902e344c1934b947a77f04d404908f8db30` | https://github.com/GREsau/schemars |
| `schemars_derive@0.8.22` | `MIT` | `MIT` | `1954992a2b32e8a2af24a4c11b726902e344c1934b947a77f04d404908f8db30`, `ea417315dc653b10a03911cbc3a724fafa75d3ed6381b358e3bed8ed2b50ffea` | https://github.com/GREsau/schemars |
| `scopeguard@1.2.0` | `MIT OR Apache-2.0` | `MIT` | `fb77f0a9c53e473abe5103c8632ef9f0f2874d4fb3f17cb2d8c661aab9cee9d7` | https://github.com/bluss/scopeguard |
| `secret-service@4.0.0` | `MIT OR Apache-2.0` | `MIT` | `0d13fdf5615ccc7e7123b58b5c88b0d2bbabe345cd70b94e094ee44034db5be6` | https://github.com/hwchen/secret-service-rs.git |
| `selectors@0.38.0` | `MPL-2.0` | `MPL-2.0` | `1a389c0a135f37855baa2fb879f86ed111d7facf8b0d5188330b35a22c4428a1` | https://github.com/servo/stylo |
| `semver@1.0.28` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/semver |
| `serde@1.0.229` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/serde-rs/serde |
| `serde-untagged@0.1.9` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/serde-untagged |
| `serde_core@1.0.229` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/serde-rs/serde |
| `serde_derive@1.0.229` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/serde-rs/serde |
| `serde_derive_internals@0.29.1` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/serde-rs/serde |
| `serde_json@1.0.154` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/serde-rs/json |
| `serde_repr@0.1.21` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/serde-repr |
| `serde_spanned@0.6.9` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `serde_spanned@1.1.3` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `serde_urlencoded@0.7.1` | `MIT/Apache-2.0` | `MIT` | `b9eb266294324f672cbe945fe8f2e32f85024f0d61a1a7d14382cdde0ac44769` | https://github.com/nox/serde_urlencoded |
| `serde_with@3.24.0` | `MIT OR Apache-2.0` | `MIT` | `7576269ea71f767b99297934c0b2367532690f8c4badc695edf8e04ab6a1e545` | https://github.com/jonasbb/serde_with/ |
| `serde_with_macros@3.24.0` | `MIT OR Apache-2.0` | `MIT` | `7576269ea71f767b99297934c0b2367532690f8c4badc695edf8e04ab6a1e545` | https://github.com/jonasbb/serde_with/ |
| `serialize-to-javascript@0.1.2` | `MIT OR Apache-2.0` | `MIT` | `e53a7165e8bc3ced5e5628404b3d2d28ecc14ad720f1c09ba28385d5151ecf93` | https://github.com/chippers/serialize-to-javascript |
| `serialize-to-javascript-impl@0.1.2` | `MIT OR Apache-2.0` | `MIT` | `e53a7165e8bc3ced5e5628404b3d2d28ecc14ad720f1c09ba28385d5151ecf93` | https://github.com/chippers/serialize-to-javascript |
| `servo_arc@0.4.3` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/servo/stylo |
| `sevenz-rust2@0.25.0` | `Apache-2.0` | `Apache-2.0` | `43070e2d4e532684de521b885f385d0841030efa2b1a20bafb76133a5e1379c1` | https://github.com/hasenbanck/sevenz-rust |
| `sha1@0.10.7` | `MIT OR Apache-2.0` | `MIT` | `b4eb00df6e2a4d22518fcaa6a2b4646f249b3a3c9814509b22bd2091f1392ff1` | https://github.com/RustCrypto/hashes |
| `sha2@0.10.9` | `MIT OR Apache-2.0` | `MIT` | `b4eb00df6e2a4d22518fcaa6a2b4646f249b3a3c9814509b22bd2091f1392ff1` | https://github.com/RustCrypto/hashes |
| `shlex@2.0.1` | `MIT OR Apache-2.0` | `MIT` | `4455bf75a91154108304cb283e0fea9948c14f13e20d60887cf2552449dea3b1` | https://github.com/comex/rust-shlex |
| `signal-hook-registry@1.4.8` | `MIT OR Apache-2.0` | `MIT` | `503558bfefe66ca15e4e3f7955b3cb0ec87fd52f29bf24b336af7bd00e946d5c` | https://github.com/vorner/signal-hook |
| `simd-adler32@0.3.10` | `MIT` | `MIT` | `42a35170233e83e18856792e748de4c1ce4a63b2afce9a370c89ef3fe23f9f2d` | https://github.com/mcountryman/simd-adler32 |
| `siphasher@1.0.4` | `MIT OR Apache-2.0` | `MIT` | `f9c4c77baa3828004ee54b8a4f2db2e88ed44a6237a493965bf551fac0fcb62d` | https://github.com/jedisct1/rust-siphash |
| `slab@0.4.12` | `MIT` | `MIT` | `8ce0830173fdac609dfb4ea603fdc002c2f4af0dc9b1a005653f5da9cf534b18` | https://github.com/tokio-rs/slab |
| `smallvec@1.16.5` | `MIT OR Apache-2.0` | `MIT` | `0b28172679e0009b655da42797c03fd163a3379d5cfa67ba1f1655e974a2a1a9` | https://github.com/servo/rust-smallvec |
| `socket2@0.6.5` | `MIT OR Apache-2.0` | `MIT` | `378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` | https://github.com/rust-lang/socket2 |
| `softbuffer@0.4.8` | `MIT OR Apache-2.0` | `MIT` | `0463bfa47d6d763d8c824333efb0e6d6df3946c13c3b077c78ed9f23ca50d0f2` | https://github.com/rust-windowing/softbuffer |
| `soup3@0.5.0` | `MIT` | `MIT` | `7ade43ec5e1a5e43000aa871d290dff4c0e9e2e16b7f6bea2ecf091e3d056d31` | https://gitlab.gnome.org/World/Rust/soup3-rs |
| `soup3-sys@0.5.0` | `MIT` | `MIT` | `7ade43ec5e1a5e43000aa871d290dff4c0e9e2e16b7f6bea2ecf091e3d056d31` | https://gitlab.gnome.org/World/Rust/soup3-rs |
| `stable_deref_trait@1.2.1` | `MIT OR Apache-2.0` | `MIT` | `5e05b024f653a5ce199e77cbbbd42fb5553562ec714b819421ed0c3e552a75d7` | https://github.com/storyyeller/stable_deref_trait |
| `static_assertions@1.1.0` | `MIT OR Apache-2.0` | `MIT` | `ea084a2373ebc1f0902c09266e7bf25a05ab3814c1805bb017ffa7308f90c061` | https://github.com/nvzqz/static-assertions-rs |
| `static_vcruntime@3.0.0` | `MIT OR Apache-2.0 OR Zlib` | `MIT` | `748c208045a6f00734dbc85a1c0e401c9a25e1f83fc756e99916750dbd2469a7` | https://github.com/ChrisDenton/static_vcruntime |
| `string_cache@0.9.0` | `MIT OR Apache-2.0` | `MIT` | `62065228e42caebca7e7d7db1204cbb867033de5982ca4009928915e4095f3a3` | https://github.com/servo/string-cache |
| `string_cache_codegen@0.6.1` | `MIT OR Apache-2.0` | `MIT` | `62065228e42caebca7e7d7db1204cbb867033de5982ca4009928915e4095f3a3` | https://github.com/servo/string-cache |
| `strsim@0.11.1` | `MIT` | `MIT` | `1e697ce8d21401fbf1bddd9b5c3fd4c4c79ae1e3bdf51f81761c85e11d5a89cd` | https://github.com/rapidfuzz/strsim-rs |
| `subtle@2.6.1` | `BSD-3-Clause` | `BSD-3-Clause` | `d1fc1bc0d155df60b2e7705b6b2ae02a05c96f948e1cec6e2fb86360b09f346b` | https://github.com/dalek-cryptography/subtle |
| `swift-rs@1.0.9` | `MIT OR Apache-2.0` | `MIT` | `f3c3e9f4febe81ff14ccc233c344e6e60051290d1c1e5a2c670d63cc3608a5bb` | https://github.com/Brendonovich/swift-rs |
| `syn@1.0.109` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/syn |
| `syn@2.0.119` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/syn |
| `syn@3.0.10` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/syn |
| `sync_wrapper@1.0.2` | `Apache-2.0` | `Apache-2.0` | `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594` | https://github.com/Actyx/sync_wrapper |
| `synstructure@0.14.0` | `MIT` | `MIT` | `219920e865eee70b7dcfc948a86b099e7f4fe2de01bcca2ca9a20c0a033f2b59` | https://github.com/mystor/synstructure |
| `system-deps@6.2.2` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/gdesmott/system-deps |
| `tao@0.37.2` | `Apache-2.0` | `Apache-2.0` | `6dc0e068dcf3a5bc8e054205b85b7720e1d49265bbc64bf515d2cf79197df69a` | https://github.com/tauri-apps/tao |
| `tar@0.4.46` | `MIT OR Apache-2.0` | `MIT` | `8ca6b96cea9e67c6c5c63f452c31bd396db8bd2406231fdea5d48ef462b48077` | https://github.com/composefs/tar-rs |
| `target-lexicon@0.12.16` | `Apache-2.0 WITH LLVM-exception` | `Apache-2.0` | `268872b9816f90fd8e85db5a28d33f8150ebb8dd016653fb39ef1f94f2686bc5` | https://github.com/bytecodealliance/target-lexicon |
| `tauri@2.12.5` | `Apache-2.0 OR MIT` | `MIT` | `9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d` | https://github.com/tauri-apps/tauri |
| `tauri-build@2.7.4` | `Apache-2.0 OR MIT` | `MIT` | `9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d` | https://github.com/tauri-apps/tauri |
| `tauri-codegen@2.7.4` | `Apache-2.0 OR MIT` | `MIT` | `9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d` | https://github.com/tauri-apps/tauri |
| `tauri-macros@2.7.4` | `Apache-2.0 OR MIT` | `MIT` | `9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d` | https://github.com/tauri-apps/tauri |
| `tauri-plugin@2.7.2` | `Apache-2.0 OR MIT` | `MIT` | `9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d` | https://github.com/tauri-apps/tauri |
| `tauri-plugin-clipboard-manager@2.5.0` | `Apache-2.0 OR MIT` | `MIT` | `89ff9689dcf9dd53968785d05a26f7898bb169dbfcada8d032b3e68cf0d55607` | https://github.com/tauri-apps/plugins-workspace |
| `tauri-plugin-dialog@2.8.4` | `Apache-2.0 OR MIT` | `MIT` | `89ff9689dcf9dd53968785d05a26f7898bb169dbfcada8d032b3e68cf0d55607` | https://github.com/tauri-apps/plugins-workspace |
| `tauri-plugin-fs@2.7.1` | `Apache-2.0 OR MIT` | `MIT` | `89ff9689dcf9dd53968785d05a26f7898bb169dbfcada8d032b3e68cf0d55607` | https://github.com/tauri-apps/plugins-workspace |
| `tauri-plugin-global-shortcut@2.4.1` | `Apache-2.0 OR MIT` | `MIT` | `89ff9689dcf9dd53968785d05a26f7898bb169dbfcada8d032b3e68cf0d55607` | https://github.com/tauri-apps/plugins-workspace |
| `tauri-plugin-opener@2.7.3` | `Apache-2.0 OR MIT` | `MIT` | `89ff9689dcf9dd53968785d05a26f7898bb169dbfcada8d032b3e68cf0d55607` | https://github.com/tauri-apps/plugins-workspace |
| `tauri-plugin-single-instance@2.5.5` | `Apache-2.0 OR MIT` | `MIT` | `b7f455413bfe5f9d7da501a4468d4c5f866a432c325947e98cc9535e8b26cd6d` | https://github.com/tauri-apps/plugins-workspace |
| `tauri-runtime@2.12.2` | `Apache-2.0 OR MIT` | `MIT` | `9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d` | https://github.com/tauri-apps/tauri |
| `tauri-runtime-wry@2.12.3` | `Apache-2.0 OR MIT` | `MIT` | `9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d` | https://github.com/tauri-apps/tauri |
| `tauri-utils@2.10.2` | `Apache-2.0 OR MIT` | `MIT` | `9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d` | https://github.com/tauri-apps/tauri |
| `tauri-winres@0.3.7` | `MIT` | `MIT` | `d9fa6b0e6fed50d4961b9c931587af6bd127df13e9473418d3e324f427db4f06` | https://github.com/tauri-apps/winres |
| `tempfile@3.27.0` | `MIT OR Apache-2.0` | `MIT` | `8b427f5bc501764575e52ba4f9d95673cf8f6d80a86d0d06599852e1a9a20a36` | https://github.com/Stebalien/tempfile |
| `tendril@0.5.1` | `MIT OR Apache-2.0` | `MIT` | `60a7062291b01ba068f300612cdbdc20382ac1d4934f07bcdd7167c15299f309` | https://github.com/servo/html5ever |
| `thiserror@1.0.69` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/thiserror |
| `thiserror@2.0.21` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/thiserror |
| `thiserror-impl@1.0.69` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/thiserror |
| `thiserror-impl@2.0.21` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/thiserror |
| `tiff@0.11.3` | `MIT` | `MIT` | `20c097d0de27e2adc179140a3d542091951a030699c7e1c78655f2a2d08d4e3e` | https://github.com/image-rs/image-tiff |
| `time@0.3.55` | `MIT OR Apache-2.0` | `MIT` | `2537228d9a1b44a5dc595241349cae7090b326c8de165aaf89bfddef4a00d0fc` | https://github.com/time-rs/time |
| `time-core@0.1.9` | `MIT OR Apache-2.0` | `MIT` | `2537228d9a1b44a5dc595241349cae7090b326c8de165aaf89bfddef4a00d0fc` | https://github.com/time-rs/time |
| `time-macros@0.2.32` | `MIT OR Apache-2.0` | `MIT` | `2537228d9a1b44a5dc595241349cae7090b326c8de165aaf89bfddef4a00d0fc` | https://github.com/time-rs/time |
| `tinystr@0.8.5` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `tokio@1.54.1` | `MIT` | `MIT` | `253cd04c6714889df2d32f3f64d669179a1c95c76ac43c40882c52eb06bc3552` | https://github.com/tokio-rs/tokio |
| `tokio-macros@2.7.2` | `MIT` | `MIT` | `0b83dc40cba89b9922bb84b0a9c7d2768ce37c1d7e138b7424fd4549915778c9` | https://github.com/tokio-rs/tokio |
| `tokio-rustls@0.26.6` | `MIT OR Apache-2.0` | `MIT` | `e20fa2b8e0a2565f24a792b94b4bf4b6c2b9d36f781d8a9516e218a036e6677a` | https://github.com/rustls/tokio-rustls |
| `tokio-util@0.7.20` | `MIT` | `MIT` | `253cd04c6714889df2d32f3f64d669179a1c95c76ac43c40882c52eb06bc3552` | https://github.com/tokio-rs/tokio |
| `toml@0.8.2` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `toml@1.2.1+spec-1.1.0` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `toml_datetime@0.6.3` | `MIT OR Apache-2.0` | `MIT` | `378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397` | https://github.com/toml-rs/toml |
| `toml_datetime@1.1.3+spec-1.1.0` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `toml_edit@0.19.15` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `toml_edit@0.20.2` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `toml_edit@0.25.19+spec-1.1.0` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `toml_parser@1.1.5+spec-1.1.0` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `toml_writer@1.2.0+spec-1.1.0` | `MIT OR Apache-2.0` | `MIT` | `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6` | https://github.com/toml-rs/toml |
| `tower@0.5.3` | `MIT` | `MIT` | `4249c8e6c5ebb85f97c77e6457c6fafc1066406eb8f1ef61e796fbdc5ff18482` | https://github.com/tower-rs/tower |
| `tower-http@0.6.11` | `MIT` | `MIT` | `5049cf464977eff4b4fcfa7988d84e74116956a3eb9d5f1d451b3f828f945233` | https://github.com/tower-rs/tower-http |
| `tower-layer@0.3.3` | `MIT` | `MIT` | `4249c8e6c5ebb85f97c77e6457c6fafc1066406eb8f1ef61e796fbdc5ff18482` | https://github.com/tower-rs/tower |
| `tower-service@0.3.3` | `MIT` | `MIT` | `4249c8e6c5ebb85f97c77e6457c6fafc1066406eb8f1ef61e796fbdc5ff18482` | https://github.com/tower-rs/tower |
| `tracing@0.1.44` | `MIT` | `MIT` | `898b1ae9821e98daf8964c8d6c7f61641f5f5aa78ad500020771c0939ee0dea1` | https://github.com/tokio-rs/tracing |
| `tracing-attributes@0.1.31` | `MIT` | `MIT` | `898b1ae9821e98daf8964c8d6c7f61641f5f5aa78ad500020771c0939ee0dea1` | https://github.com/tokio-rs/tracing |
| `tracing-core@0.1.36` | `MIT` | `MIT` | `58545fed1565e42d687aecec6897d35c6d37ccb71479a137c0deb2203e125c79`, `898b1ae9821e98daf8964c8d6c7f61641f5f5aa78ad500020771c0939ee0dea1` | https://github.com/tokio-rs/tracing |
| `tray-icon@0.25.1` | `MIT OR Apache-2.0` | `MIT` | `2ab5537b8c0cb1d475e2145b6a7994b04e7c71e6e5bd836f7f9c47c221a5ad9a` | https://github.com/tauri-apps/tray-icon |
| `tree_magic_mini@3.2.2` | `MIT` | `MIT` | `a7cea8a5a56ba02950d098a7c56eed312fc745ceecd437e4aad2f7f8d38661b8` | https://github.com/mbrubeck/tree_magic/ |
| `try-lock@0.2.5` | `MIT` | `MIT` | `c816a0749cdc6bf062a5111c159723de51b2bfac66a1dac2655abd9e6b1583eb` | https://github.com/seanmonstar/try-lock |
| `typed-path@0.13.0` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/chipsenkbeil/typed-path |
| `typeid@1.0.3` | `MIT OR Apache-2.0` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/typeid |
| `typenum@1.20.1` | `MIT OR Apache-2.0` | `MIT` | `a825bd853ab71619a4923d7b4311221427848070ff44d990da39b0b274c1683f` | https://github.com/paholg/typenum |
| `unicase@2.10.0` | `MIT OR Apache-2.0` | `MIT` | `4f6bd11a0f17fe5b085ace1daaedb7b002a08316850dd9577c246c6a68a68e8c` | https://github.com/seanmonstar/unicase |
| `unicode-ident@1.0.27` | `(MIT OR Apache-2.0) AND Unicode-3.0` | `MIT, Unicode-3.0` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3`, `f7db81051789b729fea528a63ec4c938fdcb93d9d61d97dc8cc2e9df6d47f2a1` | https://github.com/dtolnay/unicode-ident |
| `unicode-segmentation@1.13.3` | `MIT OR Apache-2.0` | `MIT` | `7b63ecd5f1902af1b63729947373683c32745c16a10e8e6292e2e2dcd7e90ae0` | https://github.com/unicode-rs/unicode-segmentation |
| `untrusted@0.9.0` | `ISC` | `ISC` | `7abd9b6960dcf7d4d0a48606a5b71bfe37d472db68d70637f3a58a56785f1621` | https://github.com/briansmith/untrusted |
| `url@2.5.8` | `MIT OR Apache-2.0` | `MIT` | `b38f11f6096706e6de553dabe2a7ed142d59b6fa8c97e290c67496154745cdd5` | https://github.com/servo/rust-url |
| `urlpattern@0.6.0` | `MIT` | `MIT` | `5cf1c9dc617e4cad7fa03181ff893601ad5243aa3be527ef39034e1cd6377676` | https://github.com/denoland/rust-urlpattern |
| `utf8_iter@1.1.0` | `Apache-2.0 OR MIT` | `MIT` | `3fa4ca83dcc9237839b1bdeb2e6d16bdfb5ec0c5ce42b24694d8bbf0dcbef72c` | https://github.com/hsivonen/utf8_iter |
| `utf_types@1.0.0` | `Apache-2.0 OR MIT` | `MIT` | `3fa4ca83dcc9237839b1bdeb2e6d16bdfb5ec0c5ce42b24694d8bbf0dcbef72c` | https://github.com/hsivonen/utf_types |
| `uuid@1.29.2` | `Apache-2.0 OR MIT` | `MIT` | `436bc5a105d8e57dcd8778730f3754f7bf39c14d2f530e4cde4bd2d17a83ec3d` | https://github.com/uuid-rs/uuid |
| `uuid-simd@0.8.0` | `MIT` | `MIT` | `71674605ec4c087fe9eb534e3e4f9e26eb2e4aabcd76a29fd156c6a844d44b3d` | https://github.com/Nugine/simd |
| `version-compare@0.2.1` | `MIT` | `MIT` | `cedfcc7ace1639adb054bd4a19b6f8585ccb2429a60b4bf51bfeb0601058c22c` | https://gitlab.com/timvisee/version-compare |
| `version_check@0.9.5` | `MIT/Apache-2.0` | `MIT` | `b7e650f3fce5c53249d1cdc608b54df156a97edd636cf9d23498d0cfe7aec63e` | https://github.com/SergioBenitez/version_check |
| `vsimd@0.8.0` | `MIT` | `MIT` | `71674605ec4c087fe9eb534e3e4f9e26eb2e4aabcd76a29fd156c6a844d44b3d` | https://github.com/Nugine/simd |
| `vswhom@0.1.0` | `MIT` | `MIT` | `b9b7e5d4bf8857061fbecab39ef1923644eba639337af22d7ad0bb9b3ce59c49` | https://github.com/nabijaczleweli/vswhom.rs |
| `vswhom-sys@0.1.3` | `MIT` | `MIT` | `b9b7e5d4bf8857061fbecab39ef1923644eba639337af22d7ad0bb9b3ce59c49` | https://github.com/nabijaczleweli/vswhom-sys.rs |
| `walkdir@2.5.0` | `Unlicense/MIT` | `MIT` | `0f96a83840e146e43c0ec96a22ec1f392e0680e6c1226e6f3ba87e0740af850f` | https://github.com/BurntSushi/walkdir |
| `want@0.3.2` | `MIT` | `MIT` | `a65f5d0a945d267751344c95665945b90c030ea107faf5c85d518929886187da` | https://github.com/seanmonstar/want |
| `wayland-backend@0.3.17` | `MIT` | `MIT` | `1cb6f4dee8c649f175736380495f0af0c753c87f22d50931692968bb24bf2161` | https://github.com/smithay/wayland-rs |
| `wayland-client@0.31.15` | `MIT` | `MIT` | `1cb6f4dee8c649f175736380495f0af0c753c87f22d50931692968bb24bf2161` | https://github.com/smithay/wayland-rs |
| `wayland-protocols@0.32.13` | `MIT` | `MIT` | `1cb6f4dee8c649f175736380495f0af0c753c87f22d50931692968bb24bf2161`, `f1a2b233e8a9a71c40f4aa885be08a0842ac85bb8588703c1dd7e6e6502e3124` | https://github.com/smithay/wayland-rs |
| `wayland-protocols-wlr@0.3.12` | `MIT` | `MIT` | `1cb6f4dee8c649f175736380495f0af0c753c87f22d50931692968bb24bf2161` | https://github.com/smithay/wayland-rs |
| `wayland-scanner@0.31.11` | `MIT` | `MIT` | `1cb6f4dee8c649f175736380495f0af0c753c87f22d50931692968bb24bf2161` | https://github.com/smithay/wayland-rs |
| `wayland-sys@0.31.11` | `MIT` | `MIT` | `1cb6f4dee8c649f175736380495f0af0c753c87f22d50931692968bb24bf2161` | https://github.com/smithay/wayland-rs |
| `web-time@1.1.0` | `MIT OR Apache-2.0` | `MIT` | `f428305bbf2e70fbd807af0f3c7ac1b7acfb95311640f451e9d2e48db6d7e96b` | https://github.com/daxpedda/web-time |
| `web_atoms@0.2.6` | `MIT OR Apache-2.0` | `MIT` | `86dd7f026f916daf7511e39951ad8ea8cf55a8db67ae64060dacf829761c18f3` | https://github.com/servo/html5ever |
| `webkit2gtk@2.0.2` | `MIT` | `MIT` | `3f39c032be4639be635da96d77c4d0f4393c3595a164450ffd86f3d17701ec10` | https://github.com/tauri-apps/webkit2gtk-rs |
| `webkit2gtk-sys@2.0.2` | `MIT` | `MIT` | `281b72cb008466810da31680e72005b9899c7e9d7705b909c86098260e12aa7d` | https://github.com/tauri-apps/webkit2gtk-rs |
| `webpki-roots@1.0.12` | `CDLA-Permissive-2.0` | `CDLA-Permissive-2.0` | `e271993808fec50ab29350b39539cdec611a9103f827e0aa26d61da70e2d33f8` | https://github.com/rustls/webpki-roots |
| `webview2-com@0.39.1` | `MIT` | `MIT` | `0dcf41516e608bbcb6cdc5229feb7b86fe4a643b85e7df251133c93408fdac73` | https://github.com/wravery/webview2-rs |
| `webview2-com-macros@0.8.1` | `MIT` | `MIT` | `0dcf41516e608bbcb6cdc5229feb7b86fe4a643b85e7df251133c93408fdac73` | https://github.com/wravery/webview2-rs |
| `webview2-com-sys@0.39.1` | `MIT` | `MIT` | `0dcf41516e608bbcb6cdc5229feb7b86fe4a643b85e7df251133c93408fdac73` | https://github.com/wravery/webview2-rs |
| `weezl@0.1.12` | `MIT OR Apache-2.0` | `MIT` | `573f5227db835cd30bbdd3aed3f82615f80119a4d5c82b4daf642dbfe2b6afde` | https://github.com/image-rs/weezl |
| `winapi@0.3.9` | `MIT/Apache-2.0` | `MIT` | `ce7bc3499fee93d5022ef430d5e4201e79a6d9154f3974e42f41349f0569e09b` | https://github.com/retep998/winapi-rs |
| `winapi-util@0.1.11` | `Unlicense OR MIT` | `MIT` | `cb3c929a05e6cbc9de9ab06a4c57eeb60ca8c724bef6c138c87d3a577e27aa14` | https://github.com/BurntSushi/winapi-util |
| `window-vibrancy@0.8.1` | `Apache-2.0 OR MIT` | `MIT` | `617b85f62a4f1b6a62d3d9f28f6d7003b7f68a70b7675a8174bbcc68401ac62d` | https://github.com/tauri-apps/tauri-plugin-vibrancy |
| `windows@0.62.2` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-collections@0.3.2` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-core@0.62.2` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-future@0.3.2` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-implement@0.60.2` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-interface@0.59.3` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-link@0.2.1` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-numerics@0.3.1` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-result@0.4.1` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-strings@0.5.1` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-sys@0.59.0` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-sys@0.60.2` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-sys@0.61.2` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-targets@0.52.6` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-targets@0.53.5` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-threading@0.2.1` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows-version@0.1.7` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows_x86_64_msvc@0.52.6` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `windows_x86_64_msvc@0.53.1` | `MIT OR Apache-2.0` | `MIT` | `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383` | https://github.com/microsoft/windows-rs |
| `winnow@0.5.40` | `MIT` | `MIT` | `cb5aedb296c5246d1f22e9099f925a65146f9f0d6b4eebba97fd27a6cdbbab2d` | https://github.com/winnow-rs/winnow |
| `winnow@1.0.5` | `MIT` | `MIT` | `cb5aedb296c5246d1f22e9099f925a65146f9f0d6b4eebba97fd27a6cdbbab2d` | https://github.com/winnow-rs/winnow |
| `winreg@0.56.0` | `MIT` | `MIT` | `ea021d8c6383c84d19c15ce90d3b1602cf448fdcc0bf3772c98e43c7bf1bf1d1` | https://github.com/gentoo90/winreg-rs |
| `wl-clipboard-rs@0.9.5` | `MIT OR Apache-2.0` | `MIT` | `d0235d2a1f657b6fd18fd4ce98988348969a9284d94281b6d117830aa259a07a` | https://github.com/YaLTeR/wl-clipboard-rs |
| `writeable@0.6.4` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `wry@0.57.2` | `Apache-2.0 OR MIT` | `MIT` | `ef88893533af22ffbb68ceef0724ba1389518df26165a34f3c99ad30b884df4f` | https://github.com/tauri-apps/wry |
| `x11@2.21.0` | `MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/AltF02/x11-rs.git |
| `x11-dl@2.21.0` | `MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/AltF02/x11-rs.git |
| `x11rb@0.13.2` | `MIT OR Apache-2.0` | `MIT` | `a72561f47c1f665535930f606739d9098a6186802d5e2e8ec491faf00f4add5b` | https://github.com/psychon/x11rb |
| `x11rb-protocol@0.13.2` | `MIT OR Apache-2.0` | `MIT` | `a72561f47c1f665535930f606739d9098a6186802d5e2e8ec491faf00f4add5b` | https://github.com/psychon/x11rb |
| `xattr@1.7.0` | `MIT OR Apache-2.0` | `MIT` | `8b427f5bc501764575e52ba4f9d95673cf8f6d80a86d0d06599852e1a9a20a36` | https://github.com/Stebalien/xattr |
| `xdg-home@1.3.0` | `MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/zeenix/xdg-home |
| `xkeysym@0.2.1` | `MIT OR Apache-2.0 OR Zlib` | `MIT` | `875e64a5df9f892ebd29c71ad0b58a765b5789d7e9142aa70ec07fae74a67686` | https://github.com/notgull/xkeysym |
| `yoke@0.8.4` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `yoke-derive@0.8.4` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `zbus@4.4.0` | `MIT` | `MIT` | `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad` | https://github.com/dbus2/zbus/ |
| `zbus@5.19.0` | `MIT` | `MIT` | `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad` | https://github.com/z-galaxy/zbus/ |
| `zbus_macros@4.4.0` | `MIT` | `MIT` | `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad` | https://github.com/dbus2/zbus/ |
| `zbus_macros@5.19.0` | `MIT` | `MIT` | `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad` | https://github.com/z-galaxy/zbus/ |
| `zbus_names@3.0.0` | `MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dbus2/zbus/ |
| `zbus_names@4.3.5` | `MIT` | `MIT` | `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad` | https://github.com/z-galaxy/zbus/ |
| `zcheapstr@1.2.0` | `MIT` | `MIT` | `59871bb1612f8a0569655c10b995636827eceab42c6578cebdc45ad2922e0142` | https://github.com/z-galaxy/zcheapstr/ |
| `zerocopy@0.8.63` | `BSD-2-Clause OR Apache-2.0 OR MIT` | `MIT` | `1a2f5c12ddc934d58956aa5dbdd3255fe55fd957633ab7d0d39e4f0daa73f7df` | https://github.com/google/zerocopy |
| `zerocopy-derive@0.8.63` | `BSD-2-Clause OR Apache-2.0 OR MIT` | `MIT` | `1a2f5c12ddc934d58956aa5dbdd3255fe55fd957633ab7d0d39e4f0daa73f7df` | https://github.com/google/zerocopy |
| `zerofrom@0.1.8` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `zerofrom-derive@0.1.8` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `zeroize@1.9.1` | `Apache-2.0 OR MIT` | `MIT` | `8c7516d4b27b1e495be5e38b612298b63de48d05f49cdac94f70f3cd70f8864b` | https://github.com/RustCrypto/utils |
| `zeroize_derive@1.5.0` | `Apache-2.0 OR MIT` | `MIT` | `b8c6939380a400f53e11923d50fcc4dd2fa1ba8339fd9d04cda38a0251b6c9b0` | https://github.com/RustCrypto/utils |
| `zerotrie@0.2.5` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `zerovec@0.11.9` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `zerovec-derive@0.11.7` | `Unicode-3.0` | `Unicode-3.0` | `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2` | https://github.com/unicode-org/icu4x |
| `zip@9.6.0` | `MIT` | `MIT` | `58545fed1565e42d687aecec6897d35c6d37ccb71479a137c0deb2203e125c79` | https://github.com/zip-rs/zip2 |
| `zlib-rs@0.6.8` | `Zlib` | `Zlib` | `e72111c52b7d96ebe25348dee19f0744f444d3c95ae6b1ecb6ccaecc5bce05ba` | https://github.com/trifectatechfoundation/zlib-rs |
| `zmij@1.0.23` | `MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dtolnay/zmij |
| `zopfli@0.8.4` | `Apache-2.0` | `Apache-2.0` | `018b1cb87efdf7a04c2fcc13d57ed63f62149113fb207b27ea13430d64f13513` | https://github.com/zopfli-rs/zopfli |
| `zune-core@0.5.3` | `MIT OR Apache-2.0 OR Zlib` | `MIT` | `d30047bca3b516639339a3c279bb84c3483124fb5a9dafe3c75056a85090e745` | https://github.com/etemesi254/zune-image |
| `zune-jpeg@0.5.15` | `MIT OR Apache-2.0 OR Zlib` | `MIT` | `d30047bca3b516639339a3c279bb84c3483124fb5a9dafe3c75056a85090e745` | https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpeg |
| `zvariant@4.2.0` | `MIT` | `MIT` | `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad` | https://github.com/dbus2/zbus/ |
| `zvariant@5.15.1` | `MIT` | `MIT` | `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad` | https://github.com/z-galaxy/zbus/ |
| `zvariant_derive@4.2.0` | `MIT` | `MIT` | `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad` | https://github.com/dbus2/zbus/ |
| `zvariant_derive@5.15.1` | `MIT` | `MIT` | `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad` | https://github.com/z-galaxy/zbus/ |
| `zvariant_utils@2.1.0` | `MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/dbus2/zbus/ |
| `zvariant_utils@4.2.0` | `MIT` | `MIT` | `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3` | https://github.com/z-galaxy/zbus/ |

## Collected license texts

The output contains 240 distinct (SPDX ID, exact-text hash) entries. The generator integrates these entries into `THIRD_PARTY_LICENSES`, preserving their copyright, source-file, and upstream repository information.

- `Apache-2.0` — SHA-256 `005fc765ddc5115da796cca915baa9557abae13ff35e0a47c47affc56f6c414d`; applies to ring@0.17.14.
- `Apache-2.0` — SHA-256 `018b1cb87efdf7a04c2fcc13d57ed63f62149113fb207b27ea13430d64f13513`; applies to zopfli@0.8.4.
- `Apache-2.0` — SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`; applies to sync_wrapper@1.0.2.
- `Apache-2.0` — SHA-256 `268872b9816f90fd8e85db5a28d33f8150ebb8dd016653fb39ef1f94f2686bc5`; applies to target-lexicon@0.12.16.
- `Apache-2.0` — SHA-256 `43070e2d4e532684de521b885f385d0841030efa2b1a20bafb76133a5e1379c1`; applies to lzma-rust2@0.25.0, sevenz-rust2@0.25.0.
- `Apache-2.0` — SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`; applies to constant_time_eq@0.6.1, ryu@1.0.23.
- `Apache-2.0` — SHA-256 `6dc0e068dcf3a5bc8e054205b85b7720e1d49265bbc64bf515d2cf79197df69a`; applies to dpi@0.1.2, tao@0.37.2.
- `Apache-2.0` — SHA-256 `90bf2d659c43045111b65c733ab2a6d4cbcb422a098368c8c58a9ba3db4ed0c5`; applies to moxcms@0.8.1, pxfm@0.1.30.
- `Apache-2.0` — SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`; applies to gethostname@1.1.0, ring@0.17.14.
- `BSD-3-Clause` — SHA-256 `c0c56f26d9c051cac4d200c34c84e7ae9aaa853e01a982a1df08b09931e518ae`; applies to alloc-no-stdlib@3.0.0, alloc-stdlib@0.3.0, brotli@9.1.1, brotli-decompressor@6.3.1.
- `BSD-3-Clause` — SHA-256 `d1fc1bc0d155df60b2e7705b6b2ae02a05c96f948e1cec6e2fb86360b09f346b`; applies to subtle@2.6.1.
- `BSL-1.0` — SHA-256 `c9bff75738922193e67fa726fa225535870d2aa1059f91452c411736284ad566`; applies to clipboard-win@5.4.3, error-code@3.4.0.
- `CC0-1.0` — SHA-256 `a2010f343487d3f7618affe54f789f5487602331c0a8d03f49e9a7c547cf0499`; applies to dunce@1.0.5.
- `CC0-1.0` — SHA-256 `e0456cbab4f7b5e7203e678faff37eadbdfef02e822db3b7a52a1dd9d3dc514a`; applies to notify@8.2.2.
- `CDLA-Permissive-2.0` — SHA-256 `e271993808fec50ab29350b39539cdec611a9103f827e0aa26d61da70e2d33f8`; applies to webpki-roots@1.0.12.
- `ISC` — SHA-256 `5b698ca13897be3afdb7174256fa1574f8c6892b8bea1a66dd6469d3fe27885a`; applies to rustls-webpki@0.103.17.
- `ISC` — SHA-256 `7abd9b6960dcf7d4d0a48606a5b71bfe37d472db68d70637f3a58a56785f1621`; applies to untrusted@0.9.0.
- `ISC` — SHA-256 `7f78b31de4e7a3e2ac8de40e2b409eda313f03637aa62b8416fbde9bdc5ce4c9`; applies to inotify@0.11.5.
- `ISC` — SHA-256 `b29f8b01452350c20dd1af16ef83b598fea3053578ccc1c7a0ef40e57be2620f`; applies to libloading@0.7.4.
- `ISC` — SHA-256 `f025ccfb7dfb6bdfedc75ca0f67acc69e6fb4998143d834f7c2f38a29989680f`; applies to ring@0.17.14.
- `ISC` — SHA-256 `f68878c9ca61b25d4b2c470de571cb85ce97804a62bb153607f7e0e0d2684c8a`; applies to inotify-sys@0.1.8.
- `MIT` — SHA-256 `025436edff4cfcdde17a5811fdea78892d8482efd1abdec5a17872d07a4f2112`; applies to flate2@1.1.11.
- `MIT` — SHA-256 `0444c6991eead6822f7b9102e654448d51624431119546492e8b231db42c48bb`; applies to ahash@0.8.12.
- `MIT` — SHA-256 `0463bfa47d6d763d8c824333efb0e6d6df3946c13c3b077c78ed9f23ca50d0f2`; applies to softbuffer@0.4.8.
- `MIT` — SHA-256 `058f01fe181608d027fcde7e528fc03ea3cf90f30903c407644b0a9bbc54f500`; applies to quick-error@2.0.1.
- `MIT` — SHA-256 `070dbc7dda03a29296f2d58bdb9b7331af90f2abc9f31df22875d1eabaf29852`; applies to powerfmt@0.2.1.
- `MIT` — SHA-256 `07919255c7e04793d8ea760d6c2ce32d19f9ff02bdbdde3ce90b1e1880929a9b`; applies to mio@1.2.5.
- `MIT` — SHA-256 `090a294a492ab2f41388252312a65cf2f0e423330b721a68c6665ac64766753b`; applies to cfg-expr@0.15.8.
- `MIT` — SHA-256 `0ab4d106b6faac07fb6a051815fd1b4d862d730895e2d7d7358c2f13565e7a38`; applies to phf@0.13.1, phf_codegen@0.13.1, phf_generator@0.13.1, phf_macros@0.13.1, phf_shared@0.13.1.
- `MIT` — SHA-256 `0b28172679e0009b655da42797c03fd163a3379d5cfa67ba1f1655e974a2a1a9`; applies to smallvec@1.16.5.
- `MIT` — SHA-256 `0b83dc40cba89b9922bb84b0a9c7d2768ce37c1d7e138b7424fd4549915778c9`; applies to tokio-macros@2.7.2.
- `MIT` — SHA-256 `0d13fdf5615ccc7e7123b58b5c88b0d2bbabe345cd70b94e094ee44034db5be6`; applies to secret-service@4.0.0.
- `MIT` — SHA-256 `0dcf41516e608bbcb6cdc5229feb7b86fe4a643b85e7df251133c93408fdac73`; applies to webview2-com@0.39.1, webview2-com-macros@0.8.1, webview2-com-sys@0.39.1.
- `MIT` — SHA-256 `0dd882e53de11566d50f8e8e2d5a651bcf3fabee4987d70f306233cf39094ba7`; applies to base64@0.21.7, base64@0.22.1.
- `MIT` — SHA-256 `0f96a83840e146e43c0ec96a22ec1f392e0680e6c1226e6f3ba87e0740af850f`; applies to aho-corasick@1.1.5, byteorder@1.5.0, byteorder-lite@0.1.0, memchr@2.8.3, walkdir@2.5.0.
- `MIT` — SHA-256 `11955c617c19899b5c36d78c916739d1686f2e7ad8e3cec684c2f0f73b0db950`; applies to directories@6.0.0.
- `MIT` — SHA-256 `123a331b5dbf04c30097fa43b8f858bc85df671fe776de498d01f3d6b7c1f69e`; applies to libc@0.2.191.
- `MIT` — SHA-256 `157cc607717b1b372a69f6b4526f6ad18c1279af9f3267835cca2943f37bc07d`; applies to keyring@3.6.3.
- `MIT` — SHA-256 `1954992a2b32e8a2af24a4c11b726902e344c1934b947a77f04d404908f8db30`; applies to schemars@0.8.22, schemars_derive@0.8.22.
- `MIT` — SHA-256 `1a2f5c12ddc934d58956aa5dbdd3255fe55fd957633ab7d0d39e4f0daa73f7df`; applies to zerocopy@0.8.63, zerocopy-derive@0.8.63.
- `MIT` — SHA-256 `1abdbbd4e3f95e56c79d4a8af190254c23ddb14db34aadadfafef9c3af041113`; applies to embed-resource@3.0.12.
- `MIT` — SHA-256 `1cb6f4dee8c649f175736380495f0af0c753c87f22d50931692968bb24bf2161`; applies to wayland-backend@0.3.17, wayland-client@0.31.15, wayland-protocols@0.32.13, wayland-protocols-wlr@0.3.12, wayland-scanner@0.31.11, wayland-sys@0.31.11.
- `MIT` — SHA-256 `1e2b7ade3fb228130408b9990cae6a7618eb314c75aa0b164bfe485d9d9756ee`; applies to cfg_aliases@0.2.2.
- `MIT` — SHA-256 `1e697ce8d21401fbf1bddd9b5c3fd4c4c79ae1e3bdf51f81761c85e11d5a89cd`; applies to strsim@0.11.1.
- `MIT` — SHA-256 `200791e43dd83f50c535582da6efa3357f47e61a46b026308b13dd0f7f492b81`; applies to libdbus-sys@0.2.7.
- `MIT` — SHA-256 `209fbbe0ad52d9235e37badf9cadfe4dbdc87203179c0899e738b39ade42177b`; applies to rand@0.8.8, rand_chacha@0.3.1, rand_core@0.6.4.
- `MIT` — SHA-256 `20c097d0de27e2adc179140a3d542091951a030699c7e1c78655f2a2d08d4e3e`; applies to tiff@0.11.3.
- `MIT` — SHA-256 `20c7855c364d57ea4c97889a5e8d98470a9952dade37bd9248b9a54431670e5e`; applies to form_urlencoded@1.2.2.
- `MIT` — SHA-256 `219920e865eee70b7dcfc948a86b099e7f4fe2de01bcca2ca9a20c0a033f2b59`; applies to synstructure@0.14.0.
- `MIT` — SHA-256 `231c837c45eb53f108fb48929e488965bc4fcc14e9ea21d35f50e6b99d98685b`; applies to deranged@0.5.9.
- `MIT` — SHA-256 `23f18e03dc49df91622fe2a76176497404e46ced8a715d9d2b67a7446571cca3`; applies to adler2@2.0.1, anyhow@1.0.104, async-channel@2.5.0, async-executor@1.14.1, async-fs@2.2.0, async-io@2.6.0, async-lock@3.5.0, async-process@2.5.0, async-signal@0.2.14, async-task@4.7.1, async-trait@0.1.92, atomic-waker@1.1.2, blocking@1.7.0, camino@1.2.6, cargo-platform@0.1.9, cargo_metadata@0.19.2, command-group@5.0.1, concurrent-queue@2.5.0, displaydoc@0.2.7, dtoa@1.0.11, dyn-clone@1.0.20, endi@1.1.1, erased-serde@0.4.11, event-listener@5.4.2, event-listener-strategy@0.5.4, fastrand@2.5.0, futures-lite@2.6.1, itoa@1.0.19, lazy_static@1.5.1, linux-raw-sys@0.12.1, once_cell@1.21.4, ordered-stream@0.2.0, parking@2.2.1, pin-project-lite@0.2.17, piper@0.2.5, polling@3.11.0, proc-macro-crate@1.3.1, proc-macro-crate@2.0.2, proc-macro-crate@3.5.0, proc-macro2@1.0.107, quote@1.0.47, ref-cast@1.0.27, ref-cast-impl@1.0.27, rustix@1.1.6, rustversion@1.0.23, semver@1.0.28, serde@1.0.229, serde-untagged@0.1.9, serde_core@1.0.229, serde_derive@1.0.229, serde_derive_internals@0.29.1, serde_json@1.0.154, serde_repr@0.1.21, servo_arc@0.4.3, syn@1.0.109, syn@2.0.119, syn@3.0.10, system-deps@6.2.2, thiserror@1.0.69, thiserror@2.0.21, thiserror-impl@1.0.69, thiserror-impl@2.0.21, typed-path@0.13.0, typeid@1.0.3, unicode-ident@1.0.27, x11@2.21.0, x11-dl@2.21.0, xdg-home@1.3.0, zbus_names@3.0.0, zmij@1.0.23, zvariant_utils@2.1.0, zvariant_utils@4.2.0.
- `MIT` — SHA-256 `248378d0a3383c173fb925f17141b88e71580b3ba17ddc6ac3d2a344683232ab`; applies to http-body@1.1.0, http-body-util@0.1.5.
- `MIT` — SHA-256 `24e5860bf589d8501643e6ea51ffb3df66db2867492b09033d486183efbfa970`; applies to async-broadcast@0.7.2.
- `MIT` — SHA-256 `2537228d9a1b44a5dc595241349cae7090b326c8de165aaf89bfddef4a00d0fc`; applies to time@0.3.55, time-core@0.1.9, time-macros@0.2.32.
- `MIT` — SHA-256 `253cd04c6714889df2d32f3f64d669179a1c95c76ac43c40882c52eb06bc3552`; applies to tokio@1.54.1, tokio-util@0.7.20.
- `MIT` — SHA-256 `260e4ff3fc493695880c90774283eb1e3d018f898fabe655bbd52dd2cbd2fcad`; applies to zbus@4.4.0, zbus@5.19.0, zbus_macros@4.4.0, zbus_macros@5.19.0, zbus_names@4.3.5, zvariant@4.2.0, zvariant@5.15.1, zvariant_derive@4.2.0, zvariant_derive@5.15.1.
- `MIT` — SHA-256 `270b0d81891aa44c7e8726cf9432dade9556a01c848bacd2f3ef839c9c1a1859`; applies to infer@0.22.0.
- `MIT` — SHA-256 `27995d58ad5c1145c1a8cd86244ce844886958a35eb2b78c6b772748669999ac`; applies to autocfg@1.5.1.
- `MIT` — SHA-256 `281b72cb008466810da31680e72005b9899c7e9d7705b909c86098260e12aa7d`; applies to webkit2gtk-sys@2.0.2.
- `MIT` — SHA-256 `29e9fe5074bd27e0e5d5d110394fbbcd841baee2651a3c4b4560a632702cede4`; applies to getrandom@0.3.4.
- `MIT` — SHA-256 `2ab5537b8c0cb1d475e2145b6a7994b04e7c71e6e5bd836f7f9c47c221a5ad9a`; applies to global-hotkey@0.8.0, muda@0.20.0, tray-icon@0.25.1.
- `MIT` — SHA-256 `2bc107bdc18a74263d20bdefdb8073e17bc1a01b6b374dc5835683a33ee5ba7d`; applies to jsonptr@0.7.1.
- `MIT` — SHA-256 `2d01890414494742ba4a509fcec8efa40f6d8be22cbd72be7cff08d6fda4ec89`; applies to hyper@1.12.0.
- `MIT` — SHA-256 `2e3edd2716fb15616e867bd53f724f0906732242e9ae76bb1faad1d88f7eeebd`; applies to enumflags2@0.7.12.
- `MIT` — SHA-256 `304b898acad7f02e03d6d8832d682a3b43ee729ab5d3a069af5a36979083b6b4`; applies to inout@0.1.4.
- `MIT` — SHA-256 `30fefc3a7d6a0041541858293bcbea2dde4caa4c0a5802f996a7f7e8c0085652`; applies to rustc-hash@2.1.3.
- `MIT` — SHA-256 `31b94860253d8ec7b4529f51901044d3b459d6292d996504a36b1bae3a36a812`; applies to cfg_aliases@0.2.2.
- `MIT` — SHA-256 `31dbbab009f1b2e59a1622d5955dfafc38ebe834000215b49d954c6f85fe927c`; applies to keyboard-types@0.7.0, keyboard-types@0.8.3.
- `MIT` — SHA-256 `3234ac55816264ee7b6c7ee27efd61cf0a1fe775806870e3d9b4c41ea73c5cb1`; applies to memoffset@0.9.1.
- `MIT` — SHA-256 `333ea3aaa3cadb819f4acd9f9153f9feee060a995ca8710f32bc5bd9a4b91734`; applies to foreign-types@0.5.0, foreign-types-macros@0.2.4, foreign-types-shared@0.3.1.
- `MIT` — SHA-256 `3488679340a49ecc34d342c4009d2dabf76f4a21f12aec2ca99b15805d656544`; applies to crc@3.4.0.
- `MIT` — SHA-256 `3521672491a3479422d5fe1aca6645dd2984090f85da6e5205abfb18fb7a6897`; applies to crypto-common@0.1.7.
- `MIT` — SHA-256 `378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397`; applies to cc@1.11.2, cfg-if@1.0.5, filetime@0.2.29, find-msvc-tools@0.1.15, pkg-config@0.3.34, socket2@0.6.5, toml_datetime@0.6.3.
- `MIT` — SHA-256 `391a5396cec6230bfabd4ef4eb2350eb895bc5efce377a2218f5702ed020d3e3`; applies to httparse@1.10.1.
- `MIT` — SHA-256 `39712155836e5c10ef1b07037b606e4ad2d633e72205d162fdd7e2290aa54f15`; applies to email_address@0.2.9.
- `MIT` — SHA-256 `39fa265207450e77c62e90c5594a06c085b655d8374c7ced4bf7894b6bd95dd2`; applies to dlopen2@0.8.2, dlopen2_derive@0.4.3.
- `MIT` — SHA-256 `3bb1e548250b3acc43febde65a5cdba9f2e0a3288018aa41f009a3c59ffab51a`; applies to num-cmp@0.1.0.
- `MIT` — SHA-256 `3bc70e239e91272782006c638fc0452a714d384224f11f0923036b7be07cf9b5`; applies to fancy-regex@0.16.2.
- `MIT` — SHA-256 `3d180008e36922a4e8daec11c34c7af264fed5962d07924aea928c38e8663c94`; applies to brotli@9.1.1.
- `MIT` — SHA-256 `3f39c032be4639be635da96d77c4d0f4393c3595a164450ffd86f3d17701ec10`; applies to webkit2gtk@2.0.2.
- `MIT` — SHA-256 `3fa4ca83dcc9237839b1bdeb2e6d16bdfb5ec0c5ce42b24694d8bbf0dcbef72c`; applies to utf8_iter@1.1.0, utf_types@1.0.0.
- `MIT` — SHA-256 `4108245a1f2df9d4e94df8abed5b4ba0759bb2f9b40a6b939f1be141077ae50b`; applies to miniz_oxide@0.8.9, miniz_oxide@0.9.1.
- `MIT` — SHA-256 `41ca90213441b294e7517d5858f0cc85bc12e19ec044001b45bc213de461d33b`; applies to ico@0.6.0.
- `MIT` — SHA-256 `4249c8e6c5ebb85f97c77e6457c6fafc1066406eb8f1ef61e796fbdc5ff18482`; applies to tower@0.5.3, tower-layer@0.3.3, tower-service@0.3.3.
- `MIT` — SHA-256 `42a35170233e83e18856792e748de4c1ce4a63b2afce9a370c89ef3fe23f9f2d`; applies to simd-adler32@0.3.10.
- `MIT` — SHA-256 `42fa16951ce7f24b5a467a40e5b449a1d41e662f97ca779864f053f39e097737`; applies to getrandom@0.2.17.
- `MIT` — SHA-256 `436bc5a105d8e57dcd8778730f3754f7bf39c14d2f530e4cde4bd2d17a83ec3d`; applies to uuid@1.29.2.
- `MIT` — SHA-256 `4455bf75a91154108304cb283e0fea9948c14f13e20d60887cf2552449dea3b1`; applies to shlex@2.0.1.
- `MIT` — SHA-256 `45f522cacecb1023856e46df79ca625dfc550c94910078bd8aec6e02880b3d42`; applies to bytes@1.12.1.
- `MIT` — SHA-256 `47d4e1803702728e03d30f8a848ae2249ba274bbd9f8443e803f7924d83cd371`; applies to reqwest@0.12.28.
- `MIT` — SHA-256 `47dc9ff29128ddfb4d6a0435383c9f89120bc374dbcc1dd00b933a0b28aa7865`; applies to ipnet@2.12.2.
- `MIT` — SHA-256 `4cada0bd02ea3692eee6f16400d86c6508bbd3bafb2b65fed0419f36d4f83e8f`; applies to ppv-lite86@0.2.21.
- `MIT` — SHA-256 `4dbda04344456f09a7a588140455413a9ac59b6b26a1ef7cdf9c800c012d87f0`; applies to nom@8.0.0.
- `MIT` — SHA-256 `4f6bd11a0f17fe5b085ace1daaedb7b002a08316850dd9577c246c6a68a68e8c`; applies to unicase@2.10.0.
- `MIT` — SHA-256 `503558bfefe66ca15e4e3f7955b3cb0ec87fd52f29bf24b336af7bd00e946d5c`; applies to signal-hook-registry@1.4.8.
- `MIT` — SHA-256 `5049cf464977eff4b4fcfa7988d84e74116956a3eb9d5f1d451b3f828f945233`; applies to tower-http@0.6.11.
- `MIT` — SHA-256 `508a77d2e7b51d98adeed32648ad124b7b30241a8e70b2e72c99f92d8e5874d1`; applies to half@2.7.1, ident_case@1.0.1.
- `MIT` — SHA-256 `523a42c25d245dde9c015f882cec7f4555aad883382a6cf19b4b7d9b2cd5419b`; applies to getrandom@0.4.3.
- `MIT` — SHA-256 `5444779e62a6f136892955bab2727694509b2cc1c0cd0f394d5edc7ff9203cd1`; applies to javascriptcore-rs@1.1.2.
- `MIT` — SHA-256 `544b3aed1fd723d0cadea567affdcfe0431e43e18d997a718f9d67256b814fde`; applies to proc-macro-error@1.0.4, proc-macro-error-attr@1.0.4.
- `MIT` — SHA-256 `5734ed989dfca1f625b40281ee9f4530f91b2411ec01cb748223e7eb87e201ab`; applies to crossbeam-channel@0.5.17, crossbeam-utils@0.8.23.
- `MIT` — SHA-256 `573f5227db835cd30bbdd3aed3f82615f80119a4d5c82b4daf642dbfe2b6afde`; applies to weezl@0.1.12.
- `MIT` — SHA-256 `58545fed1565e42d687aecec6897d35c6d37ccb71479a137c0deb2203e125c79`; applies to tracing-core@0.1.36, zip@9.6.0.
- `MIT` — SHA-256 `59871bb1612f8a0569655c10b995636827eceab42c6578cebdc45ad2922e0142`; applies to zcheapstr@1.2.0.
- `MIT` — SHA-256 `5b0ae40d1a35f7ae6591a28e44771240e6a88cb03a66c9189a45b9681639b466`; applies to plist@1.10.3.
- `MIT` — SHA-256 `5b2c207dcf571267ebe0bbdd5bf321f561ddc22ff597e7b363e96f4539507a29`; applies to quick-xml@0.41.0.
- `MIT` — SHA-256 `5c7bd92d1f096f12203dc1b601e3cb17484fa1b023e30b6b8edcc80e416237ec`; applies to cipher@0.4.4.
- `MIT` — SHA-256 `5cf1c9dc617e4cad7fa03181ff893601ad5243aa3be527ef39034e1cd6377676`; applies to urlpattern@0.6.0.
- `MIT` — SHA-256 `5e05b024f653a5ce199e77cbbbd42fb5553562ec714b819421ed0c3e552a75d7`; applies to stable_deref_trait@1.2.1.
- `MIT` — SHA-256 `5ef8fcfb6cccec8fcae043c834099a60c8b7406408db576e026d2b7e67dc5cf5`; applies to crc-catalog@2.5.0.
- `MIT` — SHA-256 `60a7062291b01ba068f300612cdbdc20382ac1d4934f07bcdd7167c15299f309`; applies to tendril@0.5.1.
- `MIT` — SHA-256 `617b85f62a4f1b6a62d3d9f28f6d7003b7f68a70b7675a8174bbcc68401ac62d`; applies to window-vibrancy@0.8.1.
- `MIT` — SHA-256 `61d383b05b87d78f94d2937e2580cce47226d17823c0430fbcad09596537efcf`; applies to crc32fast@1.5.5.
- `MIT` — SHA-256 `62065228e42caebca7e7d7db1204cbb867033de5982ca4009928915e4095f3a3`; applies to core-foundation@0.10.1, core-foundation-sys@0.8.7, core-graphics@0.25.0, core-graphics-types@0.2.0, string_cache@0.9.0, string_cache_codegen@0.6.1.
- `MIT` — SHA-256 `6226d0632e2e1a80c23597e964da9812ae193c535fe058154afb034e94167aa5`; applies to atomic-waker@1.1.2, futures-lite@2.6.1.
- `MIT` — SHA-256 `63af4bea227c94d021e99427f6ca3a4b8efddadcca93ab6130f708cf6138cf68`; applies to cbc@0.1.2.
- `MIT` — SHA-256 `6485b8ed310d3f0340bf1ad1f47645069ce4069dcc6bb46c7d5c6faf41de1fdb`; applies to bitflags@1.3.2, bitflags@2.13.2, fraction@0.15.4, glob@0.3.4, log@0.4.35, num@0.4.3, num-bigint@0.4.8, num-complex@0.4.6, num-integer@0.1.47, num-iter@0.1.46, num-rational@0.4.2, num-traits@0.2.19, regex@1.13.1, regex-automata@0.4.18, regex-syntax@0.8.11.
- `MIT` — SHA-256 `65f94e99ddaf4f5d1782a6dae23f35d4293a9a01444a13135a6887017d353cee`; applies to bumpalo@3.20.4.
- `MIT` — SHA-256 `65fc947f4bac882a2cd104ef73c246b1b037059df9f2b3a58b7f77f0b9e56071`; applies to rfd@0.17.2.
- `MIT` — SHA-256 `65fdb6c76cd61612070c066eec9ecdb30ee74fb27859d0d9af58b9f499fd0c3e`; applies to fnv@1.0.7.
- `MIT` — SHA-256 `6652c868f35dfe5e8ef636810a4e576b9d663f3a17fb0f5613ad73583e1b88fd`; applies to futures-channel@0.3.34, futures-core@0.3.34, futures-executor@0.3.34, futures-io@0.3.34, futures-macro@0.3.34, futures-sink@0.3.34, futures-task@0.3.34, futures-util@0.3.34.
- `MIT` — SHA-256 `66e3ee1fa7f909ad3c612d556f2a0cdabcd809ad6e66f3b0605015ac64841b70`; applies to nix@0.27.1, nix@0.29.0, nix@0.32.0.
- `MIT` — SHA-256 `6a2e0ade09a7d5f816f11566fee2b151b32235a7fad52b41d49cce96f833c1a9`; applies to dirs@7.0.0, dirs-sys@0.5.0.
- `MIT` — SHA-256 `6efb0476a1cc085077ed49357026d8c173bf33017278ef440f222fb9cbcb66e6`; applies to serde_spanned@0.6.9, serde_spanned@1.1.3, toml@0.8.2, toml@1.2.1+spec-1.1.0, toml_datetime@1.1.3+spec-1.1.0, toml_edit@0.19.15, toml_edit@0.20.2, toml_edit@0.25.19+spec-1.1.0, toml_parser@1.1.5+spec-1.1.0, toml_writer@1.2.0+spec-1.1.0.
- `MIT` — SHA-256 `709e3175b4212f7b13aa93971c9f62ff8c69ec45ad8c6532a7e0c41d7a7d6f8c`; applies to hyper-rustls@0.27.10, rustls@0.23.46.
- `MIT` — SHA-256 `7101390b4c8edd752f4c0de47f35d61b44d9cf0ee8dd032115e25644e277cfff`; applies to fax@0.2.7.
- `MIT` — SHA-256 `71674605ec4c087fe9eb534e3e4f9e26eb2e4aabcd76a29fd156c6a844d44b3d`; applies to uuid-simd@0.8.0, vsimd@0.8.0.
- `MIT` — SHA-256 `7365cc8878a1d7ce155a58c4ca09c3d7a6be413efa5334a80ea842912b669349`; applies to equivalent@1.0.3.
- `MIT` — SHA-256 `7377e48a67377f0baa9ccee6425e125eb7b96d5b8e706f4c99479ca4ffa21b77`; applies to is-docker@0.2.0, is-wsl@0.4.0.
- `MIT` — SHA-256 `748c208045a6f00734dbc85a1c0e401c9a25e1f83fc756e99916750dbd2469a7`; applies to static_vcruntime@3.0.0.
- `MIT` — SHA-256 `7554701d1e74fc434f68c522b1dd743c8c16d816a4b761b5213f367b898884c8`; applies to dom_query@0.28.0.
- `MIT` — SHA-256 `7576269ea71f767b99297934c0b2367532690f8c4badc695edf8e04ab6a1e545`; applies to petgraph@0.8.3, serde_with@3.24.0, serde_with_macros@3.24.0.
- `MIT` — SHA-256 `799e9ca9d179295ef372f25d3769cdda7d25bb2668add6a6a1e22d1e4c678b8d`; applies to miniz_oxide@0.8.9, miniz_oxide@0.9.1.
- `MIT` — SHA-256 `7ade43ec5e1a5e43000aa871d290dff4c0e9e2e16b7f6bea2ecf091e3d056d31`; applies to javascriptcore-rs-sys@1.1.1, soup3@0.5.0, soup3-sys@0.5.0.
- `MIT` — SHA-256 `7b63ecd5f1902af1b63729947373683c32745c16a10e8e6292e2e2dcd7e90ae0`; applies to heck@0.4.1, heck@0.5.0, unicode-segmentation@1.13.3.
- `MIT` — SHA-256 `7ca6700600dfa9c9497bf5556365067daa802c871ea78239f129309c7a2048f7`; applies to precomputed-hash@0.1.1.
- `MIT` — SHA-256 `816000f9c28f7005d931c77636eb79fa7020071eb0f6e6a995515dfccd3559cb`; applies to mime_guess@2.0.5.
- `MIT` — SHA-256 `86dd7f026f916daf7511e39951ad8ea8cf55a8db67ae64060dacf829761c18f3`; applies to html5ever@0.39.0, markup5ever@0.39.0, web_atoms@0.2.6.
- `MIT` — SHA-256 `875e64a5df9f892ebd29c71ad0b58a765b5789d7e9142aa70ec07fae74a67686`; applies to xkeysym@0.2.1.
- `MIT` — SHA-256 `8764a597675778ddfd4e25f81b08a05dbcf089ac05662df7613fe67f150e3aa2`; applies to errno@0.3.15.
- `MIT` — SHA-256 `87f7527b1066337751f7f4d717089bf514ff8f7ee7d66c4c5b891b09e744ec11`; applies to json-patch@4.2.0.
- `MIT` — SHA-256 `898b1ae9821e98daf8964c8d6c7f61641f5f5aa78ad500020771c0939ee0dea1`; applies to tracing@0.1.44, tracing-attributes@0.1.31, tracing-core@0.1.36.
- `MIT` — SHA-256 `89ff9689dcf9dd53968785d05a26f7898bb169dbfcada8d032b3e68cf0d55607`; applies to tauri-plugin-clipboard-manager@2.5.0, tauri-plugin-dialog@2.8.4, tauri-plugin-fs@2.7.1, tauri-plugin-global-shortcut@2.4.1, tauri-plugin-opener@2.7.3.
- `MIT` — SHA-256 `8a35369f3ca263b3c62fbb5032947e53b6bfebc6c8a4d1bb982de1c069f6fba5`; applies to derive_more@2.1.1, derive_more-impl@2.1.1.
- `MIT` — SHA-256 `8b427f5bc501764575e52ba4f9d95673cf8f6d80a86d0d06599852e1a9a20a36`; applies to tempfile@3.27.0, xattr@1.7.0.
- `MIT` — SHA-256 `8b43ce8accd61e9d370b5ca9e9c4f953279b5c239926c62315b40e24df51b726`; applies to idna_adapter@1.2.2.
- `MIT` — SHA-256 `8c7516d4b27b1e495be5e38b612298b63de48d05f49cdac94f70f3cd70f8864b`; applies to zeroize@1.9.1.
- `MIT` — SHA-256 `8ca6b96cea9e67c6c5c63f452c31bd396db8bd2406231fdea5d48ef462b48077`; applies to tar@0.4.46.
- `MIT` — SHA-256 `8ce0830173fdac609dfb4ea603fdc002c2f4af0dc9b1a005653f5da9cf534b18`; applies to slab@0.4.12.
- `MIT` — SHA-256 `8cf56d10131ce201cf69ab74b111d3ebac1acca3833d7efb39ae357224b70edb`; applies to atk@0.18.2, atk-sys@0.18.2, cairo-rs@0.18.5, cairo-sys-rs@0.18.2, gdk@0.18.2, gdk-pixbuf@0.18.5, gdk-pixbuf-sys@0.18.0, gdk-sys@0.18.2, gdkwayland-sys@0.18.2, gdkx11@0.18.2, gdkx11-sys@0.18.2, gio@0.18.4, gio-sys@0.18.1, glib@0.18.5, glib-macros@0.18.5, glib-sys@0.18.1, gobject-sys@0.18.0, gtk@0.18.2, gtk-sys@0.18.2, gtk3-macros@0.18.2, pango@0.18.3, pango-sys@0.18.0.
- `MIT` — SHA-256 `8ea93490d74a5a1b1af3ff71d786271b3f1e5f0bea79ac16e02ec533cef040d6`; applies to darling@0.24.1, darling_core@0.24.1, darling_macro@0.24.1.
- `MIT` — SHA-256 `8f71659370c5268d9a1dc962a46232540e8fca63462586d8efaa95aab492a208`; applies to os_pipe@1.2.3.
- `MIT` — SHA-256 `9117d922e667125508dde62b02c1f57ed22f5ad21eb536aa2e2d99e1c796e639`; applies to rustls-pki-types@1.16.0.
- `MIT` — SHA-256 `9c5a80639a57c1c945570e7ebbca0706305849ce3c098021325cca9db2f7acc4`; applies to raw-window-handle@0.6.2.
- `MIT` — SHA-256 `9dd42ea92cff2ede5cd477cbfcce051b2d0115c0ac7f368ee88cb545055dff1d`; applies to tauri@2.12.5, tauri-build@2.7.4, tauri-codegen@2.7.4, tauri-macros@2.7.4, tauri-plugin@2.7.2, tauri-runtime@2.12.2, tauri-runtime-wry@2.12.3, tauri-utils@2.10.2.
- `MIT` — SHA-256 `9dd44491b4cf95dcfa2963943b3ae91bff9a78d13049a9e0b53e0d0b0a9c8f4e`; applies to field-offset@0.3.6.
- `MIT` — SHA-256 `9df9ba60a11af705f2e451b53762686e615d86f76b169cf075c3237730dbd7e2`; applies to bytemuck@1.25.2.
- `MIT` — SHA-256 `9e0a97848ea543aef745c98e84fde696a9a3e0735538f6daefdd3cb1942effc1`; applies to hyper-util@0.1.21.
- `MIT` — SHA-256 `9e0dfd2dd4173a530e238cb6adb37aa78c34c6bc7444e0e10c1ab5d8881f63ba`; applies to digest@0.10.7, hmac@0.12.1.
- `MIT` — SHA-256 `9eff396a99bafd208e7126602420d6864f6066c08b84485a5b032320a9c01997`; applies to dpi@0.1.2.
- `MIT` — SHA-256 `9fa38f610a00115dd1f612bdd3559e3b381ea579538b6ff25a5899da250a5fce`; applies to fluent-uri@0.3.2.
- `MIT` — SHA-256 `a00511478c83ebb464ef19d840f061c7444adc4636bb561900e75c5808cfc6cc`; applies to outref@0.5.2.
- `MIT` — SHA-256 `a573f030c2ae7eab837ab7d66440954ae54c5eabe6848c1a1fe9187215f50591`; applies to jsonschema@0.33.0, referencing@0.33.0.
- `MIT` — SHA-256 `a5dea80c1f383cb5f80a6bb0da5e55a2beb9f24adb123ce6300af2cbaaa3bf65`; applies to bytecount@0.6.9.
- `MIT` — SHA-256 `a65f5d0a945d267751344c95665945b90c030ea107faf5c85d518929886187da`; applies to want@0.3.2.
- `MIT` — SHA-256 `a72561f47c1f665535930f606739d9098a6186802d5e2e8ec491faf00f4add5b`; applies to x11rb@0.13.2, x11rb-protocol@0.13.2.
- `MIT` — SHA-256 `a7cea8a5a56ba02950d098a7c56eed312fc745ceecd437e4aad2f7f8d38661b8`; applies to tree_magic_mini@3.2.2.
- `MIT` — SHA-256 `a825bd853ab71619a4923d7b4311221427848070ff44d990da39b0b274c1683f`; applies to typenum@1.20.1.
- `MIT` — SHA-256 `ab499c75a0f0da8e0fe83bf9ba3a27e5c39705310f07107c6c66b69958d2401c`; applies to base64@0.23.1.
- `MIT` — SHA-256 `ad9f07faa5f6f671df93523293cf8c1ce24f14886fa101013c21536c7890c59b`; applies to lzma-rs@0.3.0.
- `MIT` — SHA-256 `ae26bc7b1bc91fb8648f6a298decbb1a77090a56c9d04e2c23b8365e5f54acb5`; applies to rfd@0.17.2.
- `MIT` — SHA-256 `ae9baa7beea910273c2f384c2a6b721fb7bd02bda3436074a1072e4ee689f985`; applies to cpufeatures@0.2.17.
- `MIT` — SHA-256 `b16db96b93b1d7cf7bea533f572091ec6bca3234fbe0a83038be772ff391a44c`; applies to crossbeam-channel@0.5.17.
- `MIT` — SHA-256 `b38f11f6096706e6de553dabe2a7ed142d59b6fa8c97e290c67496154745cdd5`; applies to idna@1.1.0, percent-encoding@2.3.2, url@2.5.8.
- `MIT` — SHA-256 `b4eb00df6e2a4d22518fcaa6a2b4646f249b3a3c9814509b22bd2091f1392ff1`; applies to sha1@0.10.7, sha2@0.10.9.
- `MIT` — SHA-256 `b63e167d61ced1d872a588c5ac764b0dda477d62209dc4f6b6bfea5517d0a24c`; applies to notify-types@2.3.0.
- `MIT` — SHA-256 `b7e650f3fce5c53249d1cdc608b54df156a97edd636cf9d23498d0cfe7aec63e`; applies to version_check@0.9.5.
- `MIT` — SHA-256 `b7f455413bfe5f9d7da501a4468d4c5f866a432c325947e98cc9535e8b26cd6d`; applies to tauri-plugin-single-instance@2.5.5.
- `MIT` — SHA-256 `b8c6939380a400f53e11923d50fcc4dd2fa1ba8339fd9d04cda38a0251b6c9b0`; applies to zeroize_derive@1.5.0.
- `MIT` — SHA-256 `b8da543ff3646f13169e34cd178c3a741ade9bdc74747296ed8abcb78d49ff69`; applies to block2@0.6.2, dispatch2@0.3.1, objc2@0.6.5, objc2-app-kit@0.3.2, objc2-core-foundation@0.3.2, objc2-core-graphics@0.3.2, objc2-core-services@0.3.2, objc2-encode@4.1.0, objc2-exception-helper@0.1.1, objc2-foundation@0.3.2, objc2-quartz-core@0.3.2, objc2-web-kit@0.3.2.
- `MIT` — SHA-256 `b9b7e5d4bf8857061fbecab39ef1923644eba639337af22d7ad0bb9b3ce59c49`; applies to vswhom@0.1.0, vswhom-sys@0.1.3.
- `MIT` — SHA-256 `b9eb266294324f672cbe945fe8f2e32f85024f0d61a1a7d14382cdde0ac44769`; applies to serde_urlencoded@0.7.1.
- `MIT` — SHA-256 `bccaa8b6c09f94e81f06696e179dbe058464bbdfbc823b6d49cada1d71e84ac3`; applies to ctor@1.0.14.
- `MIT` — SHA-256 `c09aae9d3c77b531f56351a9947bc7446511d6b025b3255312d3e3442a9a7583`; applies to generic-array@0.14.7.
- `MIT` — SHA-256 `c2cfccb812fe482101a8f04597dfc5a9991a6b2748266c47ac91b6a5aae15383`; applies to windows@0.62.2, windows-collections@0.3.2, windows-core@0.62.2, windows-future@0.3.2, windows-implement@0.60.2, windows-interface@0.59.3, windows-link@0.2.1, windows-numerics@0.3.1, windows-result@0.4.1, windows-strings@0.5.1, windows-sys@0.59.0, windows-sys@0.60.2, windows-sys@0.61.2, windows-targets@0.52.6, windows-targets@0.53.5, windows-threading@0.2.1, windows-version@0.1.7, windows_x86_64_msvc@0.52.6, windows_x86_64_msvc@0.53.1.
- `MIT` — SHA-256 `c77a4cf9da729987d0fe7ccd811e3bd27393914ddf3d23467c18cc22954513b3`; applies to fdeflate@0.3.7, image@0.25.10.
- `MIT` — SHA-256 `c816a0749cdc6bf062a5111c159723de51b2bfac66a1dac2655abd9e6b1583eb`; applies to try-lock@0.2.5.
- `MIT` — SHA-256 `c9a75f18b9ab2927829a208fc6aa2cf4e63b8420887ba29cdb265d6619ae82d5`; applies to lock_api@0.4.14, parking_lot@0.12.5, parking_lot_core@0.9.12, rustc_version@0.4.1.
- `MIT` — SHA-256 `cb3c929a05e6cbc9de9ab06a4c57eeb60ca8c724bef6c138c87d3a577e27aa14`; applies to same-file@1.0.6, winapi-util@0.1.11.
- `MIT` — SHA-256 `cb5aedb296c5246d1f22e9099f925a65146f9f0d6b4eebba97fd27a6cdbbab2d`; applies to winnow@0.5.40, winnow@1.0.5.
- `MIT` — SHA-256 `cdbd06e25b8c9c5d6019949d2de8123f56d29cee4166396423d2ba8c81700845`; applies to async-recursion@1.2.0.
- `MIT` — SHA-256 `ce592787ff2321feab698a4c612237f4378cc658ebb1d472913e5802cc47afb4`; applies to fixedbitset@0.5.9.
- `MIT` — SHA-256 `ce7bc3499fee93d5022ef430d5e4201e79a6d9154f3974e42f41349f0569e09b`; applies to winapi@0.3.9.
- `MIT` — SHA-256 `cedfcc7ace1639adb054bd4a19b6f8585ccb2429a60b4bf51bfeb0601058c22c`; applies to version-compare@0.2.1.
- `MIT` — SHA-256 `d0235d2a1f657b6fd18fd4ce98988348969a9284d94281b6d117830aa259a07a`; applies to wl-clipboard-rs@0.9.5.
- `MIT` — SHA-256 `d13669f0eaff9b7015bff0d8637297213b53d9f239826636c07a15633acf7052`; applies to enumflags2_derive@0.7.12.
- `MIT` — SHA-256 `d288f9c9b4590446ec18c22ead8f8b5a12a3d4025b68f62dc9015063eb9cca69`; applies to hkdf@0.12.4.
- `MIT` — SHA-256 `d30047bca3b516639339a3c279bb84c3483124fb5a9dafe3c75056a85090e745`; applies to zune-core@0.5.3, zune-jpeg@0.5.15.
- `MIT` — SHA-256 `d5c22aa3118d240e877ad41c5d9fa232f9c77d757d4aac0c2f943afc0a95e0ef`; applies to block-buffer@0.10.4, block-padding@0.3.3.
- `MIT` — SHA-256 `d9fa6b0e6fed50d4961b9c931587af6bd127df13e9473418d3e324f427db4f06`; applies to tauri-winres@0.3.7.
- `MIT` — SHA-256 `daacc37f680c7dd0a758ac99a109f2f57d9f21d3683df9e9245055922842a695`; applies to embed_plist@1.2.2.
- `MIT` — SHA-256 `dc91f8200e4b2a1f9261035d4c18c33c246911a6c0f7b543d75347e61b249cff`; applies to http@1.5.0.
- `MIT` — SHA-256 `de3911c2d98c8bd2d701ee721347053d9b55995a11f9a8c955e44d3ca1b376bf`; applies to dbus@0.9.12, libdbus-sys@0.2.7.
- `MIT` — SHA-256 `df9cfd06d8a44d9a671eadd39ffd97f166481da015a30f45dfd27886209c5922`; applies to mime@0.3.18.
- `MIT` — SHA-256 `e20fa2b8e0a2565f24a792b94b4bf4b6c2b9d36f781d8a9516e218a036e6677a`; applies to tokio-rustls@0.26.6.
- `MIT` — SHA-256 `e2e245f2b566d0bfafb0f6a04c16b82d710c4e6e7d799e39bb4ef6e541b85b98`; applies to num-conv@0.2.2.
- `MIT` — SHA-256 `e53a7165e8bc3ced5e5628404b3d2d28ecc14ad720f1c09ba28385d5151ecf93`; applies to serialize-to-javascript@0.1.2, serialize-to-javascript-impl@0.1.2.
- `MIT` — SHA-256 `e63690624d604aa626c3c62b6ead1a9f4a916d2260ac8132022acbafe2556045`; applies to open@5.4.5.
- `MIT` — SHA-256 `ea021d8c6383c84d19c15ce90d3b1602cf448fdcc0bf3772c98e43c7bf1bf1d1`; applies to winreg@0.56.0.
- `MIT` — SHA-256 `ea084a2373ebc1f0902c09266e7bf25a05ab3814c1805bb017ffa7308f90c061`; applies to static_assertions@1.1.0.
- `MIT` — SHA-256 `ea417315dc653b10a03911cbc3a724fafa75d3ed6381b358e3bed8ed2b50ffea`; applies to schemars_derive@0.8.22.
- `MIT` — SHA-256 `eaf40297c75da471f7cda1f3458e8d91b4b2ec866e609527a13acfa93b638652`; applies to png@0.18.1.
- `MIT` — SHA-256 `eb227437252b2a7a9c1fc342c93ade1f3d7ce38cc6dd754f613db07d53ceff0b`; applies to libappindicator@0.9.0, libappindicator-sys@0.9.0.
- `MIT` — SHA-256 `ebaac853b53fec0ed475796f533ce676912a4df9ad9d8a621fa76d5060591e10`; applies to cfb@0.14.0.
- `MIT` — SHA-256 `ecc269ef87fd38a1d98e30bfac9ba964a9dbd9315c3770fed98d4d7cb5882055`; applies to indexmap@1.9.3, indexmap@2.14.2.
- `MIT` — SHA-256 `ee874098519e956c23a3c5c7e20581f8e9b1d70aa05eb178f983be9775090a2f`; applies to cargo_toml@1.0.1.
- `MIT` — SHA-256 `ef88893533af22ffbb68ceef0724ba1389518df26165a34f3c99ad30b884df4f`; applies to wry@0.57.2.
- `MIT` — SHA-256 `f0cf9b1c62bbe3bd3a69f5f79c7158f513f612b4940a0a812d1db39d605318bc`; applies to quick-xml@0.42.0.
- `MIT` — SHA-256 `f1a2b233e8a9a71c40f4aa885be08a0842ac85bb8588703c1dd7e6e6502e3124`; applies to wayland-protocols@0.32.13.
- `MIT` — SHA-256 `f28420c1906af38be726cd7842798f0194b27c41d6051c0d1e9ee348b8a4a0ea`; applies to cookie@0.18.2.
- `MIT` — SHA-256 `f35468024534e0f4f16bb94cd0e88c87fbd1314f919c2645a07d350fb77b40c0`; applies to arboard@3.7.0.
- `MIT` — SHA-256 `f3c3e9f4febe81ff14ccc233c344e6e60051290d1c1e5a2c670d63cc3608a5bb`; applies to swift-rs@1.0.9.
- `MIT` — SHA-256 `f428305bbf2e70fbd807af0f3c7ac1b7acfb95311640f451e9d2e48db6d7e96b`; applies to web-time@1.1.0.
- `MIT` — SHA-256 `f51ac2c59a222f7476ce507ca879960e2b64ea64bb2786eefdbeb7b0b538d1b7`; applies to bit-set@0.8.0, bit-vec@0.8.0.
- `MIT` — SHA-256 `f7715d38a3fa1b4ac97c5729740752505a39cb92ee83ab5b102aeb5eaa7cdea4`; applies to new_debug_unreachable@1.0.6.
- `MIT` — SHA-256 `f7bdb3426d045cd50efd4953026e3eb5a83d0199f458a075602611b9344da5b9`; applies to hex@0.4.3.
- `MIT` — SHA-256 `f7e8ab639afef15573680c97f796166835cbeb3865175882fea41c60d106b733`; applies to aes@0.8.4.
- `MIT` — SHA-256 `f9c4c77baa3828004ee54b8a4f2db2e88ed44a6237a493965bf551fac0fcb62d`; applies to siphasher@1.0.4.
- `MIT` — SHA-256 `fb252fecf98d9af8f1d9201c5860319a514f5a8a24700dc84fc6ea287a7a4958`; applies to downcast-rs@1.2.1.
- `MIT` — SHA-256 `fb77f0a9c53e473abe5103c8632ef9f0f2874d4fb3f17cb2d8c661aab9cee9d7`; applies to scopeguard@1.2.0.
- `MIT` — SHA-256 `ff8f68cb076caf8cefe7a6430d4ac086ce6af2ca8ce2c4e5a2004d4552ef52a2`; applies to hashbrown@0.12.3, hashbrown@0.15.5, hashbrown@0.17.1.
- `MIT-0` — SHA-256 `010492e490b92ec85db01daad2a290ea5959f29314a1c4ff25e7a172e223abf9`; applies to borrow-or-share@0.2.4.
- `MPL-2.0` — SHA-256 `1a389c0a135f37855baa2fb879f86ed111d7facf8b0d5188330b35a22c4428a1`; applies to selectors@0.38.0.
- `MPL-2.0` — SHA-256 `1f256ecad192880510e84ad60474eab7589218784b9a50bc7ceee34c2b91f1d5`; applies to dtoa-short@0.3.5.
- `MPL-2.0` — SHA-256 `66a3107d5ad6a058aab753eaac2047ccb2ed0e39465dd0fe5844da3e300d5172`; applies to option-ext@0.2.0.
- `MPL-2.0` — SHA-256 `fab3dd6bdab226f1c08630b1dd917e11fcb4ec5e1e020e2c16f83a0a13863e85`; applies to cssparser@0.37.0, cssparser-macros@0.7.1.
- `Unicode-3.0` — SHA-256 `f367c1b8e1aa262435251e442901da4607b4650e0e63a026f5044473ecfb90f2`; applies to icu_collections@2.4.0, icu_locale_core@2.4.0, icu_normalizer@2.4.0, icu_normalizer_data@2.4.0, icu_properties@2.4.0, icu_properties_data@2.4.0, icu_provider@2.4.0, litemap@0.8.3, potential_utf@0.1.6, tinystr@0.8.5, writeable@0.6.4, yoke@0.8.4, yoke-derive@0.8.4, zerofrom@0.1.8, zerofrom-derive@0.1.8, zerotrie@0.2.5, zerovec@0.11.9, zerovec-derive@0.11.7.
- `Unicode-3.0` — SHA-256 `f7db81051789b729fea528a63ec4c938fdcb93d9d61d97dc8cc2e9df6d47f2a1`; applies to unicode-ident@1.0.27.
- `Zlib` — SHA-256 `b1181a40b2a7b25cf66fd01481713bc1005df082c53ef73e851e55071b102744`; applies to foldhash@0.1.5, foldhash@0.2.0.
- `Zlib` — SHA-256 `e72111c52b7d96ebe25348dee19f0744f444d3c95ae6b1ecb6ccaecc5bce05ba`; applies to zlib-rs@0.6.8.

## Supplemental upstream notices

These package files preserve notices and copyright attribution that are separate from selected SPDX branches.
- `adler2@2.0.1` `adler2@2.0.1/LICENSE-0BSD` (license text; SHA-256 `861399f8c21c042b110517e76dc6b63a2b334276c8cf17412fc3c8908ca8dc17`).
- `adler2@2.0.1` `adler2@2.0.1/LICENSE-APACHE` (license text; SHA-256 `8ada45cd9f843acf64e4722ae262c622a2b3b3007c7310ef36ac1061a30f6adb`).
- `aes@0.8.4` `aes@0.8.4/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `ahash@0.8.12` `ahash@0.8.12/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `aho-corasick@1.1.5` `aho-corasick@1.1.5/COPYING` (notice; SHA-256 `01c266bced4a434da0051174d6bee16a4c82cf634e2679b6155d40d75012390f`).
- `anyhow@1.0.104` `anyhow@1.0.104/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `arboard@3.7.0` `arboard@3.7.0/LICENSE-APACHE.txt` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `async-broadcast@0.7.2` `async-broadcast@0.7.2/LICENSE-APACHE` (license text; SHA-256 `e4705ddab847449a2cdcb3c88b005ea10330aa249d9148ca2eef9c84c5d29895`).
- `async-channel@2.5.0` `async-channel@2.5.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `async-executor@1.14.1` `async-executor@1.14.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `async-fs@2.2.0` `async-fs@2.2.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `async-io@2.6.0` `async-io@2.6.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `async-lock@3.5.0` `async-lock@3.5.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `async-process@2.5.0` `async-process@2.5.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `async-recursion@1.2.0` `async-recursion@1.2.0/LICENSE-APACHE` (license text; SHA-256 `b8601fc79bdf8d8088f0938bea861ffc63ed0eebbedf25f44ab2cf2e85bd7edc`).
- `async-signal@0.2.14` `async-signal@0.2.14/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `async-task@4.7.1` `async-task@4.7.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `async-trait@0.1.92` `async-trait@0.1.92/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `atk@0.18.2` `atk@0.18.2/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `atomic-waker@1.1.2` `atomic-waker@1.1.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `autocfg@1.5.1` `autocfg@1.5.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `base64@0.21.7` `base64@0.21.7/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `base64@0.22.1` `base64@0.22.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `base64@0.23.1` `base64@0.23.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `bit-set@0.8.0` `bit-set@0.8.0/LICENSE-APACHE` (license text; SHA-256 `8173d5c29b4f956d532781d2b86e4e30f83e6b7878dce18c919451d6ba707c90`).
- `bit-vec@0.8.0` `bit-vec@0.8.0/LICENSE-APACHE` (license text; SHA-256 `8173d5c29b4f956d532781d2b86e4e30f83e6b7878dce18c919451d6ba707c90`).
- `bitflags@1.3.2` `bitflags@1.3.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `bitflags@2.13.2` `bitflags@2.13.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `block-buffer@0.10.4` `block-buffer@0.10.4/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `block-padding@0.3.3` `block-padding@0.3.3/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `block2@0.6.2` `third-party/license-texts/block2-0.6.2/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `blocking@1.7.0` `blocking@1.7.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `bumpalo@3.20.4` `bumpalo@3.20.4/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `bytecount@0.6.9` `bytecount@0.6.9/LICENSE.Apache2` (license text; SHA-256 `b40930bbcf80744c86c46a12bc9da056641d722716c378f5659b9e555ef833e1`).
- `bytemuck@1.25.2` `bytemuck@1.25.2/LICENSE-APACHE` (license text; SHA-256 `e3ba223bb1423f0aad8c3dfce0fe3148db48926d41e6fbc3afbbf5ff9e1c89cb`).
- `bytemuck@1.25.2` `bytemuck@1.25.2/LICENSE-ZLIB` (license text; SHA-256 `84b34dd7608f7fb9b17bd588a6bf392bf7de504e2716f024a77d89f1b145a151`).
- `byteorder@1.5.0` `byteorder@1.5.0/COPYING` (notice; SHA-256 `01c266bced4a434da0051174d6bee16a4c82cf634e2679b6155d40d75012390f`).
- `cairo-rs@0.18.5` `cairo-rs@0.18.5/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `camino@1.2.6` `camino@1.2.6/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `cargo-platform@0.1.9` `cargo-platform@0.1.9/LICENSE-APACHE` (license text; SHA-256 `8ada45cd9f843acf64e4722ae262c622a2b3b3007c7310ef36ac1061a30f6adb`).
- `cbc@0.1.2` `cbc@0.1.2/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `cc@1.11.2` `cc@1.11.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `cfg-expr@0.15.8` `cfg-expr@0.15.8/LICENSE-APACHE` (license text; SHA-256 `8173d5c29b4f956d532781d2b86e4e30f83e6b7878dce18c919451d6ba707c90`).
- `cfg-if@1.0.5` `cfg-if@1.0.5/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `cipher@0.4.4` `cipher@0.4.4/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `command-group@5.0.1` `command-group@5.0.1/COPYRIGHT` (notice; SHA-256 `79e3d78732053b970daae13f4240900019285afd4ce4bb9aa24bb18688cc1708`).
- `command-group@5.0.1` `command-group@5.0.1/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `concurrent-queue@2.5.0` `concurrent-queue@2.5.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `constant_time_eq@0.6.1` `constant_time_eq@0.6.1/LICENSE-CC0` (license text; SHA-256 `a2010f343487d3f7618affe54f789f5487602331c0a8d03f49e9a7c547cf0499`).
- `constant_time_eq@0.6.1` `constant_time_eq@0.6.1/LICENSE-MIT0` (license text; SHA-256 `9a3aeb3f63659c4c6302d315e1d98784d1e8559858a47ff9562e091bb922d28d`).
- `cookie@0.18.2` `cookie@0.18.2/LICENSE-APACHE` (license text; SHA-256 `2773e20df8f4c52a026b5b578c7f2457341f5aa3fb6612fd87e1d2e1bd8f48ad`).
- `core-foundation@0.10.1` `core-foundation@0.10.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `core-foundation-sys@0.8.7` `core-foundation-sys@0.8.7/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `core-graphics@0.25.0` `core-graphics@0.25.0/COPYRIGHT` (notice; SHA-256 `ec82b96487e9e778ee610c7ab245162464782cfa1f555c2299333f8dbe5c036a`).
- `core-graphics@0.25.0` `core-graphics@0.25.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `core-graphics-types@0.2.0` `core-graphics-types@0.2.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `cpufeatures@0.2.17` `cpufeatures@0.2.17/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `crc@3.4.0` `crc@3.4.0/LICENSE-APACHE` (license text; SHA-256 `470355a7eed93fcc4281ec2e0f82ca3b94e7af1e4d83629f91de8cfac34d750e`).
- `crc32fast@1.5.5` `crc32fast@1.5.5/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `crossbeam-channel@0.5.17` `crossbeam-channel@0.5.17/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `crossbeam-utils@0.8.23` `crossbeam-utils@0.8.23/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `crypto-common@0.1.7` `crypto-common@0.1.7/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `ctor@1.0.14` `ctor@1.0.14/LICENSE-APACHE` (license text; SHA-256 `a8ad31b1c3f40dca5a84119351b8fa8ddc868edd77fad8a8ebf6d8f2d16fa4ae`).
- `dbus@0.9.12` `dbus@0.9.12/LICENSE-APACHE` (license text; SHA-256 `453745410e3be8cf25d56872ea2aec975a78e6c9f217443d0bf908a5bce7c8ff`).
- `deranged@0.5.9` `deranged@0.5.9/LICENSE-Apache` (license text; SHA-256 `edd65bdd88957a205c47d53fa499eed8865a70320f0f03f6391668cb304ea376`).
- `digest@0.10.7` `digest@0.10.7/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `directories@6.0.0` `directories@6.0.0/LICENSE-APACHE` (license text; SHA-256 `d3174ad63e721d4c9dccb8ad4320848992d314369bc46319720b5802c9153fe9`).
- `dirs@7.0.0` `dirs@7.0.0/LICENSE-APACHE` (license text; SHA-256 `d3174ad63e721d4c9dccb8ad4320848992d314369bc46319720b5802c9153fe9`).
- `dirs-sys@0.5.0` `dirs-sys@0.5.0/LICENSE-APACHE` (license text; SHA-256 `d3174ad63e721d4c9dccb8ad4320848992d314369bc46319720b5802c9153fe9`).
- `dispatch2@0.3.1` `third-party/license-texts/dispatch2-0.3.1/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `displaydoc@0.2.7` `displaydoc@0.2.7/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `downcast-rs@1.2.1` `downcast-rs@1.2.1/LICENSE-APACHE` (license text; SHA-256 `8173d5c29b4f956d532781d2b86e4e30f83e6b7878dce18c919451d6ba707c90`).
- `dtoa@1.0.11` `dtoa@1.0.11/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `dyn-clone@1.0.20` `dyn-clone@1.0.20/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `embed_plist@1.2.2` `embed_plist@1.2.2/LICENSE-APACHE` (license text; SHA-256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).
- `enumflags2@0.7.12` `enumflags2@0.7.12/LICENSE-APACHE` (license text; SHA-256 `41ba8a366e4129d0b7551f8b98211f59635b2e30c0a3c7845e828e2a7ce42064`).
- `enumflags2_derive@0.7.12` `enumflags2_derive@0.7.12/LICENSE-APACHE` (license text; SHA-256 `68d7c3ae5b131ec533fb33a9fd63753eb82fac77d7597bb5058a5af07adf9f06`).
- `equivalent@1.0.3` `equivalent@1.0.3/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `erased-serde@0.4.11` `erased-serde@0.4.11/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `errno@0.3.15` `errno@0.3.15/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `event-listener@5.4.2` `event-listener@5.4.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `event-listener-strategy@0.5.4` `event-listener-strategy@0.5.4/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `fastrand@2.5.0` `fastrand@2.5.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `fdeflate@0.3.7` `fdeflate@0.3.7/LICENSE-APACHE` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `field-offset@0.3.6` `field-offset@0.3.6/LICENSE-APACHE` (license text; SHA-256 `a6cba85bc92e0cff7a450b1d873c0eaa2e9fc96bf472df0247a26bec77bf3ff9`).
- `filetime@0.2.29` `filetime@0.2.29/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `find-msvc-tools@0.1.15` `find-msvc-tools@0.1.15/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `fixedbitset@0.5.9` `fixedbitset@0.5.9/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `flate2@1.1.11` `flate2@1.1.11/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `fnv@1.0.7` `fnv@1.0.7/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `foreign-types@0.5.0` `foreign-types@0.5.0/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `foreign-types-macros@0.2.4` `foreign-types-macros@0.2.4/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `foreign-types-shared@0.3.1` `foreign-types-shared@0.3.1/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `form_urlencoded@1.2.2` `form_urlencoded@1.2.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `fraction@0.15.4` `fraction@0.15.4/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `futures-channel@0.3.34` `futures-channel@0.3.34/LICENSE-APACHE` (license text; SHA-256 `275c491d6d1160553c32fd6127061d7f9606c3ea25abfad6ca3f6ed088785427`).
- `futures-core@0.3.34` `futures-core@0.3.34/LICENSE-APACHE` (license text; SHA-256 `275c491d6d1160553c32fd6127061d7f9606c3ea25abfad6ca3f6ed088785427`).
- `futures-executor@0.3.34` `futures-executor@0.3.34/LICENSE-APACHE` (license text; SHA-256 `275c491d6d1160553c32fd6127061d7f9606c3ea25abfad6ca3f6ed088785427`).
- `futures-io@0.3.34` `futures-io@0.3.34/LICENSE-APACHE` (license text; SHA-256 `275c491d6d1160553c32fd6127061d7f9606c3ea25abfad6ca3f6ed088785427`).
- `futures-lite@2.6.1` `futures-lite@2.6.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `futures-macro@0.3.34` `futures-macro@0.3.34/LICENSE-APACHE` (license text; SHA-256 `275c491d6d1160553c32fd6127061d7f9606c3ea25abfad6ca3f6ed088785427`).
- `futures-sink@0.3.34` `futures-sink@0.3.34/LICENSE-APACHE` (license text; SHA-256 `275c491d6d1160553c32fd6127061d7f9606c3ea25abfad6ca3f6ed088785427`).
- `futures-task@0.3.34` `futures-task@0.3.34/LICENSE-APACHE` (license text; SHA-256 `275c491d6d1160553c32fd6127061d7f9606c3ea25abfad6ca3f6ed088785427`).
- `futures-util@0.3.34` `futures-util@0.3.34/LICENSE-APACHE` (license text; SHA-256 `275c491d6d1160553c32fd6127061d7f9606c3ea25abfad6ca3f6ed088785427`).
- `gdk@0.18.2` `gdk@0.18.2/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `gdk-pixbuf@0.18.5` `gdk-pixbuf@0.18.5/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `gdkx11@0.18.2` `gdkx11@0.18.2/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `getrandom@0.2.17` `getrandom@0.2.17/LICENSE-APACHE` (license text; SHA-256 `aaff376532ea30a0cd5330b9502ad4a4c8bf769c539c87ffe78819d188a18ebf`).
- `getrandom@0.3.4` `getrandom@0.3.4/LICENSE-APACHE` (license text; SHA-256 `aaff376532ea30a0cd5330b9502ad4a4c8bf769c539c87ffe78819d188a18ebf`).
- `getrandom@0.4.3` `getrandom@0.4.3/LICENSE-APACHE` (license text; SHA-256 `aaff376532ea30a0cd5330b9502ad4a4c8bf769c539c87ffe78819d188a18ebf`).
- `gio@0.18.4` `gio@0.18.4/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `glib@0.18.5` `glib@0.18.5/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `glib-macros@0.18.5` `glib-macros@0.18.5/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `glob@0.3.4` `glob@0.3.4/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `global-hotkey@0.8.0` `global-hotkey@0.8.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `global-hotkey@0.8.0` `global-hotkey@0.8.0/LICENSE.spdx` (notice; SHA-256 `3b3a242fa1704327a51563c614c1043deab014f58ebbbdd173262c990104497e`).
- `gtk@0.18.2` `gtk@0.18.2/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `gtk3-macros@0.18.2` `gtk3-macros@0.18.2/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `half@2.7.1` `half@2.7.1/LICENSE-APACHE` (license text; SHA-256 `a6cba85bc92e0cff7a450b1d873c0eaa2e9fc96bf472df0247a26bec77bf3ff9`).
- `hashbrown@0.12.3` `hashbrown@0.12.3/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `hashbrown@0.15.5` `hashbrown@0.15.5/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `hashbrown@0.17.1` `hashbrown@0.17.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `heck@0.4.1` `heck@0.4.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `heck@0.5.0` `heck@0.5.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `hex@0.4.3` `hex@0.4.3/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `hkdf@0.12.4` `hkdf@0.12.4/LICENSE-APACHE` (license text; SHA-256 `59013a5c8d3a19c26a457579105915a5d51bb0c09d579f8cdedf12e4203c3018`).
- `hmac@0.12.1` `hmac@0.12.1/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `html5ever@0.39.0` `html5ever@0.39.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `http@1.5.0` `http@1.5.0/LICENSE-APACHE` (license text; SHA-256 `8bb1b50b0e5c9399ae33bd35fab2769010fa6c14e8860c729a52295d84896b7a`).
- `httparse@1.10.1` `httparse@1.10.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `hyper-rustls@0.27.10` `hyper-rustls@0.27.10/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `hyper-rustls@0.27.10` `hyper-rustls@0.27.10/LICENSE-ISC` (license text; SHA-256 `7cfafc877eccc46c0e346ccbaa5c51bb6b894d2b818e617d970211e232785ad4`).
- `idna@1.1.0` `idna@1.1.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `idna_adapter@1.2.2` `idna_adapter@1.2.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `image@0.25.10` `image@0.25.10/LICENSE-APACHE` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `indexmap@1.9.3` `indexmap@1.9.3/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `indexmap@2.14.2` `indexmap@2.14.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `inout@0.1.4` `inout@0.1.4/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `ipnet@2.12.2` `ipnet@2.12.2/LICENSE-APACHE` (license text; SHA-256 `87d9feb9238c6bd8e0024fc4733b06cff036f89f36d93b7df1c8a0549bbb7a5b`).
- `itoa@1.0.19` `itoa@1.0.19/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `json-patch@4.2.0` `json-patch@4.2.0/LICENSE-APACHE` (license text; SHA-256 `b40930bbcf80744c86c46a12bc9da056641d722716c378f5659b9e555ef833e1`).
- `jsonptr@0.7.1` `jsonptr@0.7.1/LICENSE-APACHE` (license text; SHA-256 `2cf31f2ae7f3feba667d50fc1d4d21ecf217703c04c3068b83c9a73b6dde723a`).
- `keyboard-types@0.7.0` `keyboard-types@0.7.0/LICENSE-APACHE` (license text; SHA-256 `769f80b5bcb42ed0af4e4d2fd74e1ac9bf843cb80c5a29219d1ef3544428a6bb`).
- `keyboard-types@0.8.3` `keyboard-types@0.8.3/LICENSE-APACHE` (license text; SHA-256 `769f80b5bcb42ed0af4e4d2fd74e1ac9bf843cb80c5a29219d1ef3544428a6bb`).
- `keyring@3.6.3` `keyring@3.6.3/LICENSE-APACHE` (license text; SHA-256 `78ebc2ea2a1cec9cbec2e1a6fc15e73474fe22bdad7575518e7f4628651d49c4`).
- `lazy_static@1.5.1` `lazy_static@1.5.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `libappindicator@0.9.0` `libappindicator@0.9.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `libc@0.2.191` `libc@0.2.191/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `libdbus-sys@0.2.7` `libdbus-sys@0.2.7/LICENSE-APACHE` (license text; SHA-256 `453745410e3be8cf25d56872ea2aec975a78e6c9f217443d0bf908a5bce7c8ff`).
- `linux-raw-sys@0.12.1` `linux-raw-sys@0.12.1/COPYRIGHT` (notice; SHA-256 `3290ae0fbc9ddb77d2239121d710f0bb9d31b3b4744e6d97fe01e652b4c1870b`).
- `linux-raw-sys@0.12.1` `linux-raw-sys@0.12.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `linux-raw-sys@0.12.1` `linux-raw-sys@0.12.1/LICENSE-Apache-2.0_WITH_LLVM-exception` (license text; SHA-256 `268872b9816f90fd8e85db5a28d33f8150ebb8dd016653fb39ef1f94f2686bc5`).
- `lock_api@0.4.14` `lock_api@0.4.14/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `log@0.4.35` `log@0.4.35/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `markup5ever@0.39.0` `markup5ever@0.39.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `memchr@2.8.3` `memchr@2.8.3/COPYING` (notice; SHA-256 `01c266bced4a434da0051174d6bee16a4c82cf634e2679b6155d40d75012390f`).
- `mime@0.3.18` `mime@0.3.18/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `miniz_oxide@0.8.9` `miniz_oxide@0.8.9/LICENSE-APACHE.md` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `miniz_oxide@0.8.9` `miniz_oxide@0.8.9/LICENSE-ZLIB.md` (license text; SHA-256 `0a54e647fe54104658b5e563c04c6f9edf251710e47bce692e0bd990a4ddaa39`).
- `miniz_oxide@0.9.1` `miniz_oxide@0.9.1/LICENSE-APACHE.md` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `miniz_oxide@0.9.1` `miniz_oxide@0.9.1/LICENSE-ZLIB.md` (license text; SHA-256 `0a54e647fe54104658b5e563c04c6f9edf251710e47bce692e0bd990a4ddaa39`).
- `moxcms@0.8.1` `moxcms@0.8.1/LICENSE.md` (license text; SHA-256 `2aa92cada6431e75615e3fe6cb1a9082c98f777d48ae1c087c0da0e37f7b8bff`).
- `muda@0.20.0` `muda@0.20.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `muda@0.20.0` `muda@0.20.0/LICENSE.spdx` (notice; SHA-256 `b99f9d43803f2ac979c50949df630bd8eeaf2fcd1b9fdb9093710d70f5dfb63d`).
- `notify-types@2.3.0` `notify-types@2.3.0/LICENSE-APACHE` (license text; SHA-256 `b5b413d7421d728c14f33d78198d6e255a0018beef717d78c4c3f6188be3dfb4`).
- `num@0.4.3` `num@0.4.3/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `num-bigint@0.4.8` `num-bigint@0.4.8/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `num-complex@0.4.6` `num-complex@0.4.6/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `num-conv@0.2.2` `num-conv@0.2.2/LICENSE-Apache` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `num-integer@0.1.47` `num-integer@0.1.47/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `num-iter@0.1.46` `num-iter@0.1.46/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `num-rational@0.4.2` `num-rational@0.4.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `num-traits@0.2.19` `num-traits@0.2.19/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `objc2@0.6.5` `third-party/license-texts/objc2-0.6.5/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `objc2-app-kit@0.3.2` `third-party/license-texts/objc2-app-kit-0.3.2/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `objc2-core-foundation@0.3.2` `third-party/license-texts/objc2-core-foundation-0.3.2/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `objc2-core-graphics@0.3.2` `third-party/license-texts/objc2-core-graphics-0.3.2/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `objc2-core-services@0.3.2` `third-party/license-texts/objc2-core-services-0.3.2/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `objc2-encode@4.1.0` `third-party/license-texts/objc2-encode-4.1.0/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `objc2-exception-helper@0.1.1` `third-party/license-texts/objc2-exception-helper-0.1.1/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `objc2-foundation@0.3.2` `objc2-foundation@0.3.2/src/copying.rs` (notice; SHA-256 `259b22f571b54fed25f68ec5abd8eda223e7b0547a806f13ba797a0eabecd64f`).
- `objc2-foundation@0.3.2` `objc2-foundation@0.3.2/src/tests/copying.rs` (notice; SHA-256 `23d608079a47693a26dfc8a5cb01894ad0136a50714d46221d9380aa4a4b4423`).
- `objc2-foundation@0.3.2` `third-party/license-texts/objc2-foundation-0.3.2/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `objc2-quartz-core@0.3.2` `third-party/license-texts/objc2-quartz-core-0.3.2/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `objc2-web-kit@0.3.2` `third-party/license-texts/objc2-web-kit-0.3.2/UPSTREAM_LICENSE.md` (notice; SHA-256 `5901c82bb42bfbe9015097ebed64a7756eb617c7d403b1ae598990fca9a42e27`); Preserved verbatim as a NOTICE, including the Apple SDK/Xcode caveat..
- `once_cell@1.21.4` `once_cell@1.21.4/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `ordered-stream@0.2.0` `ordered-stream@0.2.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `pango@0.18.3` `pango@0.18.3/COPYRIGHT` (notice; SHA-256 `dae402989de65164815b7e2b6bc2b9576285434c3785934c8b6ece0fa055960d`).
- `parking@2.2.1` `parking@2.2.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `parking@2.2.1` `parking@2.2.1/LICENSE-THIRD-PARTY` (notice; SHA-256 `9a6d7a3c1b8edcc706c4b6c37f8dae9a77f02c7f9051df603f975db136b55419`).
- `parking_lot@0.12.5` `parking_lot@0.12.5/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `parking_lot_core@0.9.12` `parking_lot_core@0.9.12/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `percent-encoding@2.3.2` `percent-encoding@2.3.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `petgraph@0.8.3` `petgraph@0.8.3/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `petgraph@0.8.3` `petgraph@0.8.3/assets/images/LICENSE.md` (notice; SHA-256 `608fdcbb7daea2541c57228f21fc97deec310b19dafde70b2f4b84a96bf779f8`).
- `pin-project-lite@0.2.17` `pin-project-lite@0.2.17/LICENSE-APACHE` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `piper@0.2.5` `piper@0.2.5/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `pkg-config@0.3.34` `pkg-config@0.3.34/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `png@0.18.1` `png@0.18.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `polling@3.11.0` `polling@3.11.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `powerfmt@0.2.1` `powerfmt@0.2.1/LICENSE-Apache` (license text; SHA-256 `155420c6403d4e0fca34105e3c03fdd6939b64c393c7ec6f95f5b72c5474eab0`).
- `ppv-lite86@0.2.21` `ppv-lite86@0.2.21/LICENSE-APACHE` (license text; SHA-256 `0218327e7a480793ffdd4eb792379a9709e5c135c7ba267f709d6f6d4d70af0a`).
- `proc-macro-crate@1.3.1` `proc-macro-crate@1.3.1/LICENSE-APACHE` (license text; SHA-256 `8ada45cd9f843acf64e4722ae262c622a2b3b3007c7310ef36ac1061a30f6adb`).
- `proc-macro-crate@2.0.2` `proc-macro-crate@2.0.2/LICENSE-APACHE` (license text; SHA-256 `8ada45cd9f843acf64e4722ae262c622a2b3b3007c7310ef36ac1061a30f6adb`).
- `proc-macro-crate@3.5.0` `proc-macro-crate@3.5.0/LICENSE-APACHE` (license text; SHA-256 `8ada45cd9f843acf64e4722ae262c622a2b3b3007c7310ef36ac1061a30f6adb`).
- `proc-macro-error@1.0.4` `proc-macro-error@1.0.4/LICENSE-APACHE` (license text; SHA-256 `4665f973ccb9393807a7fb1264add6b3513d19abc6b357e4cb52c6fe59cc6a3b`).
- `proc-macro-error-attr@1.0.4` `proc-macro-error-attr@1.0.4/LICENSE-APACHE` (license text; SHA-256 `6fd0f3522047150ca7c1939f02bc4a15662a4741a89bc03ae784eefa18caa299`).
- `proc-macro2@1.0.107` `proc-macro2@1.0.107/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `pxfm@0.1.30` `pxfm@0.1.30/LICENSE.md` (license text; SHA-256 `2aa92cada6431e75615e3fe6cb1a9082c98f777d48ae1c087c0da0e37f7b8bff`).
- `quick-error@2.0.1` `quick-error@2.0.1/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `quote@1.0.47` `quote@1.0.47/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `rand@0.8.8` `rand@0.8.8/COPYRIGHT` (notice; SHA-256 `90eb64f0279b0d9432accfa6023ff803bc4965212383697eee27a0f426d5f8d5`).
- `rand@0.8.8` `rand@0.8.8/LICENSE-APACHE` (license text; SHA-256 `35242e7a83f69875e6edeff02291e688c97caafe2f8902e4e19b49d3e78b4cab`).
- `rand_chacha@0.3.1` `rand_chacha@0.3.1/COPYRIGHT` (notice; SHA-256 `90eb64f0279b0d9432accfa6023ff803bc4965212383697eee27a0f426d5f8d5`).
- `rand_chacha@0.3.1` `rand_chacha@0.3.1/LICENSE-APACHE` (license text; SHA-256 `aaff376532ea30a0cd5330b9502ad4a4c8bf769c539c87ffe78819d188a18ebf`).
- `rand_core@0.6.4` `rand_core@0.6.4/COPYRIGHT` (notice; SHA-256 `90eb64f0279b0d9432accfa6023ff803bc4965212383697eee27a0f426d5f8d5`).
- `rand_core@0.6.4` `rand_core@0.6.4/LICENSE-APACHE` (license text; SHA-256 `6df43f6f4b5d4587f3d8d71e45532c688fd168afa5fe89d571cb32fa09c4ef51`).
- `raw-window-handle@0.6.2` `raw-window-handle@0.6.2/LICENSE-APACHE.md` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `raw-window-handle@0.6.2` `raw-window-handle@0.6.2/LICENSE-ZLIB.md` (license text; SHA-256 `ae895c8576e682f310fb70d93d9f99b1dd82872094c54a8f6cdfe2ca5c2d6ecb`).
- `ref-cast@1.0.27` `ref-cast@1.0.27/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `ref-cast-impl@1.0.27` `ref-cast-impl@1.0.27/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `regex@1.13.1` `regex@1.13.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `regex-automata@0.4.18` `regex-automata@0.4.18/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `regex-syntax@0.8.11` `regex-syntax@0.8.11/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `regex-syntax@0.8.11` `regex-syntax@0.8.11/src/unicode_tables/LICENSE-UNICODE` (notice; SHA-256 `74db5baf44a41b1000312c673544b3374e4198af5605c7f9080a402cec42cfa3`).
- `reqwest@0.12.28` `reqwest@0.12.28/LICENSE-APACHE` (license text; SHA-256 `751963a8b88c0e3a9f27e98933079fbcf09b9998b2c13ac49c2e4d58444520d6`).
- `ring@0.17.14` `ring@0.17.14/LICENSE` (notice; SHA-256 `b3d734001a94efff3579978d953391aa7115f877657d25eb54037a43875d078a`).
- `ring@0.17.14` `ring@0.17.14/src/polyfill/once_cell/LICENSE-MIT` (license text; SHA-256 `6ee2ed6c77710de911761acd5fc1ad1da00f476beb1a7ef27e78c2d1858deafc`).
- `ring@0.17.14` `ring@0.17.14/third_party/fiat/LICENSE` (notice; SHA-256 `9eacbcb81be660840c714a560a9d65ba07913db98dd4baf969f78dd499fdd60f`).
- `rustc-hash@2.1.3` `rustc-hash@2.1.3/LICENSE-APACHE` (license text; SHA-256 `95bd3988beee069fa2848f648dab43cc6e0b2add2ad6bcb17360caf749802bcc`).
- `rustc_version@0.4.1` `rustc_version@0.4.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `rustix@1.1.6` `rustix@1.1.6/COPYRIGHT` (notice; SHA-256 `377c2e7c53250cc5905c0b0532d35973392af16ffb9596a41d99d202cf3617c9`).
- `rustix@1.1.6` `rustix@1.1.6/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `rustix@1.1.6` `rustix@1.1.6/LICENSE-Apache-2.0_WITH_LLVM-exception` (license text; SHA-256 `268872b9816f90fd8e85db5a28d33f8150ebb8dd016653fb39ef1f94f2686bc5`).
- `rustls@0.23.46` `rustls@0.23.46/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `rustls@0.23.46` `rustls@0.23.46/LICENSE-ISC` (license text; SHA-256 `7cfafc877eccc46c0e346ccbaa5c51bb6b894d2b818e617d970211e232785ad4`).
- `rustls-pki-types@1.16.0` `rustls-pki-types@1.16.0/LICENSE-APACHE` (license text; SHA-256 `45fd05c4865e7c350b98ad7ac50e1b15462d49af4a91e9b0c9dd933dc9a69742`).
- `rustversion@1.0.23` `rustversion@1.0.23/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `ryu@1.0.23` `ryu@1.0.23/LICENSE-BOOST` (license text; SHA-256 `c9bff75738922193e67fa726fa225535870d2aa1059f91452c411736284ad566`).
- `same-file@1.0.6` `same-file@1.0.6/COPYING` (notice; SHA-256 `01c266bced4a434da0051174d6bee16a4c82cf634e2679b6155d40d75012390f`).
- `scopeguard@1.2.0` `scopeguard@1.2.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `secret-service@4.0.0` `secret-service@4.0.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `semver@1.0.28` `semver@1.0.28/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `serde@1.0.229` `serde@1.0.229/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `serde-untagged@0.1.9` `serde-untagged@0.1.9/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `serde_core@1.0.229` `serde_core@1.0.229/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `serde_derive@1.0.229` `serde_derive@1.0.229/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `serde_derive_internals@0.29.1` `serde_derive_internals@0.29.1/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `serde_json@1.0.154` `serde_json@1.0.154/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `serde_repr@0.1.21` `serde_repr@0.1.21/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `serde_spanned@0.6.9` `serde_spanned@0.6.9/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `serde_spanned@1.1.3` `serde_spanned@1.1.3/LICENSE-APACHE` (license text; SHA-256 `249fdb6694467dd615b259f7ddde07be3f8ae5a72cecebb0e24f6629af4215fe`).
- `serde_urlencoded@0.7.1` `serde_urlencoded@0.7.1/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `serde_with@3.24.0` `serde_with@3.24.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `serde_with_macros@3.24.0` `serde_with_macros@3.24.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `serialize-to-javascript@0.1.2` `serialize-to-javascript@0.1.2/LICENSE-APACHE` (license text; SHA-256 `c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4`).
- `serialize-to-javascript-impl@0.1.2` `serialize-to-javascript-impl@0.1.2/LICENSE-APACHE` (license text; SHA-256 `c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4`).
- `servo_arc@0.4.3` `servo_arc@0.4.3/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `sha1@0.10.7` `sha1@0.10.7/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `sha2@0.10.9` `sha2@0.10.9/LICENSE-APACHE` (license text; SHA-256 `a9040321c3712d8fd0b09cf52b17445de04a23a10165049ae187cd39e5c86be5`).
- `shlex@2.0.1` `shlex@2.0.1/LICENSE-APACHE` (notice; SHA-256 `553fffcd9b1cb158bc3e9edc35da85ca5c3b3d7d2e61c883ebcfa8a65814b583`).
- `signal-hook-registry@1.4.8` `signal-hook-registry@1.4.8/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `siphasher@1.0.4` `siphasher@1.0.4/COPYING` (notice; SHA-256 `c962ee4d1d05ddc138b202b2540219ebc57893fcf97b364852094a9a94ce1365`).
- `siphasher@1.0.4` `siphasher@1.0.4/LICENSE-APACHE` (license text; SHA-256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).
- `smallvec@1.16.5` `smallvec@1.16.5/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `socket2@0.6.5` `socket2@0.6.5/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `softbuffer@0.4.8` `softbuffer@0.4.8/LICENSE-APACHE` (license text; SHA-256 `a4445427440043e8ce6fadf7a43fb906cb0d7f6c573a560a9f142cf9f9bf8ff4`).
- `stable_deref_trait@1.2.1` `stable_deref_trait@1.2.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `static_assertions@1.1.0` `static_assertions@1.1.0/LICENSE-APACHE` (license text; SHA-256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).
- `static_vcruntime@3.0.0` `static_vcruntime@3.0.0/LICENSE` (notice; SHA-256 `b9290e9d37bc53272ff4f3fb392c1762169fc0a8b797d164803049c81eacb48c`).
- `static_vcruntime@3.0.0` `static_vcruntime@3.0.0/LICENSE-APACHE` (license text; SHA-256 `e3ba223bb1423f0aad8c3dfce0fe3148db48926d41e6fbc3afbbf5ff9e1c89cb`).
- `static_vcruntime@3.0.0` `static_vcruntime@3.0.0/LICENSE-ZLIB` (license text; SHA-256 `c1ff532091cac33bdecedf7268ff4f9bc3ddcffba494249fb723033e9e782f1a`).
- `string_cache@0.9.0` `string_cache@0.9.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `string_cache_codegen@0.6.1` `string_cache_codegen@0.6.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `swift-rs@1.0.9` `swift-rs@1.0.9/LICENSE-APACHE` (license text; SHA-256 `908a661cf763bddfa04f2739bea561be62a246f03ad53fcb2272d3ddbd1d943f`).
- `syn@1.0.109` `syn@1.0.109/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `syn@2.0.119` `syn@2.0.119/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `syn@3.0.10` `syn@3.0.10/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `system-deps@6.2.2` `system-deps@6.2.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `tao@0.37.2` `tao@0.37.2/LICENSE.spdx` (notice; SHA-256 `02b6448337b757772d90e5f31caaa0f9e6afaad2221c01771d0a3deaa4d4196c`).
- `tar@0.4.46` `tar@0.4.46/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `tauri@2.12.5` `tauri@2.12.5/LICENSE-APACHE-2.0` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `tauri-build@2.7.4` `tauri-build@2.7.4/LICENSE-APACHE-2.0` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `tauri-codegen@2.7.4` `tauri-codegen@2.7.4/LICENSE-APACHE-2.0` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `tauri-macros@2.7.4` `tauri-macros@2.7.4/LICENSE-APACHE-2.0` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `tauri-plugin-clipboard-manager@2.5.0` `tauri-plugin-clipboard-manager@2.5.0/LICENSE.spdx` (notice; SHA-256 `eb8a6c84630461b352badcab1dbe5d0168c56d377358b2b8c86b51003272d5ef`).
- `tauri-plugin-clipboard-manager@2.5.0` `tauri-plugin-clipboard-manager@2.5.0/LICENSE_APACHE-2.0` (license text; SHA-256 `0cec06e0e55fbc3dc5cee4fca9b607f66cb8f4e4dbcf3b3c013594dd156732e9`).
- `tauri-plugin-dialog@2.8.4` `tauri-plugin-dialog@2.8.4/LICENSE.spdx` (notice; SHA-256 `eb8a6c84630461b352badcab1dbe5d0168c56d377358b2b8c86b51003272d5ef`).
- `tauri-plugin-dialog@2.8.4` `tauri-plugin-dialog@2.8.4/LICENSE_APACHE-2.0` (license text; SHA-256 `0cec06e0e55fbc3dc5cee4fca9b607f66cb8f4e4dbcf3b3c013594dd156732e9`).
- `tauri-plugin-fs@2.7.1` `tauri-plugin-fs@2.7.1/LICENSE.spdx` (notice; SHA-256 `eb8a6c84630461b352badcab1dbe5d0168c56d377358b2b8c86b51003272d5ef`).
- `tauri-plugin-fs@2.7.1` `tauri-plugin-fs@2.7.1/LICENSE_APACHE-2.0` (license text; SHA-256 `0cec06e0e55fbc3dc5cee4fca9b607f66cb8f4e4dbcf3b3c013594dd156732e9`).
- `tauri-plugin-global-shortcut@2.4.1` `tauri-plugin-global-shortcut@2.4.1/LICENSE.spdx` (notice; SHA-256 `eb8a6c84630461b352badcab1dbe5d0168c56d377358b2b8c86b51003272d5ef`).
- `tauri-plugin-global-shortcut@2.4.1` `tauri-plugin-global-shortcut@2.4.1/LICENSE_APACHE-2.0` (license text; SHA-256 `0cec06e0e55fbc3dc5cee4fca9b607f66cb8f4e4dbcf3b3c013594dd156732e9`).
- `tauri-plugin-opener@2.7.3` `tauri-plugin-opener@2.7.3/LICENSE.spdx` (notice; SHA-256 `eb8a6c84630461b352badcab1dbe5d0168c56d377358b2b8c86b51003272d5ef`).
- `tauri-plugin-opener@2.7.3` `tauri-plugin-opener@2.7.3/LICENSE_APACHE-2.0` (license text; SHA-256 `0cec06e0e55fbc3dc5cee4fca9b607f66cb8f4e4dbcf3b3c013594dd156732e9`).
- `tauri-plugin-single-instance@2.5.5` `tauri-plugin-single-instance@2.5.5/LICENSE.spdx` (notice; SHA-256 `eb8a6c84630461b352badcab1dbe5d0168c56d377358b2b8c86b51003272d5ef`).
- `tauri-plugin-single-instance@2.5.5` `tauri-plugin-single-instance@2.5.5/LICENSE_APACHE-2.0` (license text; SHA-256 `0cec06e0e55fbc3dc5cee4fca9b607f66cb8f4e4dbcf3b3c013594dd156732e9`).
- `tauri-runtime@2.12.2` `tauri-runtime@2.12.2/LICENSE-APACHE-2.0` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `tauri-runtime-wry@2.12.3` `tauri-runtime-wry@2.12.3/LICENSE-APACHE-2.0` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `tauri-utils@2.10.2` `tauri-utils@2.10.2/LICENSE-APACHE-2.0` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `tempfile@3.27.0` `tempfile@3.27.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `tendril@0.5.1` `tendril@0.5.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `thiserror@1.0.69` `thiserror@1.0.69/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `thiserror@2.0.21` `thiserror@2.0.21/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `thiserror-impl@1.0.69` `thiserror-impl@1.0.69/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `thiserror-impl@2.0.21` `thiserror-impl@2.0.21/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `tiff@0.11.3` `tiff@0.11.3/tests/COPYRIGHT` (notice; SHA-256 `c6d1d56bb6e2fe4f0aed4aa8553261dd65247db6db50c87c9e9f3fbe095e2202`).
- `time@0.3.55` `time@0.3.55/LICENSE-Apache` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `time-core@0.1.9` `time-core@0.1.9/LICENSE-Apache` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `time-macros@0.2.32` `time-macros@0.2.32/LICENSE-Apache` (license text; SHA-256 `0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594`).
- `tokio-rustls@0.26.6` `tokio-rustls@0.26.6/LICENSE-APACHE` (license text; SHA-256 `cc117d90b498b32b11a886f279b359da16a73c3b01efbb2f5cc004b20262334e`).
- `toml@0.8.2` `toml@0.8.2/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `toml@1.2.1+spec-1.1.0` `toml@1.2.1+spec-1.1.0/LICENSE-APACHE` (license text; SHA-256 `249fdb6694467dd615b259f7ddde07be3f8ae5a72cecebb0e24f6629af4215fe`).
- `toml_datetime@0.6.3` `toml_datetime@0.6.3/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `toml_datetime@1.1.3+spec-1.1.0` `toml_datetime@1.1.3+spec-1.1.0/LICENSE-APACHE` (license text; SHA-256 `249fdb6694467dd615b259f7ddde07be3f8ae5a72cecebb0e24f6629af4215fe`).
- `toml_edit@0.19.15` `toml_edit@0.19.15/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `toml_edit@0.20.2` `toml_edit@0.20.2/LICENSE-APACHE` (license text; SHA-256 `c6596eb7be8581c18be736c846fb9173b69eccf6ef94c5135893ec56bd92ba08`).
- `toml_edit@0.25.19+spec-1.1.0` `toml_edit@0.25.19+spec-1.1.0/LICENSE-APACHE` (license text; SHA-256 `249fdb6694467dd615b259f7ddde07be3f8ae5a72cecebb0e24f6629af4215fe`).
- `toml_parser@1.1.5+spec-1.1.0` `toml_parser@1.1.5+spec-1.1.0/LICENSE-APACHE` (license text; SHA-256 `249fdb6694467dd615b259f7ddde07be3f8ae5a72cecebb0e24f6629af4215fe`).
- `toml_writer@1.2.0+spec-1.1.0` `toml_writer@1.2.0+spec-1.1.0/LICENSE-APACHE` (license text; SHA-256 `249fdb6694467dd615b259f7ddde07be3f8ae5a72cecebb0e24f6629af4215fe`).
- `tray-icon@0.25.1` `tray-icon@0.25.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `tray-icon@0.25.1` `tray-icon@0.25.1/LICENSE.spdx` (notice; SHA-256 `6c1e5d0ccf5e8951401cb964036ae664d52fc8d1148173b53bc54fc534f5d404`).
- `typed-path@0.13.0` `typed-path@0.13.0/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `typeid@1.0.3` `typeid@1.0.3/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `typenum@1.20.1` `typenum@1.20.1/LICENSE` (notice; SHA-256 `db11fec9946737df39ca3898d9cd8c10ec6f6c3a884a6802b0ad0b81b4e8f23a`).
- `typenum@1.20.1` `typenum@1.20.1/LICENSE-APACHE` (license text; SHA-256 `516b24e051bf5630880ebbd55c40a25ce9552ebaf8970a53e8976eb70e522406`).
- `unicase@2.10.0` `unicase@2.10.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `unicode-ident@1.0.27` `unicode-ident@1.0.27/LICENSE-APACHE` (license text; SHA-256 `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a`).
- `unicode-segmentation@1.13.3` `unicode-segmentation@1.13.3/COPYRIGHT` (notice; SHA-256 `23860c2a7b5d96b21569afedf033469bab9fe14a1b24a35068b8641c578ce24d`).
- `unicode-segmentation@1.13.3` `unicode-segmentation@1.13.3/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `url@2.5.8` `url@2.5.8/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `utf8_iter@1.1.0` `utf8_iter@1.1.0/COPYRIGHT` (notice; SHA-256 `c30152c94a6d75e021adbc52b3a52470366a46edb917e17deae3259251af244c`).
- `utf8_iter@1.1.0` `utf8_iter@1.1.0/LICENSE-APACHE` (license text; SHA-256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).
- `utf_types@1.0.0` `utf_types@1.0.0/COPYRIGHT` (notice; SHA-256 `3210be7332b5bdf48eb24a945258b9f38616a2cceb0dfc06e3c3c7e9740475a0`).
- `utf_types@1.0.0` `utf_types@1.0.0/LICENSE-APACHE` (license text; SHA-256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).
- `uuid@1.29.2` `uuid@1.29.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `version_check@0.9.5` `version_check@0.9.5/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `walkdir@2.5.0` `walkdir@2.5.0/COPYING` (notice; SHA-256 `01c266bced4a434da0051174d6bee16a4c82cf634e2679b6155d40d75012390f`).
- `web-time@1.1.0` `web-time@1.1.0/LICENSE-APACHE` (license text; SHA-256 `54a744942eb3fa63a74d8d359989ed6962c04f3f10ff20cfa08ff8aa7efbe4e6`).
- `web_atoms@0.2.6` `web_atoms@0.2.6/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `weezl@0.1.12` `weezl@0.1.12/LICENSE-APACHE` (license text; SHA-256 `769f80b5bcb42ed0af4e4d2fd74e1ac9bf843cb80c5a29219d1ef3544428a6bb`).
- `winapi@0.3.9` `winapi@0.3.9/LICENSE-APACHE` (license text; SHA-256 `b40930bbcf80744c86c46a12bc9da056641d722716c378f5659b9e555ef833e1`).
- `winapi-util@0.1.11` `winapi-util@0.1.11/COPYING` (notice; SHA-256 `01c266bced4a434da0051174d6bee16a4c82cf634e2679b6155d40d75012390f`).
- `window-vibrancy@0.8.1` `window-vibrancy@0.8.1/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `window-vibrancy@0.8.1` `window-vibrancy@0.8.1/LICENSE.spdx` (notice; SHA-256 `7ccef0d35e8507fa02d7e904a5277d62723f9ec576d52f6a865cab7b4dbf12ad`).
- `windows@0.62.2` `windows@0.62.2/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-collections@0.3.2` `windows-collections@0.3.2/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-core@0.62.2` `windows-core@0.62.2/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-future@0.3.2` `windows-future@0.3.2/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-implement@0.60.2` `windows-implement@0.60.2/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-interface@0.59.3` `windows-interface@0.59.3/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-link@0.2.1` `windows-link@0.2.1/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-numerics@0.3.1` `windows-numerics@0.3.1/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-result@0.4.1` `windows-result@0.4.1/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-strings@0.5.1` `windows-strings@0.5.1/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-sys@0.59.0` `windows-sys@0.59.0/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-sys@0.60.2` `windows-sys@0.60.2/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-sys@0.61.2` `windows-sys@0.61.2/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-targets@0.52.6` `windows-targets@0.52.6/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-targets@0.53.5` `windows-targets@0.53.5/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-threading@0.2.1` `windows-threading@0.2.1/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows-version@0.1.7` `windows-version@0.1.7/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows_x86_64_msvc@0.52.6` `windows_x86_64_msvc@0.52.6/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `windows_x86_64_msvc@0.53.1` `windows_x86_64_msvc@0.53.1/license-apache-2.0` (license text; SHA-256 `c16f8dcf1a368b83be78d826ea23de4079fe1b4469a0ab9ee20563f37ff3d44b`).
- `wl-clipboard-rs@0.9.5` `wl-clipboard-rs@0.9.5/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `wry@0.57.2` `wry@0.57.2/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `wry@0.57.2` `wry@0.57.2/LICENSE.spdx` (notice; SHA-256 `26235c41e314fb0da35b33b2bb5bcbc4dd523839f8eeca52efcd43586f441fb5`).
- `x11rb@0.13.2` `x11rb@0.13.2/LICENSE-APACHE` (license text; SHA-256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).
- `x11rb-protocol@0.13.2` `x11rb-protocol@0.13.2/LICENSE-APACHE` (license text; SHA-256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).
- `xattr@1.7.0` `xattr@1.7.0/LICENSE-APACHE` (license text; SHA-256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`).
- `xkeysym@0.2.1` `xkeysym@0.2.1/LICENSE-APACHE` (license text; SHA-256 `d4aca2f5936fa638b062f0585df37eec086ca8620d437b2d6be2c95f01199d1a`).
- `xkeysym@0.2.1` `xkeysym@0.2.1/LICENSE-ZLIB` (license text; SHA-256 `319736510aa7dbd6f49546f6ee57c787dbe2c7dfa0507dfbd543375f86a4416d`).
- `zerocopy@0.8.63` `zerocopy@0.8.63/LICENSE-APACHE` (license text; SHA-256 `9d185ac6703c4b0453974c0d85e9eee43e6941009296bb1f5eb0b54e2329e9f3`).
- `zerocopy@0.8.63` `zerocopy@0.8.63/LICENSE-BSD` (notice; SHA-256 `83c1763356e822adde0a2cae748d938a73fdc263849ccff6b27776dff213bd32`).
- `zerocopy-derive@0.8.63` `zerocopy-derive@0.8.63/LICENSE-APACHE` (license text; SHA-256 `9d185ac6703c4b0453974c0d85e9eee43e6941009296bb1f5eb0b54e2329e9f3`).
- `zerocopy-derive@0.8.63` `zerocopy-derive@0.8.63/LICENSE-BSD` (notice; SHA-256 `83c1763356e822adde0a2cae748d938a73fdc263849ccff6b27776dff213bd32`).
- `zeroize@1.9.1` `zeroize@1.9.1/LICENSE-APACHE` (license text; SHA-256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).
- `zeroize_derive@1.5.0` `zeroize_derive@1.5.0/LICENSE-APACHE` (license text; SHA-256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).
- `zune-core@0.5.3` `zune-core@0.5.3/LICENSE-APACHE` (license text; SHA-256 `c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4`).
- `zune-core@0.5.3` `zune-core@0.5.3/LICENSE-ZLIB` (license text; SHA-256 `d201d14804d3bcd3b944147173175e4abfbd838b7c8069b6bd3452496bf13e6c`).
- `zune-jpeg@0.5.15` `zune-jpeg@0.5.15/LICENSE-APACHE` (license text; SHA-256 `c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4`).
- `zune-jpeg@0.5.15` `zune-jpeg@0.5.15/LICENSE-ZLIB` (license text; SHA-256 `d201d14804d3bcd3b944147173175e4abfbd838b7c8069b6bd3452496bf13e6c`).

No unresolved Rust package or license-text records remain.

