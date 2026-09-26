---
flow: build
priority: 135
opens: crates/thinkthen/src/core/measure crates/thinkthen/src/core/measure.rs crates/thinkthen/src/cli/audit.rs crates/thinkthen/src/cli/audit crates/thinkthen/src/cli/measure.rs crates/thinkthen/src/cli/args/command.rs crates/thinkthen/tests/audit_sets.rs crates/thinkthen/tests/audit_refusals.rs crates/thinkthen/tests/support/measure.rs crates/thinkthen/tests/fixtures/measure specification/audit.md spec/audit.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0135: audit grades recognize and relate, and refuses a run with no label

Status: built. Design review accepted on the second pass (fresh read-only Claude session). `core/measure/items.rs` crossed its budget by more than a tenth (208 against 170). Coordinator re-scored after reviewer trim list: production at most 350, `items.rs` at most 195. Coordinator re-scored the audit_sets.rs budget to 372: the review-requested order test earns its lines. The build record gives the measured numbers. Owner: Claude.

Lane: thinkthen-lane-3

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

`thinkthen audit` grades saved `recognize --details` and `relate --details` lines against a key. It prints precision, recall, and F1 over names or edges, suggests a cut, shows how steady the cut is, and writes `threshold` through `--write` under 0125's digest and ReAnchor rules. audit also refuses a run in which no answer carries a label, and names `--id` in the sentence.

Ian ruled on 2026-09-25 that audit grades every function type in 0.1 (`sdlc/issues/closed/2026-09-25-audit-is-complete-for-0-1.md`, section 1). Ticket 0125 split `recognize` and `relate` out into `sdlc/issues/2026-09-25-audit-grades-recognize-and-relate.md`. The second issue, `sdlc/issues/2026-09-25-audit-exits-0-when-no-answer-matches-the-key.md`, was filed from a benchmark's worked examples. Each design point below is the agent's decision, and Ian can overturn it.

## What 0125 and 0131 already settle

This ticket reuses their machinery and adds no second copy of it:

- `rows.rs`: `count`, `added`, `share`, and the row assembly.
- `optimize.rs`: `Measure`, the split, `tune`, `beats`, `steady`, and `crossed`.
- `key.rs`: `Key`, `Want`, and the key line reader.
- `cli/audit/write.rs` and `splice.rs`: the digest check, the ReAnchor gate, and the byte-for-byte splice.
- The 1-to-99 cut grid, the four `--optimize` tie rules, twenty seeded splits, and the key split.

## Prior evidence

- Workspace experiment 214 (`experiments/214-wikigold-audit/`) graded recognize output three ways: strict span and type F1 (0.755 to 0.801), kind accuracy given an exact boundary (0.918 to 0.955), and boundary-only F1 (0.802 to 0.854). It counted 93 partial-overlap predictions and 37 split or truncated names as errors. A text-only match would have hidden them.
- The demo replays. Demo 44 at `--threshold 0.01` prints the same three names as at 0.5 (Maria Chen 0.98, Northwind Freight 1.0, Chicago 0.6693), since the recording answers requests and the cut is local. Demo 45 at `--threshold 0.01` prints two edges, `gateway calls billing` at 0.91 and `billing calls gateway` at 0.08. A recognize question file with demo 44's kinds replays the same recording, and a record-mode run with `--jsonl --field /text` prints `input`. Measured on this branch's build before any change.
- Both commands refuse a cut of 0: "recognize thresholds are single cuts above zero and at most one" (`core/recognize_file.rs`) and "the relate threshold is one cut above zero and at most one" (`core/relate_file.rs`).
- Name candidates do not depend on the cut. `core/recognize.rs::assemble` forms runs from each token's detection answer and then drops a name whose strength is below the cut. A run at a low cut therefore shows every name a higher cut could keep.

## Sources for the grading

- Tjong Kim Sang and De Meulder, "Introduction to the CoNLL-2003 Shared Task: Language-Independent Named Entity Recognition", CoNLL 2003. Precision, recall, and F1 over names. A name is right only when its boundaries and its type both match. Counts pool over the whole corpus (micro average). This is the default here, and experiment 214's metric A.
- Segura-Bedmar, Martínez, and Herrero-Zazo, "SemEval-2013 Task 9: Extraction of Drug-Drug Interactions from Biomedical Texts", SemEval 2013. The four entity match modes: strict (boundaries and type), exact (boundaries only), partial (overlap only), and type (overlap and type). The same four modes appear in the widely used `nervaluate` scorer. This ticket offers strict and type, under the names `strict` and `overlap`. SemEval's "exact" ignores the kind, so this ticket does not use that name.
- Chinchor, "MUC-7 Named Entity Task Definition", MUC-7 1998, and Nadeau and Sekine, "A Survey of Named Entity Recognition and Classification", Lingvisticae Investigationes 30(1), 2007. MUC scoring gives TYPE credit when the type is right and the places overlap. `overlap` follows that rule.
- Taillé, Guigue, Scoutheeten, and Gallinari, "Let's Stop Incorrect Comparisons in End-to-end Relation Extraction!", EMNLP 2020. Relation extraction reports micro precision, recall, and F1 over (relation, head, tail) triples. The "strict" setting needs both arguments' boundaries and types and the relation type. `relate` endpoints are a name and a kind, so the strict rule compares the relation name and both endpoints' names and kinds.
- Yao et al., "DocRED: A Large-Scale Document-Level Relation Extraction Dataset", ACL 2019. Directed triples scored by micro F1.
- `specification/relate.md`: `--either` and `either` make a rule unordered, and "unordered output normalizes endpoints to input order". An `either` edge has no direction, so its match ignores the order.

## Decisions

1. **Name match.** `--match strict`, the default, matches a name when `start`, `end`, and `kind` are equal (CoNLL-2003). `--match overlap` matches when the kinds are equal and the places overlap: `said.start < key.end` and `key.start < said.end`. Touching names do not overlap. The name text is not compared, because equal places in one text give equal text. Each key name matches at most one said name, and each said name at most one key name. Said names are taken by strength from high to low, ties in output order. Each takes the first unmatched key name, in key order, that it matches. `--match` applies to `recognize` and changes nothing for other verbs.
2. **Why greedy by strength.** Under any cut, the names kept are a prefix of that order. Greedy over the prefix makes the same matches as greedy over the whole list, cut short. The match therefore does not move as the cut moves, and a lower name never takes a key name from a higher one.
3. **Edge match.** An edge matches a key edge when `relation`, `source.name`, `source.kind`, `target.name`, and `target.kind` are equal. When the line's `question.relations` marks that relation `either`, the swapped endpoints also match. The same greedy rule applies, by `probability`.
4. **Pooling.** Counts pool over every record in the group (micro average), as CoNLL-2003 and Taillé et al. do. `true_yes` counts matches, `false_yes` extra said items, and `false_no` missed key items. `right` is the matches, and `wrong` is extra plus missed. In a count object `n` counts the labeled records, `answered` is `right + wrong`, and `true_no` stays the integer 0. `precision`, `yes_recall`, and `f1` use the existing formulas. The row's `true_no`, `agreement`, `interval`, `mean_probability`, `auc`, `disagreements`, `calibration`, `coverage`, and `curve` print null. The row's `true_no` is already nullable, so no member changes type. A count object's `agreement` and `coverage` print null. `unresolved` and `tied` print 0. Neither command has a true "no", so those measures have no meaning. Strength is not a probability ([recognize.md](../../specification/recognize.md)), so it gets no calibration either.
5. **`--optimize accuracy` tunes F1.** The default `accuracy` tunes F1 for these two verbs, and the objective reads `most f1 on the tuning part`. The share of agreeing items, matches over matches plus extra plus missed, rises and falls with F1, so the two pick the same cut. `precision`, `recall`, and `f1` apply with 0125's tie rules. A null measure never wins.
6. **Cuts start at the run's cut.** A saved line holds only the items at or above the cut the command ran with, `question.threshold`. audit cannot see a candidate below it. The tuning grid keeps 0125's `k` from 1 to 99 and skips every `k / 100` below the largest run cut over every line in the row. One floor serves the tuning part, the held part, every seeded split, and `crossed`, so no bar is ever scored on a line that ran above it. audit does not rebuild candidates from `answer.tokens` or `answer.questions`, because that copies the tokenizer and the relation planner's direction rules outside the command. The audit page tells users to run with a low cut such as 0.01 to tune across the whole range. For `--write`, that low cut goes in the question file itself, because the digest covers the threshold and a `--threshold` typed beside `@FILE` fails the check. After a write the file holds the tuned cut. A later re-tune below it needs the file lowered again first, and ReAnchor's "keep the current bar" check then compares against the low cut the run used.
7. **A line without its run cut.** A `recognize` or `relate` line without a numeric `question.threshold`, or a `relate` line without a `question.relations` list, is refused with the "cannot grade" sentence.
8. **`--threshold` over these rows.** A single cut at or above each line's run cut rescores it. A band, or a cut below a line's run cut, is refused with one new sentence.
9. **Only `recognize` names are graded.** A `recognize` line's beta `relations` stay out, and `--write` never touches `relation_threshold`. The relation grader serves `relate`, whose edges are settled.
10. **Record ids.** A `recognize` or `relate` line with no `input` takes its one-based line number, as 0125 gave `find`. `relate --details` never prints `input`, and a single-text `recognize` run does not either.
11. **The key value takes the command's own value shape.** `recognize` takes `{"entities": [{"kind", "start", "end"}, ...]}`, and `relate` takes `[{"relation", "source": {"name", "kind"}, "target": {"name", "kind"}}, ...]`. Other members, such as `name`, `strength`, and `probability`, are ignored. A user can correct a saved `value` and use it as the key. An empty list is labeled: every said item is extra. A null value leaves the record unlabeled, as today. Any other shape is refused. A key `kind` that is not a member name of the line's `question.kinds`, or a key `relation` absent from its `question.relations`, is refused with the existing sentence `key line N names a level, label, or unit the question does not have`. A typo such as `PERSON` for `PER` would otherwise turn every key name into a miss.
12. **A partial `relate` line is failed.** A line whose `meta.failed_questions` is above 0 lacks the edges of its failed questions. It counts in `failed` and leaves every measure. A `recognize` input fails whole already ([recognize.md](../../specification/recognize.md), "Records and details").
13. **`--write`.** It reads a `recognize` question file (an object with `recognize`) or a `relate` question file (an object with `relate`), besides 0125's question files and sets. It takes the digest the command takes: `recognize_sha256` of the parsed file, or the relate question's digest. A relate digest depends on `--lines`. A `--lines` run nulls `fields`. A line may therefore carry either of the file's two relate digests. The steady cut goes into the top-level `threshold` under 0125's ReAnchor rule and report lines.
14. **A run with no label is refused.** When the key holds at least one line, at least one answer did not fail, and no answer is labeled, audit prints nothing on standard output and exits 2 with `thinkthen: audit: no answer has a label in the key; check that --id points at the key's ids and that its values fit the verb`. The check runs after grading and before `--write` touches the file. The rule reads the whole run. It does not read each group. A key that labels only some questions of a set is legitimate and keeps exit 0. An empty key, an empty results file, and a run where every answer failed keep today's exit 0.
15. **Refusal wording.** The "cannot grade" sentence lists the two new verbs: `audit grades decide, filter, choose, tag, score, rank, find, recognize, and relate`. diff's sentence stays byte for byte.
16. **Table lines.** The table switches on the row's verb being `recognize` or `relate`, since `true_yes` is present on those rows too. Such a row prints its header line as today. In place of the agreement line and the yes/no lines it prints `  matched M, extra X, missed Y: precision P   recall R   f1 F`. Its suggested line reads `held f1 A as run -> B at the cut`. Its crossed line ends `f1 X` in place of the agreement and counts. Its held line reads `  at the suggested cut on the held part: precision P, recall R, f1 F`.

## The rules as inputs vary

| Case | Outcome |
| --- | --- |
| `strict`: same places, other kind | one extra, one missed |
| `strict`: places differ by one | one extra, one missed; `overlap` matches them |
| `overlap`: one name ends where the key name starts | no overlap: one extra, one missed |
| `overlap`: two said names overlap one key name | the stronger matches; the other is extra |
| `overlap`: a weaker said name overlaps only the first key name, and a stronger one overlaps both | the stronger takes the first key name: 1 matched, 1 extra, 1 missed. Output order would give 2 matched |
| `overlap`: one said name overlaps two key names | it matches the first in key order; the other is missed |
| A directed edge said in reverse | one extra, one missed |
| An `either` edge said in reverse | a match |
| Key value `{"entities": []}` or `[]` | labeled; every said item is extra; recall null |
| A record with nothing said and key items | every key item missed; precision null |
| Key value null | unlabeled |
| Key value of another shape | refused, exit 2 |
| A line with no `input` | its line number is its id |
| A line without `question.threshold`, or a relate line without `question.relations` | refused, "cannot grade" |
| A key kind or relation the line's question lacks | refused, exit 2 |
| `relate` line with `meta.failed_questions` above 0 | failed |
| `--threshold` at or above the run's cut | rescored |
| `--threshold` below a line's run cut, or a band | refused, exit 2 |
| Run cut 1 | no grid cut remains; suggested cut null |
| Nonempty key, some unfailed answer, zero labeled | refused, exit 2, sentence names `--id` |
| Ids match, but every key value is null or does not fit the verb | refused the same way; the sentence also names the values |
| Empty key, empty results, or every answer failed | exit 0, as today |
| A question set whose key labels one question only | exit 0, as today |

## Failures

| Case | Exit | Standard error |
| --- | ---: | --- |
| A band, or a cut below a line's run cut, over `recognize` or `relate` | 2 | `thinkthen: audit: recognize and relate take a single --threshold at or above the cut they ran with` |
| A `recognize` or `relate` key value of another shape | 2 | `thinkthen: audit: key line N gives recognize or relate a value unlike the command's own` |
| No answer labeled | 2 | `thinkthen: audit: no answer has a label in the key; check that --id points at the key's ids and that its values fit the verb` |
| Another verb (changed wording) | 2 | `thinkthen: audit: results line N holds an answer audit cannot grade; audit grades decide, filter, choose, tag, score, rank, find, recognize, and relate` |

A `recognize` or `relate` value that does not have the command's shape, or a strength or probability outside 0 to 1, takes the existing "cannot grade" and probability sentences. No sentence echoes a record, an id, a key value, a name, or a path.

## Where the code lives

- `core/measure/items.rs` (new): the said items of a line, the key items, and the greedy match that returns matched, extra, and missed under a rule.
- `core/measure/answer.rs`: the two verbs, the line-number id, and one field holding the said items and the run cut. All reading of the said items, the run cut, the kinds, the relations, and the partial `relate` failure goes in `items.rs`, so `answer.rs` gains only match arms and the field.
- `core/measure/key.rs`: `Want::Items` and the match mode.
- `core/measure/rows.rs`: `count` sums the matches for these verbs; `measures` nulls what has no meaning.
- `core/measure/optimize.rs`: the grid floor and the `accuracy` to F1 mapping.
- `core/measure.rs`: two new `MeasureError` variants.
- `cli/audit.rs`: `--match` and the no-label refusal. The table moves whole into `cli/audit/table.rs` when `cli/audit.rs` would pass 480 lines, and the new table lines go with it. `cli/measure.rs`: the sentences. `cli/audit/write.rs`: the two file kinds. `cli/args/command.rs`: the help names the two verbs.

No policy change. The core stays pure, and no file gains a write.

## Proof

Every test runs the built command. Each answers the four questions of the repository `CLAUDE.md` in the columns below. None needs a test-only export, flag, or hook, because each reaches the command line. audit refuses every `recognize` and `relate` line today, so no existing test grades one, and none can catch a regression in how they grade. The hand values sit in `tests/fixtures/measure/sets/README.md`. It lists every said and key item and works each count by hand. No expected value comes from the code under test.

The names fixture, `sets/names.jsonl`, holds seven `recognize --details` record lines run at 0.3, ids `a` to `g`, with ten said names and twelve key names:

| Id | Said (kind, places, strength) | Key | `strict` | `overlap` |
| --- | --- | --- | --- | --- |
| a | PER 0-5 0.9, ORG 10-20 0.4 | PER 0-5, ORG 10-20 | 2 matched | 2 matched |
| b | PER 0-6 0.8, LOC 30-35 0.35 | PER 0-5, LOC 35-40 | 2 extra, 2 missed | 1 matched, 1 extra (touching), 1 missed |
| c | ORG 0-8 0.95 | LOC 0-8, PER 12-15 | 1 extra, 2 missed | the same |
| d | none | PER 0-4 | 1 missed | the same |
| e | PER 0-3 0.7, PER 4-8 0.6 | PER 0-8 | 2 extra, 1 missed | 1 matched, 1 extra |
| f | PER 0-10 0.5 | PER 0-4, PER 6-10 | 1 extra, 2 missed | 1 matched, 1 missed |
| g | PER 0-3 0.4, PER 2-9 0.9 | PER 0-4, PER 5-9 | 2 extra, 2 missed | 1 matched, 1 extra, 1 missed |

`strict` totals 2 matched, 8 extra, 10 missed: precision 0.2, recall 0.166667, f1 0.181818. `overlap` totals 6 matched, 4 extra, 6 missed: precision 0.6, recall 0.5, f1 0.545455.

The edges fixture, `sets/edges.jsonl`, holds three `relate --details` lines with no `input`, run at 0.6, with a directed `calls` and an `either` `peers`. Line 1 says `calls a→b` 0.9, `calls c→a` 0.7, and `peers b–c` 0.8, keyed `calls a→b`, `calls a→c`, and `peers c→b`. Line 2 says `calls a→b` 0.65, keyed the same. Line 3 says nothing, has `meta.failed_questions` 1, and is keyed `calls a→b`. Every key line has `part: "tune"`.

| Test | Behavior it protects | Proof | Planted fault that turns it red |
| --- | --- | --- | --- |
| `audit_sets::names_match_strictly_or_by_overlap` | decisions 1, 2, 4 | The names fixture under both modes gives the totals above. `true_no`, `agreement`, and `calibration` print null | Match ignores the kind (c matches under `strict`). A second plant compares places with `<=` (b's touching names match). A third lets one key name match twice (e's second name matches). A fourth takes said names in output order (g gives 2 matched) |
| `audit_sets::edges_match_by_direction` | decisions 3, 10, 12 | The edges fixture as run: 3 matched, 1 extra, 1 missed; f1 0.75; `failed` 1. Ids read `1`, `2`, and `3` from the line numbers | Ignore `either`, and f1 reads 0.5. A second plant matches a directed edge in reverse, and f1 reads 1.0. A third grades the partial line 3, and `failed` reads 0 |
| `audit_sets::cuts_start_at_the_run_cut` | decisions 5, 6 | The edges fixture: F1 is 0.75 from 0.6 to 0.65 and lower above, so the cut is 0.6 with objective `most f1 on the tuning part`. `sets/cut.jsonl` is two recognize lines run at 0.3, every record tuned. Line 1 says one right name at 0.9. Line 2 says one right name at 0.46 and three extra at 0.47, 0.48, and 0.49. F1 is 4/7 up to 0.46 and 2/3 from 0.5 to 0.9, so the cut is 0.5. Matches over records would tie from 0.3 to 0.46 and pick 0.46 | Drop the floor, and the edges cut reads 0.5. A second plant tunes `accuracy` as right over records, and the recognize cut reads 0.46 |
| `audit_sets::edge_cases` | the edge-case table rows not above | Short inline lines: an empty key list is labeled with every said item extra and recall null; a run cut of 1 gives a null cut; a line without `question.threshold` is refused; a key kind the question lacks is refused; every answer failed keeps exit 0 | Skip the kind check, and the run exits 0. A second plant reads a missing run cut as 0 |
| `audit_sets::demos_grade` | the issue's done-when, decisions 10 and 11 | Replay demo 44 at `--threshold 0.01` in record mode, keyed with its three names: precision, recall, and f1 all 1.0. Replay demo 45 at `--threshold 0.01`, keyed with `gateway calls billing`: 1 matched, 1 extra; precision 0.5, recall 1.0, f1 0.666667. The `--table` output pins the matched line and the held line | Drop the line-number id for `relate`, and the run is refused |
| `audit_sets::write_puts_the_cut_in_recognize_and_relate_files` | decision 13 | A recognize file with demo 44's kinds at `threshold` 0.01, run twice as records 1 and 2, keyed without Chicago: audit writes `0.67`, reports `it was 0.01`, and every other byte stays. Demo 45's relate file at `threshold` 0.01, run twice, keyed with the one edge: audit writes `0.5`. A `--lines` relate run cannot be replayed, because no recording holds one. The test writes the line by hand: `{"version":1,"relate":{"relations":[{"name":"calls","source":"*","target":"*"}]},"threshold":0.01}` as the file, and a result line whose `meta.question_sha256` is the `sha256sum` of the lines-form question bytes printed in the fixture README. The line also holds a `value` and a `question` with `threshold` and `relations`, so it reaches the digest check | Take the question-file digest for a recognize file, and the write is refused. A second plant accepts only the non-`--lines` relate digest, and the hand-written line is refused |
| `audit_sets::refusals` | decisions 7, 8, 11, 14, 15, secrecy | Each row of "Failures", with a planted secret in the record, the key value, and the name. Each exits 2 with its exact line, empty standard output, and no secret. A key that labels one question of a two-question set keeps exit 0. No existing test runs a nonempty key with no label, so none catches the exit | Drop the no-label check, and the run exits 0. A second plant lets a band through |
| `audit_refusals::each_failure_prints_one_line…` | decision 15 | The existing sweep's "cannot grade" constant takes the new sentence. Its `relate` row lacks a run cut and stays refused. It pins the sentence, and no other test does | The sentence keeps the old verb list |
| `spec/audit.md` | the page | One block replays demo 45 at `--threshold 0.01` and pins the table's matched line | The page's pinned line |

The existing goldens and table captures run unchanged. No golden byte moves.

## Budgets

Nonblank lines, counted with the ratchet's rule.

- Production Rust: at most 340, new and changed together. No Rust file passes 500.
- `core/measure/items.rs`: at most 170.
- Rust tests: at most 360 in `tests/audit_sets.rs`, and at most 10 changed in `tests/audit_refusals.rs` and `tests/support/measure.rs`.
- `specification/audit.md`: at most 60 lines added or changed.
- `spec/audit.md`: at most 15 lines added.
- Fixtures: one folder, `tests/fixtures/measure/sets/`, with a README that works each value by hand.
- No dependency.

The ratchet rises by the measured increase in the commit that adds the code. That commit says what grew and why, and it names where the builder looked for duplication first: `count`, `added`, and `measures` in `rows.rs`, `tune` and `beats` in `optimize.rs`, `Key::want`, and the digest code in `write.rs`.

## Stop rules

Stop, write down what happened, and re-score before any of these:

- `answer.rs` passing 480 lines, or `cli/audit.rs` passing 480 before the table moves out;
- crossing a budget by more than a tenth;
- adding a dependency;
- changing a golden or table capture byte;
- changing a diff sentence or diff output;
- an existing test that expects exit 0 from a run with a nonempty key and no label;
- rebuilding candidates from `answer.tokens` or `answer.questions`;
- touching a file another in-flight ticket owns: ticket 0132's `engine/http.rs`, backend tests, `specification/backends.md`, `specification/check.md`, and `public_controls.rs`; the flaky-test Quick Fix's `libraries/c/tests/door`, `databases/postgresql` check, and `libraries/polars/tests/throttle_equality.rs`; ticket 0133's `sdlc/scripts` rungs, the children check, and the interrupt test switch; ticket 0134's public library entity `Debug` and public API; ticket 0136's Polars files.

## Scope and exclusions

Excluded: kind-blind matching (SemEval "exact" and "partial") and half credit for a partial match. Grading `recognize`'s beta relations. Rebuilding candidates below the run's cut. Macro averages over records. Calibration for these verbs. Any change to diff. Any live or paid call.

## Overlaps

`sdlc/ratchet.json` and the `sdlc` folders are shared. No in-flight ticket opens the measure code, the audit tests, or the audit pages. Whichever lands second merges.

## Complexity

Contract 2; state and timing 1; reach 1; proof 2; cost of error 2; total 8. Final level: 2. The risk is a match rule a reader takes for the benchmark's. The page names the rule and its source.

## Specification pages

- `specification/audit.md`: the two verbs in the opening, the low-cut advice with its `--write` trade-off, the command line, `--match`, "Inputs", the key, the match rule, the grid floor, the `accuracy` mapping, the nulls, the table lines, `--write`, the four failure rows. Status stays **Settled**, amended by this ticket.
- `spec/audit.md`: the block in "Proof".

## Closes

On landing, the lander closes both issues with a status line naming this ticket:

- `sdlc/issues/2026-09-25-audit-grades-recognize-and-relate.md`
- `sdlc/issues/2026-09-25-audit-exits-0-when-no-answer-matches-the-key.md`

## What Ian can overturn

- **`strict` as the default (decision 1).** The other choice is `overlap`. It forgives the boundary errors experiment 214 found most often.
- **Two match modes (decision 1).** Other choices: add SemEval's kind-blind modes, or half credit for a partial match.
- **Cuts start at the run's cut (decision 6).** The other choice rebuilds candidates from `answer.tokens` and `answer.questions`. The cost of this choice: a user who writes a tuned cut must lower the file's cut again before the next re-tune.
- **`accuracy` tunes F1 (decision 5).** The other choice says `accuracy does not apply to recognize`, so the default run suggests nothing.
- **`recognize` relations stay ungraded (decision 9).**
- **A partial `relate` line is failed (decision 12).** The other choice grades its surviving edges and counts the rest as missed.
- **The no-label rule reads the whole run (decision 14).** The issue suggested each group. That refuses a key that labels part of a question set.
- **Exit 2 for no label (decision 14).** The other choice is a warning on standard error at exit 0.

## Evidence

- Starts from: Ian's ruling in `closed/2026-09-25-audit-is-complete-for-0-1.md` and the split issue; the no-label issue from a benchmark's worked examples; experiment 214's three recognize metrics and its 93 partial-overlap and 37 boundary errors; the demo 44 and 45 replays at a low cut, measured on this branch; tickets 0125 and 0131 as the base; CoNLL-2003, SemEval-2013 Task 9, MUC-7 and Nadeau and Sekine 2007, Taillé et al. 2020, DocRED, and `specification/relate.md` for the grading.
- Keeps: Every golden and table byte. Every existing member, its order, and its type. Every 0125 and 0131 behavior for the eight verbs audit grades today. diff's output and sentences. audit sends nothing, reads no key or setting, and writes only what `--write` names.
- Changes: audit grades `recognize` and `relate` with precision, recall, and F1, suggests and writes their cut, and gains `--match strict|overlap`. A run with a nonempty key and no label is refused. Three new refusals, and the "cannot grade" sentence lists the two verbs.
- Proof: The tests in "Proof", each with a planted fault that turns it red, from hand fixtures worked in the fixture README and the committed recordings of demos 44 and 45.
- Defers: Kind-blind and half-credit matching. `recognize` relation grading and `relation_threshold`. Candidates below the run's cut. Macro averages. Calibration for these verbs. The same no-pair gap in diff. It has its own issue.
