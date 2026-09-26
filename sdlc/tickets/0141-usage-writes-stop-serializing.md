---
flow: build
priority: 141
opens: crates/thinkthen/src/engine/usage.rs crates/thinkthen/src/engine/usage/tests.rs crates/thinkthen/src/cli/mod.rs crates/thinkthen/src/engine/facade/fork_tests.rs crates/thinkthen/src/cli/conformance_tests/command.rs crates/thinkthen/src/cli/schedule/width_tests.rs crates/thinkthen/src/engine/deadline_tests.rs crates/thinkthen/tests/backend/default_cache/usage.rs crates/thinkthen/tests/backend/harness/mod.rs specification/recording.md sdlc/planning/adr sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0141: Usage writes stop serializing requests

Status: ready. The coordinator accepted it on 2026-09-26 after a fresh read-only review. Owner: Claude. This is ticket B1 of `sdlc/issues/2026-09-26-batching-design.md`.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user runs `thinkthen filter Q --jobs 16 < titles.txt` and gets 16 requests in flight. Today the usage file sets the pace. Every request waits its turn to rewrite that file and force it to disk, twice. Experiment 268 measured about half of a 7-second run spent in that queue.

The ask is `sdlc/issues/2026-09-26-usage-file-writes-serialize-requests-in-flight.md`: usage counting should not limit how many requests run at once, and the monthly totals stay correct across processes. The batching design's B1 row names the proof as that issue's timing at `--jobs 16`. Ian's ruling 9 of 2026-09-26 puts B1 right after B0 and C1. B1 depends on neither.

The totals stay what `specification/recording.md` says they are: count-only, in the private usage folder beside the platform cache, under the same `.lock`, in the same `thinkthen.usage/1` file per month.

## What happens today

`Counters::add` in `crates/thinkthen/src/engine/usage.rs:127` bumps four process atomics. Then it takes the shared `persistent` mutex (`:133`) and calls `update()` on the caller's own thread. `update()` takes the cross-process `.lock` (`:204`), reads the month file, writes `.update.tmp`, syncs it (`:237`), renames it, and syncs the folder (`:243`). On a new month file it also syncs the lock and the folder (`:214`, `:215`).

Each live request calls `add` twice. `request.rs:100` counts the request just before the socket write, through `post_observed`. `request.rs:149` adds the reply's tokens. A replayed answer calls it once, at `request.rs:136`. Every worker queues on one mutex, and the holder waits on the disk. A second `thinkthen` process holding `.lock` stops every request of this one.

Ticket 0063 chose this on purpose: a "best-effort atomic precharge before the send", so a crash can overcount one request and never undercount one. `recording.md:26` states that guarantee.

## Design

Workers stop touching the disk. They add to memory, and one writer thread per `Counters` writes.

- `Counters` gains a `pending` list of `(month, Counts)` under a mutex, and a condition variable.
- `add` bumps the process atomics as today. Then it merges its delta into `pending` under the month `month_now()` gives at that moment, wakes the writer, and returns. It never waits on a file.
- The first `add` on counters with a path starts the writer thread. Counters with no path, as every library and SQL surface builds them, start none and keep memory counts alone.
- The writer takes everything pending, month by month. It writes each month's sum with today's `update()`, under today's `.lock`, and waits for more. One write carries every delta that piled up while the last one ran.
- `Counters::finish()` replaces `Counters::warning()`. It waits until nothing is pending and the writer is idle, then says whether persistence failed. `cli/mod.rs:89` calls it where it calls `warning()` today, after the ordered results and before the warning line.
- Dropping `Counters` finishes it, then stops the writer.
- The first failed write disables persistence and sets the warning, as today. Later deltas still reach the process atomics and are not queued.

`update()`, `read()`, the file format, the modes, the identity checks, and `thinkthen status` do not change.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **A writer thread takes the disk off the request path.** Every alternative that keeps a durable write before each send still stops every request while another process holds `.lock`. The issue asks that counting not limit requests in flight, so the write moves behind them.
2. **The precharge guarantee goes.** A crash can now lose the requests and tokens counted since the last finished write. `recording.md:26` said a crash "can undercount tokens or overcount one precharged request". It becomes: "A crash can undercount the requests and tokens counted after the last write that finished." The totals are best effort and "not a provider bill" already. The paid-call authority is the live ledger under `sdlc/scripts/live`, and that ledger keeps its own durable precharge under ADR 0022. So no spending guard rests on this file.
3. **The change takes a new ADR, number 0049.** `specification/README.md` says a Settled section changes only by a new ADR, and `recording.md` is Settled. Ticket 0139 takes 0048 and lands first. If the numbers change at build time, the ADR takes the next free one. It amends two sentences by name. One is `recording.md:26`. The other is ADR 0034's "A crash can leave a conservative overcount of one request or an undercount of later tokens", which becomes the same sentence as the new `recording.md:26`. The ADR records that ticket 0063's precharge rule is retired. ADR 0034 already says "The totals enforce no budget", so no guard rests on the precharge. The batching design marked B1 "no ADR". That row was written before this design showed the crash sentence had to change.
4. **Group commit was considered and set aside.** In a group commit, each worker waits for one shared write that carries its delta. It keeps the precharge. It still stalls every request while another process holds the lock, adds a disk round trip before each send, and has no count-based proof. Only a wall-clock race could show it.
5. **A writer thread, not one write at `finish()`.** One write at the end needs no thread and no join in `Drop`, so it is simpler. It has two costs. `thinkthen status` shows nothing of a long run until the run ends, and a kill loses the whole run's counts. The writer thread keeps both close to today's behavior. The queue owner ruled for the writer thread on these two grounds. Ian can overturn the ruling.
6. **Each delta keeps the month it was counted in.** A run that crosses midnight UTC at a month's end writes each count to the month `add` saw. Merging by month keeps that exact.
7. **`finish()` waits without a bound.** A process that holds `.lock` forever makes this command wait at its end. Today the same process makes it wait at its first request. The wait moves, and no new hang appears.
8. **The existing fault hook follows the writer.** `usage/tests.rs` injects a failure at each `update()` stage through the thread-local `FAILURE`. The writer thread copies its starter's `FAILURE` when it starts, under `cfg(test)` alone. No real boundary can fail the file sync, the rename, or the folder sync on demand. This moves an existing hook. It adds none.

## Edge cases

| Case | Expected behavior |
| --- | --- |
| Another process holds `.lock` for the whole run | Every request goes out, up to `--jobs` in flight. The command prints its results, then waits at `finish()`. The totals land when the lock is let go |
| `.lock` is never let go | The command waits at its end. Today it waits at its first request. Decision 7 |
| Two `thinkthen` processes run at once | Each has its own writer. Each write is a read-add-replace under `.lock`, so the month total is the sum of both |
| The usage folder has an unsafe mode | The first write fails. One warning prints after the unchanged results. Exit code unchanged |
| A write fails part way | Persistence stops for the process. One warning. Later counts stay in memory |
| The month turns during a run | Each count lands in the month it was counted in |
| No usage path: library, SQL, data frame | No writer thread. Memory counts as today |
| A forked child | Fresh counters and its own writer, as the facade already builds them. The parent's pending counts stay with the parent |
| Answers replayed from the platform cache | `cache_answers` counts through the same queue, and a replay never waits on the disk |
| `--jobs 1` | The same totals and output bytes as today |
| A checked addition would overflow | Refused. Persistence stops with the one warning, as today |
| The process is killed | Counts not yet written are lost. Decision 2 |
| Ctrl-C while `finish()` waits on a held lock | The first press cancels the run, but `finish()` keeps waiting on the lock. The second press kills the process by SIGINT, as today while a worker waits on the lock. Counts not yet written are lost |

## Proof

The new test runs the compiled command against a loopback listener. It lives in `crates/thinkthen/tests/backend/default_cache/usage.rs` beside the other usage tests, and it reuses `Gathering` from the backend harness. The test needs the child's handle while the lock is held. So `spawn` in `tests/backend/harness/mod.rs` splits in two: `start` builds and starts the child with today's cleared environment and returns it, and `spawn` calls `start` and then `finish`. Every existing caller keeps `spawn`.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `requests_go_out_while_another_process_holds_the_usage_lock` | The test makes the usage folder (mode 0700) and its `.lock` (mode 0600) under a private `XDG_CACHE_HOME`. It takes an exclusive lock on `.lock` from the test process. It starts `decide Q --jsonl --field /body --jobs 16 --no-cache` over 16 distinct records through `start`. The listener's replies call `Gathering::new(16).hold()`, and each reply reports 88 input and 12 output tokens. While the lock is still held, the test waits until the listener has read 16 requests, within the harness failsafe of 10 seconds. Then it polls the child for 500 ms and records whether it exited. It records the request count and `listener.peak()`, and lets the lock go. Then it asserts count 16, peak 16, that the child had not exited, exit 0, 16 rows, and empty standard error. Last, `status --json` shows this month's `requests_sent` 16, `input_tokens` 1408, `output_tokens` 192, and `cache_answers` 0 | (a) Call `update()` inside `add` again, today's code: no request arrives while the lock is held, and the count is 0. (b) The writer writes only the newest pending delta: the precharges pile up behind the held lock, so `status` shows fewer than 16 requests. (c) `finish()` and `Drop` return without waiting for the writer: the child exits while the lock is held |

The test lets the lock go before any assertion, so a red run still ends. A correct build cannot exit while the lock is held, because `finish()` waits for a write that needs the lock. So the 500 ms poll never fails a correct build. It only bounds how long plant (c) has to show itself.

The four questions:

- **What it protects.** `--jobs 16` means 16 requests in flight, whatever the usage file is doing, and the month totals are exact after the run.
- **What regression fails it.** Any write of the usage file on a worker's path: plant (a), which is today's code. Also a writer that drops deltas, plant (b), or an exit that outruns the writer, plant (c).
- **Why no existing test catches it.** Today's queue only slows a run, and it never changes a result. So every test passes on today's code. `parallel.rs` checks peaks at 4 jobs or fewer with no contended lock. The usage tests check totals and never check concurrency.
- **Does it need a test-only hook.** No. It holds the real `.lock` through the real `flock`, as a second `thinkthen` process would, and counts at the real listener.

Existing tests kept as they are: `two_processes_update_one_month_without_losing_a_cache_answer` proves totals across processes. `persistence_failure_warns_once_after_the_unchanged_judgment` proves the warning. `concurrent_updates_keep_every_count_in_one_monthly_aggregate` and `every_update_stage_warns_once_and_disables_later_persistence` keep their assertions. They call `finish()` before they read the folder. The unit tests in `cli/conformance_tests/command.rs` and `cli/schedule/width_tests.rs` read durable totals while their counters live, so each calls `finish()` first. `facade/fork_tests.rs` has no handle on the child's counters. It drops the engine before it reads the saved totals, and `Drop` finishes the writer.

No new unit test is added.

The build record also reports a loopback timing that gates nothing. It runs 306 records at `--jobs 16` against a listener that answers after 140 ms, before and after the change, on a named build. A number on a page names that record.

## Pages, comments, and issues

- `specification/recording.md:26` and the new ADR 0049, as decision 3 says.
- The doc comment on `accounting_that_outlasts_the_budget_sends_nothing` in `crates/thinkthen/src/engine/deadline_tests.rs:210` names "another process holding the usage lock" as slow accounting. After this ticket the lock never delays a send. The comment says instead that the test stands for any slow work before the attempt.
- `site/src/pages/backends.astro:48` says "A crash can undercount tokens or overcount one request that was already charged." `site/` belongs to the website agent. The build files `sdlc/issues/2026-09-26-site-states-the-retired-usage-crash-guarantee.md`, naming that line and the new sentence, and does not edit `site/`.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- `crates/thinkthen/src` production code: at most 60 added and at most 50 net, doc lines included.
- Tests: at most 80 added in `default_cache/usage.rs` and the harness split, and at most 15 changed across the existing unit tests.
- Pages and the ADR: the one sentence in `recording.md`, the one doc comment, an ADR of at most 30 lines, and the website issue.
- `sdlc/ratchet.json` moves to the measured total in the commit that adds the code. The commit says what grew. The builder looks for duplication to delete in `usage.rs` first.
- Budget ruling, 2026-09-26: The coordinator accepted the overrun: 103 added and 56 removed, net +47, within the 50 net budget. Most added lines replace removed ones, and merging the two Stage enums removes duplication. Ian can overturn this. The review fixes then brought it to 106 added, net +50, still within the net budget.
- No dependency. No public library type, method, or message changes, so the `surfaces` rung is not required.

## Stop rules

1. Stop before crossing a budget or adding a dependency.
2. Stop if any plant stays green.
3. Stop if the change needs `engine/request.rs`, the HTTP client, or the connection pool. Ticket 0142 (B2) owns the pool, and this design needs none of them.
4. Stop if another in-flight branch changes `engine/usage.rs` or `cli/mod.rs` before this one lands. The coordinator orders the two.
5. Stop if a `spec/` page or green demo turns red, or if `status` output changes.
6. Stop if the listener cannot hold 16 connections at once. Report it. Do not lower the job count to pass.

## Scope and exclusions

Excluded: the connection pool (B2, ticket 0142), `relate`'s `--jobs` (J1, ticket 0143), batching itself, and `thinkthen status`. The live ledger and `sdlc/scripts/live`, which keep their own precharge. `site/`, which the website agent owns.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling and moves a Settled guarantee, so the code review names what it checked.

## Complexity

Contract 1; state and timing 2; reach 1; proof 1; cost of error 1; total 6. Final level: 2. The risk is a lost count at exit or a hang at `finish()`. Plants (b) and (c) guard the first. Decision 7 bounds the second to today's behavior.

## Deferred gaps

- `finish()` could stop waiting when the run's cancel flag fires, so the first Ctrl-C would end the wait. It is not taken here.
- `finish()` has no time bound. A bounded wait would print the warning and exit, but it needs a timeout nobody has asked for.
- The writer still syncs the file and the folder on every write. The coalescing makes that cheap under load, so fewer syncs gain nothing today.
- The comment at `crates/thinkthen/src/engine/http.rs:127` says accounting may wait on the usage lock. After this ticket it cannot. Ticket 0142 owns that file and lands first. This build fixes the comment after it.
- `site/src/pages/backends.astro:48` keeps the retired guarantee until the website agent acts on the issue this build files.
- A fork while the writer holds `.lock` leaves a copy of that open lock in the child until the child closes it. Today a worker mid-update has the same exposure. The facade never touches the inherited counters.
- The speed target of the batching design (306 titles in under half a second) needs B2 and batching. This ticket removes one of the two filed limits.

## What Ian can overturn

- Decision 5: the queue owner's ruling for a writer thread over one write at `finish()`.
- Decision 2: dropping the precharge guarantee. The alternative is group commit (decision 4). It keeps the guarantee and still stalls every request while another process holds the lock.
- Decision 3: a new ADR, amending `recording.md` and ADR 0034, in place of the batching design's "no ADR" for B1.
- Decision 7: an unbounded wait at the end in place of a timeout.

## Closes

- `sdlc/issues/2026-09-26-usage-file-writes-serialize-requests-in-flight.md`. The lander moves it to `closed/` in the landing commit.

## Evidence

- Starts from: The issue above, filed from workspace experiment 268. At `--jobs 16`, 6 to 9 requests were in flight. A 100-title run made 200 usage updates and 400 flushes, at a median of 11.4 ms each, 4.4 s in all. A 306-title run took 6.6 to 7.2 s with the usage file on disk and 3.5 to 4.0 s with it in memory. Ticket 0063's precharge rule, `recording.md:22-26`, and the code named in "What happens today" at `origin/main` `b2d03a6f`.
- Keeps: The usage folder, its modes, its `.lock`, and its file format. Cross-process totals that sum exactly. The one warning after unchanged results, and the exit meaning. `thinkthen status` output. Memory-only counters on every library and SQL surface. Every `update()` stage check.
- Changes: `add` never waits on the disk. One writer thread per process writes coalesced deltas by month. `finish()` replaces `warning()` and waits for the writer. A crash can now undercount requests as well as tokens. ADR 0049 amends `recording.md:26` and ADR 0034's crash sentence, and retires ticket 0063's precharge. The deadline test's doc comment drops the usage lock. An issue asks the website agent to fix `backends.astro:48`.
- Proof: `requests_go_out_while_another_process_holds_the_usage_lock`. It counts 16 requests and a peak of 16 at the loopback listener while the test process holds `.lock`, then checks exact totals through `status --json`. Plants (a) today's synchronous write, (b) a dropped delta, and (c) an exit before the write each turn it red. The child must still be running while the lock is held. A loopback timing in the build record reports the speed and gates nothing.
- Defers: The `http.rs:127` comment, owned by 0142. The website sentence. A time bound on `finish()`. Fewer syncs a write. The inherited lock after a fork. The batching speed target, which needs B2 and batching.
