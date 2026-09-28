# ADR 0097: Bound advisory usage-lock acquisition at command exit

Status: **Accepted** on 2026-09-28 by the coordinator within register 31 after fresh independent design review of `1e4936a7`. [Ticket 0225](../../tickets/0225-bound-usage-lock-waits.md) builds it. Ian may change the one-second budget or restore the earlier durability choice.

## Problem

ADR 0049 item 3 made a command wait as long as another process holds the count-only usage lock. A suspended lock owner can leave the answer printed while the command cannot exit. `Counters::finish` waits for the writer; `Counters::drop` then joins it. Timing out only the condition-variable wait would still leave that join blocked on `File::lock`. The live paid-call ledger is separate and does not use this policy.

## Amendment to ADR 0049 item 3

Keep the asynchronous per-process writer and the atomic monthly counts. While the command is running, the writer may retry a busy usage lock without holding back a request. At `Counters::finish`, establish one monotonic deadline one second later for **all remaining advisory usage-lock acquisitions** by that writer. `Counters::drop` establishes the same deadline if finish was not called. Poll the writer's `File::try_lock` at a short interval. If another process still owns the lock at the deadline, fail that write, clear remaining queued deltas, mark persistence failed, and let the writer stop so finish and drop can return. Use the existing single fixed usage warning, after output and before `--facts`; do not change the judgment or exit code. A later count in that process must not restart persistence after failure.

A completed month write stays durable. The timed-out month and subsequent queued counts can be lost, just as a crash can lose counts after the last completed write. The current in-memory run facts still describe observed work. The live ledger keeps its independent reservation and authority. A private `.lock` file can remain; timeout occurs before opening or replacing a month temporary file under that lock.

The one-second budget bounds waiting **for another process to release this advisory lock after finish begins**, even when several month deltas are pending. It does not promise a one-second total process exit: opening files, sync, rename, network filesystems, the reader's shared lock, and unrelated work can still take longer. A held-lock functional case should keep the lock owned until the command exits under a generous watchdog, pin the warning and absent new count, and leave the paid ledger untouched. The source should show one shared finish deadline rather than a fresh one-second allowance per month. Amend the settled recording page in the reviewed build; this ADR supersedes ADR 0049 item 3's unlimited lock wait, not its write-behind design or count meaning.
