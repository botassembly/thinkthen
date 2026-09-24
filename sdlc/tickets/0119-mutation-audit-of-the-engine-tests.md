---
flow: build
priority: 119
opens: crates/thinkthen/src crates/thinkthen/tests sdlc/ratchet.json .gitignore sdlc/issues/2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md
---

# 0119: Mutation audit of the engine tests

Status: ready. It builds after 0098 lands and before 0.1 ships. Owner: Claude.

## Design and decisions

This ticket carries out steps 3 and 4 of `sdlc/issues/2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md`. The rule behind it is the workspace decision `2026-09-24-tests-earn-their-place.md`. Quick Fix `qf-tests-earn` already put the rule into `AGENTS.md`. Ian can overturn every numbered decision below.

1. **The tool.** `cargo-mutants` 27.1.0, pinned. It installs under `~/.cache/thinkthen-toolchains/cargo-mutants-27.1.0/` by the port guide's toolchain rule (`sdlc/planning/surfaces-port-guide.md`). The builder downloads the crate archive from `static.crates.io` once, checks it against a sha256 written in this ticket's record, refuses a mismatch, and runs `cargo install --locked --path` on the unpacked crate with `--root` at that folder. The copy in `~/.cargo/bin` does not count, because nothing pins it. The audit runs once, so the recipe lives in the record and no gate script calls it. A second audit would turn it into a script.

2. **The host and the load.** The audit runs on the Beelink under the workspace load rule. A mutation batch starts only when the one-minute load is under about two thirds of the cores and memory has room. `--jobs` fits that room, and the record states the value and the load at each run. Suite-time runs are timing evidence, so they run with the host otherwise idle.

3. **Scope.** Mutants come from `crates/thinkthen/src/core` and `crates/thinkthen/src/engine`. Deletion candidates are the `#[test]` and `#[tokio::test]` functions in `crates/thinkthen` whose code only a test reaches: inline `#[cfg(test)]` modules, the `*_tests.rs` and `tests/` folders under `src`, and `crates/thinkthen/tests`. Candidates are judged by `cargo test -p thinkthen --all-targets`, the command `cargo-mutants` runs.
   - Out of scope, kept whole: `conformance/`, the secrecy tests, `spec/` pages, green demos, `sdlc/live-test`, the shell self-tests, and the documentation tests. Secrecy tests guard code not yet written, and mutants of today's code cannot measure that. The mutation run ignores the shell tests, so its evidence errs toward keeping a test.
   - The `cli` module is out of scope for mutants. Tickets 0113 and 0114 add CLI code, and a CLI audit waits for them. Its tests still count as covering tests for engine mutants.

4. **The deletion rule.** Delete a test only when mutation evidence shows other tests catch every bug it catches.
   - A baseline run over the whole scope records each mutant as caught, missed, timeout, or unviable.
   - The builder picks candidates by reading first: tests that match a junk pattern in `AGENTS.md`, tests that pin one step of a red-green cycle, and tests that repeat one contract at several layers. A test does not become a candidate for being slow or static.
   - Candidates go in batches of one test file or one `#[cfg(test)]` module. The builder deletes the batch and reruns the mutants in the source files the batch calls. If any mutant moves out of the caught or timeout set, the batch comes back whole and the builder splits it to find the test that alone caught it. That test stays.
   - A kept test that fits none of the four kinds gets turned into one only when that takes no new test-only hook. Otherwise it stays as it is and the record names it.
   - For each deleted test the record states what it could catch and which remaining test catches the same mutants. A test that another record names as its proof gets the same line, so the old record has a forward pointer.
   - A full scope run at the end must show the baseline's caught and timeout sets unchanged. A mutant that moved restores the batch that moved it.

5. **The ratchet effect.** Each judgment runs against the suite as it stands after the batches before it, never against the baseline suite. Two tests that each cover the other cannot both go. The first one judged goes, and the second one then holds the last cover and stays. Order therefore changes the result, so the builder judges the weakest-looking tests first and records the order. Each accepted batch lands as one commit that lowers `sdlc/ratchet.json` by the lines it removed, because the ceiling equals the measured total. The room it frees goes to product code, and the ceiling never rises in this ticket.

6. **The stop rule.** The audit stops at the first of these:
   - every candidate batch has been judged;
   - three batches in a row come back whole;
   - the remaining candidates hold fewer than 500 lines.
   The builder also stops and reports if the baseline suite is red or a test fails on a rerun with no mutation. A flaky suite gives no evidence.

7. **Missed mutants are findings.** The audit adds no test and changes no product code. The record lists missed mutants by module, and one issue files them for later tickets.

8. **The report.** The record ends with a report section that gives:
   - test lines deleted, from the ratchet measure and from a count of test-only lines before and after;
   - the number of test functions before and after;
   - the mutation score before and after: caught divided by caught plus missed, with timeout and unviable counts beside it;
   - suite time before and after: wall time of `sdlc/scripts/test` and of `cargo test -p thinkthen --all-targets`, each the median of three runs on an idle host;
   - the Beelink time the audit spent in mutation runs;
   - any junk pattern that kept coming back, with counts.
   The workspace reads this report to decide whether other repos get the same audit. A pattern that kept coming back is the case for a new ratchet, and the report names it without building one.

Not in this ticket: a mutation gate in the ladder, a CLI audit, mutation tests of `conformance/`, and any audit of another repo.

## Outcome and authority

The engine's test code shrinks to tests that earn a place, with mutation evidence for each deletion and a report the workspace can act on. Ticket 0119 authorizes edits to test code and `#[cfg(test)]` modules under `crates/thinkthen`, a lower ceiling in `sdlc/ratchet.json`, a `mutants.out*` line in `.gitignore`, one issue for missed mutants, and this ticket's record and reviews. It authorizes no product code change and no ceiling rise.

## Timing

The owner decided the timing on 2026-09-24, and Ian can overturn it. Most test code lives in the engine. Tickets 0085, 0097, 0096, 0086, and 0098 edit the engine until 0098 lands, so an earlier audit would collide with them. The audit builds after 0098 and before 0.1 ships. Surface tickets add their own folders, so the audit may build beside them. It shares `crates/thinkthen` with 0113 and 0114, so it runs before or after each of them and never beside.

## Work

1. Install the pinned `cargo-mutants` by decision 1 and record the version, the sha256, and the install lines.
2. Measure the baseline: suite times, test function count, test-only lines, and one full scope mutation run.
3. Read the candidates and list them in judgment order.
4. Judge the batches by decisions 4 and 5 until the stop rule holds.
5. Run the final full scope mutation run and the suite times again.
6. File the missed-mutant issue.
7. Write the record with the report, and run the whole ladder.

## Review

- Design review: a fresh read-only Opus session reviews this ticket before any build.
- Code review: a fresh read-only Opus session picks three deleted tests at random. For each one it plants the mutant the deleted test caught and shows the named covering test fails on it. It checks that the final run's caught set matches the baseline and that no product code changed.
