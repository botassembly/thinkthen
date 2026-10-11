# 0555: Run core limit cases once and a representative subset per binding

Status: OPEN.

Milestone: 0.2

Depends on: 0549

## Outcome

The cli, rust and c consumers run every required case. Each other consumer runs the cases that cross its own binding code, plus a few representatives of each core limit and refusal group. `parity.py` checks that each binding's subset agrees with the reference consumers.

## Evidence

- Starts from: [Ian's 2026-10-11 approval](../decisions/2026-10-11-efficiency-approvals.md) and the test section of the [adversarial review](../records/2026-10-11-0-2-adversarial-review.md).
  - Of the 258 required cases, 96 are named-input cases and 24 are image-admission cases.
  - Most of them test refusals and byte limits that Rust admission enforces. A binding only passes these cases through.
- Keeps:
  - Every case in cli, rust and c.
  - In each binding, every case that exercises its own conversion or decoding: host value types, images, files, feeds, cancellation, errors, usage and each of the ten functions.
  - The routine profile.
- Changes:
  - Give each case a tier in `conformance/` data: core cases, or binding cases. The reference consumers run both tiers.
  - The other consumers run the binding cases plus a fixed sample of the core cases. Each limit or refusal group in the sample has at least one case for each failure kind.
  - Update `parity.py` and its checks so that a binding missing a binding case still fails.
- Proof:
  - `parity.py` tests cover the tier rule.
  - One full campaign passes, and its summary shows the per-consumer durations next to those of the first 0549 run.
- Defers: nothing.
