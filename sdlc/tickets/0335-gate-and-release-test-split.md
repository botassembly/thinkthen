# 0335: Split tests between the gate and the release suite

Status: ready. Plan: `sdlc/planning/cleanup-2026-09-30.md`, step 5. Issue: `sdlc/issues/2026-09-30-split-tests-between-the-gate-and-release-qa.md`. Inventory: `sdlc/planning/test-split-2026-09-30.md`.

## Outcome

`sdlc/scripts/test` stays network-free and focused on mechanics, and it loads every binding once. Each binding gets one replay smoke in the rung: it loads the installed-shape package, asks one recorded question, and checks the answer and a zero request count. Duplicate and wording-only tests are gone. Per-surface function matrices and installed-package checks move to the release suite once it runs them. After a library change the rung takes 90 s or less at a 1-minute load of 16 or less, smokes included. Today it takes 82.5 s without them.

## Evidence

- Starts from: record 0305 and ticket 0317 (cut lists and timings), and one measured run of `sdlc/scripts/test` on `2b3cb67ea`: 82.5 s wall at load 13 to 19, 1,289 nextest tests in 17.5 s, 18 consumer tests, one doctest, and 13 shell self-tests. The inventory holds the groups, the per-step times and the named tests.
- Keeps: the dense-logic unit tables, the command-line exact-output tests, the split secrecy sweep, the Rust library tests, the `conformance/` replays, `speed`, the demo runner's failing-page test, the transform tables and `live-test`. No parser, secrecy, cancellation, cache-miss, invalid-input or conflict regression goes before a stronger test holds it. The routine `surfaces` rung keeps its 31 cases per binding.
- Changes: in three slices, below. Delete 6 wording-only or duplicate tests and check steps; merge 12 tests into kept tests; rewrite 1 test in place; move 2 probe self-tests from `test` to `spec`; add one replay smoke per binding.
- Proof: per slice, `sdlc/scripts/test` wall time and test count before and after under stated load; each deleted or merged test names the kept test that catches its regression, with a mutation run where the pair is not obvious; each smoke fails once against a planted broken package before it passes.
- Defers: moving the per-surface matrices, the full per-surface replay and the installed-artifact checks until the release suite runs bindings; the long waits in `engine::deadline_tests` and `cli::schedule::width_tests`; `transforms/sweep/test.sh`'s 5.1 s; merging test binaries.

## Slices

1. **Now.** These files are outside the 0304 slice 3a branch and the ticket-checker lane.
   - Delete `demo_runner::every_recorded_demo_runs_and_every_demo_still_red_is_skipped`. `spec` runs `sdlc/scripts/demos` over the same pages.
   - Move the rows of `backend` `refusals::no_refusal_on_any_command_writes_the_key_quotes_the_evidence_or_sends_anything` and `question_file` `secrecy::no_message_from_either_home_ever_carries_the_key_or_the_evidence` into the split sweep as routes, then delete the two tests. The sweep's route guard counts the new rows.
   - Rewrite `relate_edge::relate_reads_the_names_recognize_found` to pass its `--kind` list as fixed arguments instead of parsing demo 44's README.
   - Move `probes/find-0040/self-test` (16.7 s) and `probes/probability-total-0038/self-test` from `test` to `spec`.
   - Run the external consumer under nextest when it is installed.
   - Delete the wording checks in the binding checks: Python's deadline-sentence test and `NOTES.md` step, TypeScript's deadline-sentence grep, and DuckDB's R5-23 and R2-29 sentence rows.
   - Expected: the rung falls to about 60 s.
2. **After 0304 slice 3a to 3d land.** The C door and Polars tests are green on main and the library runs on `ask_all`.
   - Add `THINKTHEN_TEST_PROFILE=smoke` to each `check.sh` and a `sdlc/scripts/smoke` that `test` calls, four at a time beside nextest, with one shared target folder for the Cargo bindings. The inventory gives the steps and the cost per binding.
   - Merge the batching, retry and JSON duplicates the inventory names: 10 tests.
   - If the smokes push the rung over 90 s, move the slowest smokes to the routine `surfaces` rung and record why.
3. **After the 0305 cleanup removes the process-wide statics and the shared test locks.** The `SERIAL` locks in `public_controls` and `public_batches` go, the deadline and width tests lose their long waits, and the test binaries may merge. Then the rung aims at 60 s after a library change.

The per-surface matrices move when the release suite runs bindings. That is the suite owner's work; this repository retires them in its own ticket then.

## Answers to the issue

1. **List the redundant tests and retire them in the lead's own tickets.** The inventory names each test by group, and this ticket's slices retire them.
2. **Name a release-candidate commit before each release.** Yes. Before each release, the queue owner records the candidate commit hash in the release ticket and in the plan's progress log. No tag is pushed. Ruling 10 holds releases until 0.1, so the first candidate comes with 0.1.
3. **Keep the specification the contract.** Yes. Every exit code and sentence the gate pins comes from `specification/`, and a change to one lands in the specification in the same commit. The smoke takes its question and expected answer from `conformance/`.

Ian can overturn the slice order, the 90 s target, and the choice to move probe self-tests to `spec`.
