# 0628: Merge the two session types into one bounded queue

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Reviews: revision 1680ce6, accept

## Outcome

- One session type owns intake, output, cancellation and settlement under one lock, or on one well-tested channel. Recovery is a mode of that type, not a second type.
- `recovering_session.rs` and its separate wake logic are deleted. The session keeps one internal wait contract, the four interests and the arm-before-check rule. Hosts never see it: they call the run function from 0629 or the run call from 0638.
- No session operation sleeps or polls on a timer.

### Proof

- On the M5 through pm job, the installed Rust library consumer runs the session checks for both ordinary and recovering calls, and the installed C library runs them for ordinary calls (the C interface exposes no recovering session today; check before relying on one): bounded push-all, try-push, cancel before wait, cancel during a held provider, and draining settlement after cancel. The C checks keep the documented lifetime contract: cancel, then join every wait, then free. Dropping the owner while a wait is active is checked only in Rust, where ownership allows it. Any new C lifetime design belongs to 0638.
- A repeated stress run of those checks (`sdlc/scripts/test-stress`) finishes with no timeout or hang.

## Evidence

The core has two session types, each with its own lock and wake-up code: `crates/thinkthen/src/public/request/session_queue.rs:34-35,85-86` and `crates/thinkthen/src/public/request/recovering_session.rs:8,53-54`. Both are hand-written with condition variables. Wake-up and deadlock bugs were still being fixed days before the 0.2 cut:

- c093f72a4 (2026-10-14) replaced a 50 ms polling timer with a shared change generation, to close a race between two independent queues.
- b994054ff (2026-10-14) fixed the order of arming a wait and checking queued output.
- 8764d374f (2026-10-15) fixed producers waiting forever on a reader.
- f01ef4cd7 and a15757cf8 (2026-10-15) adjusted session regression scheduling.

Two locking designs that must behave the same will keep producing bugs of this kind.

## Progress

- 2026-10-18 started

## Amendment, 2026-10-18

The C proof keeps the documented lifetime contract (cancel, join, then free) instead of freeing during an active wait; live-owner and wait-lifetime checks stay in Rust. 0628 does not change the C contract.
