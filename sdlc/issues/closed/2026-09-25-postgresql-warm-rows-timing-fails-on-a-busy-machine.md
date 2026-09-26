# The PostgreSQL warm-rows timing check fails on a busy machine

Status: Closed on 2026-09-26 by Quick Fix `qf-flaky-gate-tests` (record `sdlc/records/qf-flaky-gate-tests.md`).

Filed on 2026-09-25 at the landing of ticket 0130. `databases/postgresql` check `twenty_thousand_warm_rows` failed once in the surfaces rung with `took 31733 ms, over 30000 ms`. The Beelink's one-minute load was about 6, with other builders' rungs queued on the heavy lock. The same commit passed that check on the next surfaces run. Ticket 0130 does not touch PostgreSQL.

A wall-clock ceiling on a shared machine fails for reasons outside the code. The check guards a real regression: warm rows that each send, or that reread the cache per row.

## Fix

Keep the guard and drop the wall clock as its judge. Count the backend requests for the warm pass and require zero. If a time bound stays, measure it against a cold pass in the same run and require a ratio, not a fixed 30 seconds. The fix needs a Quick Fix or a ticket in the PostgreSQL surface.
