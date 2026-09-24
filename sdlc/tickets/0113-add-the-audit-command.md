---
flow: build
priority: 113
opens: crates/thinkthen/src/core crates/thinkthen/src/cli crates/thinkthen/tests specification spec sdlc/scripts sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0113: Add the audit command

Status: built, reviewed, and fixed on `ticket/0113-audit-command`, awaiting the reviewer's check of the fixes; see `sdlc/records/0113-build-audit-command.md`. Design accepted 2026-09-24 after re-review. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it. The first design review is `sdlc/records/0113-0114-design-review.md`. This page is rewritten whole after it.

## Design

`thinkthen audit RESULTS KEY` grades saved `decide` and `choose` answers against an answer key. It prints agreement with a Wilson interval, both disagreement directions, AUC, calibration, a coverage curve, and a suggested cut tuned on one part and checked on the other. It sends no request and reads no key.

Ian's ruling of 2026-09-24 (`sdlc/issues/2026-09-24-audit-and-diff-move-into-0-1.md`) moves audit into 0.1. It keeps the rest of the 2026-09-23 issue: the name, a subcommand beside `status` and `cache`, and the Beatles Bench prototype as the definition.

The definition is `scripts/tools/measure.py` in the private `botassembly/beatles-bench` repository at commit `be7cea2e4aa41097e7f629e35b62dadedeaca544`, with its `tests/test_measure.py` and `scripts/tools/README.md`. The prototype code did not change through `18c0dc8`. Where this page and the prototype disagree, the golden files decide. "Departures" lists every known difference.

The pure core holds the math. A neutral module, `cli/measure.rs`, reads inputs for audit and diff, and its failures take the command name and the input's role. `cli/audit.rs` holds the arguments and the table. Ticket 0114 builds `diff` on the same core and module and imports nothing from `cli/audit.rs`.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. The command line copies the prototype's audit grammar, `--table` included.
2. Output rows carry no `schema` member. A parsed-JSON match against the goldens forbids an extra member.
3. audit reads saved result lines only. A recording reaches audit through a replay: `thinkthen decide ... --details --replay DIR` with the key unset, piped to `audit`. A recording holds no record ids, so it cannot be graded alone.
4. `audit` routes before `Environment::read`, as `transform` does.
5. Diagnostics never echo a record, an id, a key value, or a path. They name the input's role and a one-based line number.
6. Root help lists `status`, the ten functions, `cache`, `transform`, `audit`, then generated `help`. Ticket 0114 inserts `diff` after `audit`.
7. The table keeps the prototype's words byte for byte, `unresolved` and `tied` included. Ticket 0082 bans `unresolved` from built help and teaching prose. It keeps `unresolved` as exact contract language, and these words label JSON members of the same name. The table captures therefore stay prototype-owned. If Ian prefers "not sure" in the table, the table changes, its captures become port-owned, and `spec/audit.md` changes with them.

## Open item for Ian: the prototype is private

`gh repo view botassembly/beatles-bench` reports it private. thinkthen will go public, and `cargo package` ships `crates/thinkthen/tests/`. The coordinator is asking Ian whether to make Beatles Bench public before 0113 lands. This ticket works either way.

- **While it stays private:** files that ship in the crate name no repository. The fixture README calls the source "a prototype measurement script" and gives its commit and file checksums. The fixtures themselves hold Beatles song titles, answer keys, and thinkthen's own output lines. `specification/audit.md` cites "the prototype measurement script at commit `be7cea2e`". Before thinkthen goes public, a reviewer checks every `sdlc/` line that names the repository.
- **If Ian makes it public:** three lines change. The fixture README's source line gains `botassembly/beatles-bench` and its URL. `specification/audit.md`'s citation gains the same. This paragraph closes with the date of his answer.

## Command line

```text
thinkthen audit RESULTS KEY [--by question|verb] [--threshold RULE] [--id POINTER] [--seed N] [--target A] [--table]
```

- `RESULTS` holds saved result lines, and `KEY` holds a JSONL answer key. Either may be `-` for standard input, and not both.
- `--by question` is the default. `--by verb` pools every `decide` answer into one group and every `choose` answer into another.
- `--threshold RULE` takes the grammar of `specification/threshold.md`: a cut `T` or a band `LOW:HIGH`. It rescores each answer from its saved probabilities. Without it each answer stays as printed, and the output says `"as run"`.
- `--id POINTER` is an RFC 6901 pointer into each line's `input`, default `/id`. `--id ''` takes the whole input.
- `--seed N` is an unsigned 64-bit integer, default 0. It seeds the split and the bootstrap.
- `--target A` is the agreement a `choose` suggested cut must reach, default 0.9, from 0 to 1.
- `--table` prints the results for a person instead of JSONL.

Root help row: `Grade saved decide and choose answers against an answer key.` Short and long help open with it. Long help also carries `audit sends no request and reads no key.` and `To grade a recording, replay it with --details and pass the output.` It shows the replay pipe as an example and says "not sure" for an answer inside a band.

## Inputs, read by `cli/measure.rs`

The module reads one named input (a path or `-`) as bytes, splits lines, skips blank lines while counting them, and parses each line with `core/json.rs`. That parser keeps member order and refuses duplicate names and numbers that are not finite. The module resolves `--id` with `core/pointer.rs` and hands parsed lines to the core. Its failure type carries the command name (`audit` or `diff`) and the role (`results`, `first run`, `second run`, or `key`), and renders the sentences in "Failures".

**The record id** is the value at `--id` inside `input`. A string is used as is, and an integer becomes its decimal text. Anything else is a usage error.

**One graded answer** (`core/measure/answer.rs`) comes from an entry `E`. `E` is the line itself, or one member of its `answers` object, taken in member order under its member name.

- `E` failed when it has a `failure` member, when `E.value` is an object with a `failed` member, or when it has neither `answer` nor `value`. A failed answer counts in `failed` and leaves every measure.
- The verb is `E.question.verb` when that is a nonempty string. Otherwise it is `decide` when `E.value` is a boolean, null, or absent, and `choose` otherwise. Another verb is a usage error.
- The text is `E.question.text`, or null.
- `p` is `E.answer.probability`, or null. The distribution is `E.answer.probabilities`, or null. Its top is its largest value. Its pick is the first member, in member order, equal to the top. It is tied when two or more members equal the top. A `p` or a distribution value that is not a number from 0 to 1, or an empty distribution, is a usage error.

**The key** (`core/measure/key.rs`) is JSONL. Each line is an object with `id` (string or integer), `value`, and an optional `part` of `"tune"` or `"held"`. A missing `id` or `value`, a repeated id, and any other `part` are usage errors. A `decide` value of `true` or `"yes"` is yes, and `false` or `"no"` is no. Any other `decide` value leaves the record unlabeled. A `choose` value is the right option's text, and a value that is neither text nor null is a usage error. For an `annotate` answer the key's `value` is an object, and its member under the answer name is the value. A record missing from the key, or a null value, is unlabeled. Unlabeled answers count in `unlabeled` and leave every measure.

**Duplicates** stay per command. audit refuses the same `(answer name, record id, text)` twice.

## The math

Every definition is the prototype's. The review rebuilt the statistics from this text alone and matched the goldens.

**The answer under a rule.** For `decide` as run: `true` is yes, `false` is no, and anything else is `"unresolved"`. Under a cut `T`: yes when `p >= T`, else no. Under a band: yes when `p >= HIGH`, no when `p < LOW`, else `"unresolved"`. `Threshold::judge` in `core/threshold.rs` does this, and the port reuses it. For `choose`: a tied distribution is `"tied"` under every rule. As run, a null value is `"unresolved"` and any other value is itself. Under a cut, the pick when `top >= T`, else `"unresolved"`. A band on a `choose` answer is a usage error, and so is a rule on an answer without probabilities.

**The outcome.** `"tied"` and `"unresolved"` stay themselves. Otherwise the answer is `right` when it equals the key's value and `wrong` when it does not. The port keeps these states in an enum, so an option named `tied` stays an option.

**Counts under one rule** over labeled answers: `n`, `right`, `wrong`, `unresolved`, `tied`, and `answered = right + wrong`. For `decide`, a right yes adds to `true_yes`, a right no to `true_no`, a wrong yes to `false_yes`, and a wrong no to `false_no`. `agreement = right / answered` and `coverage = answered / n`, each null on a zero denominator. `yes_recall = true_yes / (true_yes + false_no)` for `decide` when positive, else null. The count object prints `n, answered, right, wrong, unresolved, tied, true_yes, false_yes, true_no, false_no, agreement, coverage, yes_recall`. A `choose` count object carries the four directions at zero.

**Wilson at 95%.** `Z = 1.959963984540054`, `k = right`, `n = answered`, `p = k / n`:

```text
centre = (p + Z²/(2n)) / (1 + Z²/n)
half   = Z · sqrt(p(1 − p)/n + Z²/(4n²)) / (1 + Z²/n)
interval = [max(0, centre − half), min(1, centre + half)], or null when n = 0
```

**Both directions.** For `decide`, `false_yes` counts said yes where the key says no, and `false_no` the reverse. For `choose`, `disagreements` lists each `(key value, answer)` pair among wrong answers with its count, sorted by count descending, then key value, then answer, in code point order. It is null for `decide`. For `choose` the four directions and `yes_recall` are null.

**Mean and AUC** (`decide`, probabilities on every labeled answer). `mean_probability` is the mean of `p`. With `P` the `p` of answers keyed yes and `N` those keyed no, AUC is null when either is empty, else `(Σ over x in P, y in N of [x > y] + ½[x = y]) / (|P|·|N|)`. A sorted O(n log n) form is allowed if a unit test shows it equals the pairwise sum on the fixtures and on a hand case with ties.

**Calibration** (probabilities on every labeled answer). Pairs are `(p, key is yes)` for `decide`, and `(top, pick equals key)` for untied `choose` answers. No pairs gives null.

```text
bin i, i = 0..9: lo = i / 10, hi = (i + 1) / 10, each a floating-point division
x is in bin i when lo <= x < hi, or when i = 9 and x = 1.0
error = Σ over bins of |count of true in bin − Σ x in bin| / number of pairs
```

The interval is a bootstrap. Each group gets its own generator, seeded with `--seed`. It draws 1,000 resamples in order, each of `n` pairs taken in order at `pairs[index(n)]`. The sorted errors give the linear-interpolation quantiles at 0.025 and 0.975: `pos = (len − 1)·q`, `i = floor(pos)`, value `xs[i] + (pos − i)(xs[i+1] − xs[i])`, or `xs[i]` when `i + 1 >= len`. The object prints `error, interval, bins: 10, draws: 1000, seed, note`. The note is `the interval resamples the records; it does not cover rerun noise, so compare two runs of the same records`.

**SplitMix64.** Each step adds `0x9E3779B97F4A7C15` to the state. The output is `z = (z ^ (z >> 30)) · 0xBF58476D1CE4E5B9`, `z = (z ^ (z >> 27)) · 0x94D049BB133111EB`, `z ^ (z >> 31)`. All arithmetic wraps at 64 bits. Seed 0 gives `0xE220A8397B1DCDAF`, `0x6E789E6AA1B965F4`, `0x06C45D188009454F`. `index(n)` is the high 64 bits of the 128-bit product `next() · n`. The shuffle runs `i` from `len − 1` down to 1, takes `j = index(i + 1)`, and swaps `i` and `j`.

**Coverage** (probabilities on every labeled answer). For `decide`, `k` runs 50, 55, ..., 100. At 50 the rule is the cut 0.5. Otherwise it is the band `(100 − k)/100 : k/100`, written as Python writes a float, except that a whole number prints bare (`0.45:0.55`, `0:1`). For `choose`, `k` runs 5, 10, ..., 100 with the cut `k/100`. Each row is `{cut: k/100, threshold, answered, coverage, right, accuracy}`. A cut prints as a number and a band as text.

**The split.** Take the distinct ids of the group's labeled answers, sorted by code point. Rust's byte order on UTF-8 is the same. When every id has a `part`, the split is `"key"`. When some do and some do not, it is a usage error. When none does, the split is `"seeded"`: a fresh generator seeded with `--seed` shuffles the sorted ids, the first `floor(len / 2)` tune, and the rest are held out.

**The suggested cut** (probabilities on every labeled answer). Null when the tuning part is empty. Candidates are `k / 100` as a floating-point division.

- `decide`: `k` runs 1 to 99. The most right answers on the tuning part wins. A tie goes to the smallest `|k − 50|`, then the smaller `k`. The objective prints `most agreement on the tuning part`.
- `choose`: `k` runs 1 to 100. The smallest `k` whose tuning agreement reaches `--target` wins, a null agreement counting as 0. The objective prints `lowest cut with agreement >= A`. With no winner the object is `{cut: null, objective, split, seed}`.

The object prints `cut, objective, split, seed, tune, held`. `seed` is null for a key split. `tune` and `held` print `{n, at_run, at_cut}`: the counts under the command's rule and under the cut.

**One group.** The group name is the answer name, else the text, else the verb. An empty name or text falls through, as Python's `or` does, so bare lines give `"decide"`. Groups keep the order of their first answer. Two verbs among a group's unfailed answers is a usage error. The row prints `group`, `verb`, `rows` (unfailed answers), `failed`, `labeled`, `unlabeled`, `threshold`, `right`, `wrong`, `unresolved`, `tied`, `agreement`, `interval`, the four directions, `yes_recall`, `mean_probability`, `auc`, `disagreements`, `calibration`, `coverage`, `suggested`. The last five need labeled answers that all carry probabilities, and are null otherwise. A group whose answers all failed prints `verb: null`, null directions, and `disagreements: []`.

**Printing.** Every float rounds to six places. Integers print as integers. A whole float prints with `.0`. One helper, `python_float_text` in `core/measure.rs`, writes a float as Python's `str` does, for band text, the `--target` in the objective, and the table. A unit test pins `1.0`, `0.5`, `0.42`, `0.0001`, and `1e-05`. `threshold` prints `"as run"`, the cut as a number, or the band exactly as typed. `Threshold`'s own display writes `0.40:0.60` as `0.4:0.6`, so the port keeps the typed text, and a test pins `--threshold 0.40:0.60`.

## Output and failures

audit builds every row before printing. On success it prints one JSON object per group per line and exits 0. An empty RESULTS prints nothing. `--table` prints the prototype's `audit_table` text from the rounded values. Numbers print with three decimals, rounded half to even on the exact binary value. A unit test pins `0.0625` as `0.062`, `0.6875` as `0.688`, and `0.6874996` (rounded first to `0.6875`) as `0.688`.

On failure audit prints nothing on standard output and one line on standard error, pinned exactly by a test. `ROLE` is `results` or `key`.

| Case | Exit | Standard error |
| --- | ---: | --- |
| A file cannot be read | 5 | `thinkthen: audit: cannot read the ROLE file` |
| A line is not a UTF-8 JSON object | 2 | `thinkthen: audit: ROLE line N is not a JSON object` |
| No usable id | 2 | `thinkthen: audit: results line N has no string or integer id at the --id pointer` |
| `--id` is not a pointer | 2 | `thinkthen: audit: --id takes a JSON pointer such as /id or ''` |
| Another verb | 2 | `thinkthen: audit: results line N holds an answer audit cannot grade; audit grades decide and choose` |
| A bad probability or empty distribution | 2 | `thinkthen: audit: results line N holds a probability outside 0 to 1 or an empty distribution` |
| A repeated record | 2 | `thinkthen: audit: results line N repeats a record for one question` |
| A bad key line | 2 | `thinkthen: audit: key line N needs a new id, a value, and a part of tune or held when present` |
| A `choose` key value that is not text | 2 | `thinkthen: audit: key line N gives a choose value that is not text` |
| Parts on some records only | 2 | `thinkthen: audit: the key gives a part on some labeled records and not on others` |
| Two verbs in one group | 2 | `thinkthen: audit: one question holds both decide and choose answers` |
| A rule without probabilities | 2 | `thinkthen: audit: --threshold needs probabilities; rerun the question with --details` |
| A band on `choose` | 2 | `thinkthen: audit: choose takes a single cut; a band applies to decide` |
| Both inputs `-` | 2 | `thinkthen: audit: only one input may be standard input` |

A bad `--threshold`, `--seed`, or `--target` gets the existing threshold refusal or Clap's usage error, exit 2.

## Where the code lives

- `core/measure.rs` (new): SplitMix64, the shuffle, Wilson, AUC, the calibration error, the quantile, the bootstrap, rounding, and `python_float_text`. Ticket 0114 adds McNemar.
- `core/measure/answer.rs` (new): the graded answer, the answer under a rule, and the outcome.
- `core/measure/key.rs` (new): the key and the key's value for an answer.
- `core/measure/audit.rs` (new): counts, coverage, the split, the suggested cut, grouping, disagreements, and the row structs.
- `cli/measure.rs` (new): the neutral reader and its failure type.
- `cli/audit.rs` (new): Clap arguments, the table, and the call into the core.
- `crates/thinkthen/tests/support/measure.rs` (new): the golden comparison helper. Each integration test includes it with `#[path = "support/measure.rs"] mod measure_support;`.

The purity lint covers `core/measure*`.

**Policy.** Generalize `catalog_policy_failures` in `sdlc/scripts/policy.py` to take a banned-word set and allowed paths, and use it for `cli/measure.rs` and `cli/audit.rs`. The measure check refuses the catalog's banned words except `fs`, `File`, `stdin`, and `Stdin`. It also refuses the write-side names `OpenOptions`, `create`, `create_dir`, `create_dir_all`, `remove_file`, `remove_dir`, `remove_dir_all`, `rename`, `copy`, `set_permissions`, and `write` under `fs`. The check refuses those tokens and nothing more. A token check cannot prove that audit opens only the paths it was handed. The code review checks that. A second check refuses a `cli/mod.rs` where the `Audit` early return comes after `Environment::read`. The self-test plants one environment, network, clock, thread, signal, process, engine, and write-side reference, and a late early return. Each must be refused.

## Fixtures and provenance

Copy byte for byte from the prototype's `tests/fixtures/audit/` at `be7cea2e` into `crates/thinkthen/tests/fixtures/measure/`, under the same paths:

- `small/`: `decide.jsonl`, `decide-b.jsonl`, `decide-band.jsonl`, `decide-bare.jsonl`, `decide-key.jsonl`, `choose.jsonl`, `choose-b.jsonl`, `choose-key.jsonl`, `annotate.jsonl`, `annotate-key.jsonl`.
- `249/`: `control.jsonl`, `soft.jsonl`, `key.jsonl`, `key-noparts.jsonl`. These are thinkthen `decide --details` lines replayed from experiment 249's recordings with every key unset: 272 yes/no questions about Beatles songs.
- `golden/`: the fourteen outputs. This ticket turns on the eight audit files, and 0114 the six diff files. Ian's count of fifteen adds `golden/make.py`, which stays upstream.

Add three port-owned inputs, each derived from `small/`: `decide-reversed.jsonl` (the lines of `decide.jsonl` in reverse), `decide-key-noparts.jsonl` (the key without `part`), and `decide-key-odd.jsonl` (the key without `part` and without `r6`).

Capture these outputs from the prototype at `be7cea2e`, read with `git show be7cea2:scripts/tools/measure.py` into a scratch file, under Python 3.12.3. Capturing reads that repository and writes nothing to it.

- `golden/extra/audit-decide-reversed.jsonl`: `audit small/decide-reversed.jsonl small/decide-key-noparts.jsonl`. Its seeded cut is 0.71.
- `golden/extra/audit-decide-odd.jsonl`: `audit small/decide.jsonl small/decide-key-odd.jsonl`. Its seeded split tunes on 2 and holds out 3, with cut 0.45.
- `golden/table/audit-decide.txt`, `audit-choose.txt`, and `audit-annotate.txt`: the three `small/` audit lines of the table below with `--table`.

`tests/fixtures/measure/README.md` records the source as the private prototype measurement script, its commit, the capture method and Python version, and every file's SHA-256. It also carries this table:

| Output | Command line (from `tests/fixtures/measure/`) | Test |
| --- | --- | --- |
| `golden/audit-decide.jsonl` | `audit small/decide.jsonl small/decide-key.jsonl` | `audit_goldens::decide` |
| `golden/audit-decide-0.4.jsonl` | the same with `--threshold 0.4` | `audit_goldens::decide_rescored` |
| `golden/audit-decide-band.jsonl` | `audit small/decide-band.jsonl small/decide-key.jsonl` | `audit_goldens::band` |
| `golden/audit-decide-bare.jsonl` | `audit small/decide-bare.jsonl small/decide-key.jsonl` | `audit_goldens::bare` |
| `golden/audit-choose.jsonl` | `audit small/choose.jsonl small/choose-key.jsonl` | `audit_goldens::choose` |
| `golden/audit-annotate.jsonl` | `audit small/annotate.jsonl small/annotate-key.jsonl` | `audit_goldens::annotate` |
| `golden/audit-249.jsonl` | `audit 249/control.jsonl 249/key.jsonl --by verb` | `audit_goldens::beatles_key_split` |
| `golden/audit-249-seed.jsonl` | `audit 249/control.jsonl 249/key-noparts.jsonl --by verb --seed 249` | `audit_goldens::beatles_seeded` |
| `golden/extra/audit-decide-reversed.jsonl` | as above | `audit_goldens::reversed_seeded` |
| `golden/extra/audit-decide-odd.jsonl` | as above | `audit_goldens::odd_seeded` |

A test recomputes every SHA-256 with `sha2` and fails on a missing, extra, or changed file.

## Acceptance

**Goldens.** Run each table line with the built binary from `tests/fixtures/measure/`. The support helper parses output and golden as lists of JSON values. They match when the lists agree in length and order; objects agree in member names and order; strings, booleans, nulls, and integers are equal; and each golden float is a float in the output within `1e-6 + 1e-12`. The tolerance allows one last-place difference after rounding. Each run exits 0 with empty standard error. The table captures match byte for byte.

**Hand-checked values.** Port `Prng`, `Pointer`, `AuditDecide`, `AuditChoose`, `AuditAnnotate`, and the audit half of `LeaningNo` from `tests/test_measure.py` as core tests. They include the three SplitMix64 outputs; Wilson `[0.3000, 0.9032]` for 4 of 6; AUC `7/9`; calibration errors `0.375` and `0.45`; the band coverage rows; the key-part cut `0.45`; the `choose` target cut `0.71`; and the held-out Beatles numbers: accuracy 0.630, yes recall 0.311, mean p(yes) 0.397, AUC 0.725, calibration error 0.092, cut 0.42 on 134 tuning and 138 held out, then 0.645 and 0.607 at the cut.

**Planted-bug tests.** The review found these values with the prototype, and each turns red under its bug:

- The calibration error over `[(1.0, no), (0.0, no)]` is 0.5.
- Two tuning answers, p 0.45 keyed yes and p 0.54 keyed no: cuts 0.45 and 0.55 tie, and the cut is 0.45.
- `249/control.jsonl` with alternate lines rewritten in memory to two question texts, audited against `249/key.jsonl`: the second group's calibration interval is `[0.049765, 0.164636]`, the same as that group audited alone.
- A seeded split over five ids tunes on 2.

**Replay pipeline.** Replay a committed recording fixture through `thinkthen decide --details --replay DIR` with the key unset, pipe it to `audit` with a small committed key, and pin the output.

**No request, no key.** Name a counting loopback listener in `THINKTHEN_BASE_URL`, set `THINKTHEN_API_KEY` to a canary, and make the configuration and cache locations unreadable. Run every golden line, table line, and failure case. The listener accepts zero connections. The canary appears in no output, Debug line, or temporary file. No cache, counter, lock, or folder is created or changed.

**Failures.** Each failure row has a test for its exit code, exact line, and empty standard output. A secrecy test plants a distinctive id, record text, key value, and path and finds none of them in standard error.

**Help and vocabulary.** Edit the one root inventory assertion of 0082 and 0083 to insert `audit` after `transform`. Pin the root row, both introductions, and the two long-help sentences. Add `audit` to the `scan_help` loop in `sdlc/scripts/demos` for `-h` and `--help`. The check finds zero unsanctioned hits.

**Red-green.** Write each test first and watch it fail for its stated reason. Then plant each bug below, one at a time, and record which named test turns red. A bug no test catches gets a new test before landing.

1. The Wald interval in place of Wilson.
2. The `Z²/(4n²)` term dropped.
3. `false_yes` and `false_no` swapped.
4. AUC ties counted as 1.
5. The last calibration bin left open at 1.0.
6. The nearest-rank quantile.
7. `index(n)` as `next() mod n`.
8. The seeded split tuning on `ceil(len / 2)`. `audit-annotate`, `extra/audit-decide-odd`, and the five-id test catch it.
9. The ids shuffled in file order without the sort. `extra/audit-decide-reversed` catches it.
10. The `decide` tie going to the larger cut.
11. The band's low edge counted as no.
12. A tied `choose` answer counted as wrong.
13. One bootstrap generator shared across groups.

**Gates.** Focused tests, then `sdlc/scripts/install`, `lint`, `test`, and `spec` in sequence with the key and base address unset, and `git diff --check`. No live or paid call.

## Specification pages

Add `specification/audit.md`, Status **Settled** on landing. It states the command line, the inputs, the math, the output members, the failures, the table words, and the replay pipeline, and cites the prototype as the open item allows. Add its index row. Add `spec/audit.md`. It pins the first two lines of `audit --table` on `small/decide.jsonl` and one refusal with its exit code.

## Departures

No golden reaches these. Each is the agent's decision, and Ian can overturn it.

- **Failures.** The prototype exits 2 through `argparse`, exits 1 with a traceback on a missing file or bad JSON, and echoes ids and record text. The port uses the failure table and echoes nothing.
- **Bad numbers.** A probability outside 0 to 1, a value that is not a number, or an empty distribution crashes the prototype. The port refuses it.
- **JSON.** The port refuses duplicate member names and `NaN`. Python accepts both.
- **Empty input.** The prototype prints one blank line. The port prints nothing.
- **Pointer.** The prototype accepts `-1` and leading zeros as array indexes. The port's RFC 6901 pointer refuses both.
- **Rule text.** The prototype parses with `float()`. The port uses the settled grammar.
- **Seed and target.** The port refuses a negative seed and a target outside 0 to 1.
- **Key values.** Python reads a `decide` key of `1` or `0` as yes or no. The port leaves it unlabeled. The port refuses a `choose` key value that is not text.
- **Choose values.** A saved `choose` value that is neither text nor null, `true` and `false` included, is refused as an answer audit cannot grade. The prototype grades it: as run it compares the value with the key and counts it wrong, and under `--threshold` it grades the pick. Added 2026-09-24 by the builder, and Ian can overturn it.
- **State names.** The prototype reads an option named `tied` or `unresolved` as that state. The port keeps it an option.
- **Parts.** The README says every key line has a part or none does. The code checks each group's labeled records. The port follows the code.

## Builder note: keep the formats open

`sdlc/issues/2026-09-24-audit-and-diff-needs-for-graded-agent-runs.md` lists four needs for grading agent runs after 0.1. None enters 0.1, and nothing here changes a golden or an accepted input. Keep three things open.

- The key reader ignores members it does not use, beside `id`, `value`, and `part`. The result reader already ignores `meta` and other unused members. A test pins one extra member in a key line and one in a result line.
- The duplicate identity (answer name, record id, text) lives in one core function, so a sample key can join it later.
- `specification/audit.md` says that readers of audit output should ignore members they do not know, because later versions may add them.

## Budgets and the ratchet

- Production Rust: six new files and at most four existing files touched (`core/mod.rs`, `cli/mod.rs`, `cli/args/command.rs`, one failure-mapping file). At most 1,000 nonblank lines.
- Rust tests: at most 850 nonblank lines in at most three test files plus `tests/support/measure.rs`, and the edited inventory assertion.
- Scripts: `policy.py`, its self-test, and the `demos` loop, at most 70 nonblank lines.
- Fixtures: 28 copies, three derived inputs, five captures, and the README.
- Every Rust file stays under 500 nonblank lines.
- Dependencies: none. A new one stops the build for a second review.

`sdlc/ratchet.json` rises by the measured Rust increase in the commit that needs it. That message says what grew, why it earns its lines, and where the builder looked for duplication: `core/pointer.rs`, `core/threshold.rs`, `core/json.rs`, `cli/table.rs`, and `cli/transform.rs`. The ticket raises the ceiling and widens the public surface, so a second agent reviews it and names what it checked. Stop and re-score past any budget, on any dependency, or on any golden change.

## Scope and exclusions

Excluded: `diff`, other verbs, reading a recording folder directly, library or database surfaces, a `schema` member, changing a golden, editing Beatles Bench, live calls, and release artifacts.

## Dependencies and order

Depends on the ten functions done: 0086 landed, after 0082 and 0083. The queue in `sdlc/planning/one-line-plan-2026-09-24.md` places audit after the ten functions and before the release build. Ticket 0114 follows and reuses the core, `cli/measure.rs`, the fixtures, and the helper.

## Complexity

Contract 2; State/timing 0; Reach 1; Proof 2; Cost of error 1; Total 6. Final level: 2.

## Evidence

Builder note, 2026-09-24. Workspace decision `2026-09-24-experiments-reduce-risk.md` asks every product ticket to name these five parts. This note changes no design.

- Starts from: The prototype `scripts/tools/measure.py`, its `scripts/tools/README.md`, `tests/test_measure.py`, and `tests/fixtures/audit/` in the private Beatles Bench repository at `be7cea2e`. A diff of those files from `be7cea2e` to the 2026-09-24 head `b0bf4ae` is empty. `experiments/249-jev-answer-audit/README.md` in the workspace reports the held-out numbers that the `audit-249` goldens reproduce. `experiments/212-thinkthen-repeat/RESULTS.md` shows answers flipping at a 0.5 cut between identical runs, which a graded audit must expose. `experiments/206-thinkthen-accuracy/RESULTS.md` uses the same ten calibration bins. `repos/jev-experiments/experiments/029-scale-closed-lists/README.md` tunes a cut on one slice and checks it on another, as the suggested cut does. The other experiment folders checked (145, 219, 224, 228, 238, 240, 245, 246) held nothing relevant.
- Keeps: The port keeps the prototype's command grammar, math, output members, table words, and seeded split. The eight audit goldens and the three table captures decide any disagreement.
- Changes: The port refuses bad numbers, duplicate JSON members, non-finite values, bad pointers, and out-of-range options. Its failures name only the input's role and a line number. "Departures" lists every change.
- Proof: The eight goldens, the extra captures, and the three table captures match. Hand-checked core tests come from `test_measure.py`. Each planted bug in "Acceptance" turns a test red. A loopback listener counts zero requests, and a canary key never appears in output.
- Defers: The agent-run grading needs in `sdlc/issues/2026-09-24-audit-and-diff-needs-for-graded-agent-runs.md`, reading a recording folder directly, a `schema` member, and other verbs wait until after 0.1. Naming the private repository waits for Ian's answer on the open item. Two known limits stay as the prototype has them. Binned calibration error is biased upward under resampling, so its bootstrap interval can exclude its own point value (`experiments/249-jev-answer-audit/README.md`, step 3). A tie at the top counts as `tied` apart, where experiment 249 scored it wrong.

Amended 2026-09-24: the ADR 0017 amendment of that date on main renames the width setting to the throttle. This ticket names no width. Any public name or text it writes uses the throttle.
