# 0297 — R judge and plan (F6)

Status: Draft preparation. Accepted ADR 0107 settles the outcome; implementation and issue closure remain open.

## Outcome

An omitted R input returns a function judge. tt_plan takes that judge, moved from T6. Keep vector results and purrr::partial; grouped mutate sends one packed call per group. dbplyr passes SQL call through unchanged: E9 runs DuckDB, and PostgreSQL scalar per-row count is stated.

## Prerequisites and proposed files

Prerequisite: T6. Proposed file families: `libraries/r/thinkthen/R/{thinkthen.R,extendr-wrappers.R}; R Rust calls/lib, README/check/ratchets and focused tests`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Purrr partial/judge parity, grouped body count and dbplyr show_query snapshot. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: R judge form, plan, grouped mutate and dbplyr idiom.
- Proof: Purrr partial/judge parity, grouped body count and dbplyr show_query snapshot. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
