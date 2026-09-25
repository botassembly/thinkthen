---
flow: build
priority: 125
opens: crates/thinkthen/src/core/measure crates/thinkthen/src/core/measure.rs crates/thinkthen/src/cli/audit.rs crates/thinkthen/src/cli/audit crates/thinkthen/src/cli/measure.rs crates/thinkthen/src/cli/find.rs crates/thinkthen/src/cli/args crates/thinkthen/tests/audit.rs crates/thinkthen/tests/audit_refusals.rs crates/thinkthen/tests/audit_verbs.rs crates/thinkthen/tests/audit_write.rs crates/thinkthen/tests/support/measure.rs crates/thinkthen/tests/fixtures/measure specification/audit.md specification/find.md specification/question-file.md spec/audit.md sdlc/scripts/policy.py sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0125: Complete audit

Status: ready. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user grades saved answers of any function against a key. The user picks the measure that fits the cost of each mistake. audit shows how steady its suggested bar is across many splits. With `--write`, audit puts a steady bar into the question file the answers came from, so the next run uses it and nobody copies a number by hand.

Ian ruled this into 0.1 on 2026-09-25 in `sdlc/issues/2026-09-25-audit-is-complete-for-0-1.md`. That ruling overturns ticket 0113's "other verbs after 0.1" and the closed issue `2026-09-24-audit-and-diff-needs-for-graded-agent-runs.md` on graded scores. The backlog of 2026-09-25 gives the dev team the `tag` key rule and the `rank` order measure. Every other design point below is the agent's decision, and Ian can overturn it.

This ticket also states the `uNNN` unit id rule for `find`, item 4 of `sdlc/issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md`. A `find` key names units by that id, so audit needs the rule written first.

## The split from diff

The diff issue (`2026-09-25-diff-exits-0-when-nothing-pairs-and-pairs-different-questions-silently.md`) leaves this ticket. It becomes a Quick Fix of options 2 and 4: two warnings on standard error, no stdout byte changed, and the goldens hold. The backlog puts diff (item 4) ahead of audit (item 6). diff touches `cli/diff.rs` and `core/measure/diff.rs` and needs about thirty lines. It should not wait for this build. The coordinator dispatches it. This ticket leaves `cli/diff.rs` and `core/measure/diff.rs` alone. It may touch `core/measure/answer.rs`, and the Quick Fix reads `meta.question_sha256` there too. Whichever lands second merges.

## Design

audit keeps its command, its inputs, its math, and every existing output member. Four additions sit on top.

1. **More verbs.** The answer reader learns `tag`, `score`, `find`, `rank`, `recognize`, and `relate` results. `filter` rows already read as `decide`. Each verb grades by the rules in "Every function type".
2. **`--optimize accuracy|precision|recall|f1`.** It names the measure the suggested bar maximizes. `accuracy` stays the default. The rows gain `precision` and `f1` beside `yes_recall`.
3. **`suggested.steady`.** audit tunes the bar on twenty seeded splits, counts the bars, and counts how often the tuned bar beats the run's own rule on the held part.
4. **`--write QUESTIONS`.** audit writes the steady bar into the question file the results came from. It checks the file's question digest against every result line first. It changes the bytes of one `threshold` value per written bar and nothing else.

The pure core gains the measures, the tie rules, the steady count, the new verbs, and the byte splice. The command gains two options, the file read, the digest check, and one file write. The write lives in a new small module that the policy check allows to call `std::fs::write` and nothing else.

## Decisions

Each is the agent's decision unless it says otherwise. Ian can overturn any of them.

1. **diff splits out as a Quick Fix.** See "The split from diff".
2. **The measures.** For a yes/no reading, yes is the positive answer. `precision = true_yes / (true_yes + false_yes)`. `recall` is today's `yes_recall`. `f1 = 2·true_yes / (2·true_yes + false_yes + false_no)`. Each is null on a zero denominator. `accuracy` keeps today's meaning: the most right answers on the tuning part. That is Ian's design, 2026-09-25.
3. **The tie rules are Ian's.** Recall takes the highest bar with the top recall. Precision takes the lowest bar with the top precision. Accuracy and F1 take the bar nearest 0.5, then the smaller bar. A null measure never wins. With no winner the cut is null.
4. **A measure applies only where it means something.** The table in "Every function type" names the measures per verb. A measure that does not apply prints `{cut: null, objective: "MEASURE does not apply to VERB", split, seed}`. `recognize` and `relate` have no true "no" answers, so accuracy has no meaning there. Under the default they optimize F1, and the objective says so.
5. **New members sit beside old ones, and the old goldens still hold.** Every row gains `precision`, `f1`, `r_precision`, and `mean_level_distance`. Each is null where it does not apply. Every count object gains `precision` and `f1` after `yes_recall`. Every suggested object gains `steady`. A `score` suggestion also carries `cuts`. The table gains whole lines only, with fixed openings. The old goldens and table captures hold when those members and lines are removed. "Proof" gives the exact check.
6. **Twenty seeded splits show how steady the bar is.** With a seeded split, audit tunes the bar again at seeds `S` to `S+19`, where `S` is `--seed`. Each addition wraps at 64 bits. Split 0 is today's split, so today's `suggested.cut` is one of the twenty. A key split has one split. ReAnchor checks up to five folds. Twenty splits cost little here and show the spread of a small sample better.
7. **`--write` writes only a steady bar.** It follows ReAnchor's rule: keep the current bar unless another bar scores strictly better. audit writes `steady.cut` only when the bar tuned on a split beats the run's rule on that split's held part in more than half the splits. Otherwise the file keeps its bar, and audit says why.
8. **`--write` checks the digest.** Every graded line must carry `meta.question_sha256`, or `meta.questions_sha256` for a question set, equal to the digest the file resolves to with no command-line override. The digest covers the threshold (question-file rule 8). A run with `--threshold` typed beside `@FILE` therefore fails the check. The rule refuses a file that did not ask these answers, and it refuses results that mix questions.
9. **`--write` never overwrites a band.** audit tunes a single cut. A file that holds a band keeps it, and audit says so.
10. **`--write` writes in place with one `std::fs::write` of the whole new text.** It keeps the file's owner, mode, and any symbolic link. A crash between the truncate and the write could leave a short file. The file is a few hundred bytes, so this ticket accepts that risk and names it in "Defers". A temporary file and a rename would create a file the user did not name.
11. **A `score` question takes no bar in a file.** ADR 0010 settles that `score` takes no threshold. audit grades `score` and prints suggested level cuts. `--write` leaves a `score` question unchanged and says why. Putting level cuts in the question file overturns ADR 0010. That needs Ian's ruling, and it is the one question this ticket brings him.
12. **`find` and `rank` take no bar.** Neither command takes a threshold. audit grades both and suggests nothing.
13. **The `tag` key lists the labels that apply.** A label missing from the list means no. A null value leaves the record unlabeled. This matches the `tag` result, where `[]` is a complete answer. The backlog gives this rule to the dev team.
14. **A `tag` question prints one row per label and one pooled row.** Each label row grades as yes/no and carries its own suggestion. The pooled row counts every label decision and carries the one shared cut the question file takes. Labels ride in one request, so their errors move together. The pooled row therefore prints no interval, calibration, coverage, or AUC.
15. **`rank` uses R-precision.** R-precision is the share of relevant records among the top R, with R the number the key calls relevant. AUC already measures the whole order. The backlog gives this rule to the dev team.
16. **Names and edges match exactly.** A `recognize` name matches a key name with the same text and kind. Places do not count. Each name matches at most once, so a repeated name counts as often as it appears. A `relate` edge matches on the relation and both endpoints' names and kinds. An `either` relation also matches with the endpoints swapped. Experiment 214 graded names by exact boundary and kind, and it counted a partial overlap as an error.
17. **A `find` or `relate` line has no `input`.** Each run of those commands prints one result for one whole input, so audit takes the line's one-based number in RESULTS as its record id.
18. **A rule never rescores `score` or `find`.** `--threshold` over a `score` or `find` answer is refused. `score` reads its number at level cuts, and `find` takes no rule.
19. **The ungradable sentence changes.** `thinkthen: audit: results line N holds an answer audit cannot grade; audit grades decide and choose` becomes `thinkthen: audit: results line N holds an answer audit cannot grade; save it with --details`.
20. **Deferred:** `--all`, per-label cuts in a `tag` file, option weights for `choose`, and `recognize` relation edges. See "Defers".

## Command line

```text
thinkthen audit RESULTS KEY [--by question|verb] [--threshold RULE] [--id POINTER] [--seed N] [--target A]
                            [--optimize accuracy|precision|recall|f1] [--write QUESTIONS] [--table]
```

- `--optimize` defaults to `accuracy`. An unknown value is Clap's usage error, exit 2.
- `--write QUESTIONS` names the question file or question set the results came from. `-` is refused.
- Every other option keeps its meaning.

Help, short and long, gains one line per option. Long help also says: `A key may give each record a part of tune or held; without parts audit splits the records itself and shows how steady its bar is.` That teaches the `part` column, option 4 of the issue's section 3. It also says: `--write changes one threshold in the file and prints the old value on standard error.` The help lives in `cli/audit.rs` and `cli/args`. Ticket 0126 owns help outside audit, diff, and find.

## Every function type

| Verb | How audit knows it | Key `value` | Graded as | Measures | Bar it suggests | `--write` |
| --- | --- | --- | --- | --- | --- | --- |
| `decide` | `question.verb` `decide` with a number or band in `threshold` | yes or no, as today | today's rows | all four | a cut, as today | `threshold` |
| `filter` | the same `decide` rows from `filter --details` | yes or no | as `decide` | all four | a cut | `threshold` |
| `choose` | `question.verb` `choose` | the option, as today | today's rows | accuracy | the lowest cut reaching `--target`, as today | `threshold` |
| `tag` | `question.verb` `tag` | the list of labels that apply | one yes/no row per label, plus a pooled row | all four | a cut per label, and one shared cut on the pooled row | the shared cut into `threshold` |
| `score` | `question.verb` `score` | the right level's name | exact level, plus `mean_level_distance` | accuracy | level cuts | never |
| `rank` | `question.verb` `decide` with `threshold` null | yes or no (relevant or not) | as `decide`, plus `r_precision` | none | none | never |
| `find` | `question.verb` `find` | `uNNN` or `none` | as `choose` over the unit ids | none | none | never |
| `recognize` | `question.verb` `recognize` | a list of `{name, kind}` | matched, extra, and missed names | precision, recall, f1 | a strength cut | `threshold` |
| `relate` | `question.verb` `relate` | a list of `{relation, source, target}` with `{name, kind}` endpoints | matched, extra, and missed edges | precision, recall, f1 | a probability cut | `threshold` |
| `annotate` | an `answers` object, as today | an object with one member per answer name, as today | each member by its verb | each member's | each member's | `questions.NAME.threshold` |

`filter --details` prints only kept records, so its rows cannot show a miss. The audit page says to grade a filter question from `decide --details` over the same file. A key value for `score`, `tag`, or `find` that names a level, label, or unit the answer does not hold is refused: `thinkthen: audit: key line N names a level, label, or unit the question does not have`. A `choose` key keeps today's rule.

### How each new verb reads

- **`tag`.** Each label becomes a yes/no answer with `p` from `answer.probabilities`. As run, a label is yes when `value` lists it. Its group name is the question's group name, a slash, and the label. The pooled row keeps the question's group name and prints `verb: "tag"`. Under `--by verb` only the pooled row prints, one per verb.
- **`score`.** The levels come from `question.levels`. As run, the level comes from `value` read at the cuts `0.5, 1.5, …`. A value on a cut takes the higher level. `right` and `wrong` count exact levels. `mean_level_distance` is the mean of the absolute gap in levels over labeled answers. `disagreements` lists key and said levels, as for `choose`. `unresolved` and `tied` stay zero. Calibration, coverage, and AUC are null.
- **Score level cuts.** Candidates are hundredths strictly inside `0` to `K − 1`. The search starts at the midpoint cuts. It moves each cut in turn, lowest first, to the candidate between its neighbours with the most exact levels on the tuning part. A move needs strictly more right answers. A tie between candidates goes to the one nearest the current cut, then the smaller. Passes repeat until one moves nothing. The right count only rises, so the search ends. `suggested.cut` is null and `suggested.cuts` lists the cuts.
- **`rank`.** Rows read as `decide`. As run every answer is `unresolved`, because `rank` makes no selection. `--threshold` grades them at a cut. `r_precision` orders labeled answers by `p` from high to low, ties in file order, and counts relevant records among the first R. It is null when R is 0.
- **`find`.** The distribution is `answer.probabilities` over the unit ids and `none`. As run, a non-null `value` says `answer.pick`. A null `value` says `none` when `none` alone holds the top, and `tied` when `none` ties. Calibration pairs follow `choose`. Coverage and the suggestion are null.
- **`recognize` and `relate`.** audit reads `value.entities` or the `value` edge list with each item's `strength` or `probability`. Under a cut an item counts when its number reaches the cut. `true_yes` counts matches, `false_yes` counts extra items, and `false_no` counts missed key items. `true_no`, `agreement`, `interval`, calibration, coverage, and AUC are null. `right` is the match count and `wrong` counts extra and missed items. A saved row holds only the items that reached the run's cut. Candidate cuts therefore start at that cut, and the audit page says to run with a low cut to see more.

## The suggestion and its steadiness

The tuning part, the held part, and the split stay as today. For the measure `M` in `--optimize`:

| Verb | Candidates | Winner |
| --- | --- | --- |
| `decide`, `filter`, a `tag` label, the `tag` pooled row | `k/100`, `k` 1 to 99 | the best `M` on the tuning part, by decision 3's tie rules |
| `choose` | `k/100`, `k` 1 to 100 | today's lowest cut reaching `--target` |
| `score` | the level-cut search above | the most exact levels |
| `recognize`, `relate` | `k/100` from the run's cut to 99 | the best `M`, by decision 3's tie rules |

Under `accuracy` the `decide` objective still reads `most agreement on the tuning part`, and `choose` keeps its text. The others read `most precision on the tuning part`, `most recall on the tuning part`, `most f1 on the tuning part`, and `most exact levels on the tuning part`. A `recognize` or `relate` row under the default reads `most f1 on the tuning part (accuracy does not apply to names)`.

`steady` prints `{splits, cut, count, low, high, better}`:

- `splits` is 20 for a seeded split and 1 for a key split.
- `cut` is the bar tuned most often. A tie goes to the cut nearest 0.5, then the smaller. For `score`, `cut` is the most common list of cuts, the first seen winning a tie, and `low` and `high` are lists taken cut by cut.
- `count` is how many splits tuned `cut`.
- `low` and `high` are the smallest and largest tuned bars. A split with no winner adds nothing to them.
- `better` counts splits where the tuned bar scores strictly higher on that split's held part than the run's rule. The score is `M`. For `choose` it is the held agreement, and for `score` it is exact levels.

`steady` is null when `suggested.cut` and `suggested.cuts` are both null on every split.

## `--write`

### What changes in the file

| File | Bar written | Member changed |
| --- | --- | --- |
| A `decide`, `choose`, or `tag` question file | the one question's `steady.cut`, from its pooled row for `tag` | top-level `threshold` |
| A `recognize` or `relate` file | `steady.cut` | top-level `threshold`. `relation_threshold` never changes |
| A question set | each member's `steady.cut` | `questions.NAME.threshold` |
| A `score` file or `score` member | nothing | nothing |

The value is the cut in its shortest form, such as `0.42` or `1`. When the member exists, its value bytes are replaced and every other byte stays. When it is absent, audit inserts it after the object's last member. The separator copies the bytes that come before the last member's key, from the comma on. With one member it is `, `. The key and colon copy the spacing of the last member's colon. A string, number, key, or nested object elsewhere in the file never changes.

The splice reads the file text with a small scanner in `core/measure/splice.rs`. The production parser has already accepted the file, so the scanner sees valid JSON. It decodes each key's escapes before it compares the name.

### Order of work

1. Read and grade the results and the key, as today.
2. Read QUESTIONS. Parse it with the production parser for its kind: a question set when it holds `questions`, a `recognize` or `relate` file when it holds that key, and a question file otherwise. Resolve it with `core` `resolve` and no typed value, and take its digest.
3. Check the digest against every graded line.
4. Decide each bar: write, keep, or skip.
5. Build the new text with the splice.
6. When any bar changes, write the file.
7. Print the report on standard error, one line per bar, in row order.
8. Print the rows on standard output.

A refusal in steps 1 to 3 or 5 writes nothing and prints nothing on standard output. A failed write in step 6 prints nothing on standard output.

### The report

`TARGET` is `the question` for a single file and `question NAME` for a set member. `OLD` is the old value exactly as the file wrote it, or `absent`.

| Case | Standard error line |
| --- | --- |
| Written | `thinkthen: audit: wrote threshold NEW for TARGET; it was OLD` |
| Not steady | `thinkthen: audit: kept the bar for TARGET; the tuned bar beat it on B of S held parts` |
| No suggestion | `thinkthen: audit: kept the bar for TARGET; audit found no cut to suggest` |
| A band | `thinkthen: audit: kept the band for TARGET; audit suggests a single cut` |
| `score` | `thinkthen: audit: kept TARGET unchanged; a score question takes no threshold` |

The command exits 0 in every case above.

### Refusals

Each prints nothing on standard output and leaves the file byte for byte.

| Case | Exit | Standard error |
| --- | ---: | --- |
| QUESTIONS is `-` | 2 | `thinkthen: audit: --write needs a file path` |
| `--by verb` with `--write` | 2 | `thinkthen: audit: --write grades by question; drop --by verb` |
| QUESTIONS cannot be read | 5 | `thinkthen: audit: cannot read the question file` |
| QUESTIONS is not a file a run accepts | 5 | `thinkthen: audit: --write names a file that is not a valid question file` |
| A line lacks the digest or carries another | 2 | `thinkthen: audit: results line N was not asked from the question file; --write needs --details lines from that file` |
| The write fails | 5 | `thinkthen: audit: cannot write the question file` |

### Undoing a write

The report prints the old value text. The file differs from before in that one value, or in one inserted member. To undo, put the old text back by hand or through version control. A second `--write` over results from the old file refuses, because the digest moved with the threshold. That guard keeps a stale run from writing over a new bar.

## Output

Rows keep every member and its order. The new row members are `precision` and `f1` after `yes_recall`, and `r_precision` and `mean_level_distance` after `auc`. Count objects gain `precision` and `f1` after `yes_recall`. `suggested` gains `steady` last, and a `score` suggestion carries `cuts` after `cut`.

The table keeps every line. It adds these whole lines, each only where its values apply:

- `  precision P   f1 F`, after the `said yes, key no` line.
- `  r-precision R`, for `rank`.
- `  mean level distance D`, for `score`.
- `  suggested level cuts C1, C2 (OBJECTIVE; …)` for `score`, in the form of today's suggested line.
- `  steady: C on K of S splits, range L to H; beat the run's rule on B of S held parts (seed N)`, after the suggested line. A key split prints `(key split)` in place of the seed. That prints the seed, option 1 of the issue's section 3.
- `  at the suggested cut on the held part: accuracy A, precision P, recall R, f1 F`, for yes/no rows.

## `find` unit ids

`specification/find.md` and the `find --details` help line state the rule. `uNNN` is the one-based input position, zero-padded to three digits: the first unit is `u001`, and the 255th is `u255`. No output changes. `specification/result.md` already shows `u001` and stays as it is.

## Where the code lives

- `core/measure/answer.rs`: the new verbs and how each reads.
- `core/measure/key.rs`: the new key values.
- `core/measure/audit.rs`: groups, rows, and today's math. The new row members join here.
- `core/measure/optimize.rs` (new): the four measures, the tie rules, the candidate search, and `steady`.
- `core/measure/levels.rs` (new): `score` levels, the distance, and the level-cut search.
- `core/measure/names.rs` (new): `recognize` and `relate` matching.
- `core/measure/splice.rs` (new): the byte splice for one member.
- `cli/audit.rs`: the two options, the help, and the new table lines.
- `cli/audit/write.rs` (new): read QUESTIONS, parse it, check the digest, decide each bar, write once, and report.
- `cli/find.rs` or `cli/args`: one help line.

The purity lint covers every `core/measure` file.

**Policy.** `MEASURE` in `sdlc/scripts/policy.py` gains `cli/audit/write.rs`. That file alone may name `std::fs::write`, and every other write-side name stays refused there. `cli/audit.rs`, `cli/measure.rs`, and `cli/diff.rs` keep every ban. The self-test plants `fs::write` in `cli/audit.rs`, and `rename`, `OpenOptions`, and `create` in `cli/audit/write.rs`. Each must be refused. It keeps one control: `fs::write` in `cli/audit/write.rs` passes. "What audit never does" in `specification/audit.md` changes to say that audit writes the one file `--write` names.

## Proof

Every test runs the built command. Each answers the four questions of the repository `CLAUDE.md`, and each has a planted fault that turns it red.

| Test | Behavior it protects | Proof | Planted fault that turns it red |
| --- | --- | --- | --- |
| `audit::old_goldens_hold` | today's output | Every golden line and table capture of 0113 equals the output after removing the members `precision`, `f1`, `r_precision`, `mean_level_distance`, and `steady`, and the table lines with the openings above. The comparison is byte for byte. A small byte-level JSON walker in `tests/support/measure.rs` removes the members and copies every other byte | The `decide` accuracy tie goes to the larger cut. A second plant adds `precision` in place of `yes_recall` rather than beside it, and the stripped line loses `yes_recall` |
| `audit::four_measures_pick_four_bars` | `--optimize` and the tie rules | 70 saved `decide` rows about Abbey Road, keyed with every record `part: "tune"`. `accuracy` 0.85, `precision` 0.95, `recall` 0.75, `f1` 0.73. At `--threshold 0.75` the row prints precision 0.583333, recall 1.0, f1 0.736842. Experiment 259 (U04) and the issue computed these by hand | Recall takes the lowest bar with the top recall. The cut reads 0.01 |
| `audit::steady_counts_twenty_seeds` | `steady` | The same rows with no parts at seed 0: `steady` is `{splits 20, cut 0.73, count 5, low 0.58, high 0.92}`. Experiment 259 (U06) measured seeds 0 to 19 with the landed binary: 0.58 three times, 0.68 three times, 0.73 five times, 0.83 four times, 0.85 four times, 0.92 once | The splits use seeds `S+1` to `S+20`. The count and range move |
| `audit::better_counts_held_wins` | the ReAnchor gate | `small/decide.jsonl` with its keyed parts: `steady.better` is 1 when the golden's held `at_cut.right` exceeds `at_run.right`, else 0, read from the golden file. A hand fixture where the tuned cut ties the run's rule on the held part gives 0 | `better` counts a tie. The tie fixture reads 1 |
| `audit_verbs::each_verb_grades` | the table in "Every function type" | One hand fixture per verb in `tests/fixtures/measure/verbs/`, each with a hand-computed expected row: `tag` (two labels, six records, pooled row and label rows), `score` (three levels where the search moves the cuts from 0.5 and 1.5 to hand-derived cuts and raises exact levels), `rank` (R-precision), `find` (`u002`, `none`, a tie with `none`), `recognize` (a repeated name and a kind mismatch), `relate` (an `either` edge in both orders), and `annotate` (the spec's decide, choose, and score example) | One fault per verb: a tag label missing from the key read as unlabeled; the level search accepting a tie; R taken as the row count; a `none` tie read as `none`; names matched without the kind; `either` ignored; an `annotate` score member refused |
| `audit_verbs::real_output_reads` | the reader against real saved output | Replay the committed recordings of demos 39 (`tag`), 17 (`score`), 15 (`find`), 06 (`rank`), 44 (`recognize`), and 45 (`relate`) with `--details` and the key unset. Pipe each to audit with a small hand key. Pin each row's `verb`, `rows`, `labeled`, and `right` | The reader looks for `answer.level` in place of `question.levels`. The `score` row fails |
| `audit_write::writes_one_value` | `--write` | A pretty-printed `decide` file with a non-ASCII question, an escaped key, and CRLF line ends. Replay `transforms/rows/recording` through `decide @FILE --jsonl --field /body --details`, then `audit --write FILE`. The file equals the hand-written expected bytes, and the report line is exact. A second `decide @FILE --details` prints the new `threshold` and a new digest | The splice re-serializes the file. The bytes differ |
| `audit_write::inserts_when_absent` | insertion | The same file without `threshold`, and a set whose member lacks one. The expected bytes are hand-written | The separator is always `,`. The pretty file loses its line break |
| `audit_write::refusals` | the refusal table | Each row: exit code, exact line, empty standard output, and the file byte for byte. The digest rows use a file whose text differs by one word, and results that mix two questions | The digest check is skipped. The mismatch row writes the file |
| `audit_write::keeps` | the keep rows | A band file, a `score` file, and results where the tuned bar never beats the run's rule. Each exits 0, prints its exact line, and leaves the file byte for byte | A band is overwritten. The band row changes the file |
| `audit_refusals::each_failure_prints_one_line…` | secrecy | The existing sweep gains every new refusal and keep row with planted secrets. No id, record, key value, or path appears | The digest refusal names the path |
| `audit_refusals::audit_sends_no_request…` | nothing else is touched | The existing no-request test gains every new line above. The listener sees no connection. Only the file `--write` names changes | Write a sibling temporary file. The folder check fails |
| `core::measure::splice` edge-case table | the splice | Rows: compact one line; pretty; member present as a number, a string cut, and `null`; member absent in an object of one and of several members; `"threshold":` inside a string value; a nested `threshold` that must not change; an escaped key `"threshold"`; a set member path; trailing white space and no final newline. Each row pins the exact output bytes | The scanner skips string escapes. The row with `"threshold":` inside a string changes the wrong bytes |
| `policy.py --self-test` | the write allowance | The plants in "Policy" | Drop the `write.rs` limit. `rename` there passes |
| `spec/audit.md` | the page | The four-measure picks on the Abbey Road fixture and one `--write` round trip on a temporary copy of a fixture file | The page's pinned lines |

The splice table is the one core test. A unit table there covers inputs the command cannot reach one by one. It needs no test-only export, because `core/measure` is already reachable from the crate's own test module. No other test needs a hook. The replays read committed recordings with the key unset and send nothing.

The Abbey Road rows are 70 `decide --details` lines saved by a private benchmark, song titles only. They join `tests/fixtures/measure/abbey/` with the benchmark's key and one derived key with every `part` set to `tune`. The fixture README records their source as "a benchmark's saved rows", their SHA-256, and the derivation. It names no private repository.

## Specification pages

- `specification/audit.md`: every section above that changes. The command line, the verb table, the measures and tie rules, `steady`, `--write` with its report, refusals, and undo, the new members, the new table lines, the changed failure sentence, and "What audit never does". Status stays **Settled**, amended by this ticket.
- `specification/find.md`: the `uNNN` rule, in "What it prints".
- `specification/question-file.md`: one sentence under rule 8. `audit --write` puts a tuned cut into the file, and the digest moves with it.
- `spec/audit.md`: the two executable blocks in "Proof".

## Budgets

Nonblank lines, counted with the ratchet's rule.

- Production Rust: at most 1,250, new and changed together. No Rust file passes 500.
- `cli/audit/write.rs`: at most 150.
- `core/measure/splice.rs`: at most 120.
- Rust tests: at most 900 in `tests/audit.rs`, `tests/audit_refusals.rs`, `tests/audit_verbs.rs`, `tests/audit_write.rs`, and `tests/support/measure.rs`, plus the splice table.
- `sdlc/scripts/policy.py`: at most 40 added.
- `specification/audit.md`: at most 150 lines added. Other pages: at most 30 lines added in total.
- Fixtures: the Abbey Road rows and two keys, one folder of hand fixtures per verb, and the question files for `--write`.
- No dependency.

The ratchet rises by the measured increase in the commit that adds the code. That commit says what grew and why. It names where the builder looked for duplication first: `core/measure/audit.rs` (counts and suggestions), `core/measure/answer.rs` (the `choose` top and tie), `core/threshold.rs`, `core/digest.rs`, `core/question_file.rs` `resolve`, and `cli/measure.rs`.

## Stop rules

Stop, write down what happened, and re-score before any of these:

- crossing a budget by more than a tenth;
- adding a dependency;
- changing a byte of an existing golden, table capture, or replay fixture;
- a digest path that needs the environment, the engine, or anything outside `crate::core`;
- widening the policy allowance beyond `std::fs::write` in `cli/audit/write.rs`;
- touching a file another in-flight ticket owns: relate and the splitter (0123), the cache and recorder (0124), help outside audit, diff, and find (0126), the test harness (0127), `databases/` (0129), `libraries/python` (0122), or `cli/diff.rs` and `core/measure/diff.rs` (the diff Quick Fix);
- `audit` over `249/control.jsonl` taking more than one second in a release build.

## Scope and exclusions

Excluded: diff, which is the Quick Fix. `--all`. Per-label cuts in a `tag` file. `choose` option weights, which change how `choose` picks and need an ADR. `recognize` relation edges, which are beta. Level cuts in a `score` file, which wait for Ian. A backup file. Grading a recording folder directly. Cost per row, repeated samples, and run identity, which the issue keeps after 0.1. Any live or paid call.

## Dependencies and order

Build from main. Tickets 0113 and 0114 are on main. No in-flight ticket owns a file this ticket opens, apart from the shared `answer.rs` noted in "The split from diff".

## Complexity

Contract 3; state and timing 1; reach 2; proof 2; cost of error 2; total 10. Final level: 3. The risk is a write that damages a user's question file. The digest check, the ReAnchor gate, the one-value splice, and the byte-for-byte tests guard it.

## Closes

On landing, the lander closes `sdlc/issues/2026-09-25-audit-is-complete-for-0-1.md`. It marks item 4 of `sdlc/issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md` done, and that issue stays open for its other items. The diff issue closes with the Quick Fix.

## What Ian can overturn

- Every numbered decision above.
- The diff split.
- Twenty splits, and the majority rule for `better`.
- The in-place write over a temporary file and a rename.
- Leaving `find` and `rank` without a bar.
- F1 as the default measure for names and edges.
- **Needs Ian:** level cuts for `score`. Option A keeps ADR 0010: audit prints the cuts, and a user applies them with `jq`. Option B amends ADR 0010: a `score` question file gains `cuts`, and `score` prints a level beside its number. Option B widens the question file, the result, and every surface that reads a score. It costs a ticket of its own after this one. The recommendation is A for 0.1.

## Evidence

- Starts from: Ian's ruling in `sdlc/issues/2026-09-25-audit-is-complete-for-0-1.md`. Tickets 0113 and 0114 and their records `sdlc/records/0113-build-audit-command.md` and `0114-build-diff-command.md`, which built audit and diff on the prototype's math and goldens. Experiment 259 (`experiments/259-talk-claims/RESULTS.md` in the workspace), which measured the four measures by hand on the Abbey Road rows (U04) and the seed spread over seeds 0 to 19 (U06). The Beatles Bench example `examples/11-audit`, whose slide worked the same four bars out by hand. Ian's inbox report `notes/reports/2026-09-25-dspy-jev-support-vs-audit-and-diff.md` on DSPy 3.4's ReAnchor: tune only bars, maximize the user's measure, and keep the current bar unless another scores strictly better across splits. Experiment 214 (`experiments/214-wikigold-audit/report.md`), which graded names by exact boundary and kind. `experiments/249-jev-answer-audit/README.md`, which shows a tuned cut moving held-out yes recall from 0.31 to 0.61.
- Keeps: Every 0113 output member, its order, and its value. Every golden and table capture, byte for byte, once the named additions are removed. The key format and `part`. The seeded split at `--seed`. Every existing failure row except the ungradable sentence. audit sends nothing and reads no key or setting.
- Changes: Six more verbs. `--optimize` and two new measures on every row. `steady` on every suggestion. `--write` and the first file audit writes. The ungradable sentence. The `find` unit id rule on two pages.
- Proof: The tests in "Proof", each with a planted fault that turns it red. The hand values come from experiment 259 and the issue, not from the code under test. `spec/audit.md` runs the picks and a write round trip.
- Defers: diff, as a Quick Fix. `--all`. Per-label `tag` cuts in a file. `choose` option weights. `recognize` relation edges. Level cuts in a `score` file, pending Ian. A crash-safe write. A warning when no answer matches the key (experiment 259, D14). Cost per row, repeated samples, and run identity.
