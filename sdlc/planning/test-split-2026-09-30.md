# Test split between the gate and the release suite, 2026-09-30

Status: inventory for ticket 0335. Measured on `origin/main` at `2b3cb67ea` in lane claude-4. Nothing in tests or code changed. The issue is `sdlc/issues/2026-09-30-split-tests-between-the-gate-and-release-qa.md`. Ian can overturn each grouping.

## Conditions

The machine has 16 cores. Other lanes built and tested throughout. The 1-minute load average was 12.9 at the start of the run and 19.4 at its end. The run held the lane's own heavy lock. `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` were unset, and `CARGO_NET_OFFLINE=true`.

## `sdlc/scripts/test` today

One run of the whole script took 82.5 s wall, 141 s user CPU and 44 s system CPU. It passed.

| Step | Wall s | Tests |
| --- | ---: | ---: |
| Library rebuild after the fetch, then `cargo nextest run --workspace --all-targets` | 23.7 build, 17.5 run | 1,289 run, 24 skipped |
| `cargo test --doc` | 1.7 | 1 |
| External consumer, `conformance/consumer` (under `cargo test`, one binary at a time) | 10.3 | 18 run, 3 ignored |
| `probes/find-0040/self-test` | 16.7 | one script |
| `transforms/sweep/test.sh` | 5.1 | one script |
| `sdlc/live-test` | 2.9 | one script |
| The other ten shell self-tests together | 6.1 | ten scripts |

The step times other than the nextest run come from a second warm pass of the same commands at load 22. Nextest counted 43 binaries, 40 of them holding tests. Its summed test time was 244 s over 17.5 s wall.

### Nextest binaries

| Binary | Tests | Summed s |
| --- | ---: | ---: |
| `thinkthen::backend` (command line, exact output) | 556 | 158.5 |
| `thinkthen` library unit tests | 430 | 37.0 |
| `public_batches` | 43 | 3.9 |
| `question_file` | 31 | 1.9 |
| `public_controls` | 23 | 4.2 |
| `public_env` | 22 | 3.3 |
| `decide_edge` | 18 | 0.7 |
| `conformance-backend` `binary` and `listener` | 22 | 6.0 |
| `audit`, `audit_cases`, `audit_model`, `audit_output`, `audit_refusals`, `audit_sets`, `audit_verbs`, `audit_write` | 45 | 6.2 |
| `find_edge`, `diff`, `choose_and_score_edge`, `status` | 42 | 4.1 |
| 19 other binaries of 1 to 11 tests | 57 | 18.3 |

The tests over 3 s are `backend` `keeping::permutation::filter_prints_a_subsequence…` (8.1 s), `parallel::every_number_of_jobs_prints_the_bytes_that_one_job_prints` (6.4 s), `compile_contract` (4.7 s), `engine::deadline_tests::a_held_reply_ends_as_the_deadline_after_one_send` (4.6 s), `cli::table` 16 MiB edge (4.5 s), `secrecy::no_leak_on_a_hostile_entry` (4.1 s), `cli::schedule::width_tests::every_live_path_and_every_engine_share_one_cap` (3.8 s), `relate::ceiling::a_full_line_set_plans_inside_the_child_deadline` (3.2 s) and `interrupt::a_signal_after_a_hung_request_is_the_stop` (3.1 s).

## Groups

### Dense-logic unit tests to keep

About 400 of the 430 library unit tests: question parsing (`core::question_file`, `core::question`, `core::question_set`), thresholds (`core::threshold`), digests (`core::digest`), batch packing (`core::batch`, `core::pack`), record framing (`core::records`, `cli::edge`, `cli::table`), the wire encoder and adapters (`core::adapters`), answers and measures (`core::answer`, `core::measure`), usage totals, the recorder, and the store. Each is a table or an edge the command line reaches only slowly. `engine::deadline_tests` (17 tests, 14.7 s summed) and `cli::schedule::width_tests` keep their cases and wait for the static cleanup to lose their long waits.

### Outside-in mechanics to keep

- The command line in `backend` and `question_file`: exit codes, exact sentences, refusals, batching, cache and recording rules, streaming, interrupts and secrecy. The split secrecy sweep (`secrecy::no_leak_on_*`, 17 tests, 518 rows) stays whole.
- The Rust library in `public_*`, `compile_contract` and the external consumer.
- The contract replays: `backend` `loopback_cases::every_wire_case_passes_through_the_command_on_the_conformance_backend`, `cli::conformance_tests`, and the consumer's `cases::every_applicable_shared_case_passes_through_the_public_api`.
- `speed::the_speed_gate_holds_and_names_each_fault`. It counts the requests each function sends through the command. That is batching behavior, not wording.
- `demo_runner::a_page_whose_assertion_is_wrong_fails_the_run`. `demos-self-test` has no case for a wrong assertion, so this is the only proof that the runner can fail.
- The `transforms/*/test.sh` tables and `sdlc/live-test`. They prove shipped `jq` files and the paid-call ledger.

### Duplicates to merge or delete

| Test to delete | Kept test that catches its regression | Proof before deletion | Waits for |
| --- | --- | --- | --- |
| `demo_runner::every_recorded_demo_runs_and_every_demo_still_red_is_skipped` | `spec` runs `sdlc/scripts/demos` over the same pages. A broken demo then fails at a `spec` checkpoint, not in `test` | none needed | nothing |
| `public_batches/portable.rs::public_bulk_keeps_portable_max_bodies_and_row_identities` | `backend` `batching/portable.rs` `complete_cli_lines_keep_literal_question_bytes_and_keys` and `structured_cli_values_keep_order_and_question_bytes`, plus `public_batches/identity.rs` for row identity | a mutation of the portable cut and of row identity, killed by the kept tests | 0304 slice 3a |
| `polars/batching.rs::portable_max_cuts_cross_public_series_and_frame_calls` | The same command-line pair, since Polars reaches the one `ask_all` path | the same mutation run, under `--features polars` | 0304 slice 3c |
| `public_batches/splits.rs::named_group_failed_left_half_does_not_send_right` | `public_batches/splits.rs::a_failed_left_half_prevents_a_right_send_and_row`, which holds the same no-right-send rule and also checks the row | a mutation that sends the right half, killed by the kept test | 0304 slice 3a |
| `public_batches/tiers.rs::saved_annotate_batch_tiers` | `backend` `batching/tiers.rs::the_batch_setting_follows_its_tiers` | a mutation of one tier edge, killed by the kept test | 0304 slice 3a |
| `public_size_retry.rs::retry_visibility_counts_live_attempts_and_no_replay_attempt` | `backend` `backoff::status_counts_retries_as_a_subset_of_actual_sends` for the counts, and `public_size_retry.rs::retry_attempts_use_send_ordinals_and_reject_duplicate_headers` for library attempts | mutations of the retry count and of the replay attempt, killed by the kept tests | 0304 slice 3d |
| `backend` `public_json::each_json_method_prints_the_commands_bytes_on_the_shared_cases` | The consumer's `cases::every_applicable_shared_case_passes_through_the_public_api`, after it gains the byte comparison with the command | a mutation of one JSON field, killed by the consumer replay | 0304 slice 3a |

A slice deletes a row only when its mutation run is recorded. A mutation the kept test misses keeps the old test.

These stay after review, because a merge would drop a retained regression:

- `backend` `refusals::no_refusal_on_any_command_writes_the_key_quotes_the_evidence_or_sends_anything`. It checks each of 40 refusals on every verb for its own exit code, its own sentence, empty standard output and zero connections. The sweep's route holds one code and one sentence. Two recorded mutation runs rely on it.
- `question_file` `secrecy::no_message_from_either_home_ever_carries_the_key_or_the_evidence`. It forbids the evidence on standard output too, and each row brings its own question file. The sweep allows the evidence on standard output.
- `public_size_retry.rs::retry_attempts_use_send_ordinals_and_reject_duplicate_headers`. It withholds a request id equal to the key, refuses duplicate headers, drops a forbidden header, and runs the observer on the caller's thread.
- `public_batches/splits.rs` `named_group_refused_parent_and_halves_keep_request_identity` and `a_failed_left_half_prevents_a_right_send_and_row`, and `details.rs::split_record_details_name_the_refused_parent_and_the_answering_half`. They pin library request identity, the cancellation of the right half, and the details view.
- `relate_edge::relate_reads_the_names_recognize_found`. It keeps the name-versus-text regression. Slice 1 rewrites it to pass the `--kind` list as fixed arguments instead of parsing demo 44's README.

No parser, secrecy, cancellation, cache-miss, invalid-input or conflict regression goes before its replacement lands.

### Checks that move out of the landing gate

These are per-function checks on every surface, installed-package checks and finished-probe checks. None runs in `test` except the probes.

| Check | Moves to | Waits for |
| --- | --- | --- |
| `probes/find-0040/self-test` (16.7 s) and `probes/probability-total-0038/self-test` (0.6 s). They prove the harnesses of two finished paid probes, not the product | `spec`, beside the other checkpoint pages | nothing |
| The full `conformance/cases.json` replay on every surface (`surfaces --full-functional`, `test-full-cases`) | The release suite, on release candidates | the release suite running bindings |
| The hand-written per-function matrices: `libraries/go/fixtures/run_matrix.py` with `accepted_requests.jsonl`, `libraries/php/fixtures/matrix.php` with `accepted_requests.jsonl`, `libraries/cpp/fixtures/run.py` with `accepted_requests.jsonl`, `libraries/objective-c/checks/matrix.m`, `libraries/zig/Tests/matrix.zig`, `libraries/zig/Tests/run_settings.py`, `libraries/swift/Tests/fixtures/matrix.swift`, `libraries/r/tests/threshold_strings.R` | The release suite. `cases.json` already holds their request bytes | the release suite running bindings |
| The installed-artifact branch of each `check.sh` (`THINKTHEN_ARTIFACT`) and the library families of `release-smoke` | The release suite, from the release files | the release suite running bindings |

The routine `surfaces` rung keeps the 31 cases in `conformance/routine-ids.txt` per binding.

### Wording-only checks to delete

Ticket 0317 removed the wording tests from the Rust suite. The rest sit in the binding checks that `surfaces` runs.

- `libraries/python/tests/test_inputs.py::test_the_deadline_sentence_is_pinned_in_the_docstring_and_readme`
- The `NOTES.md` step in `libraries/python/check.sh`, which checks that tests named in a notes file exist
- The deadline-sentence grep over `README.md` and `index.d.ts` in `libraries/typescript/check.sh`
- The R5-23 README sentence and R2-29 ADR sentence rows in `databases/duckdb/tools/source_checks.py`. Its code rows stay.

`databases/postgresql/check.sh` `readme_grant_is_the_fixture` stays. It keeps the README's security grant equal to the fixture that `the_grant_reaches_the_extension_alone` runs.

## Counts

- Delete as wording-only or duplicated by `spec`: 6. One Rust test, one Python test, and four check steps.
- Merge into kept tests: 6 batching, retry and JSON tests, after 0304 slice 3, each with a recorded mutation run.
- Rewrite in place: 1 (`relate_edge`).
- Move out of `test`: 2 probe self-tests, to `spec`.
- Move to the release suite: the full per-surface replay, 8 hand-written matrix files, and the installed-artifact checks of 20 bindings. These stay here until the suite runs them.

## One replay smoke per binding

The standard `test` rung skips every binding. Landings broke the Polars tests, the C door tests and two package tests without a red gate. One smoke per binding closes that gap.

The smoke is a new `THINKTHEN_TEST_PROFILE=smoke` in each `check.sh`, run by a new `sdlc/scripts/smoke` that `test` calls. Each smoke:

1. Builds the binding into the installed shape. For the C door bindings that is the `native/` folder of header, shared library and `pkg-config` file that `libraries/go/check.sh` builds today. For the other bindings it is their package build.
2. Loads it from that folder, not the checkout.
3. Asks one recorded `decide` question from one shared replay fixture in `conformance/`, with the backend address on the loopback backend.
4. Checks the answer value and that the loopback backend counted zero requests.

A binding without its toolchain exits 77, as today, and `smoke` reports it as not run.

### Cost

No check records its own wall time, so these estimates come from the build lines of a routine `surfaces` run on this machine and from records 0130 and 0206.

| Binding group | Build after a library change | Warm build | Ask and check |
| --- | --- | --- | --- |
| C door, built once for 11 bindings | 18.7 s | 0.1 s | none |
| Go, C++, PHP, C#, JVM, Swift, Zig, Ada, Objective-C, COBOL, Dart | host compile only | 1 to 5 s each; JVM, C# and Dart start slower | under 1 s |
| Rust consumer | 15.2 s | under 1 s | under 1 s |
| Python | 12.9 s plus 2.8 s | under 1 s | under 1 s |
| Ruby | 13.2 s release build | under 1 s | about 1 s |
| R | 17.1 s | under 1 s | about 1 s |
| TypeScript, DuckDB, SQLite | about 15 s each, by the same pattern | under 1 s | under 1 s |
| PostgreSQL | 30 s for the extension; 119 s for a selected check with server setup (record 0206) | several seconds for server start | about 1 s |
| Polars feature | 112 s cold, 12 s warm for its check (record 0130) | 12 s | under 1 s |

Warm, the 20 smokes cost about 30 to 60 s run one after another. After a library change, each Cargo binding rebuilds the library in its own target folder, and the total reaches about 3 to 4 minutes one after another. Three steps bring that down: run the smokes four at a time beside the nextest run, point the Cargo bindings at one shared smoke target folder, and start PostgreSQL once for its smoke.

## Target

`sdlc/scripts/test` after a library change, at a 1-minute load of 16 or less: 90 s wall, binding smokes included. The Rust test run stays under 15 s, and no routine test runs over 5 s. On a warm build with no change the rung takes 60 s or less.

Slice 1 alone takes today's rung from 82.5 s to about 60 s: the probes leave (17 s) and the consumer runs under nextest (about 6 s saved). If the smokes then push the rung over 90 s, the slowest smokes, most likely PostgreSQL and Polars, move to the routine `surfaces` rung, and the ticket records why.

## Commands

```sh
THINKTHEN_HEAVY_LOCK=<scratch>/claude-4-heavy.lock /usr/bin/time -v sh sdlc/scripts/test
# per-test times: the PASS lines of the nextest output in the same log
# per-step times: each later command of sdlc/scripts/test, timed alone under the same lock
```
