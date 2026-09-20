# The live budget check does not enforce the limit

Status: Closed by ticket 0028

Found 2026-09-20 in the full-project review at `2c32524` and confirmed after ticket 0027.

`sdlc/scripts/live` refuses only when the recorded spend has already reached the limit. A run may start one token below the limit and spend any amount. The ledger then records a total above the approved limit after the paid calls have happened.

Two live scripts can also read the same starting spend, run together, and each replace the ledger with its own total. That can both exceed the limit and lose one run's spend. The script uses the shared fixed name `live-tokens.new`, which adds another collision.

The live door needs a declared maximum for each job and exclusive ownership of the ledger from the check through the final update. No gate may make a live call while proving this.

Ticket 0028 requires and permanently precharges that maximum before the child starts. An atomic directory lock serializes the ledger through child completion, and local-only tests prove the paid-work boundaries, interruption, and recovery states.
