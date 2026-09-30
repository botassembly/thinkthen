# 0298 — Rust Polars lazy expressions (F7)

Status: Draft preparation. Accepted ADR 0107 settles the outcome; implementation and issue closure remain open.

## Outcome

Add decide_expr/choose_expr/score_expr/tag_expr over pinned Polars Rust UDF API. Decide/choose probability Struct only; pass shared core tally, token and per-morsel deadline at build. catch_unwind maps panic to a compute error. Lead with cheap filters then with_columns-shaped judgment and state the filter/head pushdown hazard.

The expression deadline applies to each morsel, not the whole query. RSS/load/churn is opt-in.

## Prerequisites and proposed files

Prerequisite: T7. Proposed file families: `libraries/polars/src/ and tests; existing Polars 0.55 Cargo pin/re-export; README/spec rows and ratchet`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Lazy/eager parity, score/tag refusal, token stops later morsels, tallies and judged rows match listener. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Rust Polars lazy expression family with tally, cancellation and panic boundary.
- Proof: Lazy/eager parity, score/tag refusal, token stops later morsels, tallies and judged rows match listener. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
