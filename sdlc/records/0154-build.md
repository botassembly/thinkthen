# 0154 build record

## Preflight, 2026-09-27

The accepted ticket branch was merged with main `c76de10d` before source work. Tickets 0146 and 0155 are present. ADR 0051 sets the behavior; the later user ruling calls for focused functional proof and a related command checkpoint instead of the old full gate and mutation campaign.

The existing `core/batch/tests.rs` boundary establishes that three 20,000-byte records fit one 96,000-byte batch, while three 40,000-byte records close as two and one. The ticket's edge table now uses the latter. The existing batch and relation planners, command batching worker, listener and golden tests are the reuse points. The default retry count and live attempt counter from 0155 must remain intact. Ticket 0201 owns `engine/mod.rs`, `engine/workers.rs` and `public/options.rs`; this build must avoid them and merge the shared settings page second.

The earlier committed recording scan found no recording over 90,000 bytes in the named demo, probe, transform and test fixture trees. No prototype bytes or checksums are planned to change. Any executable fixture that changes under the new default must be identified before widening scope. The smallest current proof targets are the batch planner test, backend batching command tests, relate ceiling tests and status reason tests. No live provider call is authorized.

## Build and verification

Pending implementation.
