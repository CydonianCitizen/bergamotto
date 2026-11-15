# Managed tools archive extraction audit

## FFmpeg Linux x64 `tar.xz` benchmark

Input: `/tmp/ffmpeg-release-amd64-static.tar.xz` (40 MiB). SHA-256:
`abda8d77ce8309141f83ab8edf0596834087c52467f6badf376a6a2a4c87cf67`.

Measured existing `lzma-rs` extraction path using the debug-profile Rust integration-test binary. The child process exited successfully. The test reported 25.88 seconds; Python wall clock also measured 25.88 seconds. Peak resident set size reported by `resource.getrusage(RUSAGE_CHILDREN)` was 338,980 KiB (about 331 MiB). Run completed under the 30-second timeout, so no decompressor replacement was needed.

Reproduction from repository root (after building the test target with `cargo test -p bergamot-core --test managed_tools --no-run`; substitute the generated `managed_tools-*` executable name if Cargo changes its hash):

```sh
python3 -c 'import os,resource,subprocess,time; env=os.environ.copy(); env["BERGAMOT_FFMPEG_ARCHIVE"]="/tmp/ffmpeg-release-amd64-static.tar.xz"; cmd=["target/debug/deps/managed_tools-10a1554bd85a4c73","real_ffmpeg_archive_extraction_benchmark","--ignored","--exact","--nocapture"]; before=time.monotonic(); p=subprocess.run(cmd,env=env,capture_output=True,text=True,timeout=30); elapsed=time.monotonic()-before; print(p.stdout,end=""); print(p.stderr,end=""); print(f"elapsed_seconds={elapsed:.2f}"); print(f"max_rss_kib={resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss}"); raise SystemExit(p.returncode)'
```

Observed output:

```text
running 1 test
test real_ffmpeg_archive_extraction_benchmark ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 25.88s

elapsed_seconds=25.88
max_rss_kib=338980
```
