# Audit

Status: **Settled** by ticket 0113.

`thinkthen audit RESULTS KEY` grades saved `decide` and `choose` answers against an answer key. It prints agreement with a Wilson interval, both disagreement directions, AUC, calibration, a coverage curve, and a suggested cut tuned on one part and checked on the other. It sends no request and reads no API key.

The definition is the prototype measurement script at commit `be7cea2e`, with its tests and README. `crates/thinkthen/tests/fixtures/measure/README.md` gives the file checksums. Where this page and the prototype disagree, the golden files in that folder decide. "Departures" lists every known difference.

```sh
thinkthen decide 'Is it red?' --jsonl --details --replay runs/red < records.jsonl | thinkthen audit - key.jsonl
thinkthen audit results.jsonl key.jsonl --threshold 0.4 --table
```

## Command line

```text
thinkthen audit RESULTS KEY [--by question|verb] [--threshold RULE] [--id POINTER] [--seed N] [--target A] [--table]
```

- `RESULTS` holds saved result lines, and `KEY` holds a JSONL answer key. Either may be `-` for standard input, and not both.
- `--by question` is the default. `--by verb` pools every `decide` answer into one group and every `choose` answer into another.
- `--threshold RULE` takes the grammar of [threshold.md](threshold.md): a cut `T` or a band `LOW:HIGH`. It rescores each answer from its saved probabilities. Without it each answer stays as printed, and the output says `"as run"`.
- `--id POINTER` is an RFC 6901 pointer into each line's `input`, default `/id`. `--id ''` takes the whole input.
- `--seed N` is an unsigned 64-bit integer, default 0. It seeds the split and the bootstrap.
- `--target A` is the agreement a `choose` suggested cut must reach, default 0.9, from 0 to 1.
- `--table` prints the results for a person instead of JSONL.

audit reads saved result lines only. A recording reaches audit through a replay: `decide ... --details --replay DIR` with the key unset, piped to `audit`. A recording holds no record ids, so it cannot be graded alone.

## Inputs

Each input is read whole. Lines split on newline. A blank line is skipped and still counted. Each other line is one JSON object, with no repeated member name and no number that is not finite.

**The record id** is the value at `--id` inside `input`. A string is used as is, and an integer becomes its decimal text.

**One graded answer** comes from an entry `E`. `E` is the line itself, or one member of its `answers` object, taken in member order under its member name.

- `E` failed when it has a `failure` member, when `E.value` is an object with a `failed` member, or when it has neither `answer` nor `value`. A failed answer counts in `failed` and leaves every measure.
- The verb is `E.question.verb` when that is a nonempty string. Otherwise it is `decide` when `E.value` is a boolean, null, or absent, and `choose` otherwise. Another verb is refused.
- The text is `E.question.text`, or null.
- `p` is `E.answer.probability`, or null. The distribution is `E.answer.probabilities`, or null. Its top is its largest value. Its pick is the first member, in member order, equal to the top. It is tied when two or more members equal the top.

**The key** is JSONL. Each line is an object with `id` (string or integer), `value`, and an optional `part` of `"tune"` or `"held"`. A `decide` value of `true` or `"yes"` is yes, and `false` or `"no"` is no. Any other `decide` value leaves the record unlabeled. A `choose` value is the right option's text. For an `annotate` answer the key's `value` is an object, and its member under the answer name is the value. A record missing from the key, or a null value, is unlabeled. Unlabeled answers count in `unlabeled` and leave every measure.

**Duplicates.** audit refuses the same answer name, record id, and question text twice.

Both readers ignore members they do not use. A later version may read more of them.

## The math

**The answer under a rule.** For `decide` as run: `true` is yes, `false` is no, and anything else is `"unresolved"`. Under a cut `T`: yes when `p >= T`, else no. Under a band: yes when `p >= HIGH`, no when `p < LOW`, else `"unresolved"`. For `choose`: a tied distribution is `"tied"` under every rule. As run, a null value is `"unresolved"` and any other value is itself. Under a cut, the pick when `top >= T`, else `"unresolved"`.

**The outcome.** `"tied"` and `"unresolved"` stay themselves. Otherwise the answer is `right` when it equals the key's value and `wrong` when it does not. An option named `tied` stays an option.

**Counts under one rule** over labeled answers: `n`, `right`, `wrong`, `unresolved`, `tied`, and `answered = right + wrong`. For `decide`, a right yes adds to `true_yes`, a right no to `true_no`, a wrong yes to `false_yes`, and a wrong no to `false_no`. `agreement = right / answered` and `coverage = answered / n`, each null on a zero denominator. `yes_recall = true_yes / (true_yes + false_no)` for `decide` when positive, else null. A count object prints `n, answered, right, wrong, unresolved, tied, true_yes, false_yes, true_no, false_no, agreement, coverage, yes_recall`.

**Wilson at 95%.** `Z = 1.959963984540054`, `k = right`, `n = answered`, `p = k / n`:

```text
centre = (p + Z²/(2n)) / (1 + Z²/n)
half   = Z · sqrt(p(1 − p)/n + Z²/(4n²)) / (1 + Z²/n)
interval = [max(0, centre − half), min(1, centre + half)], or null when n = 0
```

**Disagreements.** For `choose`, `disagreements` lists each `(key, said)` pair among wrong answers with its count, sorted by count descending, then key, then answer, in code point order. It is null for `decide`, whose directions are `false_yes` and `false_no`.

**Mean and AUC** (`decide`, probabilities on every labeled answer). `mean_probability` is the mean of `p`. With `P` the `p` of answers keyed yes and `N` those keyed no, AUC is null when either is empty, else `(Σ over x in P, y in N of [x > y] + ½[x = y]) / (|P|·|N|)`.

**Calibration** (probabilities on every labeled answer). Pairs are `(p, key is yes)` for `decide`, and `(top, pick equals key)` for untied `choose` answers. Ten equal bins cover zero to one, and the last is closed at one. The error is the sum over bins of `|trues − Σ p|`, divided by the number of pairs. Sums of floats compensate each step as Python's `sum` does. The interval draws 1,000 bootstrap resamples with a SplitMix64 generator seeded with `--seed`, one generator per group, and takes the linear-interpolation quantiles at 0.025 and 0.975. The note says `the interval resamples the records; it does not cover rerun noise, so compare two runs of the same records`.

**SplitMix64.** Each step adds `0x9E3779B97F4A7C15` to the state and mixes it with the published constants. `index(n)` is the high 64 bits of the 128-bit product `next() · n`. The shuffle runs `i` from `len − 1` down to 1 and swaps `i` with `index(i + 1)`.

**Coverage** (probabilities on every labeled answer). For `decide`, `k` runs 50, 55, ..., 100. At 50 the rule is the cut 0.5, and otherwise the band `(100 − k)/100 : k/100`, written as `0.45:0.55` and `0:1`. For `choose`, `k` runs 5, 10, ..., 100 with the cut `k/100`. Each point is `{cut, threshold, answered, coverage, right, accuracy}`.

**The split.** Take the distinct ids of the group's labeled answers in code point order. When every id has a `part`, the split is `"key"`. When some do and some do not, audit refuses the key. When none does, the split is `"seeded"`: a generator seeded with `--seed` shuffles the ids, the first `floor(len / 2)` tune, and the rest are held out.

**The suggested cut** (probabilities on every labeled answer) is null when the tuning part is empty. For `decide`, `k` runs 1 to 99, the most right answers on the tuning part wins, and a tie goes to the smallest `|k − 50|`, then the smaller `k`. For `choose`, the smallest `k` from 1 to 100 whose tuning agreement reaches `--target` wins, a null agreement counting as 0. With no winner the object is `{cut: null, objective, split, seed}`. Otherwise it prints `cut, objective, split, seed, tune, held`, where `seed` is null for a key split and `tune` and `held` print `{n, at_run, at_cut}`.

## Output

One JSON object per group per line, in the order of each group's first answer. The group name is the answer name, else the question text, else the verb, and an empty name falls through. Each row prints `group`, `verb`, `rows` (unfailed answers), `failed`, `labeled`, `unlabeled`, `threshold`, `right`, `wrong`, `unresolved`, `tied`, `agreement`, `interval`, `true_yes`, `false_yes`, `true_no`, `false_no`, `yes_recall`, `mean_probability`, `auc`, `disagreements`, `calibration`, `coverage`, and `suggested`. The four directions and `yes_recall` are null for `choose`. The last five need labeled answers that all carry probabilities, and are null otherwise. A group whose answers all failed prints `verb: null`.

Every float rounds to six places. `threshold` prints `"as run"`, a cut as a number, or a band exactly as typed. An empty RESULTS prints nothing. A reader of this output ignores members it does not know, because a later version may add them.

`--table` prints the prototype's table from the rounded values. Numbers print with three decimals, rounded half to even on the exact binary value, and `-` stands for null. The table keeps the prototype's words, `unresolved` and `tied` included, because they label the JSON members of the same names.

## Failures

A failure prints nothing on standard output and one line on standard error. The line never echoes a record, an id, a key value, or a path. `ROLE` is `results` or `key`, and `N` is a one-based line number.

| Case | Exit | Standard error |
| --- | ---: | --- |
| A file cannot be read | 5 | `thinkthen: audit: cannot read the ROLE file` |
| A line is not a UTF-8 JSON object | 2 | `thinkthen: audit: ROLE line N is not a JSON object` |
| No usable id | 2 | `thinkthen: audit: results line N has no string or integer id at the --id pointer` |
| `--id` is not a pointer | 2 | `thinkthen: audit: --id takes a JSON pointer such as /id or ''` |
| Another verb, or a `choose` value that is not text | 2 | `thinkthen: audit: results line N holds an answer audit cannot grade; audit grades decide and choose` |
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

A bad `--seed`, `--target`, or `--by` gets the ordinary command-line usage error and exits 2.

## What audit never does

audit routes before any setup. It reads the two named inputs and nothing else: no API key, environment variable, configuration file, cache, or usage counter. It opens no socket, starts no process, and writes only standard output and standard error.

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
- **Choose values.** The prototype compares a saved `choose` value that is neither text nor null with the key and counts it wrong. The port refuses it as an answer it cannot grade.
- **State names.** The prototype reads an option named `tied` or `unresolved` as that state. The port keeps it an option.
- **Parts.** The prototype README says every key line has a part or none does. The code checks each group's labeled records. The port follows the code.
