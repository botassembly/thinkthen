# 0406: Rank questions preserve criteria and score ordering across core surfaces

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: later

## Outcome

Rust, C and SQL rank accept the same saved decide criteria and saved score question that the command accepts. Keep ordinary rank result types and ordering. Describe any additive carrier needed for score ranking before code. Foreign wrappers follow later reviewed slices.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 audit](../records/0405-audit-report.md). crates/thinkthen/src/public/bulk.rs:285 only admits Kind::Rank; libraries/c/src/call.rs:267 builds Question::rank from text alone; databases/postgresql/src/keyed.rs:178 and DuckDB/SQLite rank do the same. CLI graded_rank tests already prove score-file ordering.
- Keeps: existing request bytes, digests, cache identity, result shapes, failures and host adapters whenever the new capability is unused.
- Changes: rank questions preserve criteria and score ordering across core surfaces. Product work starts only after the contract and ticket receive fresh review.
- Proof: Add counted replay parity for described decide and score files, weighted score ties, model/profile overrides and retained plain rank bytes; preserve threshold refusal and zero-send invalid input.
- Defers: 0401 owns multiquestion rank; this ticket owns existing single-question equivalence. Find has no score/criteria contract and is not silently widened.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
