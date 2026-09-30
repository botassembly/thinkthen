# The SQL hosts' question store proofs are partial

Status: Closed on 2026-09-30. Merged into `../2026-09-30-old-batching-files-still-have-live-callers.md`, last section, which ticket 0304 slice 5 pays.

Kind: debt

Pay when: 0304 slice 5 lands, or before 0.1, whichever comes first.

Keeping it risks a store regression in DuckDB or PostgreSQL that no check counts.

## What is missing

- `databases/duckdb` and `databases/postgresql` shared case runners do not count answer rows in `thinkthen.sqlite`, as the SQLite (`databases/sqlite/tests/conformance.py`) and C door (`libraries/c/tests/door/cases.rs` `stored`) runners do. DuckDB's harness deletes each child's folder when the child ends, so the count needs a harness change.
- The prep note `sdlc/planning/0304-slices-3b-3d-prep.md` asked for the store's edge rows through the SQLite extension on the 3.50.0 host. The shared cases cover a hit and a miss. No test covers a read-only replay folder or a busy wait there.

## Done when

Each gap has a test that fails when the store stops answering, and this issue moves to `closed/`.
