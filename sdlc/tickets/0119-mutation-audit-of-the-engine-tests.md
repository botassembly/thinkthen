---
flow: build
priority: 119
opens: crates/thinkthen/src crates/thinkthen/tests sdlc/ratchet.json sdlc/issues/2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md
---

# 0119: Mutation audit of the engine tests

Status: ready. The design review accepted it (`sdlc/records/0119-design-review.md`). It builds after 0098 lands and before 0.1 ships. Owner: Claude.

## Design and decisions

This ticket carries out steps 3 and 4 of `sdlc/issues/2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md`. The rule behind it is the workspace decision `2026-09-24-tests-earn-their-place.md`. Quick Fix `qf-tests-earn` already put the rule into `AGENTS.md`. Ian can overturn every numbered decision below.

1. **The tool.** `cargo-mutants` 27.1.0, pinned. It installs under `~/.cache/thinkthen-toolchains/cargo-mutants-27.1.0/` by the port guide's toolchain rule (`sdlc/planning/surfaces-port-guide.md`). The builder downloads the crate archive from `static.crates.io` once, checks it against a sha256 written in this ticket's record, refuses a mismatch, and runs `cargo install --locked --path` on the unpacked crate with `--root` at that folder. The copy in `~/.cargo/bin` does not count, because nothing pins it. The audit runs once, so the recipe lives in the record and no gate script calls it. A second audit would turn it into a script.

2. **The host and the load.** The audit runs on the Beelink under the workspace load rule. A mutation batch starts only when the one-minute load is under about two thirds of the cores and memory has room. `--jobs` fits that room, and the record states the value and the load at each run. Suite-time runs are timing evidence, so they run with the host otherwise idle.

3. **Scope.** Mutants come from all of `crates/thinkthen/src`: `core`, `engine`, and `cli`. Deletion candidates are the `#[test]` and `#[tokio::test]` functions in `crates/thinkthen`: inline `#[cfg(test)]` modules, the `*_tests.rs` files and `tests/` folders under `src`, and `crates/thinkthen/tests`. Every candidate therefore faces mutants of the code it reaches, including tests that run the binary through the command line.
   - Out of scope, kept whole: `conformance/`, the secrecy tests in `crates/thinkthen/tests/backend/secrecy*.rs`, `spec/` pages, green demos, `sdlc/live-test`, the shell self-tests, and the documentation tests. Secrecy tests check every command and failure path, including ones added later. Mutants of today's code cannot measure that. The mutation run ignores the shell tests and the documentation tests, so its evidence errs toward keeping a test.
   - Every run uses one command line: `cargo mutants -p thinkthen --cargo-test-arg=--all-targets --timeout T -j J -o DIR`, plus the `-f` list decision 4 names. `T` comes from the first baseline and stays fixed for the whole audit, so load cannot move a mutant between missed and timeout. `DIR` lies outside the repository. The record writes each full command line.

4. **The deletion rule.** Delete a test only when mutation evidence shows other tests catch every bug it catches.
   - A baseline run over the whole scope records each mutant as caught, missed, timeout, or unviable. A second run with nothing deleted follows at once. A mutant that changes outcome between the two is flaky. If any mutant is flaky, the builder stops by decision 6.
   - The builder picks candidates by reading first: tests that match a junk pattern in `AGENTS.md`, tests that pin one step of a red-green cycle, and tests that repeat one contract at several layers. A test does not become a candidate for being slow or static.
   - Candidates go in batches of one test file or one `#[cfg(test)]` module. The builder deletes the batch and reruns mutants chosen by file. A `#[cfg(test)]` module or a `*_tests.rs` file selects its parent module's source files with `-f`. A batch from `crates/thinkthen/tests` or a `tests/` folder reruns the whole scope.
   - The comparison reads `caught.txt` and `timeout.txt` from the batch's output folder. Every mutant in those two files from the last accepted run, for the same source files, must appear in one of them again. A move between caught and timeout counts as no change. If any mutant drops out, the batch comes back whole, and the builder splits it to find the test that alone caught that mutant. That test stays.
   - A kept test that fits none of the four kinds gets turned into one only when that takes no new test-only hook. Otherwise it stays as it is and the record names it.
   - For each deleted test the record states what it could catch and which remaining test catches the same mutants. A test that another record names as its proof gets the same line, so the old record has a forward pointer.
   - A full scope run at the end must show every mutant in the baseline's caught or timeout files still in one of them. A move between caught and timeout counts as no change. A mutant that dropped out restores the batch that moved it. The builder finds that batch by rerunning the moved mutant with `-F` at each accepted batch commit in turn, newest first.

5. **The ratchet effect.** Each judgment runs against the suite as it stands after the batches before it, never against the baseline suite. Two tests that each cover the other cannot both go. The first one judged goes, and the second one then holds the last cover and stays. Order therefore changes the result, so the builder judges the weakest-looking tests first and records the order. Each accepted batch lands as one commit that lowers `sdlc/ratchet.json` by the lines it removed, because the ceiling equals the measured total. The room it frees goes to product code, and the ceiling never rises in this ticket.

6. **The stop rule.** The audit stops at the first of these:
   - every candidate batch has been judged;
   - three batches in a row come back whole;
   - the remaining candidates hold fewer than 500 lines.
   The builder also stops and reports if the baseline suite is red, a test fails on a rerun with no mutation, or the second baseline run shows a flaky mutant. A flaky suite gives no evidence.

7. **Missed mutants are findings.** The audit adds no test and changes no product code. The record lists missed mutants by module, and one issue files them for later tickets.

8. **The report.** The record ends with a report section that gives:
   - test lines deleted, from the ratchet measure and from a count of test-only lines before and after;
   - the number of test functions before and after;
   - the mutation score before and after: caught divided by caught plus missed, with timeout and unviable counts beside it;
   - suite time before and after: wall time of `sdlc/scripts/test` and of `cargo test -p thinkthen --all-targets`, each the median of three runs on an idle host;
   - the Beelink time the audit spent in mutation runs;
   - any junk pattern that kept coming back, with counts.
   The workspace reads this report to decide whether other repos get the same audit. A pattern that kept coming back is the case for a new ratchet, and the report names it without building one.

Not in this ticket: a mutation gate in the ladder, mutation tests of `conformance/`, and any audit of another repo.

## Outcome and authority

The engine's test code shrinks to tests that earn a place, with mutation evidence for each deletion and a report the workspace can act on. Ticket 0119 authorizes edits to test code and `#[cfg(test)]` modules under `crates/thinkthen`, a lower ceiling in `sdlc/ratchet.json`, one issue for missed mutants, and this ticket's record and reviews. It authorizes no product code change and no ceiling rise.

## Timing

The owner decided the timing on 2026-09-24, and Ian can overturn it. Most test code lives in the engine. Tickets 0085, 0097, 0096, 0086, and 0098 edit the engine until 0098 lands, so an earlier audit would collide with them. The audit builds after 0098 and before 0.1 ships. Surface tickets add their own folders, so the audit may build beside them. It shares `crates/thinkthen` with 0113 and 0114, so it runs before or after each of them and never beside.

## Work

1. Install the pinned `cargo-mutants` by decision 1 and record the version, the sha256, and the install lines.
2. Measure the baseline: suite times, test function count, test-only lines, and two full scope mutation runs with nothing deleted.
3. Read the candidates and list them in judgment order.
4. Judge the batches by decisions 4 and 5 until the stop rule holds.
5. Run the final full scope mutation run and the suite times again.
6. File the missed-mutant issue.
7. Write the record with the report, and run the whole ladder.

## Review

- Design review: a fresh read-only Opus session reviews this ticket before any build.
- Code review: a fresh read-only Opus session picks three deleted tests at random. For each one it plants the mutant the deleted test caught and shows the named covering test fails on it. It checks that the final run's caught set matches the baseline and that no product code changed.

## Evidence

Builder note, 2026-09-24. Workspace decision `2026-09-24-experiments-reduce-risk.md` asks every product ticket to name these five parts. This note changes no design.

- Starts from: Workspace decision `2026-09-24-tests-earn-their-place.md` and `sdlc/issues/2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md`, which counts 787 tests and 28,164 lines in test paths. `sdlc/records/0119-design-review.md` accepted the design. No experiment ran mutation testing on thinkthen. `experiments/170-hgparse-alteration-grammar` used it on another project and gives no finding to carry forward. The frozen tag, `probes/`, and `repos/jev-experiments`: none found.
- Keeps: No product code changes, and the ceiling never rises. `conformance/`, the secrecy tests, `spec/`, green demos, the live test, and the shell and doc tests stay whole.
- Changes: The audit deletes a test only when mutation evidence shows other tests catch the same mutants. It works in batches and lowers `sdlc/ratchet.json` with each accepted batch. It pins `cargo-mutants` 27.1.0 by checksum.
- Proof: Two baseline runs with no flaky mutants, a batch comparison on the caught and timeout lists, and a final full run that keeps the whole baseline caught set. Code review plants three mutants that deleted tests used to catch.
- Defers: Missed mutants go to one issue. No mutation gate, no audit of `conformance/`, and no audit of another repo. A recurring junk pattern gets a name, not a ratchet.
