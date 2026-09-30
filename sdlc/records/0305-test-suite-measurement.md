# 0305: Test suite measurement

Status: measured on 2026-09-29 from main at `origin/main` in lane claude-3. Nothing in tests or code changed. Ticket: `sdlc/tickets/0305-measure-the-test-suite.md`.

## Conditions

The machine has 16 cores and 27 GB of memory. Another builder ran `cargo test` in a different lane throughout. The 1-minute load average stayed between 8 and 15 (18 once, during the nextest run). Builds used `-j6`. The gate scripts ran under `taskset -c 0-5`, because their allow-list strips `CARGO_BUILD_JOBS`. The global Cargo configuration uses sccache and the mold linker. Every run set `CARGO_NET_OFFLINE=true` and unset `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL`.

## Headline numbers

| Measure | Result |
| --- | --- |
| Cold build, fresh target folder, sccache off: `cargo test --locked --offline --workspace --all-targets --no-run -j6` | 27.7 s wall, 111 s CPU |
| Cold build, lane's first build, sccache on | 26.3 s wall |
| Rebuild after touching `crates/thinkthen/src/lib.rs` | 15.3 s |
| Rebuild after touching one integration test file | 0.4 s |
| Rebuild with nothing changed | 0.2 s |
| Test run, `cargo test ... --no-fail-fast`, warm build | 82.5 s wall |
| Same tests under `cargo nextest run --test-threads 8` (nextest 0.9.132 is installed) | 38.7 s wall |
| Test executables | 43. Of these, 41 hold tests: 37 integration binaries in `thinkthen`, 2 in `conformance-backend`, 1 library unit binary, 1 empty command unit binary |
| Tests run | 1262. 1225 passed, 37 failed, 19 ignored |

The run costs three times the build. Cargo runs test binaries one after another, so the run time equals the sum of the per-binary times (82.2 s). Seven binaries take 70 s of it.

The build does not suffer from the number of binaries. After a library change, the critical path is the library's own unit-test binary (15.1 s). The 41 integration binaries build beside it on the other jobs. Together they cost 28 CPU seconds, and all of them finish by 10.8 s. `backend` is the largest at 6.4 s.

## Failures seen

Ticket 0303 owns these. Under cargo, 37 tests failed: 24 in `backend` (cache identity, cache locking, recordings, tag matrix, profile, and others), 3 in `version`, 3 in `choose_and_score_edge`, and 1 each in the library (`engine::deadline_tests::a_held_folder_ends_as_the_deadline_without_the_owner`), `public_batches`, `find_edge`, `demo_runner`, `transform`, `key_address`, and `relate_edge`. `public_batches::portable::public_bulk_keeps_portable_max_bodies_and_row_identities` fails under cargo and passes under nextest, where each test runs in its own process. It depends on state that another test in the same process leaves behind.

## Per binary

These timings come from running each binary on its own with per-test times. The kinds follow the classes in the next section.

| Binary | Wall s | Tests | Kind | Why slow |
| --- | ---: | ---: | --- | --- |
| `backend` | 17.8 | 538 | command-line exact output, secrecy, timing | The secrecy sweep alone takes 13.2 s. Run on one thread, the binary takes 92 s |
| `conformance-backend` `listener` | 10.3 | 7 | test infrastructure | One test waits out a 10 s read timeout |
| `thinkthen` lib | 9.8 | 418 | engine and unit | The 16 MiB table test takes 9.8 s. Run on one thread, the binary takes 36 s |
| `public_controls` | 9.0 | 23 | timing, cancellation | Every test holds a `static SERIAL` mutex, so the binary runs one test at a time |
| `public_batches` | 8.4 | 45 | library, timing | Same `static SERIAL` mutex |
| `conformance-backend` `binary` | 5.0 | 19 | test infrastructure | One test waits out the 5 s `wait` limit |
| `compile_contract` | 9.9 cold, 0.2 warm | 1 | library compile contract | Runs `cargo check` on an outside crate. The first run compiles it |
| `find_edge` | 1.7 | 14 | command-line exact output | Tests the aggregate byte limit at its edge |
| `settings_cases`, `audit_refusals`, `public_estimated`, `public_size_retry`, `public_env`, `ca_bundle` | about 1 each | 2 to 15 | mixed | Spawn the command |
| `speed` | 0.8 | 1 | test infrastructure | Runs `probes/speed/measure.py` |
| 27 other binaries | 0.0 to 0.5 each, 2.3 in total | 1 to 31 | mostly command-line exact output | none |

## Slowest tests

Times come from runs on one thread, or from the parallel run where noted.

| # | Test | s | Cause |
| --- | --- | ---: | --- |
| 1 | `backend` `secrecy::no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence` | 13.2 | 518 command spawns |
| 2 | lib `cli::table::tests::encoded_header_and_records_hold_at_the_sixteen_mibibyte_edge` | 9.8 to 10.6 | Builds 16 MiB inputs six times in a debug build |
| 3 | `listener` `a_connection_that_ends_before_a_whole_request_takes_no_reply` | 10.3 | Waits out the backend's read timeout |
| 4 | `backend` `limits::a_record_of_exactly_the_limit_is_judged` | 5.4 (9.0 under nextest) | Pipes a 16 MiB record through the command twice |
| 5 | `public_batches` `a_batch_reads_its_input_at_most_one_throttle_ahead_of_its_rows` | 5.4 | Waits on the backend, then sleeps 300 ms |
| 6 | `public_controls` `a_stop_during_a_batch_or_a_cache_lock_wait_sends_nothing_new` | 5.4 | Sleeps 400 ms and waits on a cache lock |
| 7 | `backend` `parallel::every_number_of_jobs_prints_the_bytes_that_one_job_prints` | 5.0 | Six job counts over each input, one spawn each |
| 8 | `binary` `a_wait_line_gives_up_at_5_s_and_holds_up_no_line_behind_it` | 5.0 | A fixed 5 s limit |
| 9 | lib `engine::deadline_tests::a_held_reply_ends_as_the_deadline_after_one_send` | 4.6 | Real deadline wait |
| 10 | lib `cli::schedule::width_tests::every_live_path_and_every_engine_share_one_cap` | 3.8 | Many engines |

Next come `backend` `interrupt::a_signal_after_a_hung_request_is_the_stop` (3.1 s), `find_edge` `aggregate_byte_limit_accepts_the_exact_boundary` (4.6 s under nextest), and four more `engine::deadline_tests` of 2 to 2.5 s each. `backend` has 20 tests over 1 s and 47 over 0.25 s. The library has 8 over 1 s.

## Tests by kind

The counts are approximate. They come from grepping for `#[test]`.

| Kind | Count | Where |
| --- | ---: | --- |
| Command-line exact output | about 620 | `tests/backend/` (538), `tests/question_file/` (31), `*_edge.rs`, `audit*.rs` (46), `diff`, `transform`, `status`, `version`, `hints`, `key_address`, `settings_cases` |
| Engine and unit | about 400 | `src/core/**`, `src/engine/{usage,recorder,http,request}`, `src/cli/{failure,table,edge,schedule}` |
| Timing, deadline, concurrency, cancellation | about 110 | `src/engine/deadline_tests*`, `width_tests`, `src/cli/interrupt`, `backend/{interrupt,parallel,scheduling,timeout,backoff,resend,cache_locking,closed_pipe}`, `public_controls*`, `public_batches/{attempts,interactive}`, `polars/{deadline,lazy,throttle_equality}`, `conformance/backend/tests` (26) |
| Public API, package, compile contract | about 110 | `public_*.rs` and their folders, `polars/` (19, feature only), `compile_contract`, `ca_bundle`, `src/public/options/tests.rs` |
| Secrecy | about 25 | `backend/secrecy*.rs`, `backend/refusals.rs`, `question_file/secrecy.rs`, `audit_refusals`, the debug-line tests in `public_env` and `public_members`, `public_backoff` redirect, `status` key hiding |
| Wording of help, pages, and docs | about 25 | Listed in the cut list below |

## Exact output pinned below the command line

These unit tests pin a sentence or bytes that a command-line test or `conformance/cases.json` already pins. Each could assert the error kind or the typed value instead.

| Unit test file | Pinned text | Also pinned by |
| --- | --- | --- |
| `src/cli/failure/tests/diagnostics.rs` | "the recording folder could not be read or written…" | `backend/{default_cache_storage,recording_durability,cache_identity}`, `default_cache/prune` |
| `src/cli/profile.rs` | "threshold tuned for profile old is running under profile new" | `backend/profile.rs`, `profile/warnings.rs`, `batching/warning.rs` |
| `src/core/recording.rs` | "is not a recording entry…" | `backend/recordings.rs`, `secrecy/routes.rs` |
| `src/core/question_set/tests.rs` | "missing its `questions` object" | `backend/annotate.rs` |
| `src/core/question_file/resolve/tests.rs` | "takes 2 to 255 options" | `backend/refusals.rs`, `question_file/grammar.rs` |
| `src/core/adapters/systemone/response_distribution_tests.rs` | the tolerance sentence | `backend/distribution_total.rs` |
| `src/core/adapters/systemone/response_tests.rs` | "not a systemone response" | `backend/choosing.rs`, `tag/matrix.rs` |
| `src/core/records/tests.rs` | "over 16 MiB…" | `backend/limits.rs` |
| `src/cli/table.rs` | "the CSV header is not valid UTF-8" | `backend/table.rs` |
| `src/core/result/tests.rs` | the exact result JSON | the goldens in `backend/exchange.rs` |
| `src/core/adapters/systemone/request.rs` (10 tests) | exact request bytes against `specification/fixtures/systemone/` | the 54 exchanges in `cases.json`, via `src/cli/conformance_tests.rs` |

`src/cli/schedule/width_tests.rs` ("throttle 4 is already active") and `src/public/options/tests.rs` ("defect: the engine panicked below the public door") have no command-line twin. Keep them until a command-line test holds the sentence.

## Gate scripts

| Script | What it runs | Time here | Result |
| --- | --- | ---: | --- |
| `test` (routine) | About 22 separate `cargo test` calls on named filters, plus a `--list` check before each | 95.7 s | Passed. It ran 91 tests. That is slower than all 1262 tests under nextest |
| `lint` | Private-name scan, `policy.py` (9.1 s), about 20 checker self-tests and checkers (`tickets`, `pages`, `catalog`, `recognize-keys`, `rekey-model`, `children`, `versions`, `workflows`, `installer-test`, `named-answers`, `ratchet`, `surfaces --registry` with planted registries, `cargo deny` with a planted git source), fmt (1.1 s), clippy (13.5 s warm), doc (2.2 s), `inventory` | stopped at 0.2 s, then at 9.4 s | Failed twice. The first failure is a private-name hit on main at `sdlc/records/0299-token-cap-build.md:48`. The second run used a placeholder list outside the repo and failed in `tickets`, because the checker demands `## Evidence` in tickets 0303 to 0305. The phase's short-ticket rule conflicts with that checker |
| `spec` | Builds the command, runs `settings` with its self-test, the `spec/` pages, transform and fixture pages, probe replays, `demos-self-test`, and `demos` | stopped at 26.3 s | Failed at `demos/12-keep-going`. An extra warning appeared: "another user may change this named cache or recording folder". This is for 0303 to judge. It may come from the lane's folder permissions |
| `package` | Library-only build and graph, `cargo test --no-default-features --all-targets`, doctests, two private-surface builds in their own target folders, `cargo package` | stopped at 26.3 s | Failed on the same library deadline test |
| `demos` | Runs inside `spec`. Also holds prose rules: a vocabulary list, status-line and title form, and the `like ""` ban | inside `spec` | as `spec` |
| `surfaces` | Every landed surface's `check.sh` against one loopback backend, then release pack and smoke | not run | Needs 20 host toolchains. Surveyed instead |
| `test-full-cases` | The whole workspace, the external consumer, and `surfaces --full-functional` | not run | Its cost is this suite plus the full surfaces run |
| `test-stress` | 7 ignored Rust stress tests, plus `surfaces --stress` | not run | Opt-in only |

## Surfaces and `conformance/`

Nine surfaces replay `conformance/cases.json` in full: rust, c, python, ruby, typescript, r, duckdb, sqlite, and postgresql. The 11 C-door bindings read `cases.json` only to look up type cases: go, php, cpp, ada, cobol, objective-c, csharp, jvm, dart, swift, and zig. Each of those runs its own hand-written matrix with its own expected requests. None of the 11 replays `settings.json`. No surface reads `backend-profiles.json` or `record-values.json`.

These can switch to one shared replay driver, built like `libraries/c/tests/door/cases.rs`:

- `libraries/go/fixtures/accepted_requests.jsonl` with `run_matrix.py`, `libraries/php/fixtures/accepted_requests.jsonl` with `matrix.php`, and `libraries/cpp/fixtures/accepted_requests.jsonl` with `run.py`. Each keeps request bytes by hand, and `cases.json` already holds them.
- `libraries/objective-c/checks/matrix.m`, `libraries/zig/Tests/matrix.zig`, and `libraries/swift/Tests/fixtures/matrix.swift`.
- `libraries/zig/Tests/run_settings.py` can replay `conformance/settings.json`.
- `libraries/r/tests/threshold_strings.R` pins a request body that the `cases.json` threshold cases already hold. By a rough grep, about half of R's checks pin their own strings.

Some surface tests pin exact error sentences: sqlite `test_error_format.py`, ruby `test_errors.rb`, and python `test_inputs.py`. `cases.json` pins only the error kind. Either add a message field to `cases.json` or cut these tests to the kind.

Some surface checks police prose or lint the surface's own files:

- python `tests/test_inputs.py:68` and the NOTES.md check in python's `check.sh`
- the README and `index.d.ts` sentence grep in typescript's `check.sh`
- duckdb `tools/source_checks.py`, which pins README and ADR sentences
- postgresql `check.sh:425`, which compares the README grant block byte for byte
- self-lint greps in ruby, typescript, python, r, sqlite, and postgresql

## Cut list

Each row names what to cut, the reason, and what still covers the behavior.

| Cut or merge | Reason | Still covered by |
| --- | --- | --- |
| `version.rs`: six of its seven tests. Keep `version_flag_prints…` | They check wording in help and pages. `no_page_or_transform_says_unresolved` scans the specification, transforms, and READMEs for banned words. Three of these fail today | The version flag test. The help itself |
| `decide_edge/help.rs` (7), `tag_edge` help (2), `relate_edge::help_names_the_beta…`, `choose_and_score_edge::the_help_of_each_verb_carries_the_advice…`, `find_edge::help_leads…`, `diff::help_names_diff…`, `audit_verbs::help_names_audit…`, `transform::transform_help_pins_its_three_introductions`, `hints::choose_help_names_tag` | Help prose policing. Keep one test that short help hides `--model`, `--record`, and `--timeout` and does not show the retired flag | That one test |
| `backend/profile.rs::contract_pages_name_the_tuned_for_key_and_never_the_old_one` | Reads `specification/result.md` and the site. It fails today | None needed |
| `demo_runner.rs`: the 10 runner-fixture tests, with `tests/fixtures/demos-*` | They test the demo runner's page rules, not the product. `spec` already runs every green demo | `spec` |
| `speed.rs`, and `audit.rs::every_fixture_keeps_its_checksum` | Test infrastructure and a checksum table | Move `speed` to `spec` or drop it |
| `relate_edge::relate_reads_the_names_recognize_found` | Parses a command out of a demo README, so it breaks when the README is edited. It fails today | The demo itself, under `spec` |
| Portable max bodies in `backend/batching/portable.rs`, `public_batches/portable.rs`, `polars/batching.rs`, and `src/core/batch/tests/portable.rs` | The same cuts are checked four times | Keep the core unit test and the command-line test |
| Batch splitting in `backend/batching/{too_large,splits}` and `public_batches/{splits,native,details}`. Tiers in `backend/batching/tiers.rs` and `public_batches/tiers.rs` | Duplicates, and ticket 0304 replaces this path | One engine test and one command-line test after 0304 |
| Audit refusals in `audit_refusals.rs`, `audit_sets` refusals, and `audit_write` refusals. `audit.rs::old_goldens_hold` against `diff.rs::goldens_match` | Duplicates | One refusal table and one goldens test |
| The 16 MiB edge in `src/cli/table.rs::encoded_header_and_records_hold_at_the_sixteen_mibibyte_edge`, `backend/limits.rs::a_record_of_exactly_the_limit_is_judged`, `find_edge::aggregate_byte_limit_*`, and `src/core/records/tests.rs` | The same edge four times, and 20 s of the run | `src/core/records/tests.rs` and one command-line edge. Or pass the limit as a parameter so a test can use a small one |
| Secrecy: `backend/refusals.rs::no_refusal_on_any_command…` and `question_file/secrecy.rs::no_message_from_either_home…` | Overlap the main sweep's route table in `secrecy/routes.rs` | Add their routes to the one sweep. Keep all distinct secrecy regressions until the replacement lands, as AGENTS requires |
| Retry and backoff counts in `backend/backoff.rs`, `public_backoff.rs`, and `public_size_retry.rs` | Overlap | One engine table and one command-line test |
| Exact sentences in the unit tests in the table above | Exact output belongs at the command line | Assert the kind or the typed value |
| The `static SERIAL` mutexes in `public_controls.rs` and `public_batches.rs` | They cost 17 s of run time. They exist because the engine keeps process-wide state. Plan step 4 removes it | The same tests, run in parallel |
| Sleeps and fixed timeouts: `listener` 10 s, `binary` 5 s, `public_batches` 300 ms sleep, `public_controls` 400 ms sleep | Wall-clock waits | Make the backend's read timeout and `wait` limit settable in tests, and signal with `round` or `wait` instead of sleeping. Or move them to `test-stress` |
| `sdlc/scripts/test` routine list | 96 s for 91 tests, slower than the whole suite under nextest | The fast suite below |
| Checker pedantry in `lint`: `tickets` (it already conflicts with the phase), `pages`, the `demos` vocabulary and form rules, `demos-self-test`, `pages-self-test`, and the planted-registry cases in `surfaces --registry` | They police wording and process. Ian's ruling 8 favors cutting them. AGENTS says never weaken lint tables, so the coordinator needs Ian's word, or a reading of ruling 8, before this row | `policy.py`, clippy, fmt, doc, and deny |

The cut list removes about 60 tests outright. The duplicate merges remove perhaps another 60 to 80 once ticket 0304 and plan step 3 settle the one batching path.

## Consolidating binaries

- Merge the 37 `thinkthen` integration binaries into two. `cli` takes `backend`, `question_file`, the `*_edge` files, `audit*`, `diff`, `status`, `version`, `hints`, `transform`, `key_address`, `settings_cases`, `ca_bundle`, and `dry_run_terminal`. `library` takes `public_*` and `compile_contract`. Merge the five `polars_*` feature binaries into one. Merge `conformance-backend`'s `binary` and `listener` into one.
- Expected gain: most of the 28 CPU seconds of integration-test compile and link, on a busy machine or at low `-j`. Incremental wall time barely moves, because the library's unit-test binary (15 s) is the critical path. The bigger win is the run: one binary shares one thread pool instead of 41 binaries running back to back. Nextest gives the same run win today with no code change.
- The merge needs the `SERIAL` mutexes gone first, or they will serialize the whole `library` binary.

## Proposed fast suite

This runs on every landing:

1. `cargo nextest run --locked --offline --workspace --all-targets`. It runs every functional test, with no hand-picked routine list. Stress tests stay `#[ignore]`.
2. `python3 sdlc/scripts/policy.py`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets -D warnings`.

Today on a warm cache this costs about 15 s of rebuild, 39 s of tests, and 24 s of lint, where lint is policy 9 s, clippy 13.5 s, and fmt 1 s. Lint can run beside the tests. The target is under 30 s of test run and under 60 s in total. Three things get there: the cut list, removing the sleeps and fixed timeouts so that no routine test runs over 2 s, and removing the `SERIAL` mutexes. The longest test sets the floor under nextest, and today it is the 13 s secrecy sweep. Split that sweep into one test per route family so that nextest can spread it.

`spec` and `demos` run at batch checkpoints, not on every landing. `surfaces` runs the 31 routine cases only for a surface whose files changed. The full surfaces run belongs to `test-full-cases` at release checkpoints.

## Commands

```sh
uptime
CARGO_NET_OFFLINE=true cargo test --locked --offline --workspace --all-targets --no-run -j6 --timings
RUSTC_WRAPPER= CARGO_TARGET_DIR=<scratch>/cold cargo test --locked --offline --workspace --all-targets --no-run -j6 --timings
touch crates/thinkthen/src/lib.rs && cargo test --locked --offline --workspace --all-targets --no-run -j6 --timings
CARGO_NET_OFFLINE=true cargo test --locked --offline --workspace --all-targets --no-fail-fast -j6
# each binary alone, per-test times on the stable toolchain:
RUSTC_BOOTSTRAP=1 <test-binary> -Zunstable-options --report-time [--test-threads=1]
cargo nextest run --locked --offline --workspace --all-targets --no-fail-fast --build-jobs 6 --test-threads 8
THINKTHEN_HEAVY_LOCK=<scratch>/heavy.lock taskset -c 0-5 sh sdlc/scripts/{test,lint,spec,package}
python3 sdlc/scripts/policy.py; cargo clippy ... -j6; cargo doc ... -j6; cargo fmt --all -- --check
```
