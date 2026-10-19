# 0694: Replace DuckDB's own suites with a replay of the shared cases

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688, 0693

## Outcome

- Changes: `databases/duckdb/tools/`, `databases/duckdb/check.sh`, `databases/duckdb/cpp/README.md`, `sdlc/scripts/children`.
- `databases/duckdb/tools/conformance.py` replays every case of `conformance/cases.json` as SQL through the installed extension and compares each result against the packet oracle `conformance/fixtures/packets.json` from 0642. It adds no case file of its own.
- DuckDB suites under `tools/` that repeat behaviors the shared cases define are deleted, with their entries in `check.sh` and in the exception table of `sdlc/scripts/children`. Tests of DuckDB-only behavior stay: loading, the two-version host checks, packaging, SQL type mapping, interrupts and statement budgets.

### Proof

- At the 0619 container checkpoint, in the arm64 container on the M5: `sh databases/duckdb/check.sh` passes on both pinned DuckDB versions, with the replay covering every shared case.
- Done once by hand and not committed: a wrong expected answer in the oracle fails the replay. Before the change the same edit passes, because today's replay checks request digests from `cases.json` and never reads the oracle.
- On the M5 natively: `pm job run --machine m5 0694 SHA --memory 8 -- sdlc/scripts/lint` passes with the deleted suites gone from `sdlc/scripts/children`.

## Evidence

- `databases/duckdb/tools/conformance.py` already runs the shared cases, each on its own `/case/ID/v1` backend arm, and counts pass, fail and not run against `cases.json`. It compares request digests, not packet answers.
- Beside it, `databases/duckdb/check.sh` runs many suites of its own under `tools/`, including `native_records.py`, `settings_suite.py`, `verbs_suite.py`, `rank_suite.py`, `rank_sets_suite.py`, `find_suite.py`, `portable_suite.py`, `files_suite.py`, `images_suite.py` and `relate_suite.py`. Many re-check behaviors the core and the shared cases already define. Commit `09e424609` changed one line of shared Rust and needed new cases in all three database suites.
- `sdlc/scripts/children` lists `databases/duckdb/tools/databases_suite.py` among its exceptions. `databases/sqlite/tests/complete/` also drives a DuckDB child for its cross-database parity; it stays unless 0695 removes it.
- The DuckDB tools are declared Linux-only in `sdlc/tools.json`, so the installed extension suites run in the 0619 container, not natively on the M5.
