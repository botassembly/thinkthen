# 0580: Return value and probability from Rust Polars

Status: OPEN.

Milestone: 0.2

Depends on: 0581

Reviews: revision eef28ca, reject

## Outcome

Rust Polars returns a value-and-probability struct and a records column for all ten functions, with no `probability` switch and no `_complete` or details frame methods.

## Evidence

- Starts from `ticket/0543-final`, surveyed 2026-10-14: Rust Polars has `probability_frame` and `input_details_column` in `frame.rs`, `PolarsExprOptions::probability(true)`, and the `*_series_complete` and `*_input_column_complete` methods. Python's `.tt` frames move with 0581's Python lane.

## Decisions

- PM decision, 2026-10-14: Polars expressions return a `{value, probability}` struct for every function, with `probability` as 0581 defines it; `probability(true)` goes. `records_column` returns the records. Delete `probability_frame`, `input_details_column` and the `_complete` frame methods.

## Scope

- Changes: `crates/thinkthen/src/public/frame.rs`, `crates/thinkthen/src/public/frame/`, `crates/thinkthen/tests/polars/`, `libraries/polars/`.
- Keeps: original row positions and null masks, filter's selected rows, one whole-set call for rank, find and relate.

## Proof

The Rust Polars nextest reads the `{value, probability}` struct and `records_column` for decide and rank on a column with a null. `tests/polars/plan_probability.rs` and the `probability_frame`, `input_details_column` and `_complete` tests go. `sdlc/scripts/test` passes.

PM amendment, 2026-10-14. `records_column` carries each original input in the supplied Series' own typed dtype. An original that does not fit one supported dtype is refused before any send. `Call.results` keeps every record unchanged. No JSON text column and no new tagged format.
