# The recording page omits the exit wait on the usage lock

Status: closed on 2026-09-27. Ticket 0163 states the exit wait in recording.md. The runtime wait remains open as register 31 in the work plan. Reviewed source: `0292cba2`; proof: `sdlc/records/0163-code-review.md`.

## What happens

The command's exit waits for the usage writer. The writer waits for the usage lock with no bound. So while another process holds the lock, `if thinkthen decide ...` has its answer and cannot exit. The report held the lock from outside for 12 seconds. `decide` printed `true` at once and exited after 11.73 seconds. A suspended ThinkThen that is mid-write, or a slow network home folder, can hold the lock.

ADR 0049 item 3 accepts this on purpose: "It waits as long as another process holds the lock." Ticket 0141 decision 7 agrees. `specification/recording.md` line 26 says only that "Counting never holds back a request", which a reader takes to mean counting never delays anything.

## Checked on main

Verified: ADR 0049 item 3 and `recording.md:26` read as quoted, and `crates/thinkthen/src/engine/usage.rs:263` takes the lock with `File::lock`, which has no timeout. The 11.73-second run comes from the report.

## What would fix it

Add one sentence to `recording.md` beside line 26: the command's exit waits for the usage write, and it waits as long as another process holds the usage lock. Cite ADR 0049 item 3.

## Done when

`recording.md` states the wait.
