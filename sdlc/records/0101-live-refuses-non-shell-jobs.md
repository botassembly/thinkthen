# 0101: Refuse a non-shell live job before any charge

Status: built on `ticket/0101-live-refuses-non-shell-jobs`, rebased onto `1b7087b0` after ticket 0083 landed. The fix is `abc58767` and the README sentence is `f89a5a33`. It awaits a fresh read-only code review before it lands.

## Result

`sdlc/scripts/live` reads the first ten bytes of the job before it takes the ledger lock. Unless they are exactly `#!/bin/sh` and a line end, it exits 2 with `live: the named live job must start with the line #!/bin/sh, so no call goes out`. No ledger row is written and the job does not run. The check also refuses an empty or unreadable job. The key check still runs first, and it charges nothing.

All 46 committed jobs under `demos/*/record.sh` and `probes/` start with `#!/bin/sh`, checked with `head -1`.

## Red and green

The new `language` case in `sdlc/live-test` runs a `#!/usr/bin/python3` job and an empty job in its own temporary checkout and ledger. Each must leave `charged_tokens 0`, exit 2 with the exact sentence above, and never run.

- Red, on `origin/main`'s `live` with the new case: `live-test: missing 'charged_tokens 0' in: limit_tokens 10 / charged_tokens 4 / remaining_tokens 6`. The Python job charged its 4 tokens and then died under `/bin/sh` with a syntax error.
- Green, at `abc58767`: `live-test: all cases passed`.

## Checks

Run with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, at `f89a5a33`.

- `sdlc/scripts/lint`: exit 0.
- `sdlc/scripts/test` (rung 2, which runs `sdlc/live-test` on Linux): exit 0, 14 `test result: ok` lines, `live-test: all cases passed`.

One earlier run of rung 2, before the second rebase, failed once in `annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` at `scheduling.rs:102`. That is the flaky check filed in `sdlc/issues/2026-09-24-the-global-queue-concurrency-check-fails-under-load.md`. This branch changes no Rust. The next two runs passed.

No real ledger, no real key, and no backend were touched. `live-test` builds every ledger under `TMPDIR`.

## Also landed

`sdlc/scripts/README.md` now says a job is a shell script whose first line must be exactly `#!/bin/sh`. That was the issue's second guard. It waited for ticket 0083, which owned the file.

## Left open

The issue's packing design question stays open.
