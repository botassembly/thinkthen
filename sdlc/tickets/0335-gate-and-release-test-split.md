# 0335: Split tests between the gate and the release suite

Status: landed. Slice 1 and slice 2 landed; slice 3 moved to ticket 0338, which landed. Plan: `sdlc/planning/cleanup-2026-09-30.md`, step 5. Issue, closed into this ticket: `sdlc/issues/closed/2026-09-30-split-tests-between-the-gate-and-release-qa.md`. Inventory: `sdlc/planning/test-split-2026-09-30.md`.

## Outcome

`sdlc/scripts/test` stays network-free and focused on mechanics, and it loads every binding once. Each binding gets one replay smoke in the rung: it loads the installed-shape package, asks one recorded question, and checks the answer and a zero request count. Duplicate and wording-only tests are gone. Per-surface function matrices and installed-package checks move to the release suite once it runs them. After a library change the rung takes 90 s or less at a 1-minute load of 16 or less, smokes included. It took 82.5 s before slice 1 and about 29 s on a warm build after it, without smokes.

## Evidence

- Starts from: record 0305 and ticket 0317 (cut lists and timings), and one measured run of `sdlc/scripts/test` on `2b3cb67ea`: 82.5 s wall at load 13 to 19, 1,289 nextest tests in 17.5 s, 18 consumer tests, one doctest, and 13 shell self-tests. The inventory holds the groups, the per-step times and the named tests.
- Keeps: the dense-logic unit tables, the command-line exact-output tests, the split secrecy sweep, the refusal sweep in `backend/refusals.rs`, the question-file secrecy test, the library retry-header and split-identity tests the inventory lists, the Rust library tests, the `conformance/` replays, `speed`, the demo runner's failing-page test, the transform tables and `live-test`. No parser, secrecy, cancellation, cache-miss, invalid-input or conflict regression goes before a stronger test holds it. The routine `surfaces` rung keeps its 31 cases per binding.
- Changes: in two slices, below, and ticket 0338. Delete 6 wording-only or duplicate tests and check steps; merge 6 tests into kept tests; rewrite 1 test in place; move 2 probe self-tests from `test` to `spec`; add one replay smoke per binding.
- Proof: per slice, `sdlc/scripts/test` wall time and test count before and after under stated load; each deleted or merged test names the kept test that catches its regression, with a mutation run where the pair is not obvious; each smoke fails once against a planted broken package before it passes.
- Ticket 0340 moved the PostgreSQL check's remaining wall-clock limits to stress-only `STEP_within_N_ms` steps, so slice 2 need not.
- Defers: moving the per-surface matrices, the full per-surface replay and the installed-artifact checks until the release suite runs bindings; the long waits in `engine::deadline_tests` and `cli::schedule::width_tests` and merging test binaries, to ticket 0338; `transforms/sweep/test.sh`'s 5.1 s.

## Three places for tests

The Beatles Bench team set this split on 2026-09-30. The inventory's "Three places" table holds it. This repository's gate keeps mechanics against recordings. The public Beatles Bench publishes accuracy, cost and speed; that is not release QA work. The private release QA suite checks release candidates live.

Keep rule: the error paths stay in this repository's loopback tests, because the release QA suite does not fake the service. These are retries, server errors, status 520, oversized replies and dropped connections. No slice moves or deletes one of these tests.

## Slices

1. **Now.** These files are outside the 0304 slice 3a branch and the ticket-checker lane.
   - Delete `demo_runner::every_recorded_demo_runs_and_every_demo_still_red_is_skipped`. `spec` runs `sdlc/scripts/demos` over the same pages. A broken demo then fails at a `spec` checkpoint instead of in `test`.
   - Rewrite `relate_edge::relate_reads_the_names_recognize_found` to pass its `--kind` list as fixed arguments instead of parsing demo 44's README.
   - Move `probes/find-0040/self-test` (16.7 s) and `probes/probability-total-0038/self-test` from `test` to `spec`.
   - Run the external consumer under nextest when it is installed.
   - Delete the wording checks in the binding checks: Python's deadline-sentence test and `NOTES.md` step, TypeScript's deadline-sentence grep, and DuckDB's R5-23 and R2-29 sentence rows.
   - Expected: the rung falls to about 60 s.
2. **After 0304 slice 3a to 3d land.** The C door and Polars tests are green on main and the library runs on `ask_all`.
   - Add `THINKTHEN_TEST_PROFILE=smoke` to each `check.sh` and a `sdlc/scripts/smoke` that `test` calls, four at a time beside nextest, with one shared target folder for the Cargo bindings. The inventory gives the steps and the cost per binding.
   - Merge the 6 batching, retry and JSON duplicates in the inventory's table. Each row names the kept test and the mutation run that must pass before the old test goes. A mutation the kept test misses keeps the old test. 3a renamed or replaced four of the named tests; the table uses the names on main after 3c.
   - If the smokes push the rung over 90 s, move the slowest smokes to the routine `surfaces` rung and record why.
3. **Moved to ticket 0338.** The "0305 cleanup" this slice waited for was never filed. Ticket 0338 now holds it: the shared test locks, the long waits in the deadline and width tests, and merging test binaries. This ticket lands after slice 2.

The per-surface matrices move when the release suite runs bindings. That is the suite owner's work; this repository retires them in its own ticket then.

## Answers to the issue

1. **List the redundant tests and retire them in the lead's own tickets.** The inventory names each test by group, and this ticket's slices retire them.
2. **Name a release-candidate commit before each release.** Yes. The coordinator tags the candidate `rc/VERSION-rc.N` before the release suite runs, per `sdlc/planning/worktrees.md` "Landing commits and tags", and the release ticket names the commit. Ruling 10 holds releases until 0.1, so the first tag is `rc/0.1.0-rc.1`, when the 0.1 blockers clear.
3. **Keep the specification the contract.** Yes. Every exit code and sentence the gate pins comes from `specification/`, and a change to one lands in the specification in the same commit. The smoke takes its question and expected answer from `conformance/`.

Ian can overturn the slice order, the 90 s target, and the choice to move probe self-tests to `spec`.

## What the build taught us

Slice 1 landed every item the slice names. The rung ran 42.1 s and 39.9 s before and 29.3 s and 29.1 s after, each on a warm build under the lane's own heavy lock, at 1-minute loads of 9.8 to 13.6 before and 10.4 to 14.3 after. Nextest ran 1,289 tests before and 1,288 after. The consumer's 18 tests moved under nextest and took 8.2 s, down from about 10 s. One fork test with a long wait, `a_parents_released_digest_lock_frees_its_waiter_while_the_child_lives`, now bounds that step alone. The expected 60 s assumed a library rebuild. A warm run falls further, because the rebuild no longer hides behind 17 s of probe self-tests.

`spec` now runs both probe self-tests beside the checkpoint pages, and it passed. Deleting the demo page run left `demo_runner::a_page_whose_assertion_is_wrong_fails_the_run` as the proof that the runner can fail. The TypeScript and DuckDB binding checks lost only sentence checks. Python lost its deadline-sentence test and the `check.sh` step that checked each test `NOTES.md` names exists. Their code rows stay, and ADR 0038 and the Python `NOTES.md` no longer promise the retired checks.

No item was deferred for the 0304 slice 3a collision. The slice touched two files that branch also edits, `sdlc/ratchet.json` and `libraries/python/ratchet.py.json`. Each holds one measured number, so the second lander remeasures it with `node sdlc/scripts/ratchet.mjs`.

The Python and DuckDB checks passed. The TypeScript check failed one test, `details equals the command --details document for the same question and text`, on a request digest this slice does not touch. It is filed as `sdlc/issues/2026-09-30-typescript-details-request-digest-differs-from-the-command.md`.

### Slice 2

`sdlc/scripts/smoke` gives 19 bindings one replay smoke each, four at a time. `test` runs it beside nextest. For each surface it starts a loopback backend and seeds a fresh cache folder with conformance case 01 through the command. The binding's `check.sh` then runs under `THINKTHEN_TEST_PROFILE=smoke`. It builds the installed shape, loads it from there, and asks case 01's question through the binding's default engine. The smoke passes when the last line is `smoke: true` and the backend counted only the seed's request. The C door hosts load the door from the header, library and `pkg-config` folder that `native_install` in `installed.sh` lays out. Python installs a wheel into a venv, TypeScript copies its addon into a fresh project, Ruby installs its gem, R installs its package, and the SQL extensions load from a copied file.

Each smoke branch first calls `smoke_guard` from `scratch.sh`. It refuses a run without a loopback address and a named cache folder, and it replaces any key with the loopback placeholder. The smoke runs the command and backend it copied from its own `target/smoke` folder, so the nextest build beside it cannot replace them mid-run. `test` starts the smoke in its own process group and stops the group when an earlier step fails. The smoke variables carry the test-only prefix, `THINKTHEN_TEST_SMOKE_QUESTION` and `THINKTHEN_TEST_SMOKE_TEXT`.

Each smoke failed once against a planted fault before it passed. One plant per binding printed the wrong answer. A second plant gave all 19 checks an empty cache, and each failed with 2 requests counted instead of 1. `test` took 77.0 s after touching `crates/thinkthen/src/lib.rs`, smokes included, at 1-minute loads of 12.5 to 19.2, and 38.9 s warm. Nextest ran 1,271 tests and the consumer 20. `spec` passed. In `surfaces`, 19 checks passed. Python and DuckDB failed when the rung's backend refused connections partway through, and both passed on a rerun with a fresh backend.

PostgreSQL and Polars have no smoke and stay in the routine `surfaces` rung. The rung had 13 s left under the 90 s target. PostgreSQL's extension rebuild alone takes about 30 s after a library change, before its server starts (record 0206). Polars rebuilds the library with its feature in its own target (112 s cold, 12 s warm, record 0130), and its folder may hold no code of its own. Ian can overturn this.

The Cargo bindings keep their own warm target folders instead of one shared smoke target. The routine checks already warm those folders, and one shared folder would run the builds one at a time behind Cargo's folder lock.

The merge table kept three of its six rows. Each mutation ran in a separate copy of the tree:

- Deleted `public_bulk_keeps_portable_question_bytes_and_keys_in_one_request`. A changed quote in `core/batch.rs` failed both command-line portable tests. A reversed row key in `public/asking.rs` failed `identity::duplicate_records_share_one_question_key_in_one_literal_request` and `identity::a_retried_filtered_row_keeps_its_observation_and_call_facts`.
- Deleted `named_group_failed_left_half_does_not_send_right`. Sending the right half after a failed left half failed `a_failed_left_half_prevents_a_right_send_and_row`.
- Deleted `retry_visibility_counts_live_attempts_and_no_replay_attempt`. Dropping the retry count failed `backoff::status_counts_retries_as_a_subset_of_actual_sends`. Counting a send on a replay answer failed 12 other tests, among them `public_controls` `call_facts::counters_and_cache_answers_match_the_real_attempts` and `backend` `default_cache::usage_tests::retries_terminal_failures_and_explicit_replay_have_the_ruled_counts`. The table's named library test did not catch it.
- Kept `saved_annotate_batch_tiers`. Ignoring a question set's own `batch` in `public/bulk.rs` failed only this test, because the library picks a set's tier in its own code.
- Kept `portable_questions_ride_one_request_across_series_and_frame_calls`. The quote mutation failed six Polars tests. A frame call that sent one record a request failed only this one.
- Kept `public_json::each_json_method_prints_the_commands_bytes_on_the_shared_cases`. The consumer compares typed values, not the bytes the command prints, so the row's precondition does not hold.

The slice paid two debts. The Dart check now finds Dart and Flutter under `~/.local/opt/flutter/bin` and skips a lock its caller holds. It also reads the shared pub cache, so it runs under `surfaces`. The R check builds its repository install in a kept `target/r` folder, and a second install took 4.8 s instead of 58.4 s.
