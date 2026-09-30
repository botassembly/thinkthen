# The PHP check waits on a lock its caller holds

Status: fixed on branch `ticket/0314-s4-remaining-ports` by `03e9a767a` (ticket 0314 slice 4e); the lander closes it. Each of the three `flock` calls now runs through a `locked` helper that skips the lock when `THINKTHEN_HEAVY_LOCK_HELD` names it, and the PHP check passes under `surface-one.sh`. Found while building ticket 0314 slice 4d on the same branch. Owner: ticket 0314 slice 4e, which rebuilds the PHP port.

Kind: debt

Pay when: before the next checkpoint tag, which needs every surface green on one commit.

Keeping it means `surfaces` reports PHP as failed after a three-minute wait, so a break in it can land unseen.

## The problem

`sdlc/scripts/surfaces` runs each check under the lane's heavy lock and exports `THINKTHEN_HEAVY_LOCK_HELD`. `libraries/php/check.sh:45`, `:68` and `:70` take `flock -w 180` on `THINKTHEN_HEAVY_LOCK` again, so each waits 180 seconds and the check exits with no test run. Ticket 0304 recorded this for PHP, Ada, Objective-C and COBOL. Slice 4d fixed Objective-C, COBOL and Ada: each skips its own flock when `THINKTHEN_HEAVY_LOCK_HELD` names the lock, as `sdlc/scripts/heavy-lock` does. PHP takes the lock around single commands, so each of its three `flock` calls needs the same test.
