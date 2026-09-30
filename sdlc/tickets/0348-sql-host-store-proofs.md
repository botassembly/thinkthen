# 0348: Each SQL host proves its question store answers

Status: ready. Plan: `sdlc/planning/issue-priorities-2026-09-30.md`, batch B4, after ticket 0347 in the same lane. Pays Debt 010, `sdlc/issues/2026-09-30-sql-host-store-proofs-are-partial.md`.

## Outcome

The DuckDB and PostgreSQL shared case runners count answer rows in `thinkthen.sqlite`, as the SQLite and C door runners do. The SQLite extension on its 3.50.0 host has a test for a read-only replay folder and one for a store another connection holds. Each new test fails when the store stops answering.

## Evidence

- Starts from: the debt issue; `databases/sqlite/tests/conformance.py` and `libraries/c/tests/door/cases.rs` `stored`, which count stored rows; DuckDB's harness, which deletes each child's folder when the child ends; `databases/postgresql/check.sh` `an_annotate_row_missing_its_part_fails_alone`, which closed the PostgreSQL missing-part row in 0304 slice 3e; `sdlc/planning/0304-slices-3b-3d-prep.md`, which asked for the store's edge rows through the SQLite extension.
- Keeps: every existing case, its expected output and its request counts; the SQLite host-SQLite store rule of 0304 slice 3a; the scratch usage folders every check uses.
- Changes: DuckDB's harness keeps each child's folder until the runner has counted its rows, then deletes it. The DuckDB and PostgreSQL runners count stored answers per case. Two SQLite extension tests: replay from a read-only folder answers and changes no file or directory metadata, as `specification/recording.md` states; and a store another connection holds in a write waits and then answers, or refuses with the sentence the command gives for the same case. The builder pins whichever the command does today. The issue moves to `closed/`.
- Proof: each new count and test fails once against a planted store that answers nothing (for example, a runner pointed at an empty folder), then passes. The three SQL checks pass. `policy.py`, workspace clippy with `-D warnings`, `tickets`, and `lint` in a clean checkout.
- Defers: nothing new.

## What the build taught us
