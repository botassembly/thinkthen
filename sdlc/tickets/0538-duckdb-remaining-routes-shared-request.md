# 0538: Move the remaining DuckDB routes to the shared request

Status: OPEN.

Milestone: 0.2

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

## Outcome

Every DuckDB function builds its call through the shared Request and owned session. No DuckDB route calls the engine directly or writes request JSON by hand.

## Evidence

- Starts from: gap 4 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md). 0495 moved the complete and file paths. About 13 routes still call the engine directly: `databases/duckdb/bridge/src/ffi/scalar/ffi.rs:105,229,254,343`, `portable_many/ffi.rs:287,311,322,332`, `portable/ffi.rs:162`, `listed.rs:75`, `images/ffi.rs:200,206` and `complete_listed/ffi.rs:23`.
- Keeps: every DuckDB function name, argument, NULL rule, JSON value, file admission bound and description that 0470, 0495 and 0519 qualified. Both DuckDB 1.5.4 and 1.5.5 builds.
- Changes: route the scalar, portable, listed, image and complete-listed functions through the shared request. Delete the direct engine calls and their JSON assembly. Lower the DuckDB ceiling.
- Proof: the routine installed DuckDB checks pass on both versions.
- Defers: nothing.
