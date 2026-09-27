# Register 76: detached sends retain their throttle permits

Status: non-issue, 2026-09-27. Checked against main `ed56d989`. Ian can overturn this disposition.

## Decision

A sent request keeps its throttle permit until that attempt ends, even when its caller has stopped waiting. This is correct: releasing the permit while the request still runs would let later calls exceed the selected concurrency width. Register 76 needs no code fix and no detached-work counter. Ticket 0168 added a clarifying settings sentence; that page change did not fix this finding.

## Evidence

Local experiment 284, file 76, describes a detached SQLite worker holding its permit and proposes a counter. The current send path already bounds that work:

- `crates/thinkthen/src/engine/mod.rs`, `Widths::acquire`, admits an attempt only while `active < width`, then increments `active`. A cancelled waiter returns before taking a permit. `Permit::drop` decrements `active` and wakes waiters.
- `crates/thinkthen/src/engine/http.rs`, `Http::post_observed`, holds the acquired permit across `send` and drops it only after that send returns. Each send has the configured timeout, further bounded by the remaining call budget. A detached worker cannot release its place merely because the caller returns.
- `crates/thinkthen/tests/public_controls.rs`, `a_stop_at_the_throttle_gate_sends_nothing_new_and_sent_work_finishes`, holds four sends, cancels later callers at the throttle gate, releases the held sends, and asserts exactly four requests reached the loopback backend. This regression passed in the combined ladder recorded by `sdlc/records/0168-build-a-stop-reaches-every-caller.md` at source commit `3b3918e7`; the relevant source is unchanged on `ed56d989`.

The ruling preserves the sent-attempt behavior in ADR 0017. Cancelling an already-sent socket request would be a separate contract change, not evidence that the existing throttle fails to bound work. The umbrella library/database issue remains open for its other findings.
