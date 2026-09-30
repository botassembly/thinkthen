# 0294 — Python explicit tally (F3)

Status: Draft preparation. Accepted ADR 0107 settles the outcome; implementation and issue closure remain open.

## Outcome

Wrap the T7 Arc-shared core Tally as tt.Tally and accept tally= for judges, streams and frame idioms. Aggregate finished-call records, sent requests, cache answers, tokens and first-start-to-last-finish seconds. Keep tt.usage as process totals; no mutable last-call slot.

## Prerequisites and proposed files

Prerequisite: T7 and F1/F2. Proposed file families: `libraries/python/src/{lib.rs,engine.rs,result.rs}; Python package API/stubs and focused tally tests`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Sixteen-thread tally versus observed request bodies and cached-response count. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Explicit Python Tally over the Arc-shared core fact sum.
- Proof: Sixteen-thread tally versus observed request bodies and cached-response count. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
