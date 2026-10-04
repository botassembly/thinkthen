# 0404: Tech debt cut, with tests held to behavior

Status: ready. Not started.

Milestone: 0.2

Lane: none of its own. Each slice fills a gap in whichever lane frees first, per the 0.2 lane order Ian approved on 2026-10-04. The coordinator names the lane when a slice starts. Slices are independent after slice A and land one at a time on `ticket/0404-tech-debt-and-tests-held-to-behavior`.

Related: ticket 0119, the engine test audit. It stays separate, and this ticket leaves its files untouched. This ticket reuses 0119's method: compare each test's assertions with a stronger test, use a targeted deliberate break when unclear, and accept a small or zero deletion. It also keeps 0119's retained families out of scope.

Closes: `sdlc/issues/2026-10-03-polars-deadline-test-races-its-deadline-under-load.md` (Debt 036), in slice B.

## Outcome

1. Tests that guard nothing the command-line, library, edge-case, contract or regression tests miss are gone. Each cut names the stronger test that still pins its behavior.
2. Dead code, duplicate paths and test-only hooks in product source are gone, or each one that stays names why.
3. A record gives lines removed, `#[test]` counts by place, and `sdlc/scripts/test` wall time and nextest time before and after, each with the 1-minute load.
4. `policy.py` refuses a change that adds an inline unit test without raising a pinned count in the same commit. The count falls with every cut.
5. `sdlc/scripts/test-stress --run` runs two performance checks: the command's own time per `decide` run, and a library call's own time. Neither runs in a routine gate.
6. The Polars deadline test no longer depends on host speed.

## Evidence

- Starts from: the docs message "Two tickets after 0.1" of 2026-10-03, Ticket A, and workspace decision 2026-09-24-tests-earn-their-place.md. No experiment preceded this ticket. Prior evidence is tickets 0317, 0338 and 0119 and their records.
  - Ian's ask: the command line is the outside, and the library's ten functions are what matter. He wants outside-in behavior tests, edge-case tables and performance checks. He wants red-green unit tests deleted after green.
  - The decision allows four kinds of test: outside-in behavior through the real entry point, an edge-case table, a contract check, and a regression that failed before its fix. Before a deletion it asks what the test can catch and which stronger test covers it. It says a ratchet comes when a junk pattern keeps coming back.
  - Measured on main at `562a59701`. `crates/thinkthen` holds 1,315 `#[test]` functions: 414 inline under `src/` in 93 files, and 901 under `tests/`. The `src/` tests split as 228 in `core`, 100 in `engine`, 69 in `cli`, 9 in `config`, 4 in `public`, 3 in `test_deadline` and 1 in `schema_tests.rs`. 39 of those 93 files are product files with an inline test module. `conformance/` holds 52 more, all under `tests/` or `consumer/`. The bindings and SQL extensions hold 66 under their `src/` folders and 35 under their `tests/` folders. 29 tests carry `#[ignore]` for `test-stress`.
  - Count over time, with the same method. At 0317's landing record, `267efff53`, `src/` held 424 and `tests/` held 835. At 0338's landing, `a26ced3db`, they held 407 and 866. Today they hold 414 and 901. The docs message reads 1,315 as scaffold tests coming back. The inline count has fallen by 10 since 0317 and risen by 7 since 0338. The growth sits in `tests/`, which holds the outside-in kind. Slice E checks that growth for one contract pinned at several layers.
  - Lines. The ratchet in `sdlc/ratchet.json` is 109,496 nonblank lines over `crates` and `conformance`. Files that `policy.py`'s `is_test_source` calls tests hold 62,595 of them. Inline test modules in product files add more.
  - Test-only hooks. 58 items in product files under `crates/thinkthen/src` sit behind `#[cfg(test)]` without being a test module or a test function. Most are in `engine/usage.rs` (10, including `maybe_fail` and `hold_queue`), `core/adapters/systemone/request.rs` (8), `engine/limits.rs` (7, including `WIDTH_CHILD`), `engine/mod.rs` (7), `engine/http.rs` (5, including the `post_observed*` methods), `cli/interrupt.rs` (3, including the `Test` cleanup variant), and one each in `engine/store.rs` (`with_busy_limit`) and `core/question_set.rs` (`resolved_json`). The decision says a test that needs a test-only hook moves to the real boundary. `engine/backoff.rs` also holds six `#[cfg(test)] #[test]` functions between lines 200 and 328. They are inline tests, not hooks.
  - Dead code. `core/answer.rs` keeps `Distribution::new` under `#[allow(dead_code, reason = "generic default for later adapters")]`. That is code kept for a later need. `core/mod.rs:59` gates an item on `cfg(any(test, feature = "cli"))`.
  - Ticket 0317 cut from 1,268 tests to 1,260 and the ratchet by 626 lines. Its method: each deletion names the test that still pins its behavior, and every timing names the 1-minute load. Its lessons: a wait that gives up silently hides a bug, so assert the count a wait returns; splitting one test drops the guards in its body; a runner check needs one failing fixture; load swings timing more than the cuts do. Its "Kept after inspection" list stays kept.
  - Ticket 0338 merged 44 test executables into 11 and optimized `serde_json` and `sha2` in the test profile. Its last timing: `sdlc/scripts/test` ran 1,273 tests in 54 s wall and 21.8 s of nextest at load 11 to 12.
  - Ticket 0119, in progress at Milestone `later`, audits the engine tests one family at a time. Its listener preflight retained ten of eleven hand-rolled listener sites for distinct guarantees. Its audit of `engine/request/tests.rs` and `engine/http/tests.rs` found no safe deletion in 18 tests. Its lesson: compare operation order and final state, because a similar test on the same topic often guards something else. A small or zero deletion is an honest result.
  - The work plan `sdlc/planning/work-plan-2026-09-27.md` records Ian's functional-gate ruling: performance, load and mutation campaigns run only as a named opt-in, outside routine gates.
  - `sdlc/scripts/test-stress` already lists twelve ignored Rust filters and the port stress profile. None times the command's or the library's own overhead. Site ticket 0050 measured a whole `decide` run at a median of 22 ms in a debug build and 18 ms in a release build, on a loaded machine (`sdlc/issues/2026-10-03-command-overhead-fsync-and-connection.md`).
- Keeps: every test kind the decision allows, and these by name.
  - The retained regressions in `AGENTS.md`: parser, secrecy, cancellation, cache-miss, invalid-input and conflict. A regression stays until a stronger replacement lands.
  - `conformance/cases.json` and its cases, the 17 `secrecy::no_leak_on_*` tests and `every_route_has_its_own_sweep`, the `spec/` pages, the green demos and `sdlc/live-test`.
  - The tests ticket 0317 kept after inspection, such as `audit::every_fixture_keeps_its_checksum`, and the families ticket 0119 audited and retained: the listener sites in its listener preflight, `engine/request/tests.rs` and `engine/http/tests.rs`.
  - Every `test-stress` campaign and its `--list`, `--check` and `--run` modes.
  - Public API, command output, exit codes and specification text. This ticket changes no behavior a user can see.
  - The shared `sdlc/scripts/ratchet.mjs` reader, which other repositories copy.
- Changes: seven slices. Slice A lands first. The rest land in any order, each in one short gap.
  - Slice A, baseline and the unit test count. Measure and record, on one machine with the 1-minute load: `#[test]` counts by place as above, nonblank lines in test files and in product files, `sdlc/scripts/test` wall time, and nextest's summed time and ten longest tests. Add `check_unit_tests` to `sdlc/scripts/policy.py` beside `check_sources`. It counts `#[test]` attributes in tracked Rust files under `crates/thinkthen/src`, and in every tracked `.rs` file under `libraries/` or `databases/` whose path has a `src` folder component. Today that gives 414 plus 66, and the 66 include the R crate and the DuckDB bridge. The check fails unless the total equals a pinned constant, as `ratchet.json` does for lines. The constant pins today's count and may only fall. `tests/` folders and `conformance/` stay uncounted, because placement there means the test runs through a public boundary. The failure sentence names the four kinds and the decision, and says a new test goes under a `tests/` folder through a public boundary. The check comes in although the inline count has fallen since ticket 0317. The docs request asked for one, it costs almost nothing, and it keeps the fall from reversing. `sdlc/scripts/README.md` and the `AGENTS.md` gates line say so in one sentence each.
  - Slice B, Debt 036. `deadline::a_deadline_stops_a_score_column_mid_batch` in `crates/thinkthen/tests/polars/deadline.rs` sets a one-second deadline at the call's start and assumes the second singleton is sent inside it. It also sleeps a fixed 1,100 ms. Make the deadline pass only after the second singleton reaches the held arm, with no fixed sleep tied to host speed. The test still proves the third singleton is never sent and that the facts count two requests.
  - Slice C, test-only hooks. For each of the 58 `#[cfg(test)]` items above, move its test to the real boundary and delete the hook, or record why the hook must stay. A failure injection for a disk error with no real-boundary trigger, such as `maybe_fail` in `engine/usage.rs`, is a likely keeper. It stays with one comment naming the regression it guards. Skip files a live lane has claimed. Windows stage 1 claims `cli/interrupt.rs`, the usage store and `config.rs` for findings W1 and W2.
  - Slice D, inline unit test families, one family per gap. Take one product file's inline tests or one `*_tests.rs` file. The six inline tests in `engine/backoff.rs` belong here. For each test, name the assertion, the credible regression that fails it, and the stronger test that also fails on that regression. Delete only when a stronger test exists. Order by likely yield: `cli/` (69) first, because the command-line tests under `tests/backend` cover the same entry point; then `config/` and `public/`; then `engine/` outside ticket 0119's claimed and audited files; then `core/` last, since most of its 228 tests are edge-case tables over pure code, such as `core/records/tests.rs` and `core/digest.rs`. Each deletion lowers the slice A count and `ratchet.json`.
  - Slice E, one contract at several layers. Find pairs where a `tests/backend` command test and a `tests/library` test, or a `tests/` test and a `conformance/cases.json` case, pin the same sentence, refusal or count for the same input. Keep the test at the outermost layer that pins it exactly. Delete the inner copy. 0317 deferred "surface package tests replaying `conformance/cases.json`"; this slice decides that per pair.
  - Slice F, dead code and duplicate paths in product source. Remove `Distribution::new` in `core/answer.rs` if no caller needs it, and check every other `allow(dead_code)` and `allow(unused` in `crates/thinkthen/src`. Check each `#[doc(hidden)]` public item in `crates/thinkthen/src/public/` for a caller in `libraries/` or `databases/`. Check each `cfg(any(test, ...))` gate. Look for two code paths that do one job, such as two parsers or two writers for the same file, and merge them where one serves both callers. The ratchet records the lines.
  - Slice G, performance checks. Add two ignored tests and list them in `test-stress` `--list`, `--check` and `--run`. One runs the built command's `decide` fifty times against the conformance backend and reports the median, the 90th percentile and the 1-minute load. The other makes fifty library calls through one engine against the same backend and reports the same figures. Each fails above a bound the builder sets at about five times the median measured at load in the build, and the bound's comment names that measurement. Update the `--check` sentence's filter count. Neither test runs under `sdlc/scripts/test`.
  - Closing record, after the last slice: lines removed, `#[test]` counts, ratchet and unit test count before and after, and `sdlc/scripts/test` wall time and nextest time before and after on the slice A machine, with load. Add a cut table: test removed, what it could catch, the stronger test that covers it.
- Proof: each slice runs focused checks and names the tests it ran.
  - Slice A: `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` passes. Two planted self-test cases in `check_unit_tests` fail it: one with an inline test added, one with a test removed and the constant unchanged. A planted test under a `tests/` folder passes it.
  - Slice B: the Polars test passes 20 repeated runs at load near the machine's core count, with the load recorded. A planted break that lets the engine send the third singleton fails it.
  - Slices C, D, E and F: for each cut, the named stronger test passes before and after the deletion and the run selects the expected test count. Where the equivalence is unclear, a targeted deliberate break in product code fails the stronger test, and the plant is restored before commit, as in ticket 0119's method. `cargo mutants --file <file>` may serve as evidence only as a named opt-in run. The slice A count and `sdlc/ratchet.json` equal their measured totals. `sdlc/scripts/test` passes at the checkpoint the coordinator names.
  - Slice G: `sdlc/scripts/test-stress --check` finds both new filters. `sdlc/scripts/test-stress --run` passes and prints both medians with the load. A hand plant that sets each bound to 1 ms fails each test, recorded in the build and restored.
  - No proof needs a paid live call.
- Defers: what other tickets own or what needs Ian.
  - Ticket 0119 and its claimed files, `engine/deadline_tests.rs` and `engine/deadline_tests/schedule.rs`. The coordinator ruled on 2026-10-04 that 0119 stays separate.
  - Files a live lane claims. A slice skips them and a later gap takes them after that lane lands.
  - The bindings' own test suites beyond the slice A count. Each binding's tests run through its door, and a later audit can take them per binding.
  - `sdlc/issues/2026-10-03-command-overhead-fsync-and-connection.md` stays open. Slice G gives its item 2 a repeatable figure. Item 1, dropping the usage fsync, changes the usage store's durability and needs its own decision. Item 3 needs a live timing run under Ian's authorization.
  - `sdlc/issues/2026-10-03-native-install-ignores-cargo-target-dir.md` does not fold in. It is a fix to `native_install` in `sdlc/scripts/installed.sh` for the site smoke, with no test debt in it, and fits a Quick Fix.
  - The other open debts, 002, 004, 007, 014, 018, 032 and 035, wait on upstream fixes, a user, or release work. None folds in.
  - A whole-crate mutation campaign, which stays opt-in under the functional-gate ruling.

## Notes for the builder

- Read the decision's four questions before keeping a test. A test that answers none of them goes. A test that is slow or static is no reason to delete it.
- Record what each deleted test could catch before deleting it. A missing stronger test keeps the test.
- Record the 1-minute load beside every timing. Measure before and after on the same machine.
- Check `sdlc/planning/cleanup-2026-09-30.md`'s Lanes table and open tickets' `opens:` lines before a slice edits a file.

## What Ian can overturn

- A new count check at all, a coordinator ruling of 2026-10-04. The decision said a ratchet waits until a junk pattern keeps coming back, and the inline count has fallen since ticket 0317. The docs request of 2026-10-03 asked for the check.
- Counting only `src/` tests. Counting `tests/` too would also tax the outside-in tests he wants.
- A count that may only fall. A new inline edge-case table then needs an approved raise or a home under `tests/`.
- Keeping ticket 0119 separate, a coordinator ruling of 2026-10-04.
- The bound of about five times the measured median in the performance checks.
