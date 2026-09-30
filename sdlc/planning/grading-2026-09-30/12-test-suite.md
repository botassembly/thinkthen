# Area 12: Test suite speed and reliability under load

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

Ticket 0352, "No wall-clock timing in routine tests", is in progress on its own branch and lands on main after this commit. This report grades the pinned commit and counts the wall-clock waits that 0352 is meant to remove.

## Scope

The test suite is the gate scripts (`test`, `spec`, `surfaces`, `test-stress`, `test-full-cases`), their locks and scratch helpers, the loopback backend and spawn harness, and the tests themselves. The area is graded on how fast it runs and whether it stays green under a loaded machine.

All paths below are under `sdlc/scripts/` unless they start with `crates/` or `conformance/`. Line counts are nonblank lines.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Gate scripts | `test` (81), `test-stress` (66), `test-full-cases` (28), `spec` (41), `surfaces` (268), `heavy-lock` (32), `time-limit` (38), `scratch.sh` (124), `allow-list` (15), `smoke` (91), `../live-test` (242): 1,026 |
| Harness | `conformance/backend/src/` (1,239: `listener.rs` 443, `arms.rs` 432, `lifetime.rs` 254), `crates/thinkthen/tests/backend/harness/mod.rs` (108), `tests/backend/support.rs` (254), `tests/support/measure.rs` (431): 2,032 |
| Tests | About 1,326 `#[test]`: 1,276 in `crates/thinkthen` (45,290 nonblank lines in `tests/`) and about 50 under `conformance/`. 26 `#[ignore]` (18 in the crate, 8 in `conformance/`). 7 integration binaries (`backend`, `library`, `polars`, `public_batches`, `public_cap`, `public_controls`, `public_estimated`), plus the library unit binary and `conformance-backend`'s `loopback`. Ticket 0338 counts 11 executables |
| Contract | `CLAUDE.md` "Build and review" and "Proof"; ADR 0111 section 9, ADR 0113 (scratch usage folder), ADR 0047 (surfaces); `sdlc/planning/test-split-2026-09-30.md`; tickets 0305, 0317, 0324, 0335, 0338, 0340, 0352; "Lessons for builders" in `sdlc/planning/cleanup-2026-09-30.md:57-71` |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 4 | About 3,060 nonblank lines of gate scripts and harness, tests excluded |
| States and concurrency | 5 | Nextest runs each test in its own process. The gate takes an exclusive `flock` and re-execs under it (`heavy-lock`), starts the binding smoke in its own process group with `setsid` or `perl setpgrp` and kills that group on exit (`test:46-58`), and `time-limit` runs a command in its own group with a watcher that sends TERM then KILL (`time-limit:14-38`). Nine ignored tests run as child processes (`cli/schedule/width_tests.rs:150`, `engine/facade/fork_tests.rs:105`, `engine/host_signal_tests.rs:80`). The Linux run needs `RLIMIT_NPROC`, `prlimit` and no `CAP_SYS_ADMIN` (`test:7-32`) |
| Rules and refusals | 3 | About 15: four capability and tool refusals in `test`, `scratch.sh` refusing five unsafe-path cases (`scratch.sh:10-20`), the heavy lock, the `allow-list` of environment names, the 500-line cap with `sdlc/ratchet.json`, the usage-folder decoy, the `test-stress` existence check for nine ignored names, two exit-2 usage refusals in `test-stress` and `test-full-cases`, `mustmatch like ""` refusal in `demos` |
| Surfaces touched | 5 | All 22. `surfaces` runs each landed binding's `check.sh` and `--stress` and `--full-functional` profiles (`surfaces:26-45`) |
| Settings | 1 | No `settings.md` row. `THINKTHEN_TEST_PROFILE` and `THINKTHEN_HEAVY_LOCK` are script variables |
| Contract weight | 2 | No spec page. Three ADRs with sections (0111, 0113, 0047): 3 |
| Churn and debt | 5 | 114 commits since 2026-09-23 on the scripts and harness. 14 flake or under-load issues filed and closed in the same 7 days. Two open issues (`2026-09-30-c-door-cases-test-over-the-file-cap.md`, `2026-09-30-crate-job-library-tests-start-a-command-never-built.md`). Ticket 0352 in flight |

Mean 3.6, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The scripts do what `CLAUDE.md` says. Gates use no network and unset the key and base URL (`spec:16-20`). `test` unsets the profile so an exported selector cannot narrow the run (`test:34`). Timing runs only through `test-stress --run`, and `--check` proves the nine ignored names exist (`test-stress:24-44`, `:59-63`). Recorded timings: `test` took 82.5 s at load 13 to 19 and 1,289 tests (`test-split-2026-09-30.md`), then 14.3 s (ticket 0317), then 38 s and 17.6 s of nextest (ticket 0338), then 54 s with 1,273 tests after slice 5. Gaps against the contract. (1) `CLAUDE.md` says load and timing run only through `test-stress --run`, yet 31 routine asserts bound elapsed time from above in 14 files (for example `tests/backend/exchange.rs:301`, `:320`, `:345`, `:455`, `:480`, `:518`, `engine/deadline_tests.rs`, `engine/store/tests.rs:262`, `:274`, `conformance/backend/tests/loopback/binary.rs`). Ticket 0352 targets exactly this. (2) Ticket 0317's outcome was "no routine test over 2 seconds", and its own table says 26 tests still took over 2 s at load 20; 0338 then set the bar at 5 s. (3) `test` falls back to `cargo test` when nextest is missing (`test:61-67`), but ticket 0338 says merging binaries exposed two `cargo test` races that nextest hides. The fallback is less safe than the script's comment says |
| Reliability | C | The suite has a record of flakes under load. 14 issues about load, races or wall-clock limits were filed and closed in 7 days, among them `ordered-output-test-races-the-next-request-under-load`, `process-cap-test-races-two-records`, `c-door-tests-race-under-nextest`, `postgresql-check-keeps-wall-clock-limits-under-load`, `duckdb-split-denials-case-failed-once-under-load` and `relate-host-interrupt-test-fails-under-load`. Ticket 0340 measured the before and after: 35 of 800 runs, 3 of 2,400, 2 of 400 and 11 of 200 failed before, 0 after. One load race is known and unfiled: `library::ca_bundle::genuine_tls_trust…` failed once at load 22 because its responder exited before binding a reserved port; ticket 0338 calls it "older than this ticket" and no issue names it (`rg genuine_tls_trust sdlc` finds only that ticket and the open issue below). An open bug fails a gate: five `library` tests start `CARGO_BIN_EXE_thinkthen` in a build with no command binary, so the release `crate` job fails on every fresh runner (`sdlc/issues/2026-09-30-crate-job-library-tests-start-a-command-never-built.md`). Wall-clock waits remain: 44 `thread::sleep` calls in the crate and about 11 in `conformance/`, for example `tests/public_controls.rs:260`, `:337`, `:475` and `tests/backend/default_cache/usage.rs:307-315`. Failure paths are well covered by counts and exit codes elsewhere in the suite |
| Maintainability | C | One owner for each concern: one heavy lock (`heavy-lock`), one scratch helper that deletes only what it made (`scratch.sh:10-50`), one loopback backend shared by every binding (`conformance/backend`), one spawner (`tests/backend/harness/mod.rs`). Binaries dropped from 44 to 11 (ticket 0338). Against that: 20 Rust files sit at 480 or more lines, 15 of them tests, and `tests/library/public_env.rs` holds exactly 500; 85 lint suppressions in tests and conformance (`#![allow(clippy::expect_used…)]` at the top of `tests/backend/support.rs:1`); nine subprocess children sit in the suite as `#[ignore]` tests that only a parent test runs, which `--ignored` also lists; the 500-line cap check covers `crates/` and `conformance/` only, so `libraries/c/tests/door/cases.rs` (768) passes the gate (open issue); the test helper `digest` repeats the product hash (`tests/backend/support.rs:73`); `surfaces` is a 268-line shell script |

## Strengths

- Every gate takes one lock, and a nested call sees the holder's variable and skips it instead of deadlocking (`heavy-lock:24-34`). A daemon started by a build cannot keep the lock because the `flock` process holds it (`heavy-lock:11-15`).
- A scratch helper removes only folders it made with `mktemp` in this run, refuses unsafe paths and the current directory, and refuses everything else (`scratch.sh:10-50`). Gates plant a decoy usage folder, so a test that writes the real totals fails the run (`test:38-39`).
- Load and timing campaigns are opt-in, listed and checked: `test-stress --list` names each one, and `--check` proves each ignored test still exists (`test-stress:2-12`, `:24-44`).
- Tickets measure before and after under stated load with the failing count, and the lessons are recorded (`sdlc/tickets/0340-load-flakes.md`, "What the build taught us"; `cleanup-2026-09-30.md:57-71`).
- The test profile optimizes only `serde_json` and `sha2`, which took a 255-line relation plan from 29 s to 1 s (`Cargo.toml:20-27`).

## Cleanup

1. **Land ticket 0352 and prove it under load.** Where: the ticket branch; files in the list above. Why: 31 routine upper-bound asserts and 55 sleeps make the gate speed-dependent. The ticket's own proof is `sdlc/scripts/test` under 24 busy processes, before and after. Size: L. Blocks 0.1: no (the gate passes at normal load; a flaky gate costs time, and this is in flight).
2. **Gate each test that starts the command binary.** Where: the five tests in `tests/library/ca_bundle.rs`, `tests/library/key_address.rs` and `tests/library/public_env/usage_totals.rs` named in `sdlc/issues/2026-09-30-crate-job-library-tests-start-a-command-never-built.md`. Why: the release `crate` job fails on every fresh runner. Add `#[cfg(feature = "cli")]` as `public_env.rs:462` does and run `sdlc/scripts/package` with a fresh target folder. Size: S. Blocks 0.1: yes (it fails a release gate).
3. **File and fix the `ca_bundle` load race.** Where: `crates/thinkthen/tests/library/ca_bundle.rs:160`, `:180` (the 10 ms polls) and the responder's port handling. Why: ticket 0338 saw `genuine_tls_trust…` fail once at load 22 and filed nothing. Reserve the port by holding the listener, or read the bound port from the responder. Size: S. Blocks 0.1: no.
4. **Run the cap check over `libraries/` and `databases/`.** Where: `sdlc/scripts/policy.py` and `libraries/c/tests/door/cases.rs` (768 lines). Why: the 500-line rule is unenforced where the C door's test lives (open issue, "Pay when: before 0.1"). Size: M. Blocks 0.1: yes (the issue's own "Pay when: before 0.1").
5. **Move the subprocess children out of the ignored list.** Where: `cli/schedule/width_tests.rs:150`, `:307`, `engine/facade/fork_tests.rs:105`, `:177`, `:231`, `:279`, `engine/host_signal_tests.rs:80`, `:191`, `cli/interrupt/tests.rs:335`. Why: nine helper functions carry `#[test]`, so `cargo test -- --ignored` lists them next to real stress tests and a reader cannot tell. A separate helper binary or a name check in `test-stress --check` would separate them. Size: M. Blocks 0.1: no.
6. **Make the `cargo test` fallback equal to nextest or remove it.** Where: `sdlc/scripts/test:61-67`. Why: two races appear only under `cargo test`, and the script says the two are equivalent. Either run nextest only, or add the `cargo test` run of the merged binaries to `package` (ticket 0338 says `package` already does). Size: S. Blocks 0.1: no.
7. **Split the files at the cap.** Where: `crates/thinkthen/tests/library/public_env.rs` (500), `tests/public_controls/call_facts.rs` (494), `tests/backend/table.rs` (493), `tests/backend/batching/tag_score.rs` (492), `engine/usage/tests.rs` (497). Why: the next added case fails the file cap. Size: M. Blocks 0.1: no.
8. **Drop the test helper's copy of the digest.** Where: `tests/backend/support.rs:73`. Why: it repeats `core::recording::Exchange::digest`. A copy is a deliberate oracle for some tests, so name it so in a comment or read the pinned value. Size: S. Blocks 0.1: no.

## Confidence: medium

Read: `test`, `test-stress`, `test-full-cases`, `spec`, `heavy-lock`, `time-limit` and the first 50 lines of `scratch.sh`; the head of `surfaces`; `tests/backend/support.rs` and `harness/mod.rs` heads; tickets 0340, 0338 and 0317 and the test-split plan in full or by table; the two open issues for this area; `CLAUDE.md`. Counted tests, ignores, sleeps, elapsed asserts, files at the cap and suppressions with `rg`. Timings are the ones recorded in tickets; none were rerun.

Not checked: no test or build was run. I did not read `surfaces` past its options, `conformance/backend/src/listener.rs` or `arms.rs`, tickets 0305, 0324 and 0335, or any binding's `check.sh`. The 14-issue flake count filters closed issue names dated 2026-09-23 to 30 by words (`load`, `race`, `wall`, `flake`, `limit`, `park`, `sleep`) and may include one or two that are not test flakes. The 31 upper-bound asserts count patterns, and I did not read each to see whether it guards a real cancellation proof that 0352 keeps. The test counts come from `#[test]` lines, so macro-generated cases are missed.
