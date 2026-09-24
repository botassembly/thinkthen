FINDINGS

# Code review: 0117, the loopback backend arms

Reviewer: a fresh, read-only Claude session that did not write the work. Date: 2026-09-24.

Read: `origin/ticket/0117-backend-arms-for-surfaces` at `e5160f41`. That covers the ticket, `sdlc/records/0117-design-review.md`, `sdlc/records/0117-build-backend-arms-for-surfaces.md`, `AGENTS.md` (the `CLAUDE.md` target) with "What reviewers keep finding", and the whole diff against the merge base `6eb1303e`. I ran everything in scratch copies under `/tmp/claude-1000/` and deleted them after. I ran no live script and read no credential.

## What holds

- **The arms match the ticket.** `/arm/delay/MS/v1` sleeps on the connection's own thread through `Canned::after`. It refuses a value that is not ASCII digits and one above 10,000 at once, with the two pinned sentences. The held gate keeps an open flag and a round number, and `release` stays permanent. `wait N` runs `Backend::wait` on a detached thread and prints `wait K` under one output lock. A bad `wait` goes to standard error only. `Backend::wait(n)` polls every 5 ms with a 5 s bound. `git diff origin/main...branch -- crates` is empty. `Cargo.toml` is unchanged.
- **No test can reach a paid backend.** Every test posts to `127.0.0.1` on a port the backend it started printed, or to an in-process `Backend`. The backend opens no outbound connection and reads no key.
- **The timing tests held under load.** The 15 tests in `binary.rs` passed 10 of 10 runs with 24 busy loops on 16 cores (load 18) and 8 of 8 runs with 48 busy loops (load 36). Every "no answer yet" check only grows safer under load. The upper bounds (1 s per answer, 800 ms for eight delays, 5 s for fifty rounds, 6 s for a `wait` that gives up) held at load 36.
- **Not over-built.** The source adds 77 nonblank lines. The `Mutex` around the record receiver exists only because `Backend` must be shared with the `wait` thread. I found no arm or line that no surface ticket names.
- **Plants I added that turned red.** (B) `wait` prints N and ignores the count: `a_wait_line_gives_up_at_5_s_…` read `wait 1`. (C) `Backend::wait` returns only when the count is above N: `a_wait_line_answers_once_…` and `fifty_rounds_…` both failed.

## Findings

1. **Blocking: the order move breaks an existing test in `crates`, which now flakes.** `serve_kept` now calls `reply(&request)` before it adds the request to `in_flight` and `peak`. The ticket assumed that no `answering` closure in `crates` blocks. Ticket 0116 landed `7e8301c4` on main before this branch's base. It added `Gathering::hold`, a closure that blocks until all requests arrive, to `crates/thinkthen/tests/backend/annotate/scheduling.rs`. The test then asserts `listener.peak() == jobs.min(6)`. A held request no longer counts as in flight, so the peak comes up short.
   - Observed: `one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` failed on the branch 2 of 3 runs, then 1 of 10 runs at a load near 9. It printed `document at 32` with left 3 and right 6, and `document at 4` with left 3 and right 4. It also failed once in the full workspace run on the merged tree.
   - With the old order restored and the rest of the branch kept, it passed 8 of 8 runs. With main's backend, it passed 8 of 8 runs.
   - Fix I tried: move only `counts.requests.fetch_add` below `reply`. Keep the `in_flight` and `peak` updates above it. The round rule needs only the request count to follow the round snapshot. With that change, the scheduling test passed 12 of 12 runs, and all 15 `binary.rs` tests passed. Update the comment and the ticket's claim about blocking closures. 0077 on main adds a second blocking closure in `src/cli/schedule/width_tests.rs`. It reads its own channel, not the peak.
   - The builder's single green ladder run at `db84ef82` does not rule this out. Run the scheduling test in a loop after the fix.

2. **The `wait +1` case cannot fail.** (A) I made the `wait` line parse with `str::parse::<usize>`, which accepts `+1`. All 15 tests stayed green. `wait +1` then waits 5 s for a count of 1 that never comes. The test's 300 ms silence window and the input close both end before it prints. Fix: send `wait +0` in place of `wait +1`. Under the plant it prints `wait 0` at once and the test turns red.

3. **Two promised behaviors have no test (not blocking).**
   - (D) I replaced `drift` with a bare `Canned::status` for the whole-number refusal, so it writes nothing to standard error. All tests stayed green. The ticket promises "the same sentence on standard error". The tests pipe standard error to null.
   - (E) I made `run` join every pending `wait` thread before the final count. All tests stayed green. The README and ticket promise "exits without waiting for a pending `wait`". A test can send `wait 1`, close the input at once, and require the exit well under 5 s.
   - The owner can add both checks here or record them as untested.

4. **A note on the round-order plant.** Reverting the order with no added sleep stayed green in 3 of 3 runs. The race window is narrow, so the ticket's 20 ms sleep plant is the right proof. This is not a finding.

## The merge with main

`git merge-tree` of `origin/main` (`9f47bd18`) and the branch conflicts only in `sdlc/ratchet.json`. Main raised the ceiling from 47007 to 48140. The merged tree measures 48449 with `sdlc/scripts/ratchet.mjs`. That equals 48140 plus this branch's 309. No other file conflicts. Main's changes do touch behavior this branch depends on: 0077 adds a second blocking `answering` closure (see finding 1). On the merged tree, the backend tests passed 15 of 15, and one suite passed 314 of 314. Cargo then stopped at the scheduling failure, so the suites after it did not run.

## Verdict

FINDINGS. Fix finding 1 and finding 2, then show the scheduling test green in a loop. Finding 3 is the owner's call.

## Confirmation at 816f253f

ACCEPT

Read `e5160f41..816f253f`. `fe40e545` fixes the code. `8d568bd1` merges main at `9f47bd18`. The last two commits change only the ticket, the build record, and `sdlc/records/0117-code-review.md`. I ran everything in a scratch copy of `816f253f` under `/tmp/claude-1000/` and deleted it after.

- **The blocker fix matches mine.** `serve_kept` now adds the request to `in_flight` and `peak` before `reply(&request)`, and adds it to `requests` after. The comment says why. The 0116 test `one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` passed 15 of 15 runs at a load near 9 to 15. It then passed 5 of 5 runs with 40 busy loops running, at a load of 36.
- **`wait +0` catches plant A.** With the `wait` line parsed by `str::parse::<usize>`, `a_wait_line_with_no_whole_number_prints_nothing` failed.
- **The two new tests fail under my plants.** Plant D (a whole-number refusal with no standard-error line) turned `a_delay_refusal_says_why_on_standard_error` red. Plant E (`run` joins every pending `wait` before the final count) turned `closing_the_input_exits_without_waiting_for_a_pending_wait` red. The first test pins both sentences exactly.
- **Nothing else changed.** Outside the Markdown records, `e5160f41..fe40e545` touches only `listener.rs` (the move above) and `binary.rs`. The `binary.rs` changes are the `wait +0` change, the two new tests, and standard error piped in place of null. The merge commit differs from `9f47bd18` only in the 0117 files and `sdlc/ratchet.json`. `git diff origin/main...816f253f -- crates` is empty.
- **The ratchet.** `sdlc/scripts/ratchet.mjs` measured 48475/48475 on the scratch copy.
- **Load.** All 17 `binary.rs` tests passed 6 of 6 runs at a load of 36. The new 1 s exit bound held on every run.
- **Main moved again.** It is now at `d8ffb5b0`, three commits that touch only Markdown notes. `git merge-tree` against it is clean.
