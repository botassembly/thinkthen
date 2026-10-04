# 0412: Document R index adapters in the binding contract

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: later

## Outcome

Binding-author guidance explicitly allows R one-based place and its relation to canonical zero-based indexes, matching the existing R runtime and conformance conversion. Preserve R outputs.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 audit](../records/0405-audit-report.md). BINDING-AUTHOR.md states index/record/probability expectations; R rank/find returns one-based place. Its runner intentionally converts to canonical indexes; recognize span conventions already document host offsets.
- Keeps: existing request bytes, digests, cache identity, result shapes, failures and host adapters whenever the new capability is unused.
- Changes: document r index adapters in the binding contract. Product work starts only after the contract and ticket receive fresh review.
- Proof: Check rank duplicates/ties, find none and offsets through current R runner; consumer examples show the index mapping. Do not change runtime indexes as a prose fix.
- Defers: Small documentation reconciliation; no universal host shape migration.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
