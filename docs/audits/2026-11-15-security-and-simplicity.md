**Bergamotto security and overengineering audit — 2026-11-15**

Reviewed commit `3ad9b254cdf8c207d2741e599998bdf44b94769b` across core, native adapters, Tauri shell/capabilities, frontend, manifests, tests, dependencies, workflows, documentation and asset inventory. Applied `ponytail-audit` to simplification; assessed security separately as requested. This report changes no application code. Existing untracked license reports were preserved.

The architecture largely serves documented requirements. The strongest security findings concern archive permissions, unbounded output and backend path validation. Priorities below describe this application's exposure, not upstream advisory scores. P2 means medium priority; prerequisites are stated explicitly.

**P2 — TAR extraction follows symlinks when changing permissions.**

At [extractor.rs:263](../../crates/bergamot-core/src/managed_tools/extractor.rs#L263), `set_permissions` follows a symlink and applies its archive mode to the target. A normal archive containing a regular executable with mode `0755`, followed by a symlink to it with conventional mode `0777`, leaves the executable world-writable. Another local user could replace it if parent directories permit access. The current pinned Python archives were not downloaded, so their actual entry ordering and resulting permissions remain unverified.

There is also a containment bypass at [extractor.rs:248](../../crates/bergamot-core/src/managed_tools/extractor.rs#L248): lexical normalization does not model existing symlinks. A TAR containing `d -> .`, followed by `bad -> d/../outside`, passes validation yet resolves outside the extraction root. The subsequent permission operation changes the outside target. Exploiting this crafted archive through the installer requires content accepted by the embedded manifest's pinned hash; downloaded bytes alone cannot bypass that hash.

Independent reviewer runs against the compiled core confirmed both cases using harmless fixtures under `/tmp`:

```text
normal symlink extraction=Ok(()); executable target mode=777
result=Ok(()); outside_mode=600->777; escaping_link=Ok("d/../outside")
```

Fix: never chmod through symlink entries; confine resolved link targets, including chains, or reject unsupported links. Apply a deliberate regular-file permission policy that strips special bits and group/other write access. ZIP extraction also copies archive permission bits without masking at [extractor.rs:128](../../crates/bergamot-core/src/managed_tools/extractor.rs#L128). Add regression coverage for ordinary executable symlinks, chained links and permission bits.

**P2 — Unbounded webhook and process output can exhaust application memory.**

[webhook.rs:192](../../crates/bergamot-core/src/executors/webhook.rs#L192) collects an entire successful response, then copies it into a `Vec`; this also happens for `outputMode: "none"`. A malicious or compromised configured endpoint can stream excessive data before the timeout. [process.rs:570](../../crates/bergamot-core/src/executors/process.rs#L570) likewise reads stdout/stderr without limits, and dispatcher aggregation retains output across files.

A reviewer run through the compiled executor captured `67,108,864` bytes in approximately `30 ms`, with resident memory around `70 MB`. This demonstrates allocation growth; no OOM was attempted. Enforce limits during reading, including chunked responses and aggregate output. Stream file responses into staging; discard unneeded response bodies. On process overflow, terminate the group with an explicit error or continue draining while retaining only bounded diagnostics.

**P2 — Uninstall accepts directory traversal from renderer IPC.**

[commands.rs:361](../../src-tauri/src/commands.rs#L361) passes arbitrary `tool` strings to [installer.rs:191](../../crates/bergamot-core/src/managed_tools/installer.rs#L191), which joins them to the tools directory and calls `remove_dir_all`. Checking for a known tool happens after deletion. Reviewer runs of the compiled helper confirmed that both `../victim-relative` and an absolute fixture path removed directories outside the tools root.

This requires access to the application's local IPC, such as a compromised bundled renderer; no external website entry point was demonstrated. The normal UI supplies embedded tool IDs. Validate membership before deletion, require exactly one normal path component in the core helper, and constrain removal to the intended managed-tool child directory.

**P2 — Managed-tool subprocesses bypass process-tree and timeout guarantees.**

[verifier.rs:35](../../crates/bergamot-core/src/managed_tools/verifier.rs#L35) and its later probes use synchronous `Command::output()` without cancellation, deadline or process groups. [extractor.rs:333](../../crates/bergamot-core/src/managed_tools/extractor.rs#L333) runs AppImage extraction similarly. Cancellation is checked only between installation stages. A hung binary can keep an installation blocked and descendants lack the cleanup guarantee required by AGENTS.md. Python detection also kills only its immediate child at [process.rs:462](../../crates/bergamot-core/src/executors/process.rs#L462).

Use a shared bounded process-group runner with deadlines and cancellation for these paths. Keep blocking work off async runtime threads. This is a lifecycle/resource guarantee gap; the pinned tools are trusted executables, and no hostile vendor binary was run during this audit.

**P2, data safety — The advertised dry run executes real actions.**

[TestSandbox.tsx:94](../../src/components/ActionStudio/TestSandbox.tsx#L94) says “Dry-Run & Action Test Sandbox”, but [line 57](../../src/components/ActionStudio/TestSandbox.tsx#L57) invokes the normal backend with the selected original path. Process/script actions can modify that file and perform their normal side effects. Rename it to an execution test and state that it runs the saved action on the selected file. If an isolated test is wanted, implement explicit copied inputs and a separate policy for external side effects.

**Dependency findings and applicability.**

A freshly fetched `cargo deny check advisories` failed: five unique vulnerability advisories and five unique maintenance advisories. These are dependency matches, not five demonstrated application exploits:

| Locked dependency | Advisory IDs | Application assessment |
| --- | --- | --- |
| `sevenz-rust 0.6.1` | [RUSTSEC-2026-0245](https://rustsec.org/advisories/RUSTSEC-2026-0245.html), [0246](https://rustsec.org/advisories/RUSTSEC-2026-0246.html) | Path traversal plus abandonment. The custom callback rejects ordinary absolute/parent paths before writing, mitigating the default extractor issue. Migrate to a maintained implementation; preserve and test confinement. |
| `zip 2.6.1` | [RUSTSEC-2026-0367](https://rustsec.org/advisories/RUSTSEC-2026-0367.html), [0368](https://rustsec.org/advisories/RUSTSEC-2026-0368.html), [0369](https://rustsec.org/advisories/RUSTSEC-2026-0369.html), [0429](https://rustsec.org/advisories/RUSTSEC-2026-0429.html) | The custom loop avoids the listed built-in extraction APIs and rejects ZIP symlink entries. Its own permission copying still warrants correction. RustSec lists `>=9.0.0` as patched; assess the API migration and retain regression tests. |
| `proc-macro-error` | [RUSTSEC-2024-0370](https://rustsec.org/advisories/RUSTSEC-2024-0370.html) | Unmaintained transitive dependency. |
| `zbus_names`, `zvariant`, `zvariant_derive` | [RUSTSEC-2026-0371](https://rustsec.org/advisories/RUSTSEC-2026-0371.html), [0370](https://rustsec.org/advisories/RUSTSEC-2026-0370.html), [0372](https://rustsec.org/advisories/RUSTSEC-2026-0372.html) | Maintenance notices affect multiple versions through keyring/Tauri plugins; coordinate upstream upgrades. |

CI currently runs no advisory gate. Add one with explicit, reviewed applicability decisions rather than suppressing all archive advisories. `npm audit` returned zero advisories at audit time.

**Additional hardening, with limited threat exposure.**

- [build.rs:1](../../src-tauri/build.rs#L1) does not register application-command permissions. The narrow capability JSON files constrain plugin/core commands but leave custom action/settings/secret/install commands available to all app webviews by default. Register them with `AppManifest::commands` and grant only needed commands per window. [Tauri documents this default](https://v2.tauri.app/security/capabilities/). No frontend injection was found that supplies an initial exploit.
- [release.yml:153](../../.github/workflows/release.yml#L153) interpolates tag/input text directly into Bash. Command substitutions in such text execute on the release runner. Tag-push/workflow-dispatch authority is required, so this is not a public pull-request exploit. Pass values via environment variables, validate the release tag, and keep write permissions limited to publishing. [GitHub explains expression-based script injection](https://docs.github.com/en/actions/concepts/security/script-injections).

**Verified controls and limits.**

Direct action arguments remain separate, inserted paths are literal and absolute, webhook redirects are disabled, secret errors are sanitized, action IDs are validated, and output publication uses no-clobber staging. Intentional user-authored scripts and explicitly configured webhook destinations were treated as trusted configuration.

All five required checks passed: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm run typecheck`, `npm run build`. Workspace tests: **66 passed, 3 ignored**. The ignored cases are the live managed-tool download test, real FFmpeg archive benchmark, and subprocess fixture entry point. Tests were rerun outside the sandbox after its local-listener restriction caused an initial failure. macOS/Windows paths received static review; this Linux audit did not execute their native UI or installers.

**Overengineering cuts, largest first — ponytail-audit.**

- `shrink` Reuse `Fields.Arguments` in GuidedWizard; retain variable pills and remove four redundant CSS rules. Approximately 50 net lines. [GuidedWizard.tsx:392](../../src/components/ActionStudio/GuidedWizard.tsx#L392), [Fields.tsx:44](../../src/components/ActionStudio/Fields.tsx#L44).
- `delete` Remove `DragMoved`, previous-position storage and discard branches; preserve start/end/modifier events. Approximately 23 net lines including tests. [os-adapter/lib.rs:70](../../crates/os-adapter/src/lib.rs#L70), [global_drag.rs:14](../../src-tauri/src/global_drag.rs#L14).
- `delete` Remove unused frontend `DragContext`, `ActionInvocation`, `ActionResult` mirrors; IPC uses different payloads. 19 lines. [actions.ts:105](../../src/types/actions.ts#L105).
- `shrink` Replace custom installer cleanup guard and UUID paths with the existing `tempfile::TempDir`, retaining cleanup and same-filesystem publication. Approximately 19 net lines. [installer.rs:97](../../crates/bergamot-core/src/managed_tools/installer.rs#L97).
- `shrink` Derive icon lookup from `AVAILABLE_ICONS`, retaining seven legacy aliases. Approximately 17 net lines. [ActionIcon.tsx:54](../../src/components/ActionIcon.tsx#L54).
- `yagni` Remove the empty `ActionMatcher` namespace wrapper; production already uses the free function. Update its test caller and export. 12 net lines. [matcher.rs:3](../../crates/bergamot-core/src/matcher.rs#L3).
- `delete` Remove unobserved `job:progress` emission, its optional publication argument and supporting clones; retain snapshots. Approximately 11 net lines. [jobs.rs:75](../../src-tauri/src/jobs.rs#L75).
- `delete` Remove unused `global_drag_enabled` IPC command/registration; Studio already receives the setting. Six lines. [global_drag.rs:145](../../src-tauri/src/global_drag.rs#L145).

Counts estimate current regions minus equivalent replacements, including ordinary formatting. No dependency was proven unnecessary. Managed tools, all five archive formats, core/UI separation, validation, staging, rollback, secrets and cancellation serve shipped features or explicit requirements; retain them.

Net removable: **approximately 157 source lines; zero dependencies**. These are recommendations, not applied deletions.
