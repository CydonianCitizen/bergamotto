**Security remediation verification — 2026-11-15**

Implemented on local branch `fix/security-audit`, based on `3ad9b25`. Seven separate English Conventional Commits address the requested fixes. No push or merge was performed. Tauri application-command permissions and the deferred overengineering cuts remain unchanged.

| Requested item | Commit | Implementation and verification actually performed |
| --- | --- | --- |
| 1. Uninstall | `ad87eef` | Checks embedded manifest membership before deletion. Core accepts exactly one normal path component and removes only that child. Seven core regressions cover traversal, absolute paths, separators, root protection, valid removal and Unix symlink target preservation; the Tauri manifest guard test also passed. |
| 2. TAR/ZIP | `1c10f72` | Validates relative symlink graphs, including forward links, cycles, expansion budgets and the `d -> .` / `d/../outside` escape. Skips link chmod, masks unsafe TAR modes before unpacking, normalizes regular files to `0755` or `0644`, creates new directories at `0755` and preserves existing directory/root permissions. Sixteen archive-security regressions passed, including raw special-bit fixtures for TAR and ZIP. |
| 3. Managed subprocesses | `7afb48c` | Reuses the process executor's async group runner for verifiers, AppImage extraction, Python detection and macOS `xcode-select`. Caller cancellation reaches subprocesses; deadlines are 30 seconds for verification, 5 seconds for detection and 300 seconds for AppImage extraction. Regular archive work runs on blocking workers and finishes before cancellation cleanup. Three managed-process regressions and the nonzero-leader descendant test passed; existing process timeout/cancellation tests remained green. |
| 4. Output limits | `7626a3b` | Enforces limits while reading: 8 MiB per process pipe and captured/discarded webhook response, 1 GiB per streamed webhook file, and 16 MiB of aggregate retained action output. Overflow produces an explicit error and terminates the process group. File responses stream into staging and flush before publication. Process tests: 12 passed; webhook unit tests: 3 passed; webhook integration tests: 6 passed. These include chunked bodies, None mode, aggregate Each output, descendant termination and staging cleanup. |
| 5. Test UI | `7954e3a` | Replaces misleading dry-run/sandbox copy with an action test explanation: the saved action executes on the selected original file and can modify it or perform its normal side effects. Typecheck and frontend build passed. No browser interaction test was performed for this copy-only change. |
| 6. Release workflow | `5e630dc` | Passes context through environment variables, validates an anchored ASCII version-tag pattern before writing outputs, and grants repository write access only to the publishing job. The actual extracted Bash block passed 12 local checks covering valid tags, dispatch fallback, malformed input, newlines and shell metacharacters. The GitHub release workflow itself was not run or published. |
| 7. Dependencies / CI | `52084c8` | Replaces `sevenz-rust` with `sevenz-rust2 0.25.0` and upgrades ZIP to `9.6.0`. Five new 7z tests passed for valid layout, unsafe names, symlink parents, collisions and resource limits. RustSec patch pages and newer upstream advisory ranges were reviewed; no ZIP or sevenz vulnerability was ignored. Four linked maintenance-only exceptions cover framework transitives. The new read-only CI advisory job is configured; it has not run remotely. See the [dependency review](security-dependencies-2026-11-15.md). |
| 8. Closure | All code above | All commands below passed. Twenty separate workspace test commands ran with four concurrent workers: each reported 108 passed, zero failed and five ignored. The checked results are recorded in [security-tests-2026-11-15.json](security-tests-2026-11-15.json). Final integrated code review found no important issues. |

The output caps keep diagnostics/text responses in bounded memory, permit larger file responses on disk, and prevent accumulation across Each jobs. Legitimate output exceeding these caps now fails explicitly.

Actual final checks:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                         # 20 runs, concurrency 4
npm run typecheck
npm run build
cargo deny check licenses
cargo deny check advisories
```

The license check passed with one pre-existing unmatched allowance warning for `Unicode-DFS-2016`; no license policy was broadened. The advisory scan fetched current RustSec data and passed with only the four documented maintenance exceptions. Sandbox restrictions prevented local listener binds during initial attempts; final HTTP tests and workspace repetitions ran outside that sandbox.

The production pinned Python Linux x64 installer test was explicitly run and passed after the final code changes. It used the embedded URL and SHA-256 pin, downloaded the real archive, installed to temporary storage, verified the interpreter, executed Python and checked normalized file permissions while preserving symlinks. The cached real-archive extraction/verification test also passed during the archive work. The archive contains 3,522 regular entries and 1,049 symlinks, including forward links and legitimate parent components.

The default workspace suite leaves five tests ignored. The two Python cases were executed separately as described above. The process fixture entry point is invoked by integration tests. The real FFmpeg extraction benchmark and the broad live-download test for every managed tool were not run as part of this remediation.

Native macOS, Windows and Linux ARM execution was not performed on this Linux x64 host. A Windows cross-target Clippy attempt stopped in `ring` because MSVC `lib.exe` is unavailable, before it could check the core. Unix-only imports and tests were reviewed and gated explicitly; this does not replace native platform testing. No macOS cross-target check or release packaging/signing run was performed.
