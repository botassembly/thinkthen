# ADR 0044: The C lane edited two comments in the contract header

Date: 2026-09-23. Status: accepted. Records the permission that lived in
`sdlc/records/0071-the-two-narrow-exceptions-granted-in-the-review-waves.md`
(item 1). The second review asked for an ADR (R2-29).

## Context

The C lane did not touch `contract/`. The header promised that an error
message "lives until the next call on the same engine". The C door moved
to per-thread error slots, which made the promise false.

## Decision

The C lane received permission to edit two comments in `contract/include/thinkthen.h`. It edited exactly two comment
locations in the contract header: the lifetime promise and the
`thinkthen_error_message` doc comment. A diff check proved the change
touched comments only. It landed in `be8374f`.

The per-thread slot is the C door's error shape. A message lives until
the same thread records its next failure on that engine, or until the
engine is freed, as the header and `libraries/c/DESIGN.md` state. A per-engine slot comes back
only with a ticket that names the caller who needs it.

## Consequences

The header states the per-thread truth. Any other change to `contract/`
from a surface lane still needs its own recorded permission.

Ian can overturn this by ruling a per-engine slot. The cost is the
use-after-free hazard `libraries/c/DESIGN.md` records for the old shape,
and a new lock.
