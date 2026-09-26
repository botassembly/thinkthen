# 0127: Build the test harness fixes

Status: built; ladder results below; ready for the coordinator to land. Owner: Claude.

Branch `ticket/0127-test-harness-fixes`. The build merged `origin/main` at `cd4b5051`, after tickets 0129 and 0130 landed, before its final ladder. No key was used, and no request left the machine. Ian can overturn every choice this record marks as decided.

## Result

- Test children build their whole environment from an allow list. `conformance/children/` holds one helper each for Python, TypeScript, Ruby, and R. `crates/thinkthen/src/test_deadline/child.rs` holds the Rust one. Every site the ticket listed moved onto its helper or gained `.env_clear()`. A re-scan found no site the list missed.
- `sdlc/scripts/children` runs from `lint` twice: `--self-test` with 34 cases, then the tree. The tree holds 0 findings.
- `conformance/children/test.sh` runs from `test`. It checks each non-Rust helper under planted sentinels and pins its refusal sentence.
- `heavy-lock` unsets every `THINKTHEN_*` name except `THINKTHEN_HEAVY_LOCK`, `THINKTHEN_HEAVY_LOCK_HELD`, `THINKTHEN_TOOLCHAINS`, and `THINKTHEN_DUCKDB_CLI`. A `lint` check pins it.
- The scripted listener sends a stalled peek into the full read. It skips a connection that ends before a whole request. It answers an impossible reset with the drift status, and it answers past the end of its script the same way. It keeps its port until the process exits, and it counts every connection. `conformance/backend/tests/listener.rs` pins five cases.
- The secrecy sweep marks the second relate entity and its kind. A failed status check prints the listener's connection count and request lines.
- `a_backend_that_answers_nothing_is_exit_four` is gone. `a_refused_port_fails_before_the_first_default_retry_wait` pins the same refusal.
- `databases/sqlite/tests/test_interrupt.py` records Ian's rusqlite ruling in its docstring.

## Measured keep lists

Each list below was measured by running its site.

| Site | Keeps from the parent, beside `PATH` | Sets |
| --- | --- | --- |
| Rust children of the `thinkthen` binary and of the test binary | nothing | the fake key and loopback address each test already set |
| Rust `kill`, `sh`, `mkfifo`, `cc`, `readelf`, `nm` | nothing | nothing |
| `tests/demo_runner.rs` | `HOME`: a demo with no `THINKTHEN_CACHE` reads the default cache folder under it. Without it, demo 12 printed "no default cache folder is available" | its own `PATH` |
| Rust nested `cargo` (`compile_contract.rs`, `libraries/c/tests/door`) | `child::CARGO`: `HOME`, `CARGO_HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`, `CARGO_TARGET_DIR`, `RUSTC_WRAPPER` | `compile_contract.rs` sets its own `CARGO_TARGET_DIR` |
| Python library backend and children | nothing | fake key, loopback address, cache |
| SQLite children | `LD_LIBRARY_PATH` | cache, fake key, loopback address |
| DuckDB children | nothing | `HOME`, the XDG folders, fake key, loopback address |
| PostgreSQL `psql` | nothing | nothing |
| TypeScript backend and children | nothing | fake key, loopback address, cache |
| Ruby children | `LD_LIBRARY_PATH` and `LANG`: without `LANG`, the conformance child read `cases.json` as US-ASCII and failed on its first non-ASCII byte | loopback address and the values each test sets |
| Ruby backend, and the public-names child | the backend nothing, the public-names child `LD_LIBRARY_PATH` | nothing |
| R children | `R_LIBS`, `HOME`, `XDG_CACHE_HOME`, `XDG_CONFIG_HOME`, `TT_TESTS`, `TT_BACKEND_ORIGIN` | loopback address, fake key, cache |

No child needed a kept `THINKTHEN_` or secret-shaped name.

## Planted faults

Every plant ran, turned its check red, and was restored. Each restored file was touched so the next build saw it.

| Plant | Result |
| --- | --- |
| `children --self-test`: the Ruby `unsetenv_others` rule always passes | Red. "a.rb: expected [...inherits...], got []", 33/34 cases hold |
| `children` on the tree: `Command::new("sh").arg("-c").arg("true").status()` added to `tests/transform.rs` | Red. The check named that line |
| `children` on the tree: `dict(os.environ)` restored in `libraries/python/tests/conftest.py` | Red. The check named that line |
| Rust helper without `env_clear()` | Red. `a_child_sees_only_path_and_the_names_it_keeps` failed |
| Python helper starts from `dict(os.environ)` | Red. "children python: FAIL the child" |
| TypeScript helper starts from `process.env` | Red. "children typescript: FAIL the child" |
| Ruby helper starts from `ENV.to_h` | Red. "children ruby: FAIL the child" |
| R helper without `-i` | Red. "children r: FAIL the child" |
| `heavy-lock` without its unset loop | Red. "lint: heavy-lock left a stray THINKTHEN_ name or dropped its own" |
| `test_secrecy.py`: `dict(os.environ)` restored in `conftest.py` `child_env` | Red. The child printed `True` for the planted `FAKE_SERVICE_API_KEY`, and the assert failed |
| Listener: stall rule removed | Red. The half-request row timed out |
| Listener: return on a closed connection | Red. The zero-byte row's second client saw a reset |
| Listener: read timeout removed | Red. The silent row timed out |
| Listener: `used == 0` branch removed | Red. The 40 KiB row read no header end |
| Listener: return when the script ends | Red. The past-the-script row saw a reset |
| `eprintln!` of the second entity's name in `cli/relate.rs` | Red in 3 tests: "the second entity is in a diagnostic" |
| `eprintln!` of the kind in `cli/relate.rs` | Red in 2 tests: "the kind is in a diagnostic" |
| A stray connection before each sweep case | Red. The message read "the listener saw 4 connections: POST /stray HTTP/1.1 (0 body bytes)" and then the case's three requests. The stray took the one scripted reply, and the command's retries of the drift status made the other connections |
| SQLite API table tail of 12 | Red. Eight `test_interrupt.py` tests read `interrupted` in place of the cancelled sentence, and conformance case 23 read kind `backend` |
| SQLite API table tail of 14 | Red, the same eight tests and case 23 |

The marker plants ran before the build trimmed the marker message. The trimmed message names the marker itself, such as "marker-second-e41d09 is in a diagnostic". The stray plant ran again after the trim.

## The sweep

The secrecy tests ran three times each with main's listener and this branch's, inside one hold of the heavy lock. Main's took 19.7, 24.6, and 19.7 seconds. This branch's took 19.9, 23.3, and 19.8 seconds. The kept listeners cost nothing measurable.

`no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence` then passed 50 runs of 50 in a row, inside the same hold, in 1002 seconds. The one-minute load ran from 10.0 down to 2.1. Item 4 asks for 50 runs at high load. This run proves the fix at normal load only, so the high-load proof is still owed.

## Choices made here

- The Rust refusal test pins the sentence for `THINKTHEN_BASE_URL` with `#[should_panic]`, as the ticket says. A secret-shaped name reaches the same assert.
- `child::CARGO` holds the nested cargo list once. Three sites share it.
- The ticket's Excluded table listed `probes/` and `databases/duckdb/vendor/`. The check holds no entry for either, because neither holds a finding. An entry with no finding would fail the stale-entry check.
- `test_interrupt.py` is the Part 4 test. Both plants turned it red, so no new test was needed.

## Budgets

| Part | Budget | Nonblank lines |
| --- | --- | --- |
| `sdlc/scripts/children` | 260 | 231 |
| `test_deadline/child.rs` and its test | 45 | 45 |
| Each helper in `conformance/children/` | 30 | 15 to 19 |
| `test.sh` and its four checks | 120 | 87 |
| `listener.rs` added | 45 | 43 |
| `tests/listener.rs` | 150 | 96 |
| `secrecy.rs` and `secrecy_relate.rs` changed | 30 | 30 |
| Rust spawn sites | 50 | 40 |
| Surface spawn sites, pandas included | 110 | 104, the Part 4 docstring included |
| `heavy-lock` added | 6 | 6 |
| Part 4 docstring | 6 | 6 |
| `sdlc/ratchet.json` rise | 280 | 184 |

The build first crossed three budgets: the Rust helper, `listener.rs`, and the secrecy files. It trimmed doc comments and folded repeated lists until each fit. The builder looked for duplication first in the spawn blocks of `tests/backend/harness/mod.rs` and `tests/support/measure.rs`, and in `serve_script` beside `serve_kept`. The harness spawns already clear the environment. `serve_script` and `serve_kept` now share the drift answer for an impossible reset.

## Ratchets

Each ceiling moved to its measured total after the last merge. Each rise is this build's own lines: `sdlc/ratchet.json` 63192 to 63376, `libraries/c` 2137 to 2139, `libraries/python` Python 2248 to 2253, `libraries/typescript` scripts 743 to 742, `libraries/ruby` Ruby 1526 to 1531, `libraries/r` R 1256 to 1266, `databases/duckdb` Python 1893 to 1894, `databases/sqlite` Python 1199 to 1209, `databases/postgresql` Python 243 to 249.

## Ladder

The final ladder ran after the merge of `origin/main` at `cd4b5051`, each rung once and none wrapped in `flock`, with `THINKTHEN_API_KEY` unset.

| Rung | Result |
| --- | --- |
| `install` | exit 0 |
| `lint` | exit 0: `children self-test: 34/34 cases hold`, `children: 0 findings` |
| `test` | exit 0: each of the four helper checks printed `ok` |
| `spec` | exit 0 |
| `surfaces` | exit 0: all ten surfaces passed |

An earlier ladder, after the merge at `3a86d814`, found three faults this build made in `surfaces`. The C door test's `mod child` sat out of `rustfmt` order. The DuckDB harness passed the fake key twice to `clean_env`, once from `extra`. The Ruby conformance child lacked `LANG` and read `cases.json` as US-ASCII. Commit `65740be7` fixed all three.

## Issues

- Closed `2026-09-25-python-test-children-inherit-the-whole-shell-environment.md`.
- The harness issue marks items 2, 3, and 10 settled. Item 4's fix landed, and its high-load proof is owed.
- Filed `2026-09-25-the-heavy-rungs-still-pass-secret-shaped-names-to-cargo.md` for the deferred runner-level allow list.
- Filed `2026-09-25-the-public-entity-debug-prints-its-kind.md`.
