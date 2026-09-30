# The files ADR 0111 deletes still have live callers

Status: open. Found by reading main at `ec130d3ee`. Owner: ticket 0304 slice 5, which deletes these files; slice 4 takes the first two if it removes `ask_chunks` and moves `recognize`. Source: `sdlc/planning/after-slice-3-prep.md`, section 1 (slice 4 items 3 and 4, slice 5 item 2) and section 4 items 7 and 8. `cache prune` and `status` are in `2026-09-30-cache-prune-and-status-miss-the-question-store.md`.

Kind: debt

Pay when: 0304 slice 5 lands.

Keeping it leaves two batching paths, so a fix to one can miss the other.

## The problem

ADR 0111 step 4 removes `ask_chunks`, and step 5 deletes `engine/schedule.rs`, `core/batch.rs` and the old recorder. Neither step names a new home for these callers:

1. **`thinkthen check`.** It sends its probe through `engine.split` and `engine.ask_chunks` (`cli/check.rs:66,116`). `check` must not read or write the cache (`specification/check.md:19`), so it needs a send-only call on the new send stage. The conformance runner and the facade tests also call `ask_chunks` (`cli/conformance_tests/runner.rs:50`, `engine/facade_tests.rs:136`).
2. **The command's many-line `recognize`.** It runs each line through `schedule::over_records` (`cli/recognize.rs:131,157`), which reaches `Engine::records` and `engine/schedule.rs`. An `Asker` returns one input's questions at once, so it cannot hold recognize's three dependent steps. The prep note's options: run each step across all lines; move a small ordered runner into `cli`; or compose steps per line on one thread. It recommends the `cli` runner, which keeps today's streaming and width.
3. **`engine/schedule.rs`.** `engine/pipeline.rs:17` and `engine/pipeline/run.rs:15` import its `Input`, and `engine/facade.rs:33` re-exports its types to `cli/schedule.rs`, which survives.
4. **`core/batch.rs`.** `quoted_plan` and `quoted_plan_of` serve `cli/asking/judged.rs`, `cli/annotate.rs:357`, `public/asking.rs:80`, `public/bulk/annotation.rs:74` and `cli/conformance_tests.rs`. The file also defines `Setting`, `BatchError` and `BatchRecord`, which the prep note counts in about 26 places outside tests.
5. **The old recorder.** `engine/store.rs:139,228` call `recorder::require_private`, the duplicate folder check that slice 2 deferred.

## What should happen

Each piece moves before its file goes. The slice 4 and slice 5 tickets name where each one lands, and their proof lists run `check`, the conformance runner and many-line `recognize`.

## Evidence

The files and lines above, read on main.
