# 0510 — rank-probability-threshold

Status: OPEN.

Milestone: 0.2

Depends on: 0475
Depends on: 0476

## Outcome

Callers can use `rank --threshold P` to retain records whose yes probability is at least P, in the existing ranked order. Omitting the option preserves current rank behavior. The Rust binding exposes the same reading without making another model call.

## Evidence

- Starts from: Ian's 2026-10-09 ruling in `inbox/thinkthen/2026-10-09-pm-thinkthen-ian-approves-rank-threshold-for-0-2.md` and the retained [2026-10-01 issue](../issues/2026-10-01-rank-keeps-only-records-over-a-threshold.md).
- Keeps: The ten functions, default uncut rank, stable ties, original records and positions, existing model questions, request counts, complete observations, and plain rank's cache/replay answers. The cutoff is a reading of stored probabilities, not question wording or routing.
- Changes: Amend `specification/rank.md` and ADR 0007's no-selection consequence under Ian's ruling. Admit a single inclusive probability cutoff through the CLI and Rust rank reading. Filter below-cut records before presenting the ordered output; combine with top without changing the eligible order. Preserve current saved-score and question-set behavior; explicitly settle their cutoff admission in the reviewed amendment rather than treating an ordinal or weighted score as a calibrated probability. Claim `crates/thinkthen/src/cli/rank/**`, `crates/thinkthen/src/public/rank/**`, `crates/thinkthen/src/core/measure/**`, `crates/thinkthen/tests/backend/rank/**`, `crates/thinkthen/tests/library/rank/**`, `specification/rank.md` and `sdlc/planning/adr/0007-flat-verbs-bare-values-and-one-threshold.md`. Confirm actual rank reading paths before coding and keep claims narrow.
- Proof: Outside-in CLI and Rust cases retain an exactly-at-cut record, drop below-cut records, preserve probability order and stable ties, compose with top, and reject invalid cutoff input before sends. Run plain rank first and the cut reading against the same cache or recording; count zero additional model requests and check the same underlying question/cache identities. With no cutoff, retain existing literal output and error cases.
- Defers: A new function, grep alias, proxy business policy, calibration, extra model calls and release management. Build after 0475 and 0476, before final installed cross-surface qualification.
