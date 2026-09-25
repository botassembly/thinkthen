# ADR 0043: PostgreSQL single-row calls stop by deadline, not cancel

Date: 2026-09-23. Status: accepted 2026-09-23. Amended 2026-09-24 by ticket 0111: a cancel now stops single-row calls and batches within one 50 ms tick, and the deadline stays the budget. Records the ruling that lived in
`sdlc/records/0070-the-postgresql-deadline-in-place-of-single-row-cancel.md`.
Record 0075 (item 2) listed it as awaiting an ADR, and the second review
asked for one (R2-29).

## Context

The extension's network call blocks a backend socket. `pg_cancel_backend`
and `statement_timeout` cannot interrupt it until the call returns.

## Decision

PostgreSQL bounds a single-row call with a deadline in place of a cancel.
The setting is `thinkthen.deadline_ms` (`-1` none, `0` spent, a positive
number a budget in milliseconds), read through the contract's checked
converter under ADR 0041. A spent budget refuses before sending with
SQLSTATE `57014` and `thinkthen deadline` in the message. The module
documentation, the README, and the setting's own description state that
a cancel does not reach a call already on the wire.

The surface stays deadline-only for single-row calls. An engine-side
interrupt that a cancel could reach is not planned. It becomes a ticket
when a caller needs to stop a single call with no deadline set.

## Consequences

Conformance case 27 runs on PostgreSQL through the deadline arm. Batches
read the same deadline (`databases/postgresql/NOTES.md` at tag `surfaces-wave7-final`,
"the batch deadline and the interrupt gate").

Ian can overturn this by asking for an engine-side interrupt. The cost is
a socket the extension can poll during the wait, the same shape the other
surfaces use for their tick.

## Amendment, 2026-09-24: ported to main, and the PostgreSQL surface ticket owns it

Ticket 0099 ported this ADR to main at its number from tag `surfaces-wave7-final` (`f6a7faea`). The ruling above stands as accepted. The PostgreSQL surface ticket (queue item 3 in `sdlc/planning/one-line-plan-2026-09-24.md`) owns it going forward. That ticket rebinds PostgreSQL to the public API and re-runs the deadline-arm conformance case against the real engine. It reads the budget through ADR 0041's owner on main. Ian can overturn this owner by moving the ruling to another ticket.

## Amendment, 2026-09-24: a cancel stops every call (ticket 0111)

The owner amends this ADR, and Ian can overturn the amendment. Record 0070 names no ruling by Ian.

- `pg_cancel_backend`, `pg_terminate_backend`, and `statement_timeout` stop a single-row call or a batch within one 50 ms tick. Every call runs on a detachable worker (0111 decision 7). The backend thread waits in ticks and reads `QueryCancelPending` and `ProcDiePending`. On either, it cancels the call's token and detaches the worker. `thinkthen.deadline_ms` stays the budget, and conformance case 24 still runs through it.
- Signals. The backend thread blocks every signal before it spawns a worker and restores its mask after. Every engine thread descends from a worker and inherits the full mask. PostgreSQL's cancel, alarm, and procsignal handlers therefore run only on the backend thread. This holds for batches and for the first engine build. The check's signal-mask test reads each thread's `SigBlk` line.
- Settings. The backend thread reads every setting and every named file before the spawn, because pgrx `GucSetting::get` panics off the backend thread.
- What a cancel does not stop. A cancel returns control to the session. A send already on the wire still completes and is billed, and its answer lands in the cache. Each detached send holds one throttle permit until it ends, and the engine's 30-second request timeout bounds that. Repeated cancels can hold at most the throttle's permits at once. A later call then waits at the throttle gate for at most 30 seconds, and its own deadline still bounds it.
- Proof. `databases/postgresql/check.sh` measures a single cancel, a statement timeout, a deadline, and a throttle-8 batch cancel, and `sdlc/records/0111-build-postgresql-surface.md` gives the results.
