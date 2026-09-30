# 0340: Load-sensitive test flakes

Status: landed. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-1. Takes only the flakes below from ticket 0338's long waits; 0338 keeps the shared locks, `engine::deadline_tests`, `cli::schedule::width_tests` and merging binaries.

## Outcome

Five tests that failed once under load and passed on rerun prove the same behavior by order, events or counts. Wall-clock limits in the PostgreSQL check run only under the stress profile.

## Evidence

- Starts from: main `f9d310c06`; the open debt issues for the graded rank question file, the ordered output race, the PostgreSQL wall-clock limits and the DuckDB split denials; the `public_controls` `a_stop_during_a_batch…` failure named in the plan's slice 3e line. `single_cancel_within_200_ms` from slice 3e is the pattern.
- Keeps: every assertion's claim. The ordered output window, the stop that sends nothing new, the cache-lock wait that sends nothing, the PostgreSQL cancel, timeout, deadline and small-batch order and count proofs, and the DuckDB split-denial bodies and slot values. No cancellation or conflict regression is dropped.
- Changes: one item per flake.
  - `backend/keeping/graded_rank.rs`: `question()` writes a per-process name and renames it into place (cleanup lesson 5).
  - `backend/scheduling.rs` `ordered_output_bounds_every_dispatched_row`: waits on request events, not a 2 s spin. Before the release it checks that no sixth request started and no row was written. It reads no standard output until request 6 arrives, then checks with a zero-timeout `poll` that record 1's row already waits in the pipe. The command writes and flushes that row on the thread that later sends request 6, so the check cannot race.
  - `public_controls` `a_stop_during_a_batch…` moves to `public_controls/stopped.rs` to keep the file under 500 lines. Requests 1 to 3 answer one `round` at a time and request 4 stays held until the stop fires. The check stops at `count >= 4`, and the test still requires exactly 4 sends. The old check needed `count == 4` while every reply went free after a 400 ms sleep, so the count could pass 4 unseen.
  - `databases/postgresql/check.sh`: `dev_zero_refuses_fast`, `single_statement_timeout`, `single_deadline`, `batch_deadline`, `batch_cancel` and `a_small_batch_answers_at_once` record their time. A `STEP_within_N_ms` twin reruns the step and checks the limit, and `check` runs every such twin only under the stress profile. `a_small_batch_answers_at_once` now also checks that the pair took one request.
  - `databases/duckdb/tools/verbs_budget.py` `PackedReplies` sends `Connection: close`. The cause was a dependency bug, filed as `sdlc/issues/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`.
- Proof: runs under load, before and after, below. A planted window of 6 turns the ordered output test red; a planted ignored interrupt in `public/pull.rs` turns the stop test red. `sdlc/scripts/test`, `spec`, workspace clippy, `policy.py`, `tickets`, `lint` in a clean checkout, and the PostgreSQL and DuckDB checks, routine and stress for PostgreSQL.
- Defers: the ureq connection reuse itself and the other Python HTTP/1.0 fixtures, in the new issue. The full `test-stress --run` runs every surface under stress; this ticket ran only the PostgreSQL stress profile, which holds every moved claim, since the coordinator asked for no every-surface sweep.

## What the build taught us

Each count ran the old and new test binaries or scripts in the same way, at 1-minute loads of 11 to 30. The first rounds ran beside a scratch release build; the coordinator then asked for smaller runs and no added load.

| Flake | How it ran | Before | After |
| --- | --- | ---: | ---: |
| graded rank question file | the four tests, two copies each, at once | 35 of 800 failed | 0 of 800 |
| ordered output | 16 copies at once | 3 of 2,400 failed | 0 of 2,400 |
| `public_controls` stop during a batch | 4 copies at once, and 1 beside the other tests | 2 of 400 failed | 0 of 200 |
| PostgreSQL time limits | the six timed steps, alternating old and new | 1 of 20 (`batch_cancel`, 270 ms) | 0 of 20 |
| DuckDB split denials | 8 copies at once | 11 of 200 failed | 0 of 30 |

- The DuckDB case had a real cause. Python's `http.server` answers over HTTP/1.0 and closes each connection without saying so. ureq-proto 0.6.4 pools the connection anyway, and the next send failed when the close arrived late. A scratch copy failed 10 of 480 runs as written and 0 of 480 with `Connection: close`. Every failure named the alpha half "the backend did not answer" and left no second body at the listener.
- An equality on a moving count (`count == 4`) is a race. Hold the backend, let rounds go one at a time, and stop on `>=`.
- A test's reader thread adds its own delay between the command's write and the test's event. Leave the pipe unread and poll it at the moment that matters.
- The PostgreSQL routine run still has 88 steps. The stress run has 8, and all passed at load 18.
- `sdlc/scripts/test` passed 1,293 tests; `spec` passed. The crate ratchet rose 49 lines and the DuckDB Python ratchet 4, as the commits say.
