# 0125 build: complete audit

Builder: Claude (Opus subagent), 2026-09-25, on `ticket/0125-audit-complete` from the accepted ticket head `9c609296`. `origin/main` was merged before the final run (see "Rungs"). Ian can overturn every decision below.

## Outcome

audit grades saved `decide`, `choose`, `filter`, `tag`, `score`, `find`, and `rank` answers against a key. `--optimize accuracy|precision|recall|f1` picks the measure the suggested bar maximizes. Every seeded suggestion carries `steady`: the bars twenty seeded splits tuned, their counts, and `better`, the number of held parts where the most common bar beats the run's own rule. `--write QUESTIONS` checks every graded line's digest against the file and writes the steady bar into one `threshold` value when `better` is more than half of `splits`. Every other byte of the file stays.

On the Abbey Road rows keyed all `tune`, the four measures pick 0.85, 0.95, 0.75, and 0.73. At `--threshold 0.75` the row prints precision 0.583333, recall 1.0, and f1 0.736842. These match the issue's section 2 and experiment 259 (U04). Seeds 0 and 1 give the same cuts as experiment 259's `audit-cuts-200-seeds.txt`.

## Where the code lives

- `core/measure/answer.rs` reads each line into answers. `core/measure/verbs.rs` holds the per-verb readers: the verb a line answers, a `tag` answer's labels, a question's names, a probability, and a distribution's top. The coordinator asked for this split to keep `answer.rs` under the 500-line cap. `answer.rs` is 448 lines.
- `core/measure/rows.rs` assembles one row: counts, disagreements, R-precision, mean level distance, and coverage. These moved out of `audit.rs`, which now only groups answers.
- `core/measure/optimize.rs` holds the four measures, their tie rules, the split, the tuning search, and `steady`.
- `core/measure/levels.rs` holds `score` level cuts and their coordinate search.
- `core/measure/splice.rs` changes one member's value in JSON text. Its edge table is `splice_tests.rs`.
- `cli/audit/write.rs` reads the file, checks the digest, splices, and writes once with `std::fs::write`. `policy.py` allows that call in this file alone.

The digest check reuses `Answer.digest`, read inside `Answer::read`, as the coordinator asked. An annotate line's digest covers the whole set, and `--write` over a set compares it with the whole set's `QuestionSet::sha256`. `--write` needs no digest per question.

## Deviations from the ticket

1. **The `steady` shape.** `Suggested.steady` is `Option<Option<Steady>>`. It is absent where the measure does not apply, null where no split tuned a bar, and the count object otherwise. The ticket named the three cases and no type.
2. **Non-ASCII in the write test.** The ticket asked for a non-ASCII question in `writes_one_value`. The replay recording holds the payment question's request bytes, so a changed question misses the recording. The splice table covers non-ASCII text beside an escaped key instead.
3. **The refused verbs.** The `recognize`, `relate`, and bare `tag` refusals are rows of the existing refusal sweep in `audit_refusals.rs`. A separate test would check the same sentences twice.
4. **The `uNNN` help sentence** sits in `cli/args/find.rs`, the file that holds `find --details` help.

## Proof

Each test below ran green on the final code. Each plant was applied alone, its test run, the file restored byte for byte, and its modification time touched. A script outside the repository holds the plants. A grep of the diff for each plant's text found none.

| Test and plant | Result | Note |
|---|---|---|
| core tie test: accuracy tie goes to the larger cut | RED | first run GREEN; see below |
| old_goldens_hold: precision in place of yes_recall | RED |  |
| four_measures: recall takes the lowest bar | RED |  |
| steady_counts: splits use S+1 to S+20 | RED |  |
| better_reads_steady_cut: each split's own bar | RED |  |
| better_reads_steady_cut: a tie counts | RED |  |
| better_for_choose: agreement alone | RED |  |
| each_verb_grades: a tag label missing from the key reads as unlabeled | RED |  |
| each_verb_grades: the pooled row prints a Wilson interval | RED |  |
| each_verb_grades: the level search moves to the farthest tie | RED | first run GREEN; see below |
| each_verb_grades: R counts unlabeled rows | RED |  |
| each_verb_grades: a none tie reads as none | RED |  |
| each_verb_grades: an annotate score member is refused | RED | first run did not compile; see below |
| real_output_reads: answer.level in place of question.levels | RED |  |
| refusals sweep: audit hands diff its verb list | RED | first run GREEN; see below |
| writes_one_value: the splice re-serializes the file | RED |  |
| inserts_when_absent: the separator is always a comma | RED |  |
| refusals: the digest check is skipped | RED |  |
| keeps: a band is overwritten | RED |  |
| secrecy sweep: the digest refusal names the path | RED |  |
| writes_one_value: a sibling temporary file | RED |  |
| splice table: the scanner skips string escapes | RED |  |
| each_verb_grades: the does-not-apply objective reworded | RED | added after code review |
| each_verb_grades: choose tunes under any measure | RED | added after code review |

Four plants needed a second run.

- The accuracy tie plant left `old_goldens_hold` green, because no golden holds two cuts at the same distance from 0.5. The core table test `a_decide_tie_between_cuts_goes_to_the_one_nearer_one_half_then_the_smaller` holds that tie, and it turned red.
- The level search plant "accept a tie" changed nothing. The search breaks a tie toward the nearest place, and the current place is nearest, so a tie never moves a cut. The replacement plant sends a tie to the farthest place, and `each_verb_grades` turned red.
- The annotate plant no longer compiled after the verb reader moved to `verbs.rs`. The replacement reads annotate members as diff reads them, and `each_verb_grades` turned red.
- The diff plant left the sweep green. Its `tag` row had no labels, so the reader refused it either way. The row now names a label, and the plant turned it red.

The policy self-test holds the writer plants: `rename`, `OpenOptions`, `File::create`, `use std::fs::write`, and a bare `fs::write` in `cli/audit/write.rs` are refused. The existing plant keeps `std::fs::write` refused in every other measuring file, `cli/audit.rs` among them. The control, `std::fs::write` in `cli/audit/write.rs`, passes.

## Budgets

Nonblank lines against `9c609296`.

| Budget | Limit | Measured |
|---|---|---|
| Production Rust, new and changed | 1,000 | 1,093 |
| `cli/audit/write.rs` | 150 | 109 |
| `core/measure/splice.rs` | 120 | 117 |
| Rust tests and the splice table | 850 | 763 |
| `policy.py` added | 40 | 24 |
| `specification/audit.md` added | 150 | 81 |
| Other pages added | 30 | 3, plus 39 in `spec/audit.md` for the two executable blocks the ticket names |
| Largest Rust file | 500 | `core/measure/answer.rs`, 448 |

Production Rust crossed its budget by 93 lines, under the one-tenth stop line of 1,100. The first full build measured 1,142. The trim removed the `Steadiness` enum, shortened the splice, and flattened loops. What remains: `rows.rs` (242) is mostly moved code, and `audit.rs` shrank by 204. `optimize.rs` (300) holds the four measures, the steady count, and three tuning rules, one per verb family. `verbs.rs` (123) and the `answer.rs` growth (100) read four new verbs.

## Ratchet

The build commit raised the ceiling from 61,768 to 63,631: production code adds 1,093 lines, the tests and the splice table add 763, and the core test module adds 7. The merge of `origin/main` at `97129349` brought main's ceiling of 62,101 and the diff Quick Fix's digest reader. This branch already read the digest the same way, so the merge keeps one reader. The measured total after the merge is 63,942, and the ceiling equals it. Before adding code the builder looked for duplication in `core/measure/audit.rs`, `core/measure/answer.rs`, `core/threshold.rs`, `core/digest.rs`, `core/question_file.rs` `resolve`, and `cli/measure.rs`. The counts, disagreements, and coverage moved from `audit.rs` to `rows.rs` whole. The digest path reuses `resolve` and `question_sha256_with_profile`, and a set reuses `QuestionSet::sha256`. The `choose` top and tie stay in `answer.rs`, and `find` reuses them. The cut helper and the band text come from `rows.rs` and serve coverage, tuning, and `--write` alike.

## For ticket 0131

`steady` does not settle "audit tunes its cut on one half and never swaps". It tunes again on twenty seeded halves and counts wins on each held half. It still reports no summed held-out count over both halves of one split, and its candidate grid stays 1 to 99. Option 1 of that issue still stands.

## Rungs

The rungs ran on the merged branch without an outside lock.

- `install`: exit 0.
- `lint`: exit 0.
- `test`: the first run failed in `demo_runner`. The new `find --details` help said "unit", a word the help check refuses. The sentence now says "line or record". The second run passed, exit 0.
- `spec`: exit 0, with `spec/audit.md` at 4 passed and 21 demos green.
- `surfaces`: not run. No library surface or public Rust item changed.

A release build graded `249/control.jsonl` (272 rows) in 0.02 seconds with the seeded key and with the part key. The stop line is one second.

## Code review fixes

The code review accepted the math, the splice, the merge, and the pure core. It asked for four fixes, made in one commit.

1. The ticket and the recognize-and-relate issue named workspace paths. They now say "workspace experiment 259" and the like.
2. Decision 4 had no test. `each_verb_grades` gains an annotate `choose` row under `--optimize precision`. It pins the whole suggested object, with no `steady`, and the table line `suggested cut: none; precision does not apply to choose`. Two plants turn it red: a reworded objective, and `choose` tuned under any measure.
3. `cli/audit.rs` says in a comment that a missing steady count means the measure does not apply.
4. `write.rs` matches the row name and the set once. `Bar` serializes a cut through `value()`. `bar_text` writes a cut and level cuts through one path.

## Defers

- The in-place write can leave a short file if the process dies between truncate and write. The ticket accepts this.
- `recognize` and `relate` stay refused. `sdlc/issues/2026-09-25-audit-grades-recognize-and-relate.md` carries them.
- The five bench issues on main go to ticket 0131.
