# The PHP and Ada checks wait on a lock their caller holds

Status: open. Found while building ticket 0314 slice 4d on branch `ticket/0314-s4-remaining-ports`. Owner: none yet.

Kind: debt

Pay when: before the next checkpoint tag, which needs every surface green on one commit.

Keeping it means `surfaces` reports PHP and Ada as failed after a three-minute wait, so a break in either can land unseen.

## The problem

`sdlc/scripts/surfaces` runs each check under the lane's heavy lock and exports `THINKTHEN_HEAVY_LOCK_HELD`. `libraries/ada/check.sh:14-15` and `libraries/php/check.sh:45`, `:68` and `:70` take `flock -w 180` on `THINKTHEN_HEAVY_LOCK` again, so each waits 180 seconds and exits with no test run. Ticket 0304 recorded this for PHP, Ada, Objective-C and COBOL. Slice 4d fixed Objective-C and COBOL: each skips its own flock when `THINKTHEN_HEAVY_LOCK_HELD` names the lock, as `sdlc/scripts/heavy-lock` does. The same change fits Ada. PHP takes the lock around single commands, so each of its three `flock` calls needs the same test.
