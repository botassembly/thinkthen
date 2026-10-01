# The Polars door cannot test lazy streaming

Status: open. Deferred by ticket 0304 slice 3c after ticket 0307 dropped `polars/streaming` from the `polars` feature. Owner: none until a user asks.

Kind: debt

Pay when: a user needs the streaming engine, or `polars/streaming` no longer pulls the three advisories on `bincode` and `quick-xml`.

Debt: 004

Severity: low

Milestone: later

Keeping it risks the streaming path breaking with no check, and the frame's own memory staying unbounded.

## What is missing

The `polars` feature selects `polars/lazy` alone, so lazy `collect()` runs the in-memory engine over the whole column. The checks cannot compile the streaming engine or `LazyFrame::collect_batches`. `crates/thinkthen/tests/polars/lazy.rs` lost its streaming collect and ADR 0107 F7's morsel-cut proof. Restoring them means restoring `polars/streaming` and three advisory ignores (ticket 0307).
