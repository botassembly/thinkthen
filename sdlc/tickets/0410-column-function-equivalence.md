# 0410: Complete dataframe function coverage

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: 0.2

## Outcome

Rust Polars eager/lazy exposes filter, rank, find, recognize and relate or documents reviewed host composition with equivalent input/output identity. Python pandas/Polars column counterparts follow later slices. Retain four judgments and annotate.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 audit](../records/0405-audit-report.md). Rust public/frame.rs and polars cases RUN/NOT_RUN cover five named functions; Python input.rs and judge.py refuse column inputs for filter/rank/find/relate. Existing pandas-Series/DuckDB-descriptions issue already owns its four pandas refusals.
- Keeps: existing request bytes, digests, cache identity, result shapes, failures and host adapters whenever the new capability is unused.
- Changes: complete dataframe function coverage. Product work starts only after the contract and ticket receive fresh review.
- Proof: Independent dataframe versus ordinary sequence comparisons, empty/null/duplicate rows, key identity, selected unit/candidate set and complete entity set, plus strict replay counts. Do not claim lazy streaming proof from eager collection.
- Defers: Existing pandas-Series issue remains owner for its overlap. Rust Polars gap defaults to 0.2; Python Polars and additional dataframe conveniences are later.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
