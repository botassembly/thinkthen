# Audit

Status: **Settled** by ticket 0113, amended by tickets 0125 and 0135.

`thinkthen audit RESULTS KEY` grades saved answers against an answer key and suggests a bar. It reads `decide`, `filter`, `choose`, `tag`, `score`, `rank`, `find`, `annotate`, `recognize`, and `relate` results. It prints agreement with a Wilson interval, both disagreement directions, precision and f1, AUC, calibration, a coverage curve, and a suggested bar tuned on one part and checked on the other. It shows how steady that bar is across twenty splits. With `--write` it puts a steady bar into the question file the results came from. It sends no request and reads no API key.

The definition is the prototype measurement script at commit `be7cea2e`, with its tests and README. `crates/thinkthen/tests/fixtures/measure/README.md` gives the file checksums. Where this page and the prototype disagree, the golden files in that folder decide. "Departures" lists every known difference.

```sh
thinkthen decide 'Is it red?' --jsonl --details --replay runs/red < records.jsonl | thinkthen audit - key.jsonl
thinkthen audit results.jsonl key.jsonl --threshold 0.4 --table
```

## Command line

```text
thinkthen audit RESULTS KEY [--by question|verb|POINTER] [--threshold RULE] [--id POINTER] [--seed N] [--target A]
                            [--optimize accuracy|precision|recall|f1] [--match strict|overlap] [--write QUESTIONS]
                            [--curve] [--pooled] [--table]
```

- `RESULTS` holds saved result lines, and `KEY` holds a JSONL answer key. Either may be `-` for standard input, and not both.
- `--by question` is the default. `--by verb` pools the answers of each verb into one group. A value that starts with `/` is an RFC 6901 pointer into each line's `input`, as `--id` reads. The string there is the group name as is, and an integer becomes its decimal text. Each value splits again by verb, so one row never mixes two verbs, and two rows may share a group name. A `tag` answer gives its pooled row alone, as under `--by verb`. Any other value is the usage error.
- `--threshold RULE` takes the grammar of [threshold.md](threshold.md): a cut `T` or a band `LOW:HIGH`. It rescores each answer from its saved probabilities. Without it each answer stays as printed, and the output says `"as run"`.
- `--id POINTER` is an RFC 6901 pointer into each line's `input`, default `/id`. `--id ''` takes the whole input.
- `--seed N` is an unsigned 64-bit integer, default 0. It seeds the split and the bootstrap.
- `--target A` is the agreement a `choose` suggested cut must reach, default 0.9, from 0 to 1.
- `--optimize M` names the measure a yes/no suggested cut maximizes: `accuracy` (the default), `precision`, `recall`, or `f1`.
- `--match M` names how a `recognize` name matches a key name: `strict` (the default) or `overlap`. See "Names and edges".
- `--write QUESTIONS` names the question file or question set the results came from. audit writes the steady bar into it. See "Writing the bar".
- `--curve` fills each row's `curve`, and the pooled line's. See "Curve".
- `--pooled` prints one more line after the rows. See "The pooled line".
- `--table` prints the results for a person instead of JSONL.

`filter --details` prints only the records it kept, so its rows cannot show a miss. Grade a filter question from `decide --details` over the same records. A `rank` row prints `verb: "rank"`.

audit reads saved result lines only. A recording reaches audit through a replay: `decide ... --details --replay DIR` with the key unset, piped to `audit`. A recording holds no record ids, so it cannot be graded alone.

## Inputs

Each input is read whole. Lines split on newline. A blank line is skipped and still counted. Each other line is one JSON object, with no repeated member name and no number that is not finite.

**The record id** is the value at `--id` inside `input`. A string is used as is, and an integer becomes its decimal text.

**One graded answer** comes from an entry `E`. `E` is the line itself, or one member of its `answers` object, taken in member order under its member name.

- `E` failed when it has a `failure` member, when `E.value` is an object with a `failed` member, or when it has neither `answer` nor `value`. A failed answer counts in `failed` and leaves every measure.
- The verb is `E.question.verb` when that is a nonempty string. A `decide` entry whose `threshold` and `value` are both null is a `rank` answer. Without a verb, the entry is `decide` when `E.value` is a boolean, null, or absent, and `choose` when it is text. Any other value without a verb is refused with the `--details` sentence. Any other verb is refused.
- A `tag` entry becomes one yes/no answer per label, in the order of `E.answer.probabilities`, else `E.question.labels`. A label is yes as run when `E.value` lists it, and its `p` is its probability.
- A `score` entry reads its levels from `E.question.levels` and its number from `E.value`.
- A `find` entry's distribution covers the unit ids and `none`. As run, a text value says `E.answer.pick`. A null value says the top, or `"tied"` when the top is tied. A `find`, `recognize`, or `relate` line with no `input` takes its one-based line number as its record id. `find` names units `u001`, `u002`, and on, as [find.md](find.md) says.
- A `recognize` entry reads its names from `E.value.entities`, each with `kind`, `start`, `end`, and `strength`, and its kinds from the member names of `E.question.kinds`. A `relate` entry reads its edges from `E.value`, each with `relation`, `source` and `target` as `{name, kind}`, and `probability`, and its relations from `E.question.relations`. Both read their run cut from `E.question.threshold`. A line without its run cut, its kinds or relations, or a value of that shape cannot be graded. A `relate` line whose `meta.failed_questions` is above 0 lost the edges of those questions, so it counts as failed.
- The text is `E.question.text`, or null.
- `p` is `E.answer.probability`, or null. The distribution is `E.answer.probabilities`, or null. Its top is its largest value. Its pick is the first member, in member order, equal to the top. It is tied when two or more members equal the top.

**The key** is JSONL. Each line is an object with `id` (string or integer), `value`, and an optional `part` of `"tune"` or `"held"`. A `decide` value of `true` or `"yes"` is yes, and `false` or `"no"` is no. Any other `decide` value leaves the record unlabeled. A `choose` value is the right option's text. A `tag` value lists the labels that apply, and a label missing from the list is no. A `score` value names the right level. A `find` value is a unit id or `none`. A `recognize` or `relate` value takes the command's own value shape: `{"entities": [{kind, start, end}, ...]}` or `[{relation, source: {name, kind}, target: {name, kind}}, ...]`, other members ignored. A corrected saved `value` is therefore a key. An empty list is labeled. A kind or relation the line's question lacks is refused, and so is any other shape. A `rank` value is yes or no, as for `decide`. A `tag`, `score`, or `find` value naming a label, level, or unit the answer does not have is refused. For an `annotate` answer the key's `value` is an object, and its member under the answer name is the value. A record missing from the key, or a null value, is unlabeled. Unlabeled answers count in `unlabeled` and leave every measure.

**Duplicates.** audit refuses the same answer name, record id, question text, and label twice.

Both readers ignore members they do not use. A later version may read more of them.

## The math

**The answer under a rule.** For `decide`, a `tag` label, and `rank` as run: `true` is yes, `false` is no, and anything else is `"unresolved"`. Every `rank` answer is `"unresolved"` as run, because `rank` makes no selection. For `score`, the level is the count of cuts at or below the number, with cuts at `0.5, 1.5, …` as run. `--threshold` over a `score` or `find` answer is refused. Under a cut `T`: yes when `p >= T`, else no. Under a band: yes when `p >= HIGH`, no when `p < LOW`, else `"unresolved"`. For `choose`: a tied distribution is `"tied"` under every rule. As run, a null value is `"unresolved"` and any other value is itself. Under a cut, the pick when `top >= T`, else `"unresolved"`.

**The outcome.** `"tied"` and `"unresolved"` stay themselves. Otherwise the answer is `right` when it equals the key's value and `wrong` when it does not. An option named `tied` stays an option.

**Counts under one rule** over labeled answers: `n`, `right`, `wrong`, `unresolved`, `tied`, and `answered = right + wrong`. For a yes/no verb, a right yes adds to `true_yes`, a right no to `true_no`, a wrong yes to `false_yes`, and a wrong no to `false_no`. `agreement = right / answered` and `coverage = answered / n`, each null on a zero denominator. For a yes/no verb, `yes_recall = true_yes / (true_yes + false_no)`, `precision = true_yes / (true_yes + false_yes)`, and `f1 = 2·true_yes / (2·true_yes + false_yes + false_no)`, each null on a zero denominator and for other verbs. A count object prints `n, answered, right, wrong, unresolved, tied, true_yes, false_yes, true_no, false_no, agreement, coverage, yes_recall, precision, f1`.

**Wilson at 95%.** `Z = 1.959963984540054`, `k = right`, `n = answered`, `p = k / n`:

```text
centre = (p + Z²/(2n)) / (1 + Z²/n)
half   = Z · sqrt(p(1 − p)/n + Z²/(4n²)) / (1 + Z²/n)
interval = [max(0, centre − half), min(1, centre + half)], or null when n = 0
```

**Disagreements.** For `choose`, `score`, and `find`, `disagreements` lists each `(key, said)` pair among wrong answers with its count, sorted by count descending, then key, then answer, in code point order. It is null for yes/no verbs, whose directions are `false_yes` and `false_no`.

**Mean and AUC** (yes/no verbs, probabilities on every labeled answer). `mean_probability` is the mean of `p`. With `P` the `p` of answers keyed yes and `N` those keyed no, AUC is null when either is empty, else `(Σ over x in P, y in N of [x > y] + ½[x = y]) / (|P|·|N|)`.

**Tie share** (`choose` and `find`). An answer that reads as tied among `k` options at the top earns `1/k` when the key is one of them, and 0 otherwise. A `find` tie among real units prints the first unit, so it reads as that unit and earns no share. `tied_holding_key` counts the ties that hold the key, and `tie_share` sums their shares. Both count under every rule, and both are null for other verbs. `tied` and `right` keep their meaning.

**Calibration pairs** (probabilities on every labeled answer). Each pair is the answer's confidence in the answer it gave as run, under the question's own bar, and how right that answer was. `--threshold` does not move the pairs.

| Answer as run | Confidence | Right |
| --- | --- | ---: |
| Yes/no said yes | `p` | 1 when the key is yes, else 0 |
| Yes/no said no | `1 − p` | 1 when the key is no, else 0 |
| Yes/no unresolved | `max(p, 1 − p)` | 0 |
| `choose` or `find` | the top | 1 or 0 by the outcome, or the tie share for a tie |

Yes/no covers `decide`, `filter`, and each `tag` label. `score` and `rank` pair nothing, so their `calibration` and `curve` are null, as are the `tag` pooled row's. Failed and unlabeled answers pair nothing.

**Calibration.** Ten equal bins cover zero to one, and the last is closed at one. The error is the sum over bins of `|Σ right − Σ confidence|`, divided by the number of pairs. Sums of floats compensate each step as Python's `sum` does. The interval draws 1,000 bootstrap resamples with a SplitMix64 generator seeded with `--seed`, one generator per group. Each resample's error, less the bias, floors at 0. The bias is the mean resampled error less the estimate. The interval takes the linear-interpolation quantiles at 0.025 and 0.975 of the shifted errors, then widens to hold the estimate. `calibration` prints `error`, `interval`, `note`, and `by_bin`: ten `{low, high, n, right, confidence}` objects, where `right` sums the pairs' right and `confidence` is the bin's mean confidence, null when `n` is 0. The note says `the interval resamples the records; it does not cover rerun noise, so compare two runs of the same records`.

**Curve** (with `--curve`, over the calibration pairs). `curve` lists `{cut, kept, right}` at each distinct confidence, highest first. `kept` counts the pairs whose confidence reaches the cut, and `right` sums their right. Without `--curve` it is null.

**The pooled line** (with `--pooled`). One last line, `{pooled: "every verb", answers, calibration, curve}`, over the calibration pairs of every unfailed, labeled answer of a verb that pairs, whatever the grouping. `tag` labels stay out, because their errors move together. The bootstrap has its own generator seeded with `--seed`. `calibration` and `curve` are null when a pairing answer lacks probabilities.

**SplitMix64.** Each step adds `0x9E3779B97F4A7C15` to the state and mixes it with the published constants. `index(n)` is the high 64 bits of the 128-bit product `next() · n`. The shuffle runs `i` from `len − 1` down to 1 and swaps `i` with `index(i + 1)`.

**Coverage** (probabilities on every labeled answer, not `score` or `find`). For yes/no verbs, `k` runs 50, 55, ..., 100. At 50 the rule is the cut 0.5, and otherwise the band `(100 − k)/100 : k/100`, written as `0.45:0.55` and `0:1`. For `choose`, `k` runs 5, 10, ..., 100 with the cut `k/100`. Each point is `{cut, threshold, answered, coverage, right, accuracy}`.

**The split.** Take the distinct ids of the group's labeled answers in code point order. When every id has a `part`, the split is `"key"`. When some do and some do not, audit refuses the key. When none does, the split is `"seeded"`: a generator seeded with `--seed` shuffles the ids, the first `floor(len / 2)` tune, and the rest are held out.

**R-precision** (`rank`). Order the labeled answers by `p`, high to low, ties in file order. `R` counts those the key calls relevant. `r_precision` is the share of relevant answers among the first `R`, null when `R` is 0. A relevant record `rank --top` left out counts in neither.

**Mean level distance** (`score`). The mean absolute gap in levels between the level said and the key's level, over labeled answers with a number.

**The measures.** For `--optimize`, `accuracy` is `right / n`, and `precision`, `recall`, and `f1` are the count object's. A measure applies to `decide`, `filter`, and `tag` rows. `choose` and `score` take `accuracy` alone. Another measure prints `{cut: null, objective: "MEASURE does not apply to VERB", split, seed}`.

**The suggested cut** (probabilities on every labeled answer, not `rank` or `find`) is null when the tuning part is empty. For a yes/no verb, `k` runs 1 to 99 and the best measure on the tuning part wins. A null measure never wins. A tie under `accuracy` or `f1` goes to the smallest `|k − 50|`, then the smaller `k`. Under `recall` it goes to the larger `k`, and under `precision` to the smaller. For `choose`, the smallest `k` from 1 to 100 whose tuning agreement reaches `--target` wins, a null agreement counting as 0. With no winner the object is `{cut: null, objective, split, seed, steady, crossed}`. Otherwise it prints `cut, objective, split, seed, tune, held, steady, crossed`, where `seed` is null for a key split and `tune` and `held` print `{n, at_run, at_cut}`. The objective reads `most agreement on the tuning part` under `accuracy`, `most MEASURE on the tuning part` under another, and `lowest cut with agreement >= A` for `choose`.

**Level cuts** (`score`). Candidates are hundredths strictly inside `0` to `K − 1`, for `K` levels. The search starts at the midpoint cuts and moves each cut in turn, lowest first, to the place between its neighbours with the most exact levels on the tuning part. A move needs strictly more. A tie between places goes to the one nearest the current cut, then the smaller. Passes repeat until one moves nothing. `cut` prints null, and `cuts` after it lists the cuts. The objective reads `most exact levels on the tuning part`.

**Steady.** With a seeded split, audit tunes again on the splits seeded `S` to `S + 19`, where `S` is `--seed` and each addition wraps at 64 bits. Split 0 is the split above. A key split is one split. `steady` prints `{splits, cut, counts, better}`. `counts` lists each bar a split tuned as `{cut, count}`, in bar order, and a split with no bar adds nothing. `cut` is the bar with the largest count. A tie goes to the cut nearest 0.5, then the smaller, and for level cuts to the first list. `better` counts the splits whose held part `steady.cut` scores strictly better on than the run's rule:

| Verb | `steady.cut` wins a held part when |
| --- | --- |
| `decide`, `filter`, a `tag` label, the `tag` pooled row | its measure is strictly higher than the run's rule's, and a null measure never wins |
| `choose` | it reaches `--target` and the run's rule does not; when both or neither do, it has strictly more right answers and no lower agreement |
| `score` | it gets strictly more exact levels than the midpoint cuts |

`steady` is null when no split tuned a bar.

**Crossed.** `suggested.crossed` tunes a bar on each part of split 0 by the same rules, scores each part at the bar the other part tuned, and adds the counts: `{cuts: [A, B], held}`. `A` comes from the tuning part and `B` from the held part. `held` is a count object whose measures come from the summed counts. It is null when a part tunes no bar, and for `score` and the `tag` pooled row. It is absent where `steady` is absent.

**A `tag` question** prints its pooled row, named for the question, then one row per label, named `GROUP/LABEL`. Under `--by verb` it prints the pooled row alone. The labels ride in one request, so their errors move together. The pooled row therefore prints `interval`, `mean_probability`, `auc`, `calibration`, and `coverage` as null. It keeps the pooled counts and measures and tunes one shared cut, and its split keeps each record's labels on one side.

## Names and edges

`recognize` and `relate` say a set of items per record, so audit grades them as the named-entity and relation-extraction literature does: precision, recall, and F1 over items, pooled over every record of the group.

- **Names.** Under `--match strict` a said name matches a key name with the same `start`, `end`, and `kind`, as in the CoNLL-2003 shared task (Tjong Kim Sang and De Meulder, 2003). Under `--match overlap` the kinds are equal and the places overlap: `said.start < key.end` and `key.start < said.end`. This is the "type" mode of SemEval-2013 Task 9 (Segura-Bedmar et al., 2013) and MUC's TYPE credit. Touching names do not overlap. The text is not compared.
- **Edges.** An edge matches when `relation` and both endpoints' `name` and `kind` are equal, the strict setting of Taillé et al. (2020). An edge of a relation the line's question marks `either` also matches with its endpoints swapped, since [relate.md](relate.md) gives such a relation no direction.
- **One match each.** Said items are taken by strength or probability, high to low, ties in output order. Each takes the first unmatched key item, in key order, that it matches. The items a cut keeps are a prefix of that order, so the matches do not move as the cut moves.
- **Counts.** `true_yes` and `right` count matches, `false_yes` extra said items, and `false_no` missed key items. `wrong` is extra plus missed. `yes_recall`, `precision`, and `f1` follow "The math". A count object's `n` counts labeled records, `true_no` stays 0, and its `agreement` and `coverage` are null. The row's `true_no`, `agreement`, `interval`, `mean_probability`, `auc`, `disagreements`, `calibration`, `coverage`, and `curve` are null. Neither command has a true no, and strength is not a probability.
- **The cut.** `--optimize accuracy` tunes `f1` for these rows, since matches over matches, extra, and missed rise and fall with F1. The objective reads `most f1 on the tuning part`. A saved line holds nothing below the cut it ran with, so the grid skips every cut below the highest run cut in the row. `--threshold` takes a single cut at or above each line's run cut.
- **Run with a low cut.** To tune across the whole range, run with a low cut such as `--threshold 0.01`. Neither command takes 0. For `--write`, put that low cut in the question file itself, because the digest covers the threshold. After a write the file holds the tuned cut, and a later re-tune below it needs the file lowered again first.

`recognize`'s beta relations are not graded.

## Output

One JSON object per group per line, in the order of each group's first answer. The group name is the answer name, else the question text, else the verb, and an empty name falls through. Each row prints `group`, `verb`, `rows` (unfailed answers), `failed`, `labeled`, `unlabeled`, `threshold`, `right`, `wrong`, `unresolved`, `tied`, `tied_holding_key`, `tie_share`, `agreement`, `interval`, `true_yes`, `false_yes`, `true_no`, `false_no`, `yes_recall`, `precision`, `f1`, `mean_probability`, `auc`, `r_precision`, `mean_level_distance`, `disagreements`, `calibration`, `coverage`, `curve`, and `suggested`. `suggested` ends with `crossed`. The four directions, `yes_recall`, `precision`, and `f1` are null for `choose`, `score`, and `find`. "Names and edges" gives the members of `recognize` and `relate` rows. `r_precision` is null except for `rank`, and `mean_level_distance` except for `score`. The last five need labeled answers that all carry probabilities, and are null otherwise. A group whose answers all failed prints `verb: null`.

Every float rounds to six places. `threshold` prints `"as run"`, a cut as a number, or a band exactly as typed. An empty RESULTS prints nothing. A reader of this output ignores members it does not know, because a later version may add them.

`--table` prints the prototype's table from the rounded values. Numbers print with three decimals, rounded half to even on the exact binary value, and `-` stands for null. The table keeps the prototype's words, `unresolved` and `tied` included, because they label the JSON members of the same names. It adds these lines where their values apply:

- `  precision P   f1 F`, after the `said yes, key no` line of a yes/no row.
- `  r-precision R` for `rank`, and `  mean level distance D` for `score`.
- `  suggested level cuts C1, C2 (OBJECTIVE; …)` for `score`, in the form of the suggested cut line.
- `  steady: C on K of S splits, range L to H; beat the run's rule on B of S held parts (seed N)`, after the suggested line. A key split prints `(key split)`.
- `  ties holding the key: H of T, share S`, after the agreement line, for `choose` and `find` rows with ties.
- `  crossed: cuts A and B, each checked on the other part: agreement X, R right of N answered`, after the steady line.
- `  every verb pooled: calibration error E (95% L to H) over N answers`, as the last line, with `--pooled`.
- `  at the suggested cut on the held part: accuracy A, precision P, recall R, f1 F`, for yes/no rows.

A `recognize` or `relate` row prints `  matched M, extra X, missed Y: precision P   recall R   f1 F` in place of the agreement line and the yes/no lines. Its suggested line ends `held f1 A as run -> B at the cut`, its crossed line ends `f1 F`, and its held line reads `  at the suggested cut on the held part: precision P, recall R, f1 F`.

## Writing the bar

`--write QUESTIONS` reads the question file or question set, a set when it holds `questions`. A file that holds `recognize` or `relate` is that command's question file. audit takes the digest that command prints. A `relate` run under `--lines` nulls the fields, so a `relate` line may carry either of the file's two digests. Their bar is the top-level `threshold`. It resolves the file with no command-line value and takes its question digest. Every graded line must carry that digest in `meta.question_sha256`, or `meta.questions_sha256` for a set. The digest covers the threshold, so a run with `--threshold` typed beside `@FILE` fails. `rank` and `find` lines skip the check, since they write nothing.

audit follows ReAnchor's rule: keep the current bar unless another scores strictly better. It writes `steady.cut` only when `better` is more than half of `splits`. A `tag` question takes its pooled row's cut. A set member's cut goes to `questions.NAME.threshold`. The value prints in its shortest form, such as `0.42` or `1`. A present member's value bytes change, and every other byte stays. An absent member goes after the object's last member, with the separator copied from before that member's key and its key and colon spacing copied too. When a bar changes, audit writes the whole file once, in place. It prints one standard error line per bar, `TARGET` being `the question` or `question NAME`, and exits 0:

| Case | Standard error line |
| --- | --- |
| Written | `thinkthen: audit: wrote threshold NEW for TARGET; it was OLD` (`OLD` as the file wrote it, or `absent`) |
| Not steady | `thinkthen: audit: kept the bar for TARGET; the steady bar beat it on B of S held parts` |
| No suggestion | `thinkthen: audit: kept the bar for TARGET; audit found no cut to suggest` |
| A band | `thinkthen: audit: kept the band for TARGET; audit suggests a single cut` |
| `score` | `thinkthen: audit: kept TARGET unchanged; a score question takes no threshold` |
| `rank` or `find` | `thinkthen: audit: kept TARGET unchanged; audit suggests no bar for rank or find` |

To undo a write, put the old value back by hand or through version control. A second `--write` from the old results refuses, because the digest moved with the threshold. A crash during the write could leave a short file; the file is small, and a temporary file would be one the user did not name.

## Failures

A failure prints nothing on standard output and one line on standard error. The line never echoes a record, an id, a key value, or a path. `ROLE` is `results` or `key`, and `N` is a one-based line number.

| Case | Exit | Standard error |
| --- | ---: | --- |
| A file cannot be read | 5 | `thinkthen: audit: cannot read the ROLE file` |
| A line is not a UTF-8 JSON object | 2 | `thinkthen: audit: ROLE line N is not a JSON object` |
| No usable id | 2 | `thinkthen: audit: results line N has no string or integer id at the --id pointer` |
| `--id` is not a pointer | 2 | `thinkthen: audit: --id takes a JSON pointer such as /id or ''` |
| A value without its question verb that audit cannot read | 2 | `thinkthen: audit: results line N holds an answer without its question; save it with --details` |
| Another verb, a malformed `answers`, a `choose` value that is not text, or a `recognize` or `relate` line without its run cut or value shape | 2 | `thinkthen: audit: results line N holds an answer audit cannot grade; audit grades decide, filter, choose, tag, score, rank, find, recognize, and relate` |
| A key value naming a level, label, unit, kind, or relation the answer lacks | 2 | `thinkthen: audit: key line N names a level, label, or unit the question does not have` |
| `--threshold` over `score` or `find` | 2 | `thinkthen: audit: score and find answers take no --threshold` |
| A band, or a cut below a line's run cut, over `recognize` or `relate` | 2 | `thinkthen: audit: recognize and relate take a single --threshold at or above the cut they ran with` |
| A `recognize` or `relate` key value of another shape | 2 | `thinkthen: audit: key line N gives recognize or relate a value unlike the command's own` |
| The key has lines, some answer did not fail, and no answer is labeled | 2 | `thinkthen: audit: no answer has a label in the key; check that --id points at the key's ids and that its values fit the verb` |
| A bad probability or empty distribution | 2 | `thinkthen: audit: results line N holds a probability outside 0 to 1 or an empty distribution` |
| A repeated record | 2 | `thinkthen: audit: results line N repeats a record for one question` |
| A bad key line | 2 | `thinkthen: audit: key line N needs a new id, a value, and a part of tune or held when present` |
| A `choose` key value that is not text | 2 | `thinkthen: audit: key line N gives a choose value that is not text` |
| Parts on some records only | 2 | `thinkthen: audit: the key gives a part on some labeled records and not on others` |
| Two verbs in one group | 2 | `thinkthen: audit: one question holds both decide and choose answers` |
| A rule without probabilities | 2 | `thinkthen: audit: --threshold needs probabilities; rerun the question with --details` |
| A band on `choose` | 2 | `thinkthen: audit: choose takes a single cut; a band applies to decide` |
| Both inputs `-` | 2 | `thinkthen: audit: only one input may be standard input` |
| A bad `--threshold` | 2 | `thinkthen: audit: --threshold: ` and the threshold refusal |
| `--write -` | 2 | `thinkthen: audit: --write needs a file path` |
| `--write` with `--threshold` | 2 | `thinkthen: audit: --write reads each answer as it ran; drop --threshold` |
| `--write` with `--by verb` | 2 | `thinkthen: audit: --write grades by question; drop --by verb` |
| `--write` with a `--by` pointer | 2 | `thinkthen: audit: --write grades by question; drop the --by pointer` |
| `--by` starts with `/` and is not a pointer | 2 | `thinkthen: audit: --by takes question, verb, or a JSON pointer such as /category` |
| No string or integer at the `--by` pointer, or no `input` | 2 | `thinkthen: audit: results line N has no string or integer value at the --by pointer` |
| `--curve` with `--table` | 2 | `thinkthen: audit: --curve prints JSON lines; drop --table` |
| QUESTIONS cannot be read | 5 | `thinkthen: audit: cannot read the question file` |
| QUESTIONS is not a question file or set a run accepts | 5 | `thinkthen: audit: --write names a file that is not a valid question file` |
| A line lacks the digest or carries another | 2 | `thinkthen: audit: results line N was not asked from the question file; --write needs --details lines from that file` |
| The write fails | 5 | `thinkthen: audit: cannot write the question file` |

A `--write` refusal leaves the file byte for byte.

A bad `--seed`, `--target`, or `--by` gets the ordinary command-line usage error and exits 2.

## What audit never does

audit routes before any setup. It reads the named inputs and nothing else: no API key, environment variable, configuration file, cache, or usage counter. It opens no socket and starts no process. It writes standard output, standard error, and the one file `--write` names.

## Departures

No golden file reaches these. Each is the agent's decision, and Ian can overturn it.

- **Failures.** The prototype exits 2 through `argparse`, exits 1 with a traceback on a missing file or bad JSON, and echoes ids and record text. The port uses the failure table and echoes nothing.
- **Bad numbers.** A probability outside 0 to 1, a value that is not a number, or an empty distribution crashes the prototype. The port refuses it.
- **JSON.** The port refuses duplicate member names and `NaN`. Python accepts both.
- **Empty input.** The prototype prints one blank line. The port prints nothing.
- **Pointer.** The prototype accepts `-1` and leading zeros as array indexes. The port's RFC 6901 pointer refuses both.
- **Rule text.** The prototype parses with `float()`. The port uses the settled grammar.
- **Seed and target.** The port refuses a negative seed and a target outside 0 to 1.
- **Key values.** Python reads a `decide` key of `1` or `0` as yes or no. The port leaves it unlabeled. The port refuses a `choose` key value that is not text.
- **Choose values.** The prototype grades a saved `choose` value that is neither text nor null, `true` and `false` included. As run it compares the value with the key and counts it wrong, and under `--threshold` it grades the pick. The port refuses it as an answer it cannot grade.
- **State names.** The prototype reads an option named `tied` or `unresolved` as that state. The port keeps it an option.
- **Calibration.** Ticket 0131 moved calibration off the prototype's rule, and the goldens were recaptured with that change. The prototype pairs `(p, key is yes)`, drops ties, and takes a plain percentile interval. The port pairs the answer given, counts a tie at its share, and shifts the interval by the bootstrap bias. Errors from an older run do not compare with newer ones.
- **Parts.** The prototype README says every key line has a part or none does. The code checks each group's labeled records. The port follows the code.
