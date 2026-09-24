# 0101: Refuse a non-shell live job before any charge

Status: built at `465a730e` on `ticket/0101-live-refuses-non-shell-jobs`. It awaits a fresh read-only code review before it lands.

## Result

`sdlc/scripts/live` reads the first ten bytes of the job before it takes the ledger lock. Unless they are exactly `#!/bin/sh` and a line end, it exits 2 with `live: the named live job must start with the line #!/bin/sh, so no call goes out`. No ledger row is written and the job does not run. The check also refuses an empty or unreadable job. The key check still runs first, and it charges nothing.

All 46 committed jobs under `demos/*/record.sh` and `probes/` start with `#!/bin/sh`, checked with `head -1`.

## Red and green

The new `language` case in `sdlc/live-test` runs a `#!/usr/bin/python3` job and an empty job in its own temporary checkout and ledger. Each must leave `charged_tokens 0`, exit 2 with the exact sentence above, and never run.

- Red, on `origin/main`'s `live` with the new case: `live-test: missing 'charged_tokens 0' in: limit_tokens 10 / charged_tokens 4 / remaining_tokens 6`. The Python job charged its 4 tokens and then died under `/bin/sh` with a syntax error.
- Green, at `465a730e`: `live-test: all cases passed`.

## Checks

Run with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, at `465a730e`.

- `sdlc/scripts/lint`: exit 0.
- `sdlc/scripts/test` (rung 2, which runs `sdlc/live-test` on Linux): exit 0, 13 `test result: ok` lines, `live-test: all cases passed`.

No real ledger, no real key, and no backend were touched. `live-test` builds every ledger under `TMPDIR`.

## Left open

The issue's second guard is a sentence in `sdlc/scripts/README.md` saying a job is a shell script. Ticket 0083 owns that file, so the sentence waits for it. The issue's packing design question stays open.
