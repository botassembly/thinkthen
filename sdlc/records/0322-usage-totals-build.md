# 0322 slice 1: every surface adds to the usage totals — build

Status: slice 1 landed on main from `ticket/0322-spend-totals-design`. Ticket: `sdlc/tickets/0322-every-surface-adds-to-usage-totals.md`. Design: ADR 0113. Every run was offline against loopback backends with `THINKTHEN_API_KEY` unset. No paid call ran and the live ledger was not touched.

## Built

- `EngineBuilder::from_env()` seeds the counters with `config::usage_path()`. `EngineBuilder::new()` still passes no folder.
- `Engine::finish_usage()` checks the `Guarded` owner with atomics only, then finishes this process's own counters. The public inventory declares it in the ticket.
- The usage writer thread masks host signals.
- PostgreSQL registers `on_proc_exit` on the backend thread when a backend builds its first engine. SQLite and DuckDB register `atexit` when each builds its first engine. Each hook runs inside `thinkthen::contained` and takes the engine list with `try_lock`. SQLite's `check.sh` guard now expects two `contained(` calls.
- `Guarded` drops only state this process built. A forked child that drops an inherited engine leaks it.
- `scratch.sh` gains `usage_home` and `usage_guard`. `sdlc/scripts/test` and `surfaces` run under the decoy. Every surface `check.sh` except Swift and Zig, which already set their own `HOME` and `XDG_CACHE_HOME`, takes its own scratch usage folder. So does `release-smoke` and the type corpus self-test.
- The README, `specification/recording.md`, the three SQL READMEs and the changelog say every surface adds to the totals.

## Proof

| Proof | With the change | With the hook removed |
| --- | --- | --- |
| `public_env::usage_totals`: the command and a `from_env` engine, one 503 | `status --json` reports 3 requests, 1 retry, 624 input and 96 output tokens | Without the seeding: fails |
| C door `a_freed_c_engine_adds_its_send_to_the_usage_totals` | month file reports 1 | not run |
| SQLite exit under a held lock for 0.3 s | 1 | fails: held 0, wanted 1 |
| SQLite unload and reload | 2 | fails: 0, wanted 2 |
| SQLite forked child | child exits in under 0.25 s; parent counted once | fails: 0, wanted 1 |
| PostgreSQL `each_backend_adds_its_sends_at_exit` | 2 | fails: "want: 2 got: 0" |
| DuckDB `the_exit_flushes_a_call_while_the_usage_lock_is_held` | month file reports 1 | fails: "wanted [1], got []" |
| `fork_tests::a_child_finishing_usage_leaves_its_parents_counters_alone` | returns inside the bound; file reads 1, then 2 | not run |
| `fork_tests::a_child_dropping_an_inherited_engine_leaves_its_state_alone` | passes | without the drop rule: fails |
| `an_engine_built_by_hand_writes_no_usage` and the `0755` folder row | pass | not run |

Guard proof: a script under `usage_guard` ran the command against a loopback 503 listener with the inherited environment. The run wrote the decoy and failed with "FAIL something wrote the decoy usage folder ..., not its own scratch one (ADR 0113)". The real usage file's modification time did not change. The guard also caught a real leak. The first full `sdlc/scripts/test` failed on the decoy, and the type corpus self-test, whose `check.py` builds C door engines from the environment, was the cause.

## Checks

`sdlc/scripts/test` passes under the decoy: 1265 nextest tests, doctests, consumer and shell self-tests. `cargo clippy --workspace --all-targets -- -D warnings` passes, and so does clippy for the PostgreSQL, SQLite, DuckDB and C crates. `policy.py`, `tickets` and `sdlc/scripts/lint` pass. The PostgreSQL (87 passed), SQLite and DuckDB `check.sh` runs pass, each under a decoy guard. The ratchets rise: root 105729, C 4334, PostgreSQL 3022, SQLite 2801 and 2196, DuckDB 4203 and 3935.

One full test run failed `public_controls::a_stop_during_a_batch_or_a_cache_lock_wait_sends_nothing_new` once under load. It passed five times alone and in the next two full runs. Issue `2026-09-30-public-batch-holds-one-send-under-a-throttle.md` covers its timing.

## For the code reviewer

- The macOS mirror is unverified. On macOS the checks that search for the home folder inside `Library` compare against the scratch `HOME`, which weakens them.
- The `spec` rung sets no scratch usage folder. Its command runs can write the real usage folder, as they could before this ticket.
- `default_engine` statics in the ports still lose the counts after their last flush at exit. The ticket defers them.
