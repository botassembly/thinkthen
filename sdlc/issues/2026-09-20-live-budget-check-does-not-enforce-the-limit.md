# The live budget check does not enforce the limit

Status: Partly closed by ticket 0028; reopened for ticket 0034

Found 2026-09-20 in the full-project review at `2c32524` and confirmed after ticket 0027.

`sdlc/scripts/live` refuses only when the recorded spend has already reached the limit. A run may start one token below the limit and spend any amount. The ledger then records a total above the approved limit after the paid calls have happened.

Two live scripts can also read the same starting spend, run together, and each replace the ledger with its own total. That can both exceed the limit and lose one run's spend. The script uses the shared fixed name `live-tokens.new`, which adds another collision.

The live door needs a declared maximum for each job and exclusive ownership of the ledger from the check through the final update. No gate may make a live call while proving this.

Ticket 0028 requires and permanently precharges that maximum before the child starts. An atomic directory lock serializes the ledger through child completion, and local-only tests prove the paid-work boundaries, interruption, and recovery states.

The follow-up at `a556b97` found that `sdlc/scripts/live` derives both the ledger and lock from its own checkout. Registered linked worktrees hold separate copies of the full allowance. The same-checkout concurrency tests do not cover this. Its child wait also has no deadline, and manual recovery cannot clear an active hung child. No paid call was made to inspect either path.

Ticket 0034 moves authority outside tracked worktree files into one shared Git common directory and replaces the FIFO supervisor with bounded interruption and checked recovery. Its real two-worktree test must prove combined reservations against one allowance. `annotate` remains the next product command; its paid measurements wait for this repair and the audited migration.
