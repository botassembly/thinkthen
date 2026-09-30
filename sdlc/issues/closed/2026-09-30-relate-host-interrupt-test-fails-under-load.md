# The relate host interrupt test fails under load

Status: closed by ticket 0342. Found by ticket 0341 on main `02750a206`. Owner: the queue owner.

Kind: debt

Pay when: before 0.1, or when it fails a landing's `test` run.

Debt: 023

Severity: medium

Paid: 2026-09-30

Keeping it lets `sdlc/scripts/test` fail at random and hide a real cancellation regression behind a rerun.

## The problem

`crates/thinkthen/tests/public_controls.rs` `a_host_interrupt_during_relate_chunks_sends_nothing_new` failed once in a full `sdlc/scripts/test` run. It fails with `60 entities`, `left: None`, `right: Some(Cancelled)`: the relate call finished without the interrupt taking hold. With main's code, one copy at a time under nextest's stress runs, it failed 1 of 50 at load 6 and 7 of 50 at load 13. The 0341 branch, whose change touches only the connection pool's idle age, failed 15 of 200 in the same runs.

The test's check returns true only while `backend.count() == 4`, and a helper releases the held replies 400 ms after the fourth request. This looks like the moving-count equality that ticket 0340 fixed in `public_controls/stopped.rs`: hold the backend, stop on `>=`, and require the exact count after.

## Resolution

The engine runs a host check only while no send of the call is out (`engine/mod.rs`, `stop_between_sends`). The check saw exactly four sends only when all four held replies finished before the host dispatched a fifth; under load one reply came back first, the fifth went out, and the check never matched. Ticket 0342 stops on four or more sends and asserts that nothing is sent after the stop, with exactly four for 40 entities.

