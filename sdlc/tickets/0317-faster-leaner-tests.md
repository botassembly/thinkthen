# 0317: Faster, leaner tests

Status: landed. Plan: `sdlc/planning/cleanup-2026-09-30.md`, step 5.

## Outcome

The landing suite runs in under 30 seconds of test time on a warm build, with no routine test over 2 seconds. Duplicate and wording-only tests are gone. Behavior coverage stays.

## Evidence

- Starts from: record 0305 (1,262 tests, 39 s under nextest, cut list), ticket 0303's deletions, and tonight's landings.
- Keeps: parser, secrecy, cancellation, cache-miss, invalid-input and conflict regressions; command-line exact-output tests; the conformance cases.
- Changes: split the 518-spawn secrecy sweep; keep one 16 MiB edge test instead of four; merge duplicate batch-limit, audit-refusal and retry tests; delete exact-sentence tests below the command line that the command line or `conformance/cases.json` already pins; cut fixed sleeps and long timeouts to the shortest that still proves the behavior. Run `ceiling.rs` `a_later_ordinary_command_request_is_refused_by_the_process_cap` at `--jobs 1`; at `--jobs 4` its second record sometimes reserves first (closed issue `2026-09-30-process-cap-test-races-two-records.md`).
- Proof: nextest time and count before and after, under stated load; each deletion names the test that still pins its behavior.
- Defers: a command-line test that a record of exactly 16 MiB is judged; `cli::edge` and `core::records` unit tests cover the exact-limit acceptance. The shared test locks in `public_controls` and `public_batches`, which go with the process-wide statics after ADR 0111 slice 3; merging test binaries; surface package tests replaying `conformance/cases.json`.

## Build

Measured with `cargo nextest run --locked --offline --workspace --all-targets --no-fail-fast --build-jobs 6 --test-threads 8` on a warm build, 16 cores. Other lanes kept the machine busy throughout.

| Run | Tests | Wall | Summed test time | 1-minute load |
| --- | ---: | ---: | ---: | --- |
| Before, `origin/main` at `dd72e15e8` | 1,268 | 49.0 s | 304 s | 16.6 to 16.9 |
| After, run 1 | 1,256 | 34.2 s | | 13.3 to 19.6 |
| After, run 2 | 1,256 | 29.3 s | 226 s | 19.6 to 21.6 |
| Midway, before the last cuts | 1,256 | 27.0 s | | 12.7 to 14.7 |
| After rebase on `e5d9b8ec9` | 1,259 | 37.0 s | | 21.1 to 29.6 |
| Final, after review fixes, through `sdlc/scripts/test` | 1,260 | 14.3 s | | 16.2 to 17.8 |

The longest test fell from 23.5 s (the secrecy sweep) to 5.4 s at load 20, and to 9.0 s at load 30. At this load 26 tests still take over 2 s. Most sit in areas the ADR 0111 slice 2 lane owns, listed under deferred below.

| Change | What still pins the behavior |
| --- | --- |
| `secrecy::no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence` split into 17 `secrecy::no_leak_on_*` tests, one per route, with the same 518 rows. `every_route_has_its_own_sweep` fails on a route with no test, on other than one hostile route, or on a case total other than 518. The unreachable routine selection is gone, since `test` unsets the profile | The split tests |
| `backend/limits.rs` `a_record_of_exactly_the_limit_is_judged` deleted | `cli::edge` `a_record_of_exactly_the_limit_arrives_whole_with_its_ending`, `core::records` `a_record_over_the_limit_is_refused_and_one_at_the_limit_is_taken`, and the command-line over-limit refusals in `limits.rs` |
| `cli::table` 16 MiB edge cut from 22 parses to 8: the BOM with `\r\n` header, `\r\n` and no ending records, one quoted record | The same test |
| `listener` silent-connection case (8 s read timeout) and `binary` `a_wait_line_gives_up_at_5_s…` moved to `test-stress` | The same tests under `test-stress --run` |
| `public_batches` `a_batch_reads_its_input…` and `public_controls` `a_stop_during_a_batch…` wait for 1 held send. They waited for 2 and 4 and always timed out at 5 s | The same tests. Filed the bug `sdlc/issues/2026-09-30-public-batch-holds-one-send-under-a-throttle.md`. The ignored `public_batches::a_batch_keeps_a_throttle_of_sends_in_flight_while_they_are_held` asserts the throttle's worth and fails today |
| `ceiling.rs` process-cap case runs at `--jobs 1` | Passed 30 stress runs |
| Help prose: 5 in `version.rs`, 5 in `decide_edge/help.rs`, 2 in `tag_edge.rs` (file gone), 1 each in `choose_and_score_edge`, `audit_verbs`, `relate_edge`, `find_edge`, `diff`, `hints`, `transform` | `decide_edge/help.rs` keeps the short-and-long help test and the retired-flag test |
| `profile::contract_pages_name_the_tuned_for_key_and_never_the_old_one` | Prose check of a page; nothing needed |
| `demo_runner.rs` 5 runner-fixture tests and their fixtures | `spec` runs the demo runner; `every_recorded_demo_runs…` stays; `a_page_whose_assertion_is_wrong_fails_the_run` and `fixtures/demos-wrong` stay, so the runner is still shown to fail |
| Sentences below the command line cut to the kind: `cli/profile.rs`, `question_set`, `question_file/resolve`, systemone response and distribution tests, `core/records`; `cli::table` `invalid_utf8_names_only_its_location` deleted | `backend/profile.rs`, `backend/annotate.rs`, `question_file/grammar.rs`, `backend/distribution_total.rs`, `backend/choosing.rs`, `backend/limits.rs`, `backend/table.rs` |

Kept after inspection: `audit::every_fixture_keeps_its_checksum` guards the provenance of goldens whose source history is gone. The audit refusal tables in `audit_refusals`, `audit_sets`, and `audit_write` hold different refusals, and `audit::old_goldens_hold` and `diff::goldens_match` test different commands. `find_edge` `aggregate_byte_limit_*` tests `find`'s own limit across units. `core/result/tests.rs` and the systemone `request.rs` tests stay: they are serialization contracts, and no case-by-case map to `cases.json` exists yet.

Deferred because the ADR 0111 slice 2 lane owns the area or step 3 changes it: the portable, split, and tier batching duplicates (each door still has its own batching code until step 3); retry and backoff counts; `engine::deadline_tests`; `cli::schedule::width_tests`; `backend/{parallel,keeping,batching,relate,interrupt}` slow cases; the recording sentences in `cli/failure/tests/diagnostics.rs` and `core/recording.rs`.

Checks: `sdlc/scripts/test`, workspace clippy with `-D warnings`, `policy.py`, `tickets`, and `lint` pass. The ratchet falls by 626 lines, from 106,093 to 105,467 after merging main. The restored guards, runner fixture test, and known-failing throttle test add back 53.

## What the build taught us

- A wait that gives up silently hides a bug. Two tests waited for more held sends than the batch ever made, timed out at 5 s, and passed. Assert the count a wait returns.
- Splitting one test into many drops the guards that sat in its body. The split sweep needed its hostile-route and case-total checks restated in the route guard.
- A runner check needs one failing fixture. Without it, nothing shows the runner can fail.
- Load swings timing more than the cuts do. Record the 1-minute load beside every timing.
