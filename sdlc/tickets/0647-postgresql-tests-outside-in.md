# 0647: Replace PostgreSQL's shell test with a replay of the shared cases

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- A small driver loads the installed PostgreSQL extension, replays `conformance/cases.json` as SQL, and compares each result against the 0642 oracle. It adds no case file of its own.
- `check.sh` keeps only building the extension and starting and stopping the server. Its inline test steps and the matching `tests/*_cases.py` files are deleted.
- A new core behavior adds cases to `conformance/cases.json` only.

### Proof

- On the M5 through pm job, the installed PostgreSQL 16, 17 and 18 extensions pass the replay.
- Done once by hand and not committed: a wrong expected answer in the oracle fails the replay.

## Evidence

`databases/postgresql/check.sh` is 1,639 lines of bash 5 with 125 shell functions. It builds the extension, runs a server, restarts it per test, times statements, starts loopback proxies and matches literal output (its header comments around lines 2-166). A second Python layer sits under `databases/postgresql/tests/`, with 16 files. Commit `09e424609` changed one line of shared Rust and needed new cases in three separate database suites (`databases/duckdb/tools/native_records.py`, `databases/postgresql/tests/native_records.py`, `databases/sqlite/tests/test_typed_family.py`). DuckDB and SQLite get the same change in 0694 and 0695.
