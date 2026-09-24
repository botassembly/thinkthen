# 0101: Refuse a non-shell live job before any charge

Status: landed 2026-09-24. The branch `ticket/0101-live-refuses-non-shell-jobs` was rebased onto `18c0dc10` and merged to main. The fix is `c40bdb79` and the README sentence is `e41f4db6`. A fresh read-only review of `dae009ad` (`sdlc/records/0101-review.md`) found one wording error, the job count. This commit fixes it.

## Result

`sdlc/scripts/live` reads the first ten bytes of the job before it takes the ledger lock. Unless they are exactly `#!/bin/sh` and a line end, it exits 2 with `live: the named live job must start with the line #!/bin/sh, so no call goes out`. No ledger row is written and the job does not run. The check also refuses an empty or unreadable job. The key check still runs first, and it charges nothing.

45 tracked files under `demos/` and `probes/` start with exactly `#!/bin/sh` and a line end. The count came from `git ls-files demos probes` and a byte check of each file's first ten bytes with `head -c 10 | od -An -c`. The non-shell helpers there, such as `demos/16-triage-pipeline/triage` and `probes/find-0040/*.py`, are never run through `live`. Every job a page runs through `live` passes.

## Red and green

The new `language` case in `sdlc/live-test` runs a `#!/usr/bin/python3` job and an empty job in its own temporary checkout and ledger. Each must leave `charged_tokens 0`, exit 2 with the exact sentence above, and never run.

- Red, on `origin/main`'s `live` with the new case: `live-test: missing 'charged_tokens 0' in: limit_tokens 10 / charged_tokens 4 / remaining_tokens 6`. The Python job charged its 4 tokens and then died under `/bin/sh` with a syntax error.
- Green, at `c40bdb79`: `live-test: all cases passed`.

## Checks

Run with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset at `089cc6da`, rebased onto `18c0dc10`. The one-minute load stayed between 5.7 and 9.3.

- `sdlc/scripts/lint`: exit 0.
- `sdlc/scripts/test`, first run: exit 101. The one failure was `annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` at `scheduling.rs:102`, "document at 4". That is the flaky check filed in `sdlc/issues/2026-09-24-the-global-queue-concurrency-check-fails-under-load.md`. This branch changes no Rust.
- `sdlc/scripts/test`, second run: exit 0, 14 `test result: ok` lines, `live-test: all cases passed`.

The commit after `089cc6da` changes only this record. Lint and the test rung ran again on it before the merge.

No real ledger, no real key, and no backend were touched. `live-test` builds every ledger under `TMPDIR`.

## Also landed

`sdlc/scripts/README.md` now says a job is a shell script whose first line must be exactly `#!/bin/sh`. That was the issue's second guard. It waited for ticket 0083, which owned the file.

## Left open

The issue's packing design question stays open.
