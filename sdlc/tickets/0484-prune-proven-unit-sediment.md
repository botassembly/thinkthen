# 0484 — prune-proven-unit-sediment

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, accept

## Outcome

Remove tests that merely repeat constants or implementation steps, while keeping distinct behavior and edge-case coverage in one useful outside-in set.

## Evidence

- Starts from: architecture/test-sediment PM message of 2026-10-08, ask 1; start with core/result/tests.rs, threshold.rs, text.rs, answer_tests.rs, systemone/request_tests.rs, digest vectors and cli/schedule/width_tests.rs. The 32-test sample does not establish 220 removable tests.
- Keeps: Parser negatives, secrecy, cancellation/concurrency, memory safety, threshold edges and cache/version compatibility vectors that catch real behavior drift. Do not delete a digest vector merely because it pins a string.
- Changes: Inspect each candidate and name its observable behavior and existing outside-in replacement. Delete only proven tautologies/duplicates; add a small missing behavior case first when needed. Consolidate duplicate assertions without hiding distinct edge cases. Lower only measured source ratchets after removals. Report actual unit-test and source counts before/after with the counting method, not an extrapolation.
- Proof: Reviewer verifies every proposed deletion against the retained behavior set. Run affected behavior checks and ordinary applicable tests/lint. Keep one concise ticket record, no per-test receipts or new counting gate.
- Defers: Broad test architecture and unverified sample-based deletion targets. Coordinate file ownership with active recognition/C fixes.
