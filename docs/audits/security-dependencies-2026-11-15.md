# Dependency advisory review — 2026-11-15

## Archive dependency decision

The migration is complete: the lockfile uses `zip 9.6.0` and replaces `sevenz-rust` with `sevenz-rust2 0.25.0`. On 2026-11-15, `cargo deny --locked check advisories` and `cargo deny --locked check licenses` passed. The focused managed-tool and archive-security suites passed (40 tests; four manual or live-archive tests remain ignored).

The reviewed targets are [zip 9.6.0](https://crates.io/crates/zip/9.6.0) and [sevenz-rust2 0.25.0](https://crates.io/crates/sevenz-rust2/0.25.0). The `zip 9.6.0` release is dated 2026-11-15. The official registry record for `sevenz-rust2 0.25.0` marks it non-yanked and requires Rust 1.93; the local Rust toolchain is 1.96.0. Production disables default features and enables ZIP `deflate`; the 7z writer's `compress` feature is dev-only for security fixtures.

### ZIP advisory patch review

RustSec lists these five advisories for `zip`. The fixed-version column records the RustSec patch statement, with the stricter upstream patch noted where its advisory disagrees.

| Advisory | Issue | RustSec patched release | Effective minimum |
| --- | --- | --- | --- |
| [RUSTSEC-2025-0168](https://rustsec.org/advisories/RUSTSEC-2025-0168.html) | Symlink path canonicalization can permit writes outside the extraction root. | `2.3.0` | `2.3.0` |
| [RUSTSEC-2026-0367](https://rustsec.org/advisories/RUSTSEC-2026-0367.html) | ZIP extraction can preserve SUID/SGID bits. | `9.0.0` | `9.0.0` |
| [RUSTSEC-2026-0368](https://rustsec.org/advisories/RUSTSEC-2026-0368.html) | ZIP extraction can create symlinks whose targets escape the destination. | `9.0.0` | `9.0.0` |
| [RUSTSEC-2026-0369](https://rustsec.org/advisories/RUSTSEC-2026-0369.html) | Permission changes can follow an extracted symlink outside the destination. | RustSec says `9.0.0`; [upstream GHSA-4684-h2q9-2vcp](https://github.com/zip-rs/zip2/security/advisories/GHSA-4684-h2q9-2vcp) says affected through `9.0.0`, fixed in `9.0.1`. | `9.0.1` |
| [RUSTSEC-2026-0429](https://rustsec.org/advisories/RUSTSEC-2026-0429.html) | An attacker-controlled symlink size can cause an allocation abort. | `9.0.0` | `9.0.0` |

Recent upstream advisories require a later floor than the current RustSec entries. [GHSA-m665-v7pm-7grj](https://github.com/zip-rs/zip2/security/advisories/GHSA-m665-v7pm-7grj) affects through `9.0.4` and is fixed in `9.0.5`; [GHSA-w577-2rfp-8gph](https://github.com/zip-rs/zip2/security/advisories/GHSA-w577-2rfp-8gph) identifies a `9.0.5` regression fixed in `9.1.0`. [GHSA-h5xf-qc2v-94w6](https://github.com/zip-rs/zip2/security/advisories/GHSA-h5xf-qc2v-94w6) affects through `9.1.3` and is fixed in `9.2.0`; [GHSA-mpc8-q7jp-wjrf](https://github.com/zip-rs/zip2/security/advisories/GHSA-mpc8-q7jp-wjrf) is also fixed in `9.2.0`. The reviewed upstream patch floor is therefore `9.2.0`, and `9.6.0` is the selected release. No ZIP advisory is ignored.

### 7z advisory review

| Advisory | Finding | Patched release | Resolution |
| --- | --- | --- | --- |
| [RUSTSEC-2026-0245](https://rustsec.org/advisories/RUSTSEC-2026-0245.html) | All `sevenz-rust` releases can extract relative or absolute paths outside the destination. | None; the repository was deleted. | Replace `sevenz-rust` with `sevenz-rust2`; retain application-level cross-platform path confinement, safe file creation, file-count and expanded-byte limits. |
| [RUSTSEC-2026-0246](https://rustsec.org/advisories/RUSTSEC-2026-0246.html) | `sevenz-rust` is unmaintained. | None. | Same maintained-fork migration; no advisory ignore. |

The extractor uses `sevenz-rust2`'s archive reader and retains application-level cross-platform path confinement, safe file creation, file-count and expanded-byte limits. Regression tests cover valid nested extraction, absolute and parent paths with both separator styles, an existing symlink-parent escape, output collisions and the resource limits.

## Narrow unmaintained-advisory exceptions

The four ignores in `deny.toml` are individual RustSec unmaintained advisories, not vulnerability waivers. RustSec reports no patched release for each. The current target-specific Tauri/keyring dependency graph reaches these crates transitively, so this workspace cannot replace them without a compatible parent-crate update. The ignore reasons link to the primary advisory records. Reassess the exceptions when that dependency graph changes.

| Advisory | RustSec finding and upstream migration guidance |
| --- | --- |
| [RUSTSEC-2024-0370](https://rustsec.org/advisories/RUSTSEC-2024-0370.html) | `proc-macro-error` is unmaintained and has no patched release. RustSec lists [`manyhow`](https://crates.io/crates/manyhow) and [`proc-macro2-diagnostics`](https://github.com/SergioBenitez/proc-macro2-diagnostics) as alternatives; the current transitive macro chain must move before this workspace can drop it. |
| [RUSTSEC-2026-0371](https://rustsec.org/advisories/RUSTSEC-2026-0371.html) | `zbus_names` is unmaintained with no patched release; upstream folded its name types into `zbus` 6. |
| [RUSTSEC-2026-0370](https://rustsec.org/advisories/RUSTSEC-2026-0370.html) | `zvariant` is unmaintained with no patched release; upstream folded its wire module into `zbus` 6. |
| [RUSTSEC-2026-0372](https://rustsec.org/advisories/RUSTSEC-2026-0372.html) | `zvariant_derive` is unmaintained with no patched release; upstream moved its derive macros into `zbus` 6. |

All other vulnerability and unmaintained advisories remain subject to the default deny policy. The policy does not suppress ZIP advisories or broadly disable unmaintained checks.
