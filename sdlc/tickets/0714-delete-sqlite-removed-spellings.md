# 0714: Delete the SQLite removed-spelling stubs

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `databases/sqlite/src/scalars.rs`, `databases/sqlite/src/lib.rs`, `databases/sqlite/tests/test_error_format.py`, `databases/sqlite/tests/test_probability.py`, `databases/sqlite/tests/test_settings.py`, `databases/sqlite/tests/test_trusted_schema.py`, `databases/sqlite/tests/test_values.py`, `databases/sqlite/README.md`.
- `register_removed` in `databases/sqlite/src/scalars.rs` and the registration it makes are deleted: the old judgment arities, `thinkthen_warm`, `thinkthen_probability` and the setting functions replaced by `thinkthen_configure`. The `thinkthen_recognize_document` rename refusal in `register` goes too. `Failure::plain_usage` and its test in `databases/sqlite/src/lib.rs` go when nothing else calls them.
- The Python tests that assert those refusal sentences are deleted, and the README stops describing removed spellings in `thinkthen_functions()` and the trusted-schema rules.
- Calling any of those names gives SQLite's own "no such function" error.

### Proof

- On the M5 natively: `pm job run --machine m5 0714 SHA --memory 8 -- sh databases/sqlite/check.sh` passes. The check builds its own pinned SQLite host and starts its own loopback backends, so it runs without `surfaces`.
- In that run, `SELECT thinkthen_warm('a', 'b')` fails with `no such function: thinkthen_warm`. Before the change it fails with the registered sentence `thinkthen_warm was removed; pack records with thinkthen_decide_many`.

## Evidence

- Split from 0644, which keeps the DuckDB stubs and workspace. 0715 takes PostgreSQL. The three land in any order.
- `databases/sqlite/src/scalars.rs` defines `register_removed`, called from `register`, which refuses removed call forms with migration sentences. `register` also refuses `thinkthen_recognize_document` with "was renamed thinkthen_relations". `databases/sqlite/src/lib.rs` defines `Failure::plain_usage` for "an explicitly removed SQL spelling" and tests it with a `thinkthen_warm` sentence.
- Tests that assert removed spellings: `tests/test_error_format.py`, `tests/test_probability.py`, `tests/test_settings.py`, `tests/test_trusted_schema.py` and `tests/test_values.py`.
- That is backward-compatibility code, which the 2026-10-12 no-compatibility-before-1.0 ruling forbids.
- 0695 also rewrites `databases/sqlite/tests/`; whichever lands second rebases on the first.
