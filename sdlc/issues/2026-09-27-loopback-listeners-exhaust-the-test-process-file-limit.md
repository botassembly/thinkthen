# Loopback listeners exhaust the test process's file limit

Status: open. Verified 2026-09-27 while checking `ece8b1f1`; its listener source matches main at `95f0458b`. Component: Release and tooling. Severity: 3.

## Evidence

The test rung ran with a soft open-file limit of 1,024 and a hard limit of 524,288. The shared backend test process passed 297 of 399 cases, then 102 cases failed while opening sockets or starting commands. All 102 failure sections contain `Too many open files`. The log is `target/codex-logs/postgresql-qf-test.log` in the Codex lane.

A second full test rung passed with a child-only soft limit of 4,096. Sampling that backend test process observed 1,275 open descriptors. The retry and sample are `target/codex-logs/postgresql-qf-test-nofile4096.log` and `target/codex-logs/fd-diagnosis/full-test-fd-sampling.json`. This higher limit allows the unrelated PostgreSQL check fix to be verified; it does not bound the fixture's lifetime.

`conformance/backend/src/listener.rs:196` explicitly keeps the port until process exit. `Listener::serving` and `answering_observed` move their sockets into detached accept loops. The fixture has no shutdown on drop, so completed cases keep sockets and threads in the shared test process.

## Retained behavior

Ticket 0127 deliberately kept scripted ports open so a late connection could not consume another test's reply. It also retained extra-connection diagnostics. [The earlier harness issue](2026-09-25-test-harness-and-review-leftovers.md), section 4, records that evidence. A fix must preserve isolation between tests and those diagnostics; simply closing a port when its response script ends would undo that correction.

## Outcome and proof

Give fixtures a bounded, safe lifetime so completed work does not accumulate listening sockets or detached threads. Verify repeated fixture creation and retirement, late connections, and held responses through the real loopback boundary. The full backend suite must pass at a 1,024-file soft limit with bounded descriptor and thread counts. Raising the gate's default limit alone does not close this issue.

Ticket 0146 opens `crates/thinkthen/tests` but does not open `conformance/backend`. Claim the exact fixture and proof files before building, and merge any new caller changes first. This issue authorizes no implementation by itself.
