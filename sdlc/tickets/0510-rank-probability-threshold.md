# 0510 — rank-probability-threshold

Status: OPEN.

Milestone: 0.2

Depends on: 0475
Depends on: 0476

Reviews: revision 1d15f4c6f, accept

## Outcome

Callers can use `rank --threshold P` to retain records whose yes probability is at least P, in the existing ranked order. Omitting the option preserves current rank behavior. The Rust binding exposes the same reading without making another model call.

## Evidence

- Starts from: Ian's 2026-10-09 ruling in `inbox/thinkthen/2026-10-09-pm-thinkthen-ian-approves-rank-threshold-for-0-2.md` and the retained [2026-10-01 issue](../issues/2026-10-01-rank-keeps-only-records-over-a-threshold.md).
- Keeps: The ten functions, default uncut rank, stable ties, original records and positions, existing model questions, request counts, complete observations, and plain rank's cache/replay answers. The cutoff is a reading of stored probabilities, not question wording or routing.
- Changes: Amend `specification/rank.md`, `specification/threshold.md` and ADR 0007's no-selection consequence under Ian's ruling. Admit an optional inclusive single cutoff with the existing finite domain `0 < P <= 1`; reject bands. Apply it to plain and saved decide rank through the CLI and Rust reading. Filter below-cut records before presenting the ordered output; combine with top without changing the eligible order. Omission preserves existing behavior. Reject an authored cutoff on saved-score and question-set routes before sends; preserve their existing uncut behavior without treating a weighted score as a probability. Claim `crates/thinkthen/src/core/order.rs`, `crates/thinkthen/src/cli/args.rs`, `crates/thinkthen/src/cli/asked/rank.rs`, `crates/thinkthen/src/cli/schedule.rs`, `crates/thinkthen/src/public/bulk/complete.rs`, `crates/thinkthen/src/public/complete/rank.rs`, `crates/thinkthen/tests/backend/keeping/rank_top.rs`, `crates/thinkthen/tests/backend/native_results/rank.rs`, `crates/thinkthen/tests/public_batches/ranks.rs`, `crates/thinkthen/tests/native_complete/aggregates.rs`, `specification/rank.md`, `specification/threshold.md` and `sdlc/planning/adr/0007-flat-verbs-bare-values-and-one-threshold.md`. Confirm the smallest public reading seam before coding; change claims if it needs other files, without claiming all core code.
- Proof: Outside-in CLI and Rust cases retain an exactly-at-cut record, drop below-cut records, preserve probability order and stable ties, compose with top, and reject invalid cutoff input before sends. Run plain rank first and the cut reading against the same cache or recording; count zero additional model requests and check the same underlying question/cache identities. With no cutoff, retain existing literal output and error cases.
- Defers: A new function, grep alias, proxy business policy, calibration, extra model calls and release management. Build after 0475 and 0476, before final installed cross-surface qualification.

## Shared request amendment

Ian's ruling also covers every migrated surface through the shared Request. Admit the optional rank cutoff in Request options and native admission, apply it through the same rank reading, and generate its request schema. MCP and language adapters forward the contract instead of implementing cutoff logic themselves. Preserve the same saved-score and question-set refusals before sends.

Extend the narrow claim to `crates/thinkthen/src/public/request/options.rs`, `crates/thinkthen/src/public/request/admission.rs`, `crates/thinkthen/src/public/request/execution.rs`, `crates/thinkthen/src/public/request/tests.rs`, and `specification/request.schema.json`. If authored definition admission needs a change, name its exact file before implementation. Keep the CLI and Rust outside-in proofs, and add one MCP or Python contract case proving the cutoff and zero extra requests through the generated path. Select its existing test file before implementation and add that file to the claim.

This amendment needs a fresh ticket review before coding. It follows ask 3 of the rank-threshold approval message and keeps the dependency order unchanged.
