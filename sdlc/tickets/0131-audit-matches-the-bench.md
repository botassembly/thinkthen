---
flow: build
priority: 131
opens: crates/thinkthen/src/core/measure crates/thinkthen/src/core/measure.rs crates/thinkthen/src/cli/audit.rs crates/thinkthen/src/cli/measure.rs crates/thinkthen/tests/audit.rs crates/thinkthen/tests/audit_refusals.rs crates/thinkthen/tests/support/measure.rs crates/thinkthen/tests/fixtures/measure specification/audit.md spec/audit.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0131: Make audit give the numbers a benchmark needs

Status: ready. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it. The first design review returned findings on 2026-09-25, and this page is rewritten whole after it.

## Outcome and authority

A benchmark that grades its runs with `thinkthen audit` gets every number it publishes from audit. It gets these numbers:

- credit for ties that hold the right answer
- a held-out score in which every record is held out once
- a standard calibration error with its bins, per group and pooled across every verb
- the coverage curve at every confidence
- groups by any field of the record

Five issues filed on 2026-09-25 ask for this. A benchmark owner filed them while moving the benchmark's grading onto audit. Ian ruled on 2026-09-25 that everything goes into 0.1. Commit `82336e9a` records the ruling in `sdlc/planning/one-line-plan-2026-09-25.md`. This ticket adds a line naming itself to each of the five issues. Where the benchmark and audit use different methods, this ticket picks the method audit keeps from the issue's evidence and the standard literature. Each pick is the agent's decision, and Ian can overturn it.

## Order and preconditions

**This ticket builds after ticket 0125 lands.** 0125 rewrites a large part of audit: `--optimize`, `steady` over twenty seeded splits, `--write`, four more verbs, and the new files `core/measure/rows.rs`, `core/measure/optimize.rs`, `core/measure/levels.rs`, and `core/measure/splice.rs`. This design sits on 0125's result. Every name below that 0125 adds, such as `rows.rs`, `optimize.rs`, `steady`, `precision`, `f1`, or the tie rules of `--optimize`, means 0125's version.

**Precondition: 0125 lands `core/measure/answer.rs` under 500 lines.** 0125's own budget says no Rust file passes 500, and its branch holds 556 lines today. If 0125 lands it at 500 or more, 0131 stops before any code and the queue owner re-scores. 0131 adds nothing to `answer.rs` beyond one `use` line. It moves `Top` and its reader out into a new `core/measure/top.rs`, so `answer.rs` shrinks. The `--by` pointer reads in a new `core/measure/group.rs` and never enters `answer.rs`.

It also builds after the diff Quick Fix (`ticket/qf-diff-warnings`) lands, or merges it. See "Overlaps".

## What 0125 already settles

The issue `audit-tunes-its-cut-on-one-half-and-never-swaps` asks for three things. 0125 settles none of them:

- `steady` tunes the bar on twenty splits and counts the bars. It shows how steady the bar is. Each split still checks its bar on one half only. No number scores every record held out once.
- The candidates stay the hundredths grid.
- The split stays the seeded shuffle or the key's `part`.

This ticket adds the swap as `suggested.crossed`, keeps the grid, and keeps the split. The issue closes with this ticket.

## Prior evidence

Experiment 263 (`experiments/263-audit-vs-bench-methods/` in the workspace) ran the benchmark's Python and audit's methods side by side on five committed runs, with no model call.

1. **Candidates.** The benchmark's cross-fitted count uses the distinct probabilities plus 1.01. Audit's grid uses hundredths. They give the same held-out count in 13 of 35 question sets and differ in 22. Neither wins throughout: one set reads 19 against 18, another 18 against 24. On the tuning halves alone the grid reaches the best count in 61 of 70 halves. Six of the nine misses are one baseline whose probabilities are all 0, where the best bar says yes to everything. Most held-out differences come from where the tie rule puts the bar inside a gap between two probabilities.
2. **Calibration pairs.** On one model's 228 `decide` answers, pairing `(p(yes), key is yes)` gives 0.0830. Pairing each answer's confidence in the answer it gave gives 0.0470. On another model the two give 0.0353 and 0.0054.
3. **The interval.** Over 1,501 answers with an error of 0.0202, the benchmark's bias-shifted bootstrap gives 0.0058 to 0.0368. A plain percentile bootstrap over other draws gives 0.0154 to 0.0437.
4. **Ties.** One run has 15 tied answers. 8 hold the key, and their share is 4.00. That is the gap of 4.0 the benchmark's ticket names between its right answers and audit's.
5. **A benchmark bug.** The benchmark's calibration counts a bin's right answers as the number of nonzero credits. A tie holding the key therefore counts as fully right there, while its coverage curve sums the shares. Summing the shares moves one run's error from 0.0202 to 0.0182, and one baseline's from 0.6102 to 0.1573. audit sums the shares (decision 3). The benchmark owner filed this as `sdlc/issues/2026-09-25-calibration-counts-a-tie-holding-the-key-as-fully-right.md` in the benchmark, commit `0104beab`.

The issues name the benchmark's scripts: `scripts/score/score.py` (`default`, `credit`, `confidence`) and `scripts/score/stats.py` (`calibration`, `ece_interval`, `cross_cut`, `coverage`).

## Decisions

Each is the agent's decision unless it says otherwise. "What Ian can overturn" lists them.

1. **A tie gets its share.** A `choose` or `find` answer tied at the top among `k` options earns `1/k` when the key is one of them, and nothing otherwise. That is the expected accuracy of a random pick among the tied options. McSherry and Najork (2008, "Computing information retrieval performance measures efficiently in the presence of tied scores", ECIR) define a tie-aware measure as the mean over every order of the tied items. For top-1 accuracy that mean is `1/k`. `tied` keeps its meaning, and `right` never counts a share. The row adds `tied_holding_key` and `tie_share` after `tied`. Both are null for verbs that cannot tie.
2. **Calibration pairs each answer with its confidence in the answer the command gave.** The given answer is the saved answer as run, under the question's own bar. That is what the benchmark's `default()` grades. It is not `p ≥ 0.5`. For a yes/no answer that said yes, the confidence is `p`. For one that said no, it is `1 − p`. Under a bar of 0.7, an answer at `p` 0.6 said no, so its confidence is 0.4. An unresolved yes/no answer pairs at `max(p, 1 − p)` and is wrong, as the benchmark pairs an answer with no value. For `choose` and `find` the confidence is the top, because the given answer is the pick or nothing. This is the calibration of the answers a user actually receives. It follows Guo, Pleiss, Sun, and Weinberger (2017, "On calibration of modern neural networks", ICML), built on Naeini, Cooper, and Hauskrecht (2015, AAAI), who pair the confidence in the predicted answer with its correctness. Today audit pairs `(p, key is yes)` for `decide`, which measures the yes probability alone. That reading stays visible through `mean_probability` and `auc`. The pairs ignore `--threshold`, as today. Ten equal bins stay. The benchmark adopts this rule. At its bar of 0.5 it matches the benchmark's `confidence()` and `default()`.
3. **A tie enters calibration at its share.** Today audit drops tied `choose` answers from calibration. They now pair their top with the share from decision 1. A bin's right count is the sum of shares. That keeps calibration consistent with `tie_share` and avoids the benchmark bug in "Prior evidence" item 5.
4. **The interval is the bias-shifted percentile bootstrap.** Binned calibration error is biased upward, and a resample's error runs higher still (Kumar, Liang, and Ma 2019, "Verified uncertainty calibration", NeurIPS). A plain percentile interval can then sit above the estimate. audit keeps its 1,000 draws and SplitMix64. It estimates the bias as the mean resampled error minus the estimate (Efron and Tibshirani 1993, *An Introduction to the Bootstrap*, section 10.2). It subtracts that bias from each resample, floors each at 0, and takes the 0.025 and 0.975 quantiles. The interval then widens to hold the estimate if it does not already. This is the benchmark's method. The draws differ, because the benchmark seeds Python's generator with a string. The benchmark matches audit in method and not draw for draw.
5. **The calibration object prints its bins.** `calibration` adds `by_bin` last: ten `{low, high, n, right, confidence}` objects. `right` is the sum of shares, and `confidence` is the bin's mean confidence, null when `n` is 0. The `note` text stays.
6. **`--pooled` adds one calibration line across every verb.** The benchmark's published error pools every answer of a run, `decide` and `choose` together. With `--pooled`, audit prints one more JSON line after the rows: `{"pooled": "every verb", "answers": N, "calibration": {...}, "curve": [...]}`. Its pairs are decision 2's over every unfailed, labeled answer of a verb that pairs, whatever the grouping. The bootstrap resamples those pairs with one generator seeded with `--seed`. `curve` follows `--curve`. `tag` labels stay out, because 0125 bans pooling their correlated errors. `score` and `rank` stay out, because they give no answer to pair. When a pairing answer lacks probabilities, `calibration` and `curve` are null, as for a row. Without `--pooled` nothing changes, so every golden and every existing reader holds. Under `--table` it prints `  every verb pooled: calibration error E (95% L to H) over N answers`.
7. **The cut keeps the hundredths grid.** The candidates stay `k/100`, `k` 1 to 99 for yes/no answers and 1 to 100 for `choose`. Fayyad and Irani (1992, "On the handling of continuous-valued attributes in decision tree generation", *Machine Learning* 8) show that the best cut on a sample lies at a boundary between two adjacent sorted values. Any cut in that gap scores the same on the tuning part. The grid finds nearly every such gap: 61 of 70 halves in experiment 263. A bar of two decimals reads well in a question file, and `--write` writes it there. The 0113 goldens pin the grid. The benchmark adopts audit's grid when it moves its held-out column, and its numbers shift by the amounts in experiment 263.
8. **`suggested.crossed` scores every record held out once.** audit tunes a bar on each part of the run's split, scores each part at the bar tuned on the other, and adds the counts. This is two-fold cross-validation (Hastie, Tibshirani, and Friedman 2009, *The Elements of Statistical Learning*, section 7.10). It uses the split `suggested` reports: the key's parts, or the seeded split at `--seed`. A caller who wants its own halves writes them as `part` in the key, as the benchmark can from the SHA-256 of each id. No new split method enters.
9. **`--by POINTER` groups by a field of the record.** A value that starts with `/` is an RFC 6901 pointer into each line's `input`, as `--id` reads. A string value is the group name as is, and an integer becomes its decimal text. Each value splits again by verb, so one row never mixes two verbs. `--pooled` gives the one number across verbs. A missing value, or one of another type, is refused. `question` and `verb` keep their meaning.
10. **`--curve` adds the coverage curve at every confidence.** The row's `curve` member lists `{cut, kept, right}` at each distinct confidence from the highest down. `kept` counts the answers whose confidence reaches the cut. `right` is their sum of shares. The pairs are decision 2's. This is the empirical risk and coverage curve of selective classification (El-Yaniv and Wiener 2010, *JMLR* 11; Geifman and El-Yaniv 2017, NeurIPS). Without `--curve` the member is null, so a row stays short over many records. The grid `coverage` stays as it is.
11. **`--curve` with `--table` is refused.** The table has no curve lines, and a user who asks for a curve should not get silence.
12. **`--write` needs `--by question`.** 0125 refuses `--by verb` beside `--write`. A pointer is refused the same way, with its own line.
13. **Failed answers stay out of calibration, the curve, and the pooled line.** The benchmark pairs a refused answer at confidence 0 and counts it wrong. audit's rule stays: a failed answer leaves every measure. The row's `failed` count lets a caller add them back.
14. **`rank` pairs nothing.** 0125 reads every `rank` answer as unresolved, because `rank` gives no yes or no. With no given answer there is nothing to pair, so a `rank` row's `calibration` and `curve` are null.

## The output shape

0.1 freezes the output. Every existing member keeps its name, order, and type. The changes are few:

| Where | Change |
| --- | --- |
| Row | `tied_holding_key` and `tie_share` after `tied` |
| Row | `curve` after `coverage`, null without `--curve` |
| `calibration` | `by_bin` last |
| `suggested` | `crossed` last, after 0125's `steady` |
| `calibration.error`, `calibration.interval` | new values, by decisions 2, 3, and 4 |
| A last line | the pooled line, only with `--pooled` |
| Command line | `--by` takes a pointer, and `--curve` and `--pooled` are new |
| Table | three new whole lines, below |

The table adds these lines, each only where it applies:

- `  ties holding the key: H of T, share S`, after the agreement line, for `choose` and `find` rows with `tied` above 0.
- `  crossed: cuts A and B, each checked on the other part: agreement X, R right of N answered`, after 0125's steady line.
- `  every verb pooled: calibration error E (95% L to H) over N answers`, as the last line, with `--pooled`.

## Command line

```text
thinkthen audit RESULTS KEY [--by question|verb|POINTER] [--curve] [--pooled] …every 0125 option…
```

- `--by` takes `question`, `verb`, or a value that starts with `/`. Any other value is Clap's usage error, exit 2. The whole standard error is pinned: `error: invalid value 'colour' for '--by <question|verb|POINTER>': --by takes question, verb, or a JSON pointer such as /category`, a blank line, and `For more information, try '--help'.`
- A value that starts with `/` and is not a valid pointer, such as `/a~2`, is audit's refusal in "Failures".
- `--curve` fills each row's `curve`, and the pooled line's.
- `--pooled` prints the pooled line.
- Help gains one line each for `--curve` and `--pooled`, and one clause in `--by`: `or a JSON pointer such as /category into each line's input`.

## The rules

### Tie share

| Answer | `tied` | `tied_holding_key` | `tie_share` |
| --- | ---: | ---: | ---: |
| Untied pick equal to the key | 0 | 0 | 0 |
| Tie of 2 holding the key | 1 | 1 | 0.5 |
| Tie of 4 holding the key | 1 | 1 | 0.25 |
| Tie of 2 missing the key | 1 | 0 | 0 |
| `find` tie of `u002` and `none`, key `none` | 1 | 1 | 0.5 |
| Any `decide`, `filter`, `tag`, `score`, or `rank` row | as today | null | null |

The share counts under every rule, because `tied` holds under every rule. `Top` in the new `core/measure/top.rs` keeps the options that hold the top, in member order.

### Calibration pairs

| Answer as saved | Confidence | Right |
| --- | --- | ---: |
| Yes/no said yes, `p` 0.9, key yes | 0.9 | 1 |
| Yes/no said no, `p` 0.1, key yes | 0.9 | 0 |
| Yes/no said yes, `p` 0.5, key no | 0.5 | 0 |
| Yes/no said no at a bar of 0.7, `p` 0.6, key no | 0.4 | 1 |
| Yes/no said no, `p` 0.35, key no | 0.65 | 1 |
| Yes/no unresolved under a band, `p` 0.55, key yes | 0.55 | 0 |
| `choose` untied pick equal to the key | top | 1 |
| `choose` tie of 2 holding the key | top | 0.5 |
| `choose` tie missing the key | top | 0 |
| `choose` below its bar, value null | top | 0 |
| Failed or unlabeled | not paired | |
| A labeled answer without probabilities in the group | calibration and curve null, as today | |

Yes/no covers `decide`, `filter`, and each `tag` label. A `tag` label said yes when the saved value lists it.

The error is the sum over bins of `|Σ right − Σ confidence|`, divided by the number of pairs. Sums compensate as today.

### Where each piece applies

| Row | Tie members | Calibration and curve | `crossed` |
| --- | --- | --- | --- |
| `decide`, `filter` | null | yes | yes |
| `choose` | yes | yes | yes |
| `find` | yes | yes | null |
| a `tag` label row | null | yes | yes, per label on that label's parts |
| the `tag` pooled row | null | null, as 0125 rules | null |
| `score` | null | null, as 0125 rules | null |
| `rank` | null | null (decision 14) | null |
| an `annotate` member | by the member's verb | by the member's verb | by the member's verb |

### Crossed

| Case | `crossed` |
| --- | --- |
| Both parts nonempty and each tunes a bar | `{cuts: [A, B], held: COUNTS}`. A is tuned on the tuning part and B on the held part. `held` sums the held part at A and the tuning part at B |
| A seeded split of an odd count | the same. The parts hold `floor(n/2)` and the rest, and every record counts once |
| One part empty, such as a key with every record `tune` | null |
| A part with no winner, such as a `choose` part that never reaches `--target` | null |
| A `tag` label row | as a yes/no row, on the label's own parts |
| The `tag` pooled row, `score`, `rank`, `find` | null. None tunes one cut on its own answers |

`COUNTS` is 0125's count object. Its measures come from the summed counts, not from a mean of two. Each part tunes by the measure in `--optimize` with 0125's tie rules, or for `choose` by the lowest cut reaching `--target`. The tuner is the one `suggested.cut` uses.

### Curve

| Case | Points `{cut, kept, right}` |
| --- | --- |
| Five `choose` answers at 0.6 (right), 0.6 (wrong), 0.45 (tie missing the key), 0.4 (tie of 2 holding the key), 0.25 (tie of 4 holding the key) | `{0.6, 2, 1}`, `{0.45, 3, 1}`, `{0.4, 4, 1.5}`, `{0.25, 5, 1.75}` |
| The first, second, third, fourth, and fifth yes/no rows of "Calibration pairs" | `{0.9, 2, 1}`, `{0.65, 3, 2}`, `{0.5, 4, 2}`, `{0.4, 5, 3}` |
| Every answer at one confidence | one point holding every answer |

The benchmark's `stats.coverage` gives these same points over the same pairs.

### `--by POINTER`

| Value at the pointer | Group |
| --- | --- |
| `"lead-set"` | `lead-set`, one row per verb in first-seen order |
| `3` | `3` |
| missing, null, a boolean, an array, or an object | refused, exit 2 |
| a `find` line, which has no `input` | refused, exit 2 |
| an `annotate` line | each member joins the row of the record's value and the member's verb |
| a `tag` answer | the pooled row only, as 0125 does under `--by verb` |
| `/a~1b` | the member `a/b` |
| `/a~2` | refused as a malformed pointer, exit 2 |

The row's `group` is the value. Two rows share a group name when the value holds two verbs, and their `verb` members differ. `group.rs` reads the value from each numbered line's `input`, and `audit.rs` joins it to each answer by its line number.

## Failures

Each prints nothing on standard output and one line on standard error. None echoes a record, id, key value, path, or pointer.

| Case | Exit | Standard error |
| --- | ---: | --- |
| `--by` starts with `/` and is not a JSON pointer | 2 | `thinkthen: audit: --by takes question, verb, or a JSON pointer such as /category` |
| No string or integer at the `--by` pointer | 2 | `thinkthen: audit: results line N has no string or integer value at the --by pointer` |
| `--curve` with `--table` | 2 | `thinkthen: audit: --curve prints JSON lines; drop --table` |
| A pointer `--by` with `--write` | 2 | `thinkthen: audit: --write grades by question; drop the --by pointer` |

The first row follows the `--id` sentence, `thinkthen: audit: --id takes a JSON pointer such as /id or ''`.

## Where the code lives

- `core/measure/top.rs` (new): `Top` and its reader, moved out of `answer.rs`. `Top` keeps the options that hold the top. `answer.rs` gains one `use` line and loses the moved code.
- `core/measure/group.rs` (new): read the `--by` value from each numbered line's `input`, with its refusal. diff never calls it.
- `core/measure/audit.rs`: grouping by the value and the verb, and the pooled line.
- `core/measure/rows.rs` (0125's): the tie members, the calibration pairs, and the curve.
- `core/measure.rs`: `calibration` takes `(confidence, right)` pairs with a real right, and adds the shift, the floor, the widening, and `by_bin`.
- `core/measure/optimize.rs` (0125's): `crossed`, through the tuner `suggested.cut` uses.
- `core/measure/tests.rs`: any core test that names `Top` or calibration moves with the change.
- `cli/audit.rs`: `--by` parsing, `--curve`, `--pooled`, the two command-line refusals, and the three table lines.
- `cli/measure.rs`: the two pointer refusals' sentences.

No policy change. The core stays pure, and no file gains a write.

## Proof

Every test runs the built command. Each has a planted fault that turns it red. None needs a test-only export, flag, or hook: each reaches the command line.

| Test | Behavior it protects | Proof | Planted fault that turns it red | Why no existing test catches it |
| --- | --- | --- | --- | --- |
| `audit::tie_share` | decision 1 | A hand fixture of the five `choose` answers in "Curve", keyed as written there, plus the `find` tie row. The row prints `tied` 3, `tied_holding_key` 2, `tie_share` 0.75, `right` 1, `wrong` 1. The `find` row prints 1, 1, 0.5. The table prints `  ties holding the key: 2 of 3, share 0.750` | The share uses `1/(k−1)`. A second plant counts a tie missing the key | No fixture holds a tie with a key among the tied options |
| `audit::calibration_pairs_the_answer_given` | decisions 2, 3, 5 | Two hand fixtures. The yes/no fixture is the first five rows of "Calibration pairs". Its error is 0.45. `p ≥ 0.5` in place of the given answer gives 0.31, and `(p, key is yes)` gives 0.49. The `choose` fixture in "Curve" gives 0.11. The old rule, ties dropped, gives 0.1. `by_bin` lists each bin's counts, written by hand in the fixture README | Pair by `p ≥ 0.5`. A second plant counts a nonzero share as 1, which gives 0.22 on the `choose` fixture | The goldens pin today's pairing and have no answer said under a bar other than 0.5 |
| `audit::pooled` | decision 6 | Both fixtures in one run with `--pooled`. The last line holds 10 answers and error 0.17, by hand: the bins give 0, 0.25, 0.5, 0.15, and 0.8, over 10. A `tag` fixture from 0125 in the same run adds no answer. Without `--pooled` the output equals the run without it, line for line | Pool the `tag` labels. The count reads above 10. A second plant seeds the pooled bootstrap with `--seed + 1`, and the pinned interval moves | No line pools across verbs today |
| `audit::goldens_match` and `audit::tables_match_byte_for_byte` | decision 4 and every other value | The audit goldens under `golden/` and `golden/extra/`, and the table captures under `golden/table/`. Each is recaptured from the prototype at `be7cea2e` with one patch for decisions 1 to 5: the tie members, the given-answer pairs, ties at their share, the bias shift, and `by_bin`. The patch is at most 40 lines. The fixture README prints it whole with the new checksums. Golden bytes change only in `calibration` and the two tie members. Table bytes change only in the calibration line's numbers and the new ties line. 0125's walker strips `curve` and `crossed` as it strips its own members | Drop the bias shift. Every interval moves. A second plant drops the widening | This is the existing pair of tests. Their expected bytes change by the patch, and an independent script computes them |
| `audit::crossed` | decision 8 | A hand fixture of eight yes/no records keyed with parts. The fixture README tunes each part by hand, names both cuts, and sums the held counts. A second run with `--optimize f1` gives other cuts, also by hand. A two-label `tag` fixture gives one `crossed` per label row and null on the pooled row. The seeded case uses the 70 Abbey Road rows from 0125 at `--seed 0`. Its expected counts come from the patched prototype's split with the swap added | Score each part at its own cut. A second plant tunes by accuracy under `--optimize f1`. A third fills `crossed` on the `tag` pooled row | No test scores a part at a cut tuned elsewhere |
| `audit::curve` | decision 10 | The `choose` fixture and the yes/no fixture give the points in "Curve". Without `--curve` the member is null | Compare with `>`, which drops the answers at the cut. A second plant counts a tie as 0 | No curve exists today |
| `audit::by_pointer` | decision 9 | A hand fixture of six lines whose inputs carry `category`, with `decide` and `choose` in one category. Rows come out in first-seen order with the right groups, verbs, and counts. `/a~1b` reads its member | Read the pointer from the line, not from `input`. The first line is refused. A second plant skips the verb split, and the mixed category is refused as two verbs | `--by` takes no pointer today |
| `audit_refusals::each_failure_prints_one_line…` | secrecy and the four refusals | The existing sweep gains each row of "Failures" with planted secrets in the record and the pointer. Each exits 2 with its exact line and empty standard output. `--by colour` pins Clap's whole standard error from "Command line" | The pointer refusal prints the pointer. A second plant lets `--by colour` fall through as a pointer | New failure rows |
| `spec/audit.md` | the page | One block runs the tie fixture with `--table` and pins the ties line. One runs `--by /category --pooled` and pins the group names and the pooled line | The page's pinned lines | The page teaches the new options |

The hand values come from hand work, the benchmark's `stats.calibration` and `stats.coverage` run over the same pairs by hand, and the patched prototype. None comes from the code under test.

## Budgets

Nonblank lines, counted with the ratchet's rule.

- Production Rust: at most 350 new and changed together, not counting lines moved into `top.rs`. No Rust file passes 500.
- `core/measure/group.rs`: at most 60.
- Rust tests: at most 350 in `tests/audit.rs`, `tests/audit_refusals.rs`, and `tests/support/measure.rs`.
- `specification/audit.md`: at most 70 lines added.
- `spec/audit.md`: at most 20 lines added.
- Fixtures: one folder of hand fixtures, the recaptured goldens and tables, and the fixture README with the patch.
- No dependency.

The ratchet rises by the measured increase in the commit that adds the code. That commit says what grew and why. It names where the builder looked for duplication first: 0125's tuner and count in `optimize.rs` and `rows.rs`, `record_id` and `Pointer` in `core/measure.rs` and `core`, and `calibration` in `core/measure.rs`.

## Stop rules

Stop, write down what happened, and re-score before any of these:

- 0125 lands `answer.rs` at 500 lines or more;
- crossing a budget by more than a tenth;
- adding a dependency;
- changing a golden byte outside `calibration` and the two tie members;
- changing a table capture byte outside the calibration line's numbers and the ties line;
- changing the candidate grid or the split;
- changing a byte of diff's output or failure sentences;
- touching a file another in-flight ticket owns outside "Overlaps";
- `audit --curve --pooled` over `249/control.jsonl` taking more than one second in a release build.

## Scope and exclusions

Excluded: a cut of 0 or 1. A curve in the table. `crossed` for `score`. Tie members in the count objects. More than two folds. Calibration for `rank`. Any change to diff. Any live or paid call.

## Overlaps

Whichever lands second merges.

- **0125.** This ticket builds on it and opens its new files: `rows.rs`, `optimize.rs`, the audit tests, the fixtures folder, and both audit pages. 0131 does not start until 0125 lands.
- **The diff Quick Fix.** It edits `core/measure/answer.rs`, `core/measure/tests.rs`, `tests/fixtures/measure/README.md`, and `tests/fixtures/measure/small/annotate.jsonl`. This ticket edits `answer.rs`, `core/measure/tests.rs`, and the fixture README. It recaptures `golden/audit-annotate.jsonl` and `golden/table/audit-annotate.txt` from whatever `small/annotate.jsonl` holds on main.
- **0126.** It opens all of `cli`, `tests`, `specification`, and `spec`. That covers `cli/audit.rs`, `cli/measure.rs`, the audit tests, and both audit pages.
- **0124.** It opens all of `specification`, which covers `specification/audit.md`. It also edits the `Audit` variant in `cli/args/command.rs`. This ticket leaves that file alone. The new options live in `cli/audit.rs`.
- **0123.** It opens all of `cli`, which covers `cli/audit.rs` and `cli/measure.rs`. Its work sits in relate, and no line should collide.
- **0127, 0128, 0129, 0130.** No file in common apart from `sdlc/ratchet.json` and the `sdlc` folders every ticket writes.

## Complexity

Contract 2; state and timing 1; reach 1; proof 2; cost of error 2; total 8. Final level: 2. The risk is a changed calibration value that a reader compares with an older run. The Departures entry and the recaptured goldens name the change.

## Specification pages

- `specification/audit.md`: the command line, `--by`, `--curve`, and `--pooled`, the tie share, the calibration pairs, bins, and interval, the pooled line, `crossed`, the curve, the new members, the three table lines, the four failures, and a Departures entry. The entry says calibration now departs from the prototype: pairs by the answer given, ties at their share, and a bias-shifted interval. Status stays **Settled**, amended by this ticket.
- `spec/audit.md`: the two blocks in "Proof".

## Closes

This commit adds a line to each of the five issues. The line names Ian's ruling at `82336e9a` and this ticket. On landing, the lander closes the five issues, with a status line naming this ticket:

- `sdlc/issues/2026-09-25-audit-gives-no-share-to-a-tie-that-holds-the-right-answer.md`
- `sdlc/issues/2026-09-25-audit-tunes-its-cut-on-one-half-and-never-swaps.md`. The line says audit keeps the grid, and 0125's `steady` answers the spread and not the swap.
- `sdlc/issues/2026-09-25-audit-calibration-differs-from-the-benchs-ece-and-its-interval.md`
- `sdlc/issues/2026-09-25-audit-reports-coverage-at-one-rule-not-along-every-confidence.md`
- `sdlc/issues/2026-09-25-audit-cannot-group-by-a-record-field.md`

## What Ian can overturn

- **Calibration by the answer the command gave (decision 2).** Other choices: keep `(p, key is yes)` for `decide` and add an option, or pair by `p ≥ 0.5`. The benchmark adopts the chosen rule.
- **The bias-shifted interval (decision 4).** Other choices: today's plain percentile interval, or Efron's BCa interval, which needs a normal quantile function in the core.
- **Ties at their share in calibration and the curve (decision 3).** The other choice drops ties, as audit does today.
- **The pooled line behind `--pooled` (decision 6).** Other choices: print it whenever the output holds two verbs, or add a mixed-verb row. Leaving `tag` labels out of it follows 0125's pooling ban.
- **The grid over the distinct probabilities (decision 7).** The distinct set matches the benchmark's held-out column exactly. It changes the 0113 suggested cuts and writes bars such as `0.764795` into question files.
- **Two folds on the run's split (decision 8).** Other choices: more folds, or a split by a hash of the id.
- **`--by POINTER` splits by verb (decision 9).** The other choice allows a mixed-verb row, with null for every measure that needs one verb.
- **A missing field is refused (decision 9).** The other choice groups such records under a null name.
- **`--curve` is opt-in and refused with `--table` (decisions 10 and 11).**
- **Failed answers stay out of calibration (decision 13).** The benchmark pairs them at confidence 0.
- **`rank` pairs nothing (decision 14).**

## Evidence

- Starts from: The five issues filed on 2026-09-25 while a benchmark moved its grading onto audit. Ian's ruling that everything goes into 0.1, at `82336e9a`. Ticket 0125, which this builds on. Experiment 263 (`experiments/263-audit-vs-bench-methods/` in the workspace): the grid reaches the best tuning count in 61 of 70 halves and the held-out count differs in 22 of 35 sets; the given-answer and yes pairings differ up to six times; the bias shift moves the low end of an interval from 0.0154 to 0.0058; one run's 15 ties hold a share of 4.00; the benchmark counts a nonzero share as fully right in its calibration, filed there at `0104beab`. The prototype at `be7cea2e` and its goldens.
- Keeps: Every existing output member, its name, order, and type. Every golden byte outside `calibration` and the two tie members. The candidate grid, the split, the seed, `right`, `tied`, and the grid `coverage`. `--by question` and `--by verb`. Every 0125 behavior. diff's output and every diff sentence. audit sends nothing, reads no key or setting, and writes only what 0125's `--write` writes.
- Changes: The tie share on `choose` and `find` rows. Calibration pairs by the answer the command gave, ties at their share, a bias-shifted interval, and bins in the output. The pooled line under `--pooled`. `suggested.crossed`. `--by POINTER`. `--curve`. Four refusals and three table lines. `Top` moves to `top.rs`.
- Proof: The tests in "Proof", each with a planted fault that turns it red. The expected values come from hand work, the benchmark's Python run over the same pairs, and the prototype with a printed patch.
- Defers: A cut of 0 or 1 for "always yes" and "always no", which only the all-zero baseline in experiment 263 needed. A curve in the table. `crossed` for `score`. Tie members in the count objects. More than two folds. Calibration for `rank`.
