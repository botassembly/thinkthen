# 0348: Each SQL host proves its question store answers

Status: landed 2026-09-30. Plan: `sdlc/planning/issue-priorities-2026-09-30.md`, batch B4, after ticket 0347 in the same lane. Pays Debt 010, `sdlc/issues/closed/2026-09-30-sql-host-store-proofs-are-partial.md`.

## Outcome

The DuckDB and PostgreSQL shared case runners count answer rows in `thinkthen.sqlite`, as the SQLite and C door runners do. The SQLite extension on its 3.50.0 host has a test for a read-only replay folder and one for a store another connection holds. Each new test fails when the store stops answering.

## Evidence

- Starts from: the debt issue; `databases/sqlite/tests/conformance.py` and `libraries/c/tests/door/cases.rs` `stored`, which count stored rows; DuckDB's harness, which deletes each child's folder when the child ends; `databases/postgresql/check.sh` `an_annotate_row_missing_its_part_fails_alone`, which closed the PostgreSQL missing-part row in 0304 slice 3e; `sdlc/planning/0304-slices-3b-3d-prep.md`, which asked for the store's edge rows through the SQLite extension.
- Keeps: every existing case, its expected output and its request counts; the SQLite host-SQLite store rule of 0304 slice 3a; the scratch usage folders every check uses.
- Changes: DuckDB's harness keeps each child's folder until the runner has counted its rows, then deletes it. The DuckDB and PostgreSQL runners count stored answers per case. Two SQLite extension tests: replay from a read-only folder answers and changes no file or directory metadata, as `specification/recording.md` states; and a store another connection holds in a write waits and then answers, or refuses with the sentence the command gives for the same case. The builder pins whichever the command does today. The issue moves to `closed/`.
- Proof: each new count and test fails once against a planted store that answers nothing (for example, a runner pointed at an empty folder), then passes. The three SQL checks pass. `policy.py`, workspace clippy with `-D warnings`, `tickets`, and `lint` in a clean checkout.
- Defers: nothing new.

## What the build taught us

- The store answers where the ticket expected. The DuckDB runner reads the first child's count, which is every case kind's main run; the counters child runs later in a folder of its own. The PostgreSQL runner refuses to pass a case without a `STORE` folder, and each case gets a fresh one from `fresh`. A case wants one row per distinct question key in its exchanges, less its failed questions, as the SQLite and C door runners count.
- The SQLite host waits on a held store. A `BEGIN EXCLUSIVE` from another connection blocks the lookup in the store's DELETE journal mode, and the call sends nothing until the holder lets go. Then it answers and stores its row. `test_store.py` pins that behavior. The child prints a ready line before its call, so the held-store check cannot pass on a child that never reached the store.
- Replay from a folder at mode 0500 with its store at 0400 answers with no send and leaves each entry's mode, size, modification time, change time and inode as they were. The comparison leaves out access time, which `specification/recording.md` does not cover.
- Planted proofs: with the DuckDB harness counting an empty folder, 44 of 55 cases failed with `stored answers`; the 9 that passed expect no stored row. With the PostgreSQL runner pointed at an empty folder, 44 cases failed the same way. Replay from an empty folder failed with the replay miss sentence, and the held-store test failed with a send while held when its child used an empty cache folder.
- Review found two gaps, both fixed: the ready line above, and a store read error in the DuckDB conformance runner now fails one case instead of stopping the run.
- `databases/postgresql/check.sh` printed a fixed case total of 54 after the case file grew to 55. It now reads `case_count` from `conformance/cases.json`.
- Under load, `sdlc/scripts/test` failed once on `batching::a_pause_sends_the_open_batch`, a Rust timing test this ticket does not touch. It passed on the rerun at a lower load. The SQLite check's `test_aggregate_once_join_is_bounded_at_one_hundred_thousand` also ran past its 60 s child limit once at a load near 29, and passed on the rerun.
- `lint` in a clean checkout first failed on main's own two site issue files, which named a private project in their status lines. This branch rewords both. Then `lint` passed (inventory 559 items).
- Checks: `sdlc/scripts/test` (1,280 passed), `spec` (24 demos green), workspace clippy with `-D warnings`, `policy.py`, `tickets`, the C door tests (31 passed), and the SQLite (53 of 55 cases, 2 not run), DuckDB (53 of 55) and PostgreSQL (89 steps, 52 of 55 cases, 3 not run) checks passed.
