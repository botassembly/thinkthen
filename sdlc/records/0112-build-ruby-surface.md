# 0112: Build record for the Ruby surface

Status: built on `ticket/0112-port-ruby-surface`, awaiting code review and landing. `libraries/ruby/check.sh` passes. Built by Claude on 2026-09-25 in the surface batch (`sdlc/planning/one-line-plan-2026-09-25.md`).

## Result

- The folder came from tag `surfaces-wave7-frozen-2026-09-24b`. The stand-in imports, the connector, the Dockerfile, the fixture build, `ENGINE_NULL`, the wire stub, `CountingStub`, `scripts/check_public_names.py`, and the skip-table reader are gone.
- `thinkthen-ruby` is magnus 0.7.1 over rb-sys 0.9.130 in its own workspace. `src/ffi.rs` holds every `unsafe` line and the crate's one `missing_docs` allow. `src/lib.rs` holds the guard, the kind table, the handoff, the worker start, and the settings. `src/call.rs` maps each verb to the public API. Production Rust is 771 nonblank lines in three files, each under 500.
- `ThinkThen::Engine.new(base_url:, model:, throttle:, max_requests:, cache:, cache_bytes:)` starts from `EngineBuilder::from_env()`. Width is now throttle everywhere.
- The lock adds exactly the ticket's 22 crates. `cargo deny --offline check advisories bans licenses sources` with the root `deny.toml` printed "advisories ok, bans ok, licenses ok, sources ok". `policy.py`'s binding checks pass, and `surfaces --registry` passes with Ruby landed.
- Nothing was downloaded. The pinned Ruby 3.4.11 prefix and its stamp already existed from experiment 256's setup. Every crate was in the cargo cache.

## The check

`check.sh 0` under the shared lock, 2026-09-25, after the code review's fixes: exit 0.

| Step | Result |
| --- | --- |
| File checks (R7-10, R4-9, R7-3, R4-19, R3-32, R2-31, R4-2, R2-8) | pass |
| deny on the lock, and the `file://` plant | ok; the plant exits 8 with `source-not-allowed` |
| Toolchain probe | Ruby 3.4.11, stamp matches `toolchain.env` |
| `build.sh`, `cargo fmt --check`, Clippy `-D warnings` | pass |
| `cargo test --lib` | 4 passed |
| 9 Ruby test files | 34 tests, 0 failures |
| Conformance | 49 passed, 0 failed, 5 not run, 54 of 54 |
| Examples | 10 of 10 |
| Gem check, slide sample | pass |

Budgets: 13 Ruby test files and 1,139 nonblank lines, the runner 196 of them. `thinkthen.rb` and `version.rb` 387. Scripts 178. `ratchet.json` holds 791 and `ratchet.rb.json` holds 1,526, the measured totals.

## Plants

Each plant was applied to the committed source, built when it touched Rust, and run against the named test alone. Git then restored the source and the build was redone. Git's restore rewrites each file with a new modification time, so Cargo rebuilt after each one. As a further guard, the R4-2 plant was rerun after touching the sources and cleaning the crate. The planted library held `rb_thread_call_with_gvl` and the test stayed green on 3 of 3 runs. The restored library holds no such call. `check.sh` then passed again from a clean crate build.

| Row | Plant | Result |
| --- | --- | --- |
| R1-20 | Join the worker after a stop | red: the batch's `Thread#raise` never arrived within the child's 30 s read; the single Ctrl-C test timed out too |
| R1-20 | Leave the own token unfired | red: count past 8 after release |
| R5-9 | No slice and no unblock function | red: Ctrl-C missed 150 ms |
| R4-2 | Retake the lock inside the released region and run `rb_thread_check_ints` there | green on 3 of 3 runs. The worker never touches Ruby, so this plant was harmless (review ruling, below) |
| R4-2 | `rb_thread_check_ints` with no lock, in `wait_for` | green on 3 of 3 runs |
| R4-2 | `rb_thread_check_ints` with no lock, in the unblock function `wake` | red on 3 of 3 runs: the flood round's raise never reached the parent |
| R4-2 | `rb_thread_call_with_gvl` in `src` | red: "src retakes the VM lock inside the released region" |
| R2-15 | A wake closes the handoff | red: the held batch raised |
| R2-15 | The caller's token is the engine's cancel | red: the sibling cancelled |
| R3-5 | Row removal outside `ensure` | red: 3,000 rows left; after the review's change, red with the tick running 9,000 more times with no call in flight |
| R3-5 | Drop `rb_protect` around `rb_thread_check_ints` | green on the storm; red on the batch raise test, which counted 200 sends, not 8 |
| R3-15b | Check only the first record's keys | red |
| R3-16 | A tick kept on the module | green at first; red after the test fix below (a tick ran 5 times on another thread's call) |
| R3-16 | `nil` crosses as `"null"` | red |
| R7-9, R2-8, R4-18 | The row holds the tick through a `WeakRef` | green at first; red once the plant also drops the crossing's local. The review then retired these rows (below) |
| R7-9, R2-8, R4-18 | An `Opaque<Value>` in a wrapped struct; a `Value` field in a wrapped struct | red: "src holds a Ruby object"; "a wrapped struct above holds a Ruby Value" |
| R5-35 | A stub prefix with the real stamp and a `ruby` that prints 3.2.3 | red: "is not Ruby 3.4.11 behind a matching stamp" |
| R5-35 | `RUBY=/usr/bin/ruby` | red: "RUBY names /usr/bin/ruby" |
| R3-28 | A `file://` git source in a scratch crate | `check.sh` plants it on every run and requires exit 8 with `source-not-allowed` |
| R5-14, R6-12 | Make `from_native` public | red |
| R2-10 | Any negative deadline means none | red |
| R1-11 | The shim converts with `Duration::from_secs_f64` | red: `DefectError` in place of `UsageError` |
| R2-25 | Records cross through `to_s` | red: `{note: "refund"}` |
| R1-10 | Remove the guard | red: the unit test panicked |
| R1-31 | `Deadline` maps to `BackendError` | red |
| Settings | `Engine::builder()` in place of `from_env()` | red: the child raised, because the plain builder has no key; no count was reached |
| Settings | Drop the throttle mapping | red: 4 in flight, not 8 |
| Settings | Read the variable after the keyword | red: counts `[1, 0]` |
| Tick | No token fire from a raising tick | red: the tick's error missed its bound |
| R3-30, R5-32 | Skip one case silently | red: 53 of 54 |
| R7-3, R4-9, R7-10, R4-19, R2-31, R3-32 | One plant each in `check.sh`, `Cargo.toml`, a Dockerfile, `build.sh`, and `src` | red, each with its own line |
| R1-29 | Change one hash in `toolchain.env` | exit 77, "not run" |
| R1-34 | Remove the license | red: "the gem is not MIT" |

Two tests changed after their plants stayed green.

- The own-thread tick test, now in `test_tick_thread.rb`. The busy thread set its tick and called before the idle thread set one, so a module-held tick still read the busy thread's tick. Now the idle thread sets its tick first, and the held call runs on a thread with no tick.
- The `WeakRef` plant. The in-flight `crossing` frame holds the tick in a local, so the row is not the only root in this design. The plant now also clears that local, and the collector then takes the tick. The design has two roots on purpose. The first plant as written cannot fail here.

## Stop rules crossed

- R4-2: the ticket says the builder stops if three runs of the lock-retake plant stay green. They did, and the builder stopped. The code review ruled that the worker never touches Ruby, so that plant was harmless. `check.sh` now fails when `src` names `rb_thread_call_with_gvl`. An interrupt check with no lock in `wake` turns the flood test red, so `test_flood.rb` keeps its R4-2 half with that plant as proof.

## After the code review

The review of `2001843e` found the extension sound and returned eight findings. All are fixed.

- The deny run and the `file://` plant moved into `check.sh`, after the TypeScript surface's form. `lint` reaches Ruby only through `surfaces --registry`. The ticket's R5-35 row now says so.
- A wrong Ruby behind a matching stamp fails. It no longer prints "not run".
- The collector test left. R2-8, R7-9, and R4-18 closed by design: Rust holds no Ruby object. `check.sh` fails when `src` names `Opaque`, `BoxValue`, `rb_gc_register`, or a `Value` field in a `#[magnus::wrap]` struct. The own-thread tick test (R3-16) stays, in `test_tick_thread.rb`.
- The gemspec raises when the extension is missing. The gem check requires exactly `lib/thinkthen.rb`, `lib/thinkthen/thinkthen.<DLEXT>`, and `lib/thinkthen/version.rb`, and `ThinkThen::VERSION` equal to the crate's version.
- The raise storm reads no private state. A counting tick must not run in the 0.3 s after the storm and `GC.start`.
- `relate` is no longer capped by the record limit, because the engine does not cap it.
- `TestBackend.run` returns the child's stderr. The secrecy test writes every message it saw there, adds a cancelled path and a deadline path, and requires stderr to hold neither the fake key nor `hunter2`.
- Ticket decision 9 is amended for `Question::from_json`.

## Deferred gaps

- A fork while a detached worker is in flight. The fork test forks between calls only.

## Deviations

- Decision 9: the keyword builders write question-file JSON and call `Question::from_json` for every verb, in place of `choose_labels` and `tag_labels`. One route keeps keywords and files on one digest.
- The runner reads main's 54 cases. The ticket's case numbers 17, 18, and 68 came from the tag's file. Main's counters case is 40, and its accent-and-emoji case is 41. Both pass. Main has no cancel-mid-batch case. The held-arm tests cover that.
- Five cases do not run, each with its reason printed: `18-find-second` and `19-find-none` (`none: true`), `18-annotate-two-groups` (a record read in parts), `25-defect-fault` (the unit test covers the guard), and `30-local-question-file` (no question-file loader in Ruby).
- `test_errors.rb` keeps only the set boundaries. The runner's error paths cover each fault kind once.
- The binding refuses a list longer than `max_requests` before any send. The engine's lazy batch sends up to the limit first, as documented.
- The slide's drawn block runs unchanged. Only its harness changed. The generic arm answers every text yes at 0.9, so filter keeps 8 of 8, rank keeps input order, and score lands nearest "Routine.". The backend counts 15 sends.
- `examples.json` was re-derived against the generic arm. `recognize` now reads `name`, and `relate` takes `[name, kind]` pairs and returns `Edge` ends.
- The error index was not edited. This record carries the rows.

## Engine findings

- A cache folder binds to the first backend address that wrote it. Another address on that folder raises `LocalError`. The tests use `cache: false` on other arms.
- A forked child's counters start at zero (0096). The fork test pins that.
- A cancelled token does not end requests already sent. That is why joining the worker stalls. It matches 0073.
