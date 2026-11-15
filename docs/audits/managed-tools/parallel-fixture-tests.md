# Parallel verifier fixture audit

Verified on Linux x86_64, 2026-11-15.

The first attempted fix wrote each ImageMagick fixture to a unique temporary file, called `sync_all`, dropped its writer, then renamed and made it executable. Twenty consecutive default `cargo test --workspace` runs still failed eight times: ImageMagick in runs 5, 8, 10, 16, 17; Python in runs 1, 6, 18. The failure was `ExecutableFileBusy` / `Text file busy`. Unique paths and an explicitly closed and synced writer therefore did not remove the race. See `parallel-tests-initial.json` and `parallel-fixture-failure.txt`.

The final fix removes runtime-created executable files from the FFmpeg, Python and ImageMagick verifier tests. Each test runs an existing shell with test-specific command arguments and synthetic output through a cloned `VerificationConfig`. Positive and negative cases still exercise the actual verifier. The exact kernel-level owner of the writable descriptor was not traced; this fix removes the affected executable inode from the write/execute race instead of retrying a failed exec. Resolver path fixtures remain separate.

Final verification: **20 consecutive runs, zero failures**. Each invocation was exactly `cargo test --workspace`, with default test parallelism. The runner removed any inherited `RUST_TEST_THREADS` setting (none was present); it did not set a thread limit, retry individual tests, sleep, or use serial attributes. Source files stayed unchanged throughout the final campaign. Runs were sequential; tests within each run used the default parallel harness. Local socket permission was enabled for redirect unit tests.

Per-run exits and elapsed times: `parallel-tests-final.json`. Full final workspace output: `parallel-workspace-final.txt`. The Windows shell branch was reviewed but not executed on this Linux host.
