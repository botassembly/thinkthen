# 0495: Move DuckDB onto the shared request contract

Status: OPEN.

Milestone: 0.2

Depends on: 0470
Depends on: 0519

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

Adopt shared Request and generated results in DuckDB through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current DuckDB adapter repeats admission/result construction.
- Keeps: DuckDB FileSystem authority, bounded calling-thread readers, order and cancellation; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0470 and0503 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `databases/duckdb/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields. DuckDB currently imports SQLite's `complete_native` compatibility module and calls its dispatcher from `ffi/complete/ffi.rs`; 0499 retains that live route. Remove the dependency in this migration before removing the shared dispatcher, and coordinate any resulting SQLite-only deletion with its owner.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: large surface migration.

## 2026-10-09 amendment

Follow 0511 shared admission, 0503 owned session and 0519 SQL consistency under the thin, first-class ruling. Keep calling-thread FileSystem access and bounded feeds; SQL owns only its host types, authority and cancellation idiom. Remove the imported SQLite grammar and dispatcher after installed DuckDB parity, coordinating the deletion with 0499. Review this amendment and narrow actual DuckDB and shared dependency files before implementation.
