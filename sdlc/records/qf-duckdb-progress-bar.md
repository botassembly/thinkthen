# Quick Fix qf-duckdb-progress-bar: DuckDB test children turn off the progress bar

Status: built in lane claude-2; fresh review accepted. Ian can overturn the per-connection setting.

## Why

Under host load, the DuckDB check `sixteen_held_plans_refuse_without_eviction` failed once with `Expecting value: line 2 column 1 (char 1)`. Alone it took about 1.5 s and passed.

The pinned DuckDB Python 1.5.5 turns the progress bar on by default with `progress_bar_time` 2000. A query that runs past 2 s prints a progress line that starts with `\r`, even into a pipe. The suites read each child's standard output with `text=True`, where a bare `\r` ends a line. A held query that runs past 2 s therefore puts an empty line before the JSON line the test parses.

## Change

Every Python DuckDB child whose standard output a check reads runs `SET enable_progress_bar = false` right after `duckdb.connect`, before `LOAD`:

- `databases/duckdb/tools/harness.py`: the shared `CHILD`.
- `databases/duckdb/tools/settings_suite.py`: `HELD_PLANS` and `FORKED`.
- `databases/duckdb/tools/signal_suite.py`: all four children.
- `databases/duckdb/tools/databases_suite.py`: the `db()` prelude and the read-only file connection.
- `databases/duckdb/cpp/verify_interrupt.py`: `LATE_CHILD`.
- `databases/duckdb/check.sh`: the replay smoke child. `sdlc/scripts/smoke` compares its whole output to `smoke: true`.

No assertion changed. `databases/duckdb/ratchet.py.json` rises from 4182 to 4193 for the ten new settings and one comment.

Two DuckDB facts shaped the change. `enable_progress_bar` is a session setting, so `duckdb.connect(config=...)` refuses it with "Could not set option as a global option". `SET progress_bar_time` turns the bar back on, so the forcing below had to avoid it. Cursors from a connection keep the session's setting.

`cpp/verify_package.py` runs only `LOAD` and reads no query output, so it stays as is. The `proof/` scripts are history and are not run by any check.

## Checks

- Reproduced on main by holding the 16 sends 2.5 s longer (`time.sleep(2.5)` after `backend.wait(16)` in the test): 3 of 3 runs failed with `Expecting value: line 2 column 1 (char 1)`. One run of main without forcing also failed the same way on the loaded host (load average about 21).
- With the fix and the same forcing: 10 of 10 solo runs passed, 3.7 to 4.5 s each.
- Without forcing, on a rebuilt extension: `settings_suite.py` (30 cases), `signal_suite.py` (11), `databases_suite.py` (7), and `cpp/verify_interrupt.py` passed once each, at nice 19.
- `sdlc/scripts/lint` exit 0. `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` exit 0.
- Not run: the full `databases/duckdb/check.sh` and the replay smoke, because a checkpoint run held the host.

## Deferred gap

None known.
