# 0287 — Python keywords and probability (T5)

Status: Accepted design at preserved preparation `8bf14799`; implementation in progress. ADR 0105 settles the outcome; issue closure remains open.

## Outcome

Keywords on every verb; `.probability` on `Call` (decide and choose; None on
score and tag); `Engine(max_requests_total=)`; `true_`/`false_` → `true`/
`false`; `deadline=` seconds → `deadline_ms=` milliseconds with the plain
rename message; `descriptions=` folded into the members map on choose, score
and tag (recognize keeps its own).

## Prerequisites and proposed files

Prerequisite: T1 and T7; before F1/F2. Proposed file families: `libraries/python/src/{lib.rs,asked.rs,engine.rs,engine/settings.rs,result.rs,frame.rs,input.rs}; libraries/python/thinkthen/{__init__.py,__init__.pyi}; selected Python tests/README/ratchets`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Selected Python replay pins one reply value/probability, nulls, rename messages and real cap. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Python uniform keywords, Call probability, active engine cap and exact renames.
- Proof: Selected Python replay pins one reply value/probability, nulls, rename messages and real cap. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
