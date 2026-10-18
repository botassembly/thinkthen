# 0694: Replace DuckDB's own suites with a replay of the shared cases

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688, 0693

## Outcome

- A small driver loads the installed DuckDB extension, replays `conformance/cases.json` as SQL, and compares each result against the 0642 oracle. It adds no case file of its own.
- DuckDB's duplicated case files and suites are deleted. Tests of DuckDB-only behavior, such as loading and version checks, stay.

### Proof

- On the M5 through pm job, the installed extension passes the replay on both pinned DuckDB versions.
- Done once by hand and not committed: a wrong expected answer in the oracle fails the replay.

## Evidence

The DuckDB extension carries its own Python suites under `databases/duckdb/tools/` (for example `native_records.py` and `settings_suite.py`) for behaviors the core and the shared cases already define. Commit `09e424609` changed one line of shared Rust and needed new cases in all three database suites.

## Progress

- 2026-10-18 Carried from 0628 stress job 1792330323744-7191: the native Mac DuckDB check lacks its matching pinned CLI and Python toolchain; this ticket decides whether a native Mac repair is needed after the pinned-container checks, and no repair starts before phase 3.
