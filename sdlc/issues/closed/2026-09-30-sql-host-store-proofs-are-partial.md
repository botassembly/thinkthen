# The SQL hosts' question store proofs are partial

Status: closed 2026-09-30 by ticket 0348. Reopened on 2026-09-30 when ticket 0304 slice 5 paid the rest of Debt 003 (`2026-09-30-old-batching-files-still-have-live-callers.md`), into which this issue had merged.

Resolution: paid by ticket 0348. The DuckDB and PostgreSQL shared case runners count answer rows in each case's `thinkthen.sqlite`, one per good question, as the SQLite and C door runners do. DuckDB's harness counts each child's rows before it deletes the folder. `databases/sqlite/tests/test_store.py` replays from a read-only folder with no file or directory metadata change, and pins a lookup that waits while another connection holds the store, then answers and stores. Each count and test failed once against a planted empty store.

Kind: debt

Pay when: before 0.1.

Debt: 010

Severity: medium

Deferred by ticket 0304 slice 3b. Slice 3e closed the PostgreSQL missing-part row with `databases/postgresql/check.sh` `an_annotate_row_missing_its_part_fails_alone`. Keeping the rest risks a store regression in DuckDB or PostgreSQL that no check counts.

- The `databases/duckdb` and `databases/postgresql` shared case runners do not count answer rows in `thinkthen.sqlite`, as the SQLite (`databases/sqlite/tests/conformance.py`) and C door (`libraries/c/tests/door/cases.rs` `stored`) runners do. DuckDB's harness deletes each child's folder when the child ends, so the count needs a harness change.
- The prep note `sdlc/planning/0304-slices-3b-3d-prep.md` asked for the store's edge rows through the SQLite extension on the 3.50.0 host. The shared cases cover a hit and a miss. No test covers a read-only replay folder or a busy wait there.

Done when each gap has a test that fails when the store stops answering. A ticket of its own pays it before 0.1.
