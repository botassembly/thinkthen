# 0407: Supply independent context for each record

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: later

## Outcome

Expose a typed record/context input separate from evidence for eligible judgment batches and annotations, with SQL and CLI adapters. Settle eligible functions and request packing rules in a reviewed contract before implementation. Preserve the existing shared-context route.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 audit](../records/0405-audit-report.md). CallOptions stores one context string; Decisions carries one clone; CLI Context reads one file for a run. SQL scalar context expressions can vary by row through contextual singleton calls, but keyed many calls have one context.
- Keeps: existing request bytes, digests, cache identity, result shapes, failures and host adapters whenever the new capability is unused.
- Changes: supply independent context for each record. Product work starts only after the contract and ticket receive fresh review.
- Proof: Count sends for two items with different contexts, repeated contexts, empty/invalid context, pointer disclosure, split requests, cache identity and strict replay; ensure original evidence item remains separately recoverable.
- Defers: Per-record context for complete find/entity-set items needs its own semantics; foreign adapter releases follow later. No implicit scratch files.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
