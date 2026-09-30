# Area 13: Probability, thresholds, cuts and calibration profiles

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

This area turns a probability into yes, no or not sure under one cut or a band, then measures saved answers against a key (`audit`, `diff`), tunes and writes a steady cut, and names the backend a cut was tuned on so a run elsewhere warns.

Paths below are under `crates/thinkthen/` unless they start with `specification/`, `sdlc/`, `transforms/` or `conformance/`.

| Kind | Paths (nonblank lines, tests excluded) |
| --- | --- |
| Code, rule | `src/core/probability.rs` (45), `src/core/threshold.rs` (175), `src/core/answer.rs` (55 before its test module, 405 with the distribution and score code) |
| Code, measuring core | `src/core/measure.rs` (359), `measure/{answer 450, diff 433, optimize 330, rows 273, audit 260, items 256, key 181, splice 137, verbs 129, levels 104, pairs 76, group 26}.rs`: 3,014 together |
| Code, commands | `src/cli/audit.rs` (253), `audit/{write 265, table 263, cases 244}.rs`, `src/cli/diff.rs` (319), `src/cli/measure.rs` (301): 1,645 together |
| Code, calibration identity | `src/cli/profile.rs` (103), `src/core/result/profile_warning.rs` (31), `conformance/calibration.json`, `transforms/calibration/calibration.jq` (75), `transforms/band/band.jq` (77). `profiles/` holds only a README |
| Tests | Unit: `threshold.rs` (10, with 2 property tests), `probability.rs` (2), `answer_tests.rs` (8, property tests), `answer_distribution_tests.rs` (4), `measure/tests.rs` (8), `diff_tests.rs` (1), `splice_tests.rs` (2). Integration: `tests/backend/audit*.rs` (45 across 8 files), `diff.rs` (13), `threshold_args.rs` (3), `distribution_total.rs` (1), `keeping/graded_rank.rs` (4), plus `tests/fixtures/measure/` goldens |
| Contract | `specification/threshold.md`, `audit.md`, `diff.md`, `result.md` "What a high probability does not mean", `settings.md`, `question-file.md`; ADRs 0007, 0019, 0032, 0054, 0085, 0095, 0102, 0103 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 4 | About 5,070 nonblank lines of production code across 24 files: rule 275, measuring core 3,014, commands 1,645, calibration identity 134 |
| States and concurrency | 2 | Pure functions in `core`. The commands read files, then write one question file or one new `--write-to` output through a temporary file. No threads |
| Rules and refusals | 5 | About 65. The audit failure table has about 40 rows, the threshold grammar has 5 refusals, diff adds about 10 rows and 3 warnings, and the write report has 9 sentences. The audit refusal test table holds 47 rows (`tests/backend/audit_refusals.rs:64`) |
| Surfaces touched | 5 | 22 of 22 for the threshold and the calibration identity. Threshold appears in every binding folder and all three SQL hosts. `audit` and `diff` are command only (`specification/settings.md:98-107`) |
| Settings | 5 | More than 10 rows: Threshold, Backend profile, Calibration identity, Batch, and the seven Audit rows and the Diff row (`specification/settings.md:48`, `:75-76`, `:98-107`) |
| Contract weight | 5 | Six pages with a section on it, with `audit.md` and `diff.md` the longest; eight ADRs, and ADR 0111 amends how the batch setting is saved |
| Churn and debt | 5 | 57 non-merge commits in 7 days on these paths (13 in the last 3 days). One open issue, deferred past 0.1 (`sdlc/issues/2026-09-27-nothing-lists-the-uncertain-hard-or-flip-flopping-cases.md`) |

Mean 4.4, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The threshold rule matches its page cell for cell. `judge` is yes at or above the cut or high side, no below the low side, and not sure between (`src/core/threshold.rs:135-155`), and `the_worked_boundaries_follow_the_specification` pins all 15 cells of `specification/threshold.md:29-35` (`threshold.rs:212`). A cut of 0 and a percent are refused (`threshold.rs:88-99`). One sentence of contract text has drifted: `specification/audit.md:202-204` gives the model refusals as `kept the model for TARGET; ...`, and the code and its tests write `kept the model for the question; ...` (`src/cli/audit/write.rs:119`, `tests/backend/audit_model.rs:15`). The drift is on the `--write` report line, a secondary path |
| Reliability | B | Every audit refusal has a pinned exit code and sentence: 47 rows in one table (`tests/backend/audit_refusals.rs:64`). A second test proves audit opens a loopback listener and reads nothing (`:411-418`), and a third proves no failure echoes a record, id, value or path (`:340`). Three fixes landed in the window: a regression where audit and diff lost the batch setting (`e877e43df`), invalid saved batch settings refused before a write (`d67506f29`), and the model kept when audit reads one a later run would refuse (`c493287f7`). The in-place `--write` is not atomic, and the page says a crash could leave a short file (`specification/audit.md:206`). No property test covers the Wilson interval, the SplitMix64 shuffle, the tie rules or the tuning search |
| Maintainability | B | One owner for the rule (`threshold.rs`) and one shared input reader for both commands (`src/cli/measure.rs`). The threshold rule has property tests, as `sdlc/planning/rust-standards.md:60` requires (`threshold.rs:311`). Several files sit near the 500-line cap: `core/measure/answer.rs` 450, `core/measure/diff.rs` 433, `tests/backend/audit_refusals.rs` 480, `tests/backend/audit.rs` 475. One lint suppression sits in production code: `Distribution::new` carries `allow(dead_code)` for "later adapters" and only tests call it (`src/core/answer.rs:42`). The measuring math follows a prototype whose history was deleted, so its three checksums and the goldens are the only reference (`tests/fixtures/measure/README.md`) |

## Strengths

- The rule is a tiny pure type. A checked constructor builds each shape, and a cut and a band cannot be mixed up (`src/core/threshold.rs:52-133`).
- `Probability` refuses NaN, infinities and out-of-range values at the edge, with a test per case (`src/core/probability.rs:30-38`, `:54-85`).
- The refusal table pins the exit code, the sentence and the secrecy of every audit failure in one loop (`tests/backend/audit_refusals.rs:64`, `:340`).
- Golden files from the prototype, with recorded checksums, back the math. The port records each departure with a reason (`specification/audit.md:265-279`, `tests/fixtures/measure/README.md`).
- `audit` and `diff` say what they never do: no key, no environment, no socket, no process (`specification/audit.md:261`, `specification/diff.md:136`).

## Cleanup

1. **Correct the model-refusal sentences in `audit.md`.** Where: `specification/audit.md:202-204`. Why: the page says `kept the model for TARGET`, and the command prints `for the question`, which tests pin. A script that matches the sentence follows the page and fails. Size: S. Blocks 0.1: yes.
2. **Write the in-place `--write` through a temporary file and a rename.** Where: `src/cli/audit/write.rs`, `specification/audit.md:206`. Why: a crash can leave the user's question file short, and the page admits it. `--write-to` already publishes safely, so the machinery exists. Size: M. Blocks 0.1: no.
3. **Add property tests for the measuring core.** Where: `src/core/measure/tests.rs`, `src/core/measure/optimize.rs`. Why: `rust-standards.md:60` asks for property tests on total functions. Candidates: the Wilson interval stays inside 0 to 1, the shuffle is a permutation, the suggested cut never lies outside its grid, and steady counts add to the split count. Size: M. Blocks 0.1: no.
4. **Remove the `dead_code` suppression.** Where: `src/core/answer.rs:42`. Why: `Distribution::new` is only a test helper. Move it under `cfg(test)` or call it. Size: S. Blocks 0.1: no.
5. **Split the files near the cap.** Where: `src/core/measure/answer.rs` (450), `src/core/measure/diff.rs` (433), `tests/backend/audit_refusals.rs` (480), `tests/backend/audit.rs` (475). Why: the next rule added to audit or diff fails the file cap. Size: M. Blocks 0.1: no.
6. **Replace the deleted prototype with a written reference.** Where: `tests/fixtures/measure/README.md`, `specification/audit.md:7`. Why: the page says the golden files decide where it and the prototype disagree, but no person can rerun the prototype. A short derivation of the calibration and steady rules from the goldens would let a new maintainer change the math safely. Size: M. Blocks 0.1: no.
7. **Build the case-ranking feature after 0.1, as filed.** Where: `sdlc/issues/2026-09-27-nothing-lists-the-uncertain-hard-or-flip-flopping-cases.md`. Why: `audit --cases` now prints every case, but nothing ranks cases by uncertainty. The issue is deferred by decision; Ian can overturn it. Size: L. Blocks 0.1: no.

## Confidence: medium

What was read: `probability.rs` and `threshold.rs` in full with their tests, the head of `answer.rs`, `cli/measure.rs` and the audit write path around the model and batch lines, `specification/threshold.md` and `audit.md` in full, the failure tables of `diff.md`, the audit refusal test's table and its sends-nothing test, the measure fixtures README, and the git history of the area's paths.

Not checked: the body of `core/measure/*` (the Wilson, bootstrap and tuning code) beyond test names, so no claim is made about their numeric correctness. About 25 of the 62 sentences in the audit and diff failure tables were located in the code by text search. The rest contain placeholders, and I did not confirm each against its test row. `transforms/band` and `transforms/calibration` were skimmed. The 13 bindings' threshold checks belong to bundle D. No test was run.
