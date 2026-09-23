# ADR 0043: PostgreSQL single-row calls stop by deadline, not cancel

Date: 2026-09-23. Status: accepted. Records the ruling that lived in
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
read the same deadline (`databases/postgresql/NOTES.md`, "the batch
deadline and the interrupt gate").

Ian can overturn this by asking for an engine-side interrupt. The cost is
a socket the extension can poll during the wait, the same shape the other
surfaces use for their tick.
