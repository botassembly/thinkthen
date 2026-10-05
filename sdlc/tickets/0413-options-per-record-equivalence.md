# 0413: Supply candidate options independently for each record

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: later

## Outcome

Rust/C/SQL integrated many-record choose can carry candidate options per record, equivalent to CLI choose --options. Keep fixed-question calls unchanged. Define candidate identity and description/order rules before code.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 audit](../records/0405-audit-report.md). CLI Asks::FromRecord uses a pointer and record.choices; Rust/C fixed many questions capture one option set. SQL scalar members expression and caller separate calls already allow per-item candidates, but a keyed many collection does not.
- Keeps: existing request bytes, digests, cache identity, result shapes, failures and host adapters whenever the new capability is unused.
- Changes: supply candidate options independently for each record. Product work starts only after the contract and ticket receive fresh review.
- Proof: Compare two records with different shortlists and descriptions, duplicate labels, missing pointer, excluded correct option and none; pin original record, candidate identity, wire bytes and strict-replay counts.
- Defers: Candidate set belongs to item, not reusable wording. Dynamic tag/annotate options already have a separate open issue; do not duplicate it. Foreign batch adapters later.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
