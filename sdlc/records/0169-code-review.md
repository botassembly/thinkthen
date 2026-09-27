# 0169 code review

A fresh read-only Codex Sol Medium reviewer inspected candidate `e8005b8b` and found two defects: missing `ModelsDiffer` and `UsageOverflow` signal conversion, and a check report printed after a stopped probe. The author reproduced the held-401/SIGTERM report, corrected both paths at `7c651734`, and merged main metadata into `84de0a7d`.

The same independent reviewer accepted the bounded correction at `84de0a7d`. It checked bare and stopped backend-cause coverage, cancellation before check report output, the held probe's empty stdout/stderr, actual SIGTERM exit and one request, and the recorded red/green proof. It measured 76,097 nonblank Rust lines and found no file over 500 or new dependency. Final growth is 373 lines: 112 product and 261 test lines. The correction adds 7 product and 13 test lines and reuses the listener helper. The reviewer checked test value and duplication rationale. It ran no builds or tests.

The separate SIGUSR1 observation remains in test-harness issue item 14. No engine or worker source changed in 0169. The review did not call an isolated retry a fix; the coordinator assigned its handler-interference path to a separate bounded Quick Fix.

Native fresh-review spawning hit its thread limit, so this review used a fresh read-only CLI session, `01a0e47d-f074-73f0-a5e2-0cdc5460e561`, then resumed it only for its two findings. The raw review and rereview outputs are retained under ignored `target/codex-builds/0169/review/`.
