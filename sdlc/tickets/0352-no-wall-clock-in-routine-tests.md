# 0352: No wall-clock timing in routine tests

Status: in progress. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-1.

## Outcome

No routine rung (`test`, `spec`, the surface checks) depends on how fast the machine runs.

This ticket delivers it for `test`, `spec`, and the Python, DuckDB and SQLite surface checks. The other binding checks remain as Debt 027.

## Evidence

- Starts from: ticket 0340, which fixed the first load flakes, and the closed issues it and later tickets settled. These are `2026-09-24-parallel-lock-test-fails-under-load.md`, `2026-09-24-the-global-queue-concurrency-check-fails-under-load.md`, `2026-09-24-the-model-mismatch-cancel-check-fails-under-load.md`, `2026-09-30-duckdb-split-denials-case-failed-once-under-load.md`, `2026-09-30-ordered-output-test-races-the-next-request-under-load.md`, `2026-09-30-process-cap-test-races-two-records.md` and `2026-09-30-relate-host-interrupt-test-fails-under-load.md`. Ticket 0342 narrowed the relate interrupt race and did not remove it. Ticket 0351 recorded one timing test that failed once under load. Python and DuckDB surface checks passed only on a rerun. A loaded baseline on main failed 1 of 5 runs of `sdlc/scripts/test`, in the batching ceiling test.
- Keeps: every cancellation, deadline, pacing and ordering regression. Each moved millisecond promise keeps a stress twin under `test-stress --run`. The relate stop while four sends overlap keeps its old 60- and 40-entity rows as the stress case `overlap::a_host_interrupt_while_relate_sends_overlap_stops_between_four_and_eight`. The port twins run under `THINKTHEN_TEST_PROFILE=stress` through `surfaces --stress`. Python selects them with the `stress` mark. DuckDB's `harness.py` selects them by name. SQLite's `check.sh` selects them in `test_interrupt.py`, `test_usage.py` and `test_try_budget.py`.
- Changes: the rules and the list below.
- Proof: `sdlc/scripts/test` and the Python, DuckDB and SQLite surface checks under 24 busy processes, before and after, in the tables below. `test-stress --run` passes on a quiet machine. `spec` runs replayed document examples and two probes. Its only timed wait is the triage demo's file wait, which this ticket raises to 30 s. A search of `spec`'s inputs found no other sleep or elapsed-time check.
- Defers: the other binding checks, filed as Debt 027. The C door tests, which ticket 0346 owns. The engine's 50 ms input pause, which a stalled reader thread can still reach, filed as Debt 028. The margins named under Exceptions.

## Rules

- A routine test proves order with an event: a channel, a barrier, a counted request, a held reply, or a file the child writes.
- A promise in milliseconds, such as "cancelled within 100 ms", runs only under `THINKTHEN_TEST_PROFILE=stress`. The routine run proves the same stop by order: the cancel arrives while the reply is still held.
- A wait for something that must happen gets a guard of 30 to 60 s. A passing run never waits it out.
- An upper bound stays only as a hang guard of at least 10 s, and only where the behavior it rules out takes 30 s or more. Every shorter bound either becomes a guard of 30 to 60 s or moves to stress.
- A lower bound stays. So does a wait for something that must not happen. Load can only make either pass more easily.
- A deadline that the test expects to fire stays. Load can only make it fire sooner. The work it cuts off is held, waits 30 s, or waits longer than the deadline, as the column test's 100 ms replies outlast its 50 ms deadline.

## Exceptions

A few tests still need in-process work to finish inside a margin. No outside event marks that work. Each margin covers only work inside one process and loopback round trips, and a failure needs a stall of the whole margin.

- The ordered-schedule deadline tests: 300 and 500 ms deadlines that must outlast emitting one record.
- The Polars deadline test: a 1 s deadline that must outlast one held-arm round.
- The HTTP retry deadline test: a 1 s deadline that must outlast one loopback send.
- The width test's 400 ms wait for a permit that is already free.
- The annotate equal-records test: the reply waits 1 s while the engine reads a line already in its pipe.
- SQLite's two-row budget test: a 700 ms budget that must outlast one held-arm round and the second send.

## Changes

Conformance backend:

- The `wait` line waits up to 30 s instead of 5 s (`WAIT_BOUND` in `conformance/backend/src/arms.rs`). The README, the Python `conftest.py` and the Ruby helper say so.
- `binary.rs` splits its bound in two. Promptness checks use 10 s, and waits for a line use 40 s. `eight_delayed_replies_wait_in_parallel` becomes a stress case. A new routine case, `eight_held_requests_are_counted_while_every_reply_is_held`, proves the same parallel serving by order. The 30 s wait case is renamed and moves to stress. `listener.rs` waits up to 20 s.

Rust engine unit tests:

- The store test sets a flag just before COMMIT and checks it, instead of timing 250 ms.
- Hang guards rise to 30 s in the process, fork, facade, width, host-signal, ordered-schedule and interrupt tests. Client timeouts in tests whose subject is not the timeout rise to 30 s.
- Two HTTP retry tests wait 30 s between tries. The deadline test uses a 1 s deadline instead of 200 ms.
- The held-response deadline test waits 30 s for its request, and its client timeout rises to 30 s.
- The accounting deadline test keeps a 10 s hang guard. Its accept check proves that no attempt went out.

Rust command tests (`tests/backend`):

- The batching ceiling test reads its 120 KB input from a file, through a new harness call `spawn_file`. A pipe filled past its buffer let the writer stall, and the 50 ms input pause then closed a batch early.
- A new harness helper `Tally` counts written replies, so one reply can wait until another is written. The annotate failure-order test and the batching failed-request test use it.
- The annotate closed-pipe test holds records 2 and 3 at a rendezvous until the test has closed the output pipe.
- The parallel tests gather the first requests before answering. The peak test now asserts the full bound. The resume test holds the first run's four replies until all four are in.
- The annotate stop test gathers records 1 and 2 before record 1 fails. The equal-records test gives the second record a full second to join the first one's request.
- `cache_convert` measures its lock wait from the lock itself.
- The retry-floor test keeps its lower bound. Its 3 s upper bound moves to the stress case `exchange::a_server_retry_floor_is_waited_once`.
- The exchange and resend tests hold replies for 60 s or ask for 30 s waits, so a 10 s or 30 s hang guard separates the two outcomes.
- Other positive waits rise from 2 to 5 s to 30 s: interrupt, rank-top, scheduling, closed pipe, usage lock, default-cache usage and parallel.

Rust library tests:

- The relate host-interrupt test runs at throttle 1 in a fresh copy of the test binary. The check fires at exactly four sends, and nothing more is sent. At throttle 4, overlapping replies could leave no moment with no send out, so the check could miss every chance. The throttle-4 form, with both its 60- and 40-entity rows, moves to `public_controls/overlap.rs` as a stress case. It also runs alone, since another row's explicit throttle would narrow it.
- The profile-split test holds the alpha reply until beta's is written. The order test holds each slow reply until the fast one beside it is written.
- Waits in the cap, estimated, batch attempt, interactive and native tests rise to 30 s. `public_controls.rs`'s shared `BOUND` rises from 3 s to 30 s. It guards the fired-check and call-facts waits, and the deadline parsing rows only need a valid value. The retry-after tests ask for 30 s and allow 10 s.

Consumer:

- The fork probe's child waits for a file that the parent writes after the waiter returns, instead of sleeping 8 s. The waiter's bound becomes a 30 s hang guard.

Python, DuckDB and SQLite surfaces:

- A stop test's 100 ms bound applies only under the stress profile. The routine run checks that the cancel arrived while the reply was held.
- The Python release test drives the held arm from the parent. Its old timer form, where a cancel can land before the send, stays as a stress case. The column-timing test splits into a deadline half and a held-token half. The token fires once the held arm counts a send. At most eight sends go out, and none after the cancel.
- DuckDB's `harness.py` adds `timed` cases, which run in both profiles and time only under stress. The bridge case repeats SIGINT until the answer arrives. The between-queries case asks the child how many queries it has seen instead of sleeping 20 ms. The queued-relate case waits for a stdin line instead of sleeping.
- SQLite's conformance and interrupt tests wait on held requests. Where a timer releases the held reply after 30 s, the routine run keeps a 10 s hang guard on the stop, so a stop that waited for the reply still fails. `check.sh` runs the usage and try-budget files under stress too.
- Waits rise to 30 or 60 s across these tests.

Shell self-tests:

- The triage demo, `sdlc/live-test`, `sdlc/scripts/smoke` and `sdlc/scripts/installed.sh` wait up to 30 s for a file or a port.

## Proof

Each row runs while 24 busy loops run. Each session started below a one-minute load of 8.

| Code | `sdlc/scripts/test` runs | Failed | Load during runs | Failures |
| --- | --- | --- | --- | --- |
| main, earlier baseline | 5 | 1 | 32 to 47 | batching ceiling |
| main's test files | pending | pending | pending | pending |
| this branch | pending | pending | pending | pending |

| Code | Surface check | Runs | Failed |
| --- | --- | --- | --- |
| main's test files | Python, DuckDB, SQLite | pending | pending |
| this branch | Python, DuckDB, SQLite | pending | pending |
