# 0695: Replace SQLite's own suites with a replay of the shared cases

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688, 0714, 0740

## Outcome

- Changes: `databases/sqlite/tests/`, `databases/sqlite/check.sh`, `databases/sqlite/README.md`, `sdlc/scripts/children`, `sdlc/tools/sqlite.json`, `conformance/parity.py`.
- `databases/sqlite/tests/conformance.py` replays every case of `conformance/cases.json` as SQL through the installed extension and compares each result against the packet oracle `conformance/fixtures/packets.json` from 0642. It adds no case file of its own.
- SQLite suites under `tests/` that repeat behaviors the shared cases define are deleted, with their entries in `check.sh` and `sdlc/scripts/children`. Tests of SQLite-only behavior stay: loading and the host floor, trusted schema, thread mode, file reads, interrupts and statement budgets.
- `tests/complete/` and `tests/host_sqlite.sh` stay, because `conformance/test_parity.py` and `sdlc/scripts/release-workflow` use them.
- SQLite's Linux tools, including its source-built SQLite host, are declared in `sdlc/tools/sqlite.json` and join the Linux image through 0740; a missing tool fails the check, never skips.
- The existing full-routine failures pass: the macOS interrupt and filename fixtures, the settings writer, the slide library path and the error JSON schema.
- SQLite's parity consumer joins the 0621 scheduling table in `conformance/parity.py` with its measured peak memory and time.

### Proof

- On the M5 natively: `pm job run --machine m5 0695 SHA --memory 8 -- sh databases/sqlite/check.sh` passes, with the replay covering every shared case. The check builds its own pinned SQLite host and starts its own loopback backends, so it runs without `surfaces`.
- Linux arm64: `pm job run --machine m5 0695 SHA --memory 16 -- sdlc/scripts/linux-check surface databases/sqlite`; Linux x86_64: the same command with `--machine gator --memory 7`.
- Done once by hand and not committed: a wrong expected answer in the oracle fails the replay. Before the change the same edit passes, because today's replay checks request digests from `cases.json` and never reads the oracle.

## Evidence

- `databases/sqlite/tests/conformance.py` already runs every shared case as SQL, each in its own child on the backend's case arm, and compares recomputed request digests. It does not compare packet answers.
- `databases/sqlite/check.sh` also runs every `tests/test_*.py` file, the installed list (`test_typed_family.py`, `test_probability.py`, `test_rank.py`, `test_portable_batch.py`, `test_complete_request.py`, `test_complete_tables.py`, `test_rank_sets.py` and more), and `tests/complete/parity.py` and `facts.py`. Many re-check behaviors the core and the shared cases already define. Commit `09e424609` changed one line of shared Rust and needed new cases in all three database suites, including `test_typed_family.py`.
- `sdlc/scripts/children` lists `databases/sqlite/tests/test_usage.py`, `complete/facts.py` and `complete/parity.py`.
- 0714 also edits `databases/sqlite/tests/`; whichever lands second rebases on the first.

## Progress

- 2026-10-20 PM assigns the full-routine failures from0737 job1792506845610-51245 here: macOS interrupt and filename fixtures, settings writer, slide library path, and error JSON schema;0737 owns only the portable count guards and CLI package fix.
