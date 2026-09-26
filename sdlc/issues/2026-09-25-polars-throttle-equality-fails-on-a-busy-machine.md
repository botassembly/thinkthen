# The Polars throttle-equality check fails on a busy machine

Status: Open

Filed on 2026-09-25 at the landing of ticket 0128 Phase 1. `crates/thinkthen/tests/polars/throttle_equality.rs:89`, `a_series_runs_at_the_throttle_as_a_slice_does`, failed in the surfaces rung: `series 3.212053481s and slice 3.054558941s differ by more than 5 percent`. Other builders' rungs were queued on the heavy lock. The same test passed in the surfaces runs just before on the same code. It also failed once in a cloud container, as ticket 0128's record notes. Ticket 0128 does not touch Polars.

The test guards a real regression: a Series call that runs at a different throttle than a slice call. A 5 percent wall-clock bound on two back-to-back runs of about 3 seconds fails whenever the scheduler slows one of them.

## Fix

Judge the throttle by what the loopback backend saw, not by the clock. Count the peak number of requests in flight for each call and require the two to match. If a time bound stays, run each call twice and compare the faster runs, with a bound that holds at a load of 10. `sdlc/issues/2026-09-25-postgresql-warm-rows-timing-fails-on-a-busy-machine.md` has the same shape, and one Quick Fix can settle both.
