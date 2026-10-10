# 0495: Make DuckDB thin and first-class

Status: COMPLETE.

Milestone: 0.2

Depends on: 0470
Depends on: 0519
Depends on: 0513

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision 9a882af553fc03b5136b23eb33d0a3be5cf23f59, accept

Landed: 9b03d58

## Outcome

DuckDB admits every call through the shared Request and 0503's owned session, and returns generated results through named typed SQL functions. The extension keeps only SQL host types, file authority and cancellation idiom. Rust owns every rule.

## Evidence

- Starts from: the [2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The DuckDB adapter repeats admission and result construction. It imports SQLite's `complete_native` compatibility module and calls its dispatcher from `databases/duckdb/bridge/src/ffi/complete/ffi.rs`, `ffi/complete_files/ffi.rs` and `ffi/complete_questions/ffi.rs`. 0499 keeps that dispatcher alive for SQLite.
- Keeps: DuckDB FileSystem authority, calling-thread readers, the bounded feed from 0470, order and cancellation. All ten functions and their input, result, error, cache and replay behavior. Missing stays distinct from null, and permitted unknown result fields are tolerated. 0519's SQL NULL, binary image, native JSON and description rules hold.
- Changes: Follow 0511 shared admission, 0503's owned session and 0519's SQL contract.
  - Translate SQL arguments into Request and read generated result types.
  - Remove DuckDB's dependency on SQLite's `complete_native` grammar and dispatcher after installed DuckDB parity. Coordinate the resulting SQLite-only deletion with 0499's owner.
  - Remove the old copies and update the extension README after parity.
  Claim `databases/duckdb/**` and its installed typed consumer cases, narrowed to the actual DuckDB and shared dependency files per slice before coding.
- Proof: The full shared cases run through the installed extension's typed functions. They cover files and images, context and options, original positions, facts, failures, invalid input with zero sends and cancellation. Raw JSON pass-through does not count. Record handwritten code removed and added in the landing record.
- Defers: The proxy and any platform ruling change without evidence. Neither needs a ticket in 0.2.

## Progress

- 2026-10-10 started
