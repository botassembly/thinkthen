# 0644: Delete removed-function stubs in all three databases and the retired DuckDB workspace

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- `databases/duckdb/Cargo.toml`, `databases/duckdb/Cargo.lock` and `databases/duckdb/src/` are deleted. The binding policy reads `bridge/Cargo.toml` instead.
- `cpp/src/removed.cpp`, `cpp/src/removed.hpp`, their registration call, and the tests and documentation lines for the two stubs are deleted.
- The SQLite stubs and their tests, and any PostgreSQL stub and its tests, are deleted.
- Calling `thinkthen_warm` gives each database's own unknown-function error.

### Proof

- On the M5 through pm job, the installed DuckDB extension on both pinned versions, the SQLite extension and the PostgreSQL 16, 17 and 18 extensions pass their suites.
- `SELECT thinkthen_warm('a','b')` fails with each database's own unknown-function error.
- `lint` passes with the retired workspace gone.

## Evidence

The shipped DuckDB extension is `cpp/` linked with `bridge/` (`databases/duckdb/NOTES.md`). The retired pure Rust workspace is still tracked, as `databases/duckdb/Cargo.toml` (crate `thinkthen_duckdb_retired`), `databases/duckdb/Cargo.lock` and `databases/duckdb/src/lib.rs`, kept "for the binding policy". `databases/duckdb/cpp/src/removed.cpp` registers `thinkthen_warm` and `thinkthen_probability` only to refuse them with a pointer to their replacements. SQLite carries the same stubs (`databases/sqlite/src/scalars.rs:320-324`, `src/lib.rs:214-218`, `tests/test_values.py:229`), and PostgreSQL tests them (`check.sh:1063-1066`). That is backward-compatibility code, which the 2026-10-12 no-compatibility-before-1.0 ruling forbids.
