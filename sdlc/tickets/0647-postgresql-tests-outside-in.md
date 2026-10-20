# 0647: Replace PostgreSQL's shell test with a replay of the shared cases

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `databases/postgresql/check.sh`, `databases/postgresql/tests/`, `databases/postgresql/fixtures/`, `databases/postgresql/README.md`.
- `databases/postgresql/tests/runner.py` replays every case of `conformance/cases.json` as SQL through the installed extension and compares each result against the packet oracle `conformance/fixtures/packets.json` from 0642. It adds no case file of its own.
- `check.sh` keeps only building and packaging the extension for the requested major, starting and stopping the server, running the replay, and the PostgreSQL-only checks: loading, privileges, secrecy of the fake key, and the `catch_unwind` and guard checks. Its inline test steps that repeat shared behaviors, the matching `tests/*_cases.py` files, and the fixtures only they read are deleted.
- A new core behavior adds cases to `conformance/cases.json` only.

### Proof

- At the 0619 container checkpoint, in the arm64 container on the M5: `sh databases/postgresql/check.sh 16`, `17` and `18` pass, with the replay covering every shared case.
- Done once by hand and not committed: a wrong expected answer in the oracle fails the replay. Before the change the same edit passes, because today's `runner.py` compares against `cases.json` and never reads the oracle.

## Evidence

- `databases/postgresql/check.sh` is about 1,640 lines of bash with 111 `check` steps. It builds the extension, runs a server, restarts it per test, times statements, starts loopback proxies and matches literal output. Its `conformance` step already drives `tests/runner.py` over `cases.json`, one case per server restart, and `runner_never_excuses_by_a_note` self-tests it with `fixtures/runner-excuse.json`.
- A second Python layer sits under `databases/postgresql/tests/`, with 16 files, including `batching_cases.py`, `complete_image_cases.py`, `complete_request_cases.py`, `find_cases.py`, `image_cases.py`, `rank_sets_cases.py`, `settings_cases.py` and `native_records.py`. Commit `09e424609` changed one line of shared Rust and needed new cases in three separate database suites (`databases/duckdb/tools/native_records.py`, `databases/postgresql/tests/native_records.py`, `databases/sqlite/tests/test_typed_family.py`). DuckDB and SQLite get the same change in 0694 and 0695.
- `check.sh MAJOR` already builds and checks PostgreSQL 16, 17 and 18, and `sdlc/scripts/surfaces` runs all three majors. `conformance/test_parity.py` and `conformance/cases.json` name `databases/postgresql/check.sh` as an entry point, so the file stays.
- The pinned PostgreSQL servers run in the Linux container, so the installed check runs at the 0619 checkpoint and not natively on the M5. 0715 also edits `check.sh`; whichever lands second rebases on the first.
