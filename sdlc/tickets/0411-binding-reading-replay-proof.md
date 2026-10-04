# 0411: Prove changed reading rules on every binding route

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: later

## Outcome

Every supported binding route has a named counted strict-replay test for each free reading rule, with no second send. Report intentional functions without reading cuts and unsupported stages explicitly.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 audit](../records/0405-audit-report.md). The 0405 matrix locates ordinary function and settings replay cases but not a complete changed-rule/no-send matrix. CLI disposable probes and Rust core reading establish engine behavior, not every host adapter.
- Keeps: existing request bytes, digests, cache identity, result shapes, failures and host adapters whenever the new capability is unused.
- Changes: prove changed reading rules on every binding route. Product work starts only after the contract and ticket receive fresh review.
- Proof: Change cuts while keeping wire bytes; pin outputs and listener counts. Cover column variants, generic JSON, native Call.details, member rules, recognition relation dependencies and host index conversions.
- Defers: No new store fingerprint or broad live tests. 0408 owns missing carriers; 0404 owns unrelated gate cleanup.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
