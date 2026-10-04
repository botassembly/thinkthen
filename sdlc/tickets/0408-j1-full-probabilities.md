# 0408: Expose complete question probabilities through C JSON and SQL details

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: 0.2

## Outcome

Add an explicit details route for generic C JSON bulk/aggregate calls and SQL aggregate functions that preserves each underlying question probability, identity and failure. Keep bare value/facts and all existing row shapes unchanged. Foreign wrappers inherit additive C support or follow later.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 audit](../records/0405-audit-report.md). C ENVELOPE only permits details on four judgments. Generic find writes picked winner, annotate bare named values, recognize entities and relate accepted edges. Native Rust observation and CLI detailed output already hold richer data. SQL find candidates are complete and must not be treated as a gap.
- Keeps: existing request bytes, digests, cache identity, result shapes, failures and host adapters whenever the new capability is unused.
- Changes: expose complete question probabilities through c json and sql details. Product work starts only after the contract and ticket receive fresh review.
- Proof: Compare full distribution, dropped filter rows, failed annotation member versus null, find none, recognition stages and rejected relation edges with native observations. Count strict replay sends and pin secrecy.
- Defers: Existing run-accounting remainder issue owns earlier unproved full-detail evidence. This ticket supplies the narrower confirmed C/SQL carrier gap; do not duplicate backend choice or change bare outputs.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
