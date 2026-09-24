FINDINGS

# Review of ThinkThen Quick Fix 0101 at dae009ad

Reviewer: fresh read-only Claude session. Branch `ticket/0101-live-refuses-non-shell-jobs`, worktree `worktrees/thinkthen-0101`. No real ledger, key, or backend was touched. All live runs used temporary ledgers under the scratchpad.

## Findings

1. The job count is wrong in the ticket and the record. Both say "All 46 committed jobs". A byte check of every tracked file under `demos/` and `probes/` finds 45 files whose first line is exactly `#!/bin/sh`, at both HEAD and origin/main. The filesystem count is also 45. Every job that any page names in a `live --max-tokens` command passes the rule. The rule holds. The number does not. Fix the number in `sdlc/tickets/0101-live-refuses-non-shell-jobs.md` and `sdlc/records/0101-live-refuses-non-shell-jobs.md`, or name the command that yields 46.

## Landing condition (not a code finding)

The branch sits on `1b7087b0`, not on origin/main `c7daaebc`. `git merge-tree --write-tree origin/main HEAD` is clean. The 0103 changes on main touch no file this branch touches. The record's status line still says "rebased onto `1b7087b0`". Rebase onto current main, update that line, and rerun lint and test on the exact commit that lands.

## What was checked

1. Order of operations in `sdlc/scripts/live`. `main` runs `capture_key`, then `legacy_absent`, then `launch`. In `launch`, the usage check, the reservation parse, the `isfile` check, and the new `shell_job` check all run before `lock_authority`, `read_ledger`, and the `charge` append. No path reaches the ledger write without passing `shell_job`. Edge cases, each run against a temporary ledger with a dummy key and `--max-tokens 1`, then `--status`:
   - CRLF `#!/bin/sh\r\n`: exit 2, refusal sentence, charged 0.
   - UTF-8 BOM before `#!/bin/sh`: exit 2, charged 0.
   - `#!/bin/sh -e`: exit 2, charged 0.
   - `#!/bin/sh` with no line end: exit 2, charged 0.
   - `#!/bin/shx`, leading space: exit 2, charged 0.
   - Empty file: exit 2, charged 0.
   - Unreadable file (mode 000): exit 2, charged 0.
   - Missing file, directory, FIFO, dangling symlink: exit 2, "the named live job is not a file", charged 0.
   - Symlink to a non-shell file, absolute path to a non-shell file: exit 2, charged 0.
   - Symlink to a good job and a good job: exit 0, charged 1 each, job ran.
   None of the refused jobs ran.
   Note: the check reads the file before the lock and `/bin/sh` opens it again after the charge. A file swapped in between would run. This guard targets an honest mistake, so the gap is acceptable.
2. Committed jobs: 45 files under `demos/` and `probes/` start with exactly `#!/bin/sh`. The non-shell executables (`demos/16-triage-pipeline/triage` and `self-test`, `probes/find-0040/*.py`) are helpers, and no page runs them through `live`. Every named live job passes.
3. Red and green: the branch's `sdlc/live-test` run against origin/main's `live` fails with `missing 'charged_tokens 0' in: limit_tokens 10 / charged_tokens 4 / remaining_tokens 6`. Against the branch's `live` it prints `live-test: all cases passed`.
4. README sentence: "A job is a shell script. Its first line must be exactly `#!/bin/sh`, or the wrapper refuses it before any charge." This matches the code. The sentence does not mention the status 2, and it does not need to.
5. Files: the diff against the merge base touches exactly the five files the ticket's `opens:` line lists, plus the ticket itself. The merge onto origin/main is clean.
6. Gates, THINKTHEN_ variables unset, one-minute load 4.5 to 6.4:
   - `sdlc/scripts/lint`: exit 0.
   - `sdlc/scripts/test`, first run: exit 101. The one failure was `annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` at `scheduling.rs:102`, "document at 32". That is the known flaky check in `sdlc/issues/2026-09-24-the-global-queue-concurrency-check-fails-under-load.md`. This branch changes no Rust.
   - `sdlc/scripts/test`, second run: exit 0, 14 `test result: ok` lines, `live-test: all cases passed`.

Scratch copies were deleted.
