# 0105 build: port the Python surface

Builder: Claude (Opus subagent), 2026-09-25, on `ticket/0105-port-python-surface` from `71717631`. Code review: pending. Ian can overturn every decision below.

## Outcome

`libraries/python` holds the crate `thinkthen-python` in its own Cargo workspace (ADR 0047). `import thinkthen as tt` has the ten verbs, `decide_many`, `details`, `question`, `usage`, the six exception classes, `CancelToken`, and `tt.Engine`. Every call reaches the real engine through the public `thinkthen` API on a detachable worker thread. `sdlc/surfaces.txt` lists the folder as landed.

`check.sh 0` on beelink, with a loopback backend on the port: exit 0 in 33 s. It ran the flag check, the one-guard count, `cargo fmt --check`, Clippy with warnings denied, 4 Rust unit tests with libpython linked, 28 pytest tests, the shared cases (50 passed, 0 failed, 4 not run, of 54), 14 of 14 examples, and the release wheel's content check.

## Starts from, keeps, and changes

- Starts from the tag `surfaces-wave7-frozen-2026-09-24b`, spike 255 (the abi3 wheel, the worker pattern, the receiver move, the listener), and experiment 252's run of the tag.
- Keeps the public names, `None` for "not sure", question text in the first slot, `options`, `levels`, and `labels` beside a text, `rank` with `top`, the `{"index", "record" or "unit", "probability"}` shapes, the failed marker, and the deadline rule of ADR 0041.
- Changes: the engine is the real one. The stand-in, `ENGINE_NULL`, `THINKTHEN_NULL`, the `synthetic-partial` feature, the stub on port 8211, the generated registration, the spec readers, `TokenBridge`, and the `bulk()` poll hook are gone. pandas objects and, until 0106, Arrow objects are refused. `tt.Engine` holds the settings.

## Design as built

`libraries/python/NOTES.md` records the decisions. The ones a reviewer should weigh:

1. Four `_Engine` methods serve the verbs by shape: `ask` (one text), `many` (`decide_many`, `filter`), `order` (`rank`, `find`), and one each for `annotate`, `recognize`, and `relate`. The package names the verb. A first draft with one method per verb measured 1,252 production lines, over the 1,200 budget. The shared methods removed the repeated argument lists.
2. `decide`, `choose`, `score`, and `tag` each read the value of one `details_with` call. This serves runtime labels with no typed `Choice` and adds no send (decision 8, G5).
3. `recognize` and `relate` take keywords (`kinds`, `relations` as name to `(source, target)`, `either`, the thresholds) or a spec path or `dict` as the second argument (decision 10). `relate` reads `(name, kind)` pairs, dictionaries, or `Entity` values.
4. `tt.Engine` checks `throttle` itself before the builder (change 13). `cache=True` gives the default folder, `False` no cache, and any other value a folder.
5. Serde JSON is not a dependency. The package composes question-file JSON in Python and parses `Details::to_json` with `json.loads`.

## Budgets

| Budget | Limit | Measured |
|---|---|---|
| Production Rust, 5 files, each under 500 | 1,200 | 1,196 (engine.rs 425) |
| Rust unit-test lines | 250 | 250 |
| `__init__.py` and `__init__.pyi` | 460 | 353 |
| Python test files and lines | 12 and 1,800 | 7 and 649 |
| Conformance runner | 250 | 190 |
| `check.sh` and `build-wheel.sh` | 220 | 106 |
| Documentation, net | 260 | about 115 |

`ratchet.json` sets `src` at 1,300 (production and unit tests). `ratchet.py.json` sets `thinkthen` and `tests` at 1,082. The root ceiling does not change. The tag's duplicate code went first: its 1,703-line `lib.rs`, its generator, and its spec readers do not come across. Each block earns its lines as follows: the kind table and guard (`lib.rs`), the worker and tick (`worker.rs`), whole-list reading and the refusals (`input.rs`), the question and result values (`asked.rs`), and the engine settings and verb shapes (`engine.rs`).

## Error-index rows and plants

Each plant ran in a scratch copy of the worktree, rebuilt with `maturin develop` under the heavy lock, then restored. RED means the named test failed with the bug in place. A clean run passed before and after.

| Row | Plant | Result |
|---|---|---|
| R1-6 | pandas module check renamed | RED, `test_containers_are_refused_before_any_send` |
| R1-24 | `CallOptions::cancel` omitted | RED, `test_stopping.py`: the released workers send past the count |
| R2-11 | container refusal dropped from list reading | RED, same containers test |
| R3-18 | first verb taken | RED, `test_a_question_takes_one_verb` |
| R4-14 | new failing `tests/test_planted.py` | RED, pytest over `tests/` |
| R4-14 | wheel without `py.typed` | RED, "it lacks thinkthen/py.typed" |
| R4-23 single | the call waits for the worker before the tick | RED, `test_ctrl_c_stops_a_held_single_send_at_once` (30 s) |
| Change 3 | the worker joined, not detached | RED, `test_ctrl_c_stops_a_held_batch_at_once` (30 s) |
| Change 3 | internal token left uncancelled on a signal | RED, same test: sends past 8 after the release |
| R5-7 | `-1` refused | RED, `test_deadlines_follow_adr_0041` |
| R5-8 | bool check dropped | RED, same test |
| R6-11 | bare `TypeError` | RED, `test_wrong_questions_are_usage_errors_that_name_the_verb` |
| R6-13 | message fixed on `decide` | RED, same test |
| R2-25 | `Cancelled` for every stored error | RED, `test_a_handlers_system_exit_passes_through_unchanged` |
| R2-10 | any negative read as no deadline | RED, `test_deadlines_follow_adr_0041` |
| R1-34 | license field removed | RED, "its METADATA lacks License: MIT" |
| R4-19 and R5-34 | `--locked` dropped from `build-wheel.sh` | RED, the flag step |
| R3-30 | one case skipped silently | RED, "49 passed, 0 failed, 4 not run, of 54" |
| R1-31 | `Deadline` mapped to `BackendError` | RED, `each_kind_raises_its_own_class` |
| R2-31 | a second `catch_unwind` | RED, the count step |
| R1-10 | guard removed | RED, `a_panic_is_a_defect_and_the_next_call_runs` |
| R1-11 | `Duration::from_secs_f64` in the shim | RED, `test_deadlines_follow_adr_0041` |
| Change 8 | wheel built with `probe` | RED, "carries the test hook _live_workers" |
| Change 11 | throttle mapping dropped | RED, `test_throttle_eight_holds_exactly_eight_in_flight` |
| Change 11 | `base_url` keyword ignored | RED, `test_the_base_url_keyword_wins_over_the_environment` |
| Change 13 | throttle extracted straight into `u8` | RED, `test_bad_settings_are_usage_errors_that_send_nothing` |
| Change 14 | helper passes the parent's environment | RED, `test_the_child_environment_holds_the_fake_key_beside_loopback` |
| Change 14 | `unset` dropped, sentinel key set | RED, the pytest session check exits |

Not planted: the change 13 cache-seed plant ("the engine ignores `THINKTHEN_CACHE`") lives in `crates/thinkthen`, which this ticket must not touch. The test runs and passes.

A control run of `cargo test` with no plant passed, so the Rust REDs are the plants.

One clean sandbox run before the second plant batch reported 1 failure in 21 tests under `-x`, most likely the Ctrl-C `SystemExit` test while the load average was near 12. Fourteen later clean runs of the stop tests and six of the whole suite passed at load 11 to 15. The cause is not proven.

## Other acceptance

- Case 41 (offsets past an accent and an emoji) passes through Python with the Rust runner's `start` and `end`.
- The fork test warms the process engine, forks, and the child answers. The parent's counters do not move.
- The shim holds no `unsafe`.
- The fake key reached a loopback `http.server` listener as `Bearer sk-fake-loopback-python-0105`. No message or `repr` carries it or a URL credential.
- The 100 ms Ctrl-C and token tests act only after the backend's count shows the sends in flight.
- A background job starts its children with SIGINT ignored, and Python then installs no handler. The stop tests' children restore Python's default handler first. This was found when `check.sh` ran in the background.

## Deviations from the ticket

- `max_requests=2` over three texts: `EngineBuilder::max_requests` documents that a streaming call sends the records under the limit and then refuses. `decide_many` sent 2. The zero-send test uses `rank`, which refuses before any request.
- The Rust unit tests do not need `--test-threads 1`. Only one test starts a worker.
- A bare `cargo test` does not link, because the default feature builds the extension module without libpython. ADR 0047 item 6 says a bare `cargo test` runs every Rust test. The Python section of ADR 0047 now records the exception.
- `sdlc/scripts` is unchanged. The brief for this batch bars ladder-script changes, so the `sources` deny call and the `file://` plant in `lint` (amendment changes 1 and 9) are not built. `cargo deny ... advisories bans licenses sources` run by hand: sources ok.

## Findings

1. **Deny refuses pyo3's build dependency.** `target-lexicon 0.13.5`, a build dependency of `pyo3-build-config`, is licensed only `Apache-2.0 WITH LLVM-exception`. The root `deny.toml` does not allow it, so `surfaces --registry` fails on `libraries/python`: "licenses FAILED". Every pyo3 build carries it. A one-line crate exception fixes it: `exceptions = [{ crate = "target-lexicon", allow = ["Apache-2.0 WITH LLVM-exception"] }]`. Run in a scratch copy, that line passed the Python lock, the root lock (a warning for the unmatched exception), and the Rust binding's lock. The root file says a widened license list needs a record, so this build does not change it. Recommendation: accept the exception with the code review. The license is Apache-2.0 plus a linking exception that only widens use.
2. **The engine prints a large deadline in full.** `deadline=1e300` raises a message with a 301-digit number. The test pins it. It is readable but long. The engine owns the sentence.

## New dependencies for the code reviewer

The binding lock holds 70 packages. Beyond the root lock it adds `pyo3`, `pyo3-build-config`, `pyo3-ffi`, `pyo3-macros`, and `pyo3-macros-backend` at 0.29.2, `portable-atomic 1.15.0`, and `target-lexicon 0.13.5`. `thinkthen`'s own tree matches the root lock (`policy.py` passed). Deny: advisories ok, bans ok, sources ok, licenses failed on finding 1.

## Deferred

- The Polars and Arrow door (0106).
- The Python 3.10 floor stays untested in the gate. The test pins need 3.12.
- Release wheels, manylinux tags, and uploads (queue item 4).
- The `sources` deny call and its `file://` plant in `lint`, and the `target-lexicon` exception, wait for a ladder change.
