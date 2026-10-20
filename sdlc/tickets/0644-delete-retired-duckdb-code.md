# 0644: Delete the DuckDB removed-function stubs and the retired DuckDB workspace

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `databases/duckdb/Cargo.toml`, `databases/duckdb/Cargo.lock`, `databases/duckdb/src/lib.rs`, `databases/duckdb/bridge/Cargo.toml`, `databases/duckdb/cpp/src/removed.cpp`, `databases/duckdb/cpp/src/removed.hpp`, `databases/duckdb/cpp/src/thinkthen.cpp`, `databases/duckdb/cpp/sources.cmake`, `databases/duckdb/cpp/verify_tables.py`, `databases/duckdb/cpp/README.md`, `databases/duckdb/NOTES.md`, `databases/duckdb/tools/portable_suite.py`, `databases/duckdb/check.sh`, `sdlc/surfaces.txt`, `sdlc/scripts/versions`, `sdlc/scripts/hosted-preflight.py`, `sdlc/scripts/release-workflow`.
- The retired pure Rust workspace (`databases/duckdb/Cargo.toml`, `Cargo.lock` and `src/`) is deleted. The surface registry names `bridge` as the DuckDB crate folder in `sdlc/surfaces.txt`, so the binding policy and `cargo deny` read `bridge/Cargo.toml`. `sdlc/scripts/versions` reads the DuckDB package release number from `bridge/Cargo.toml`, and the cargo fetch lists in `hosted-preflight.py` and `release-workflow` drop the retired manifest.
- `cpp/src/removed.cpp`, `cpp/src/removed.hpp`, the `RegisterRemoved` call in `cpp/src/thinkthen.cpp`, the `removed` entry in `cpp/sources.cmake`, and the tests and documentation lines for `thinkthen_warm` and `thinkthen_probability` are deleted.
- Calling `thinkthen_warm` or `thinkthen_probability` in DuckDB gives DuckDB's own unknown-function error.
- SQLite's removed spellings go in 0714 and PostgreSQL's removed functions in 0715.

### Proof

- On the M5 natively: `pm job run --machine m5 0644 SHA --memory 8 -- sdlc/scripts/lint` passes with the retired workspace gone. Its `versions` and `surfaces --registry` steps fail before the registry and version changes, because they still name `databases/duckdb/Cargo.toml`.
- At the 0619 container checkpoint, in the arm64 container on the M5: `sh databases/duckdb/check.sh` passes on both pinned DuckDB versions, and `SELECT thinkthen_warm('a', 'b')` fails with DuckDB's unknown-function error. Before the change the same query returns the registered refusal sentence.

## Evidence

- The shipped DuckDB extension is `cpp/` linked with the `bridge/` static library (`databases/duckdb/NOTES.md`). The retired workspace is still tracked as `databases/duckdb/Cargo.toml` (crate `thinkthen_duckdb_retired`, whose header comment says the binding policy still checks it), `databases/duckdb/Cargo.lock` and `databases/duckdb/src/lib.rs`.
- Readers of the retired manifest: `sdlc/surfaces.txt` (the `databases/duckdb landed` line has no crate folder, so `surfaces` reads the folder's own `Cargo.toml`), the `CARGO` table in `sdlc/scripts/versions`, the manifest list in `sdlc/scripts/hosted-preflight.py`, and the `cargo fetch` loop in `sdlc/scripts/release-workflow`. `bridge/Cargo.toml` carries version `0.0.1` today; the release number moves there, and that is a package release number, which the 2026-10-19 no-version ruling keeps.
- `databases/duckdb/cpp/src/removed.cpp` defines `RegisterRemoved`, which registers `thinkthen_warm` and `thinkthen_probability` only to refuse them with a pointer to their replacements. Tests of those refusals: `removed_warm_and_probability_forms_refuse_without_transport` and `ordinary_errors_show_retryability_while_removed_warm_stays_plain` in `tools/portable_suite.py` (the first is named in `check.sh`), and the `thinkthen_warm` statements in `cpp/verify_tables.py`. `cpp/README.md` (including its `verify_decide.py` paragraph) and `NOTES.md` describe them.
- That is backward-compatibility code, which the 2026-10-12 no-compatibility-before-1.0 ruling forbids. `specification/rank.md` already says the per-row `thinkthen_probability` is removed on all three databases, so it needs no change.
- The DuckDB tools are declared Linux-only in `sdlc/tools.json`, so the installed extension suites run in the 0619 container, not natively on the M5.
