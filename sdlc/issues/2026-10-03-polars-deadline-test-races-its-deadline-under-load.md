# The Polars deadline test races its own deadline under load

Status: open. Filed 2026-10-03 from the surfaces checkpoint sweep of 2026-10-03 on `release/0.1`. Owner: the queue owner.
Milestone: 0.2
Kind: debt
Debt: 036
Severity: low
Pay when: the next checkpoint sweep, or a second failure.

A flaky test teaches people to rerun failures instead of reading them.

`deadline::a_deadline_stops_a_score_column_mid_batch` in `crates/thinkthen/tests/polars/deadline.rs` failed once during that sweep and passed on rerun. The sweep ran many suites at once on one host.

The child failed at `deadline.rs:46`, `backend.wait(2)`, "the second singleton reached the held arm", after 30.97 s. The parent then failed at `common/mod.rs:106`.

The likely cause: the call's deadline is one second from the start. The test assumes the engine sends the second singleton within that second. On a loaded host the deadline can pass before the second send, so the engine stops correctly after one request and `wait(2)` times out. The engine behaved as designed. The test's timing is the defect.

A fix makes the test independent of host speed. One option: set the deadline after the second singleton reaches the held arm, or hold the clock with a longer deadline and release past it. The test should still prove that the third singleton is never sent.
