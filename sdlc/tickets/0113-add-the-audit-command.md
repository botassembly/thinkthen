---
flow: build
priority: 113
opens: crates/thinkthen/src/core crates/thinkthen/src/cli crates/thinkthen/tests specification spec sdlc/scripts sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0113: Add the audit command

Status: design draft; review pending. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Build `thinkthen audit`. It grades saved `decide` and `choose` answers against an answer key and prints agreement, both disagreement directions, AUC, calibration, a coverage curve, and a suggested cut. It sends no request and reads no key.

Ian's ruling of 2026-09-24 (`sdlc/issues/2026-09-24-audit-and-diff-move-into-0-1.md`) moves audit into 0.1. It amends the timing in `sdlc/issues/2026-09-23-a-measurement-tier-for-thinkthen-audit-and-diff.md` and keeps the rest: the name, a subcommand beside `status` and `cache`, and the Beatles Bench prototype as the definition.

The definition is `scripts/tools/measure.py` in the public `botassembly/beatles-bench` repository at commit `be7cea2e4aa41097e7f629e35b62dadedeaca544`, with `tests/test_measure.py` and `scripts/tools/README.md` at the same commit. Where this ticket and the prototype disagree, the golden files decide. The known differences are listed under "Where this port departs from the prototype". Ticket 0114 builds `thinkthen diff` on the reader and the statistics this ticket lands.

## Decisions

Each of these is the agent's decision, and Ian can overturn any of them.

1. The command line copies the prototype's audit grammar, including `--table`. The issue asks for data for pipelines and a table for a person, and the prototype prints both.
2. Output rows carry no `schema` member. A parsed-JSON match against the goldens forbids an extra member. A later schema line needs a prototype change and new goldens first.
3. audit reads saved result lines only. A recording or cache folder reaches audit through a replay, as the prototype's README says: `thinkthen decide ... --details --replay DIR` with the key unset, piped to `audit`. A recording holds requests and replies and no record ids, so it cannot be graded without the records. An acceptance test runs that pipeline.
4. `audit` routes before `Environment::read`, as `transform` does. It reads only the paths named on its command line and standard input for `-`.
5. Diagnostics never echo a record, an id, a key value, or a path. They name the input by its role (`results` or `key`) and a one-based line number.
6. Root help lists `status`, the ten functions, `cache`, `transform`, `audit`, then generated `help`. Ticket 0114 inserts `diff` after `audit`.

## Command line

```text
thinkthen audit RESULTS KEY [--by question|verb] [--threshold RULE] [--id POINTER] [--seed N] [--target A] [--table]
```

- `RESULTS` is a file of saved result lines. `-` reads standard input.
- `KEY` is a JSONL answer key. `-` reads standard input. RESULTS and KEY cannot both be `-`.
- `--by question` is the default. It groups answers by question text, and an `annotate` answer by its answer name. `--by verb` pools every `decide` answer into one group and every `choose` answer into another.
- `--threshold RULE` takes the settled threshold grammar of `specification/threshold.md`: a cut `T` or a band `LOW:HIGH`. It rescores every answer from its saved probabilities. Without it each answer stays as it was printed, and the output says `"as run"`.
- `--id POINTER` is an RFC 6901 pointer into each line's `input`. The default is `/id`. `--id ''` takes the whole input.
- `--seed N` is an unsigned 64-bit integer, default 0. It seeds the split and the bootstrap.
- `--target A` is the agreement a `choose` suggested cut must reach, default 0.9. It is a finite number from 0 to 1.
- `--table` prints the same results for a person instead of JSONL.

The golden command lines, run from the fixture folder, are exactly:

```text
thinkthen audit small/decide.jsonl small/decide-key.jsonl
thinkthen audit small/decide.jsonl small/decide-key.jsonl --threshold 0.4
thinkthen audit small/decide-band.jsonl small/decide-key.jsonl
thinkthen audit small/decide-bare.jsonl small/decide-key.jsonl
thinkthen audit small/choose.jsonl small/choose-key.jsonl
thinkthen audit small/annotate.jsonl small/annotate-key.jsonl
thinkthen audit 249/control.jsonl 249/key.jsonl --by verb
thinkthen audit 249/control.jsonl 249/key-noparts.jsonl --by verb --seed 249
```

Root help row: `Grade saved decide and choose answers against an answer key.` Short and long help open with the same sentence. Long help also carries these sentences: `audit sends no request and reads no key.` and `To grade a recording, replay it with --details and pass the output.` It shows the replay pipe as an example. Help says "not sure" for an answer inside a band. It never prints `unresolved`, and it passes the vocabulary check of ticket 0082.

## Inputs

**Result lines.** Each nonblank line of RESULTS is one JSON object in the shape `specification/result.md` fixes. Three shapes are graded. `decide --details` and `choose --details` lines carry probabilities. Default lines `{"input":RECORD,"value":ANSWER}` carry none. In `annotate --details` lines, each member of `answers` is its own answer under its member name. Blank lines are skipped and still counted for line numbers. The crate's own tree in `core/json.rs` reads the JSON. It keeps member order and refuses duplicate names.

**The record id** is the value at `--id` inside the line's `input`. A string is used as is. An integer becomes its decimal text. Any other value, or a missing one, is a usage error.

**One graded answer** is built from an entry `E`: the line itself, or one member of `answers`.

- `E` failed when it has a `failure` member, when `E.value` is an object with a `failed` member, or when it has neither `answer` nor `value`. A failed answer is counted in `failed` and left out of every measure.
- The verb is `E.question.verb` when it is a nonempty string. Otherwise it is `decide` when `E.value` is a boolean, null, or absent, and `choose` when it is anything else. Any verb other than `decide` or `choose` is a usage error.
- The question text is `E.question.text`, or null.
- The yes probability `p` is `E.answer.probability`, or null.
- The option distribution is `E.answer.probabilities` when it is a nonempty object. Its top is the largest value. Its pick is the first member, in member order, whose value equals the top. It is tied when two or more members equal the top.
- The same `(answer name, record id, question text)` twice is a usage error.

**The answer key** is JSONL. Each line is an object with `id` (string or integer), `value`, and an optional `part` of `"tune"` or `"held"`. A repeated id and any other `part` are usage errors. A `decide` value of `true` or `"yes"` means yes, and `false` or `"no"` means no. Any other `decide` value leaves the record unlabeled. A `choose` value is the right option's text. A `choose` value that is neither a string nor null is a usage error. For an `annotate` answer, `value` is an object and the member under the answer name is the value. A record missing from the key, or a null value, is unlabeled. An unlabeled answer is counted in `unlabeled` and left out of every measure.

## The math

Every definition below is the prototype's. The source line in `measure.py` is named where it matters.

**The answer under a rule** (`said`). For `decide` as run: `true` is yes, `false` is no, and anything else is not sure (`"unresolved"`). Under a cut `T`: yes when `p >= T`, else no. Under a band: yes when `p >= HIGH`, no when `p < LOW`, else `"unresolved"`. This is `Threshold::judge` in `core/threshold.rs`, and the port reuses it. For `choose`: a tied distribution is `"tied"` under every rule. As run, a null value is `"unresolved"` and any other value is itself. Under a cut: the pick when `top >= T`, else `"unresolved"`. A band on a `choose` answer is a usage error. A rule on an answer with no probabilities is a usage error.

**The outcome.** `"tied"` and `"unresolved"` stay themselves. Otherwise the answer is `right` when it equals the key's value and `wrong` when it does not.

**Counts under one rule** over a list of labeled answers: `n`; `right`, `wrong`, `unresolved`, `tied`; `answered = right + wrong`. For `decide` only: a right yes adds to `true_yes`, a right no to `true_no`, a wrong yes to `false_yes`, and a wrong no to `false_no`. `agreement = right / answered` and `coverage = answered / n`, each null when its denominator is zero. `yes_recall = true_yes / (true_yes + false_no)` for `decide` when the denominator is positive, else null. The count object prints `n, answered, right, wrong, unresolved, tied, true_yes, false_yes, true_no, false_no, agreement, coverage, yes_recall` in that order. A `choose` count object still carries the four direction members at zero.

**The Wilson interval** at 95%. `Z = 1.959963984540054`, `k = right`, `n = answered`, `p = k / n`:

```text
centre = (p + Z²/(2n)) / (1 + Z²/n)
half   = Z · sqrt(p(1 − p)/n + Z²/(4n²)) / (1 + Z²/n)
interval = [max(0, centre − half), min(1, centre + half)], or null when n = 0
```

**Both disagreement directions.** For `decide`, `false_yes` counts said yes where the key says no, and `false_no` counts said no where the key says yes. For `choose`, `disagreements` lists each `(key value, answer)` pair among wrong answers with its count. It sorts by count descending, then key value, then answer, both in code point order. For `decide`, `disagreements` is null. For `choose`, the four direction members and `yes_recall` are null.

**Mean probability and AUC** (`decide` with probabilities on every labeled answer). `mean_probability` is the mean of `p`. AUC takes `P`, the `p` of answers the key calls yes, and `N`, those it calls no. It is null when either list is empty. Otherwise it is `(Σ over x in P, y in N of [x > y] + ½[x = y]) / (|P|·|N|)`. The port may compute this by sorting, in O(n log n), provided a unit test shows it equals the pairwise sum on the fixtures and on a hand case with ties.

**Calibration** (probabilities on every labeled answer). The pairs are `(p, key is yes)` for `decide`, and `(top, pick equals key)` for `choose` with tied answers left out. Null when there are no pairs. The error is:

```text
bin i, for i = 0..9: lo = i / 10, hi = (i + 1) / 10, each a floating-point division
an answer is in bin i when lo <= x < hi, or when i = 9 and x = 1.0
error = Σ over bins of |count of true in bin − Σ x in bin| / number of pairs
```

The interval is a bootstrap. One generator per group is seeded with `--seed`. It draws 1,000 resamples in order. Each resample takes `n` pairs in order, each at `pairs[index(n)]`. The 1,000 errors are sorted ascending. The interval is the linear-interpolation quantile at 0.025 and 0.975: `pos = (len − 1)·q`, `i = floor(pos)`, and the value is `xs[i] + (pos − i)(xs[i+1] − xs[i])`, or `xs[i]` when `i + 1 >= len`. The object prints `error, interval, bins: 10, draws: 1000, seed, note`. The note is exactly `the interval resamples the records; it does not cover rerun noise, so compare two runs of the same records`.

**The generator** is SplitMix64. Each step adds `0x9E3779B97F4A7C15` to the state, wrapping at 64 bits. The output mixes it: `z = (z ^ (z >> 30)) · 0xBF58476D1CE4E5B9`, `z = (z ^ (z >> 27)) · 0x94D049BB133111EB`, `z ^ (z >> 31)`, all wrapping. Seed 0 gives `0xE220A8397B1DCDAF`, `0x6E789E6AA1B965F4`, `0x06C45D188009454F` first. `index(n)` is the high 64 bits of the 128-bit product `next() · n`. The shuffle is Fisher-Yates from the last position down: for `i` from `len − 1` to 1, `j = index(i + 1)`, and positions `i` and `j` swap.

**The coverage curve** (probabilities on every labeled answer). For `decide`, `k` runs 50, 55, ..., 100. At `k = 50` the rule is the cut 0.5. Otherwise it is the band `(100 − k)/100 : k/100`, written as text the way thinkthen reads it: a whole number prints bare (`0`, `1`), and any other value prints its shortest decimal (`0.45`, `0.3`, `0.05`). For `choose`, `k` runs 5, 10, ..., 100, and the rule is the cut `k/100`. Each row is `{cut: k/100, threshold: rule, answered, coverage, right, accuracy}` from the counts under that rule. A cut prints as a number, and a band as its text.

**The split** for the suggested cut. Take the distinct ids of the group's labeled answers and sort them by code point. Rust's byte order on UTF-8 strings is the same order. When every one of those ids has a `part` in the key, the split is `"key"`: `tune` ids tune, and `held` ids check. When some have one and some do not, it is a usage error. When none has one, the split is `"seeded"`. A fresh generator seeded with `--seed` shuffles the sorted ids. The first `floor(len / 2)` ids tune, and the rest are held out. The seeded split is repeatable for one seed.

**The suggested cut** (probabilities on every labeled answer). It is null when the tuning part is empty. Candidate cuts are `k / 100` as a floating-point division.

- `decide`: `k` runs from 1 to 99. The winner has the most right answers on the tuning part. A tie goes to the smallest `|k − 50|`, then to the smaller `k`. The objective prints `most agreement on the tuning part`.
- `choose`: `k` runs from 1 to 100. The winner is the smallest `k` whose tuning agreement reaches `--target`, where a null agreement counts as 0. The objective prints `lowest cut with agreement >= A`, with `A` printed as Python prints a float (`0.9`, `1.0`). When no `k` qualifies, the object is `{cut: null, objective, split, seed}`.

The object prints `cut, objective, split, seed, tune, held`. `seed` is the seed for a seeded split and null for a key split. `tune` and `held` each print `{n, at_run, at_cut}`. `at_run` holds the counts under the command's own rule, and `at_cut` under the suggested cut.

**One group.** Groups keep the order of their first answer in RESULTS. For each group: `ok` is its answers that did not fail, and two verbs among them is a usage error. The labeled answers give the counts under the command's rule. The row prints, in this order: `group`, `verb` (null when every answer failed), `rows` (answers in `ok`), `failed`, `labeled`, `unlabeled`, `threshold` (`"as run"`, a cut number, or the band text as typed), `right`, `wrong`, `unresolved`, `tied`, `agreement`, `interval`, the four directions, `yes_recall`, `mean_probability`, `auc`, `disagreements`, `calibration`, `coverage`, `suggested`. Unless the group has labeled answers and every one carries probabilities, `mean_probability`, `auc`, `calibration`, `coverage`, and `suggested` are null.

**Rounding.** Every float in the output is rounded to six decimal places before printing. Integers print as integers. A float with an integer value prints with `.0`, as `serde_json` does.

## Output and failures

audit builds every row before it prints. On success it writes one JSON object per group, one per line, and exits 0. An empty RESULTS prints nothing and exits 0. `--table` prints the prototype's `audit_table` text: the same lines, labels, spacing, and `-` for null. Its numbers print with three decimals, rounded half to even on the exact binary value as Python's `format` does. A unit test pins `0.0625` as `0.062` and `0.6875` as `0.688`.

On any failure audit prints nothing on standard output and one line on standard error. The table below is the whole set. Each sentence is pinned exactly by a test.

| Case | Exit | Standard error |
| --- | ---: | --- |
| RESULTS cannot be opened or read | 5 | `thinkthen: audit: cannot read the results file` |
| KEY cannot be opened or read | 5 | `thinkthen: audit: cannot read the key file` |
| A line is not UTF-8 JSON or not an object | 2 | `thinkthen: audit: results line N is not a JSON object` (or `key line N`) |
| No usable record id | 2 | `thinkthen: audit: results line N has no string or integer id at the --id pointer` |
| `--id` is not a pointer | 2 | `thinkthen: audit: --id takes a JSON pointer such as /id or ''` |
| A verb other than decide or choose | 2 | `thinkthen: audit: results line N holds an answer audit cannot grade; audit grades decide and choose` |
| A record twice for one question | 2 | `thinkthen: audit: results line N repeats a record for one question` |
| A key line lacks `id` or `value`, repeats an id, or has another `part` | 2 | `thinkthen: audit: key line N needs a new id, a value, and a part of tune or held when present` |
| A `choose` key value that is not text | 2 | `thinkthen: audit: key line N gives a choose value that is not text` |
| Parts on some labeled records only | 2 | `thinkthen: audit: the key gives a part on some labeled records and not on others` |
| Two verbs in one group | 2 | `thinkthen: audit: one question holds both decide and choose answers` |
| A rule over an answer with no probabilities | 2 | `thinkthen: audit: --threshold needs probabilities; rerun the question with --details` |
| A band over a `choose` answer | 2 | `thinkthen: audit: choose takes a single cut; a band applies to decide` |
| RESULTS and KEY both `-` | 2 | `thinkthen: audit: only one input may be standard input` |

An invalid `--threshold`, `--seed`, or `--target` value uses the existing threshold refusal or Clap's own usage error, and exits 2.

## Where the code lives

The pure core does the math, and the command reads the files.

- `crates/thinkthen/src/core/measure.rs` (new): SplitMix64, the shuffle, the Wilson interval, AUC, the calibration error, the quantile, the bootstrap, and six-place rounding. Ticket 0114 adds exact McNemar here.
- `crates/thinkthen/src/core/measure/answer.rs` (new): one graded answer from a parsed `Json` entry, the key, `said`, the key's value, and the outcome. Ticket 0114 reuses it whole.
- `crates/thinkthen/src/core/measure/audit.rs` (new): counts, the coverage curve, the split, the suggested cut, grouping, disagreements, and the output structs, serialized in the member order above.
- `crates/thinkthen/src/cli/audit.rs` (new): the Clap arguments, reading RESULTS and KEY as bytes, splitting lines, parsing each with `core/json.rs`, calling the core, the table, and the failure mapping.

The core receives parsed values and returns typed rows. It opens no file and reads no clock. The existing purity lint holds `core/measure*`. Extend `sdlc/scripts/policy.py` so `cli/audit.rs` may open only paths it was handed and standard input, and may not reference the environment, the network, a process, the engine, the cache, recordings, usage counters, or the interrupt carrier. Plant one forbidden reference of each kind in the policy self-test and prove each is refused. Reuse `core/pointer.rs` for `--id` and `core/threshold.rs` for the rule and its parsing.

## Fixtures and provenance

Copy these files byte for byte from `botassembly/beatles-bench` at `be7cea2e4aa41097e7f629e35b62dadedeaca544`, `tests/fixtures/audit/`, into `crates/thinkthen/tests/fixtures/measure/` under the same relative paths:

- `small/`: `decide.jsonl`, `decide-b.jsonl`, `decide-band.jsonl`, `decide-bare.jsonl`, `decide-key.jsonl`, `choose.jsonl`, `choose-b.jsonl`, `choose-key.jsonl`, `annotate.jsonl`, `annotate-key.jsonl`.
- `249/`: `control.jsonl`, `soft.jsonl`, `key.jsonl`, `key-noparts.jsonl`. These are thinkthen `decide --details` lines replayed from experiment 249's recordings with every key unset. They hold 272 yes/no questions about Beatles songs.
- `golden/`: all fourteen golden outputs, eight for audit and six for diff. This ticket turns on the eight audit files. Ticket 0114 turns on the six diff files. Copying them together keeps one provenance record.

The two `make.py` scripts stay upstream. Ian's ruling counts fifteen golden files: the fourteen outputs plus `golden/make.py`.

Add `crates/thinkthen/tests/fixtures/measure/README.md`. It names the source repository, the commit, the date 2026-09-24, and the SHA-256 of every copied file. It names the golden command line of each output and says a golden changes only when the prototype's rules change and the upstream goldens are rewritten. A test recomputes each SHA-256 with the crate's `sha2` and fails on a missing, extra, or changed file.

The build also captures three table outputs from the prototype at the same commit, with standard output only: `audit small/decide.jsonl small/decide-key.jsonl --table`, `audit small/choose.jsonl small/choose-key.jsonl --table`, and `audit small/annotate.jsonl small/annotate-key.jsonl --table`. They go under `golden/table/`, and the README records the exact commands that made them. Running the prototype reads that repository and writes nothing to it.

## Acceptance

**Goldens.** For each of the eight audit command lines, run the built binary from `tests/fixtures/measure/` and parse its standard output and the golden file as lists of JSON values. They match when:

- the two lists have the same length and order;
- objects have the same member names in the same order, and arrays have the same length;
- strings, booleans, and nulls are equal;
- a golden number written without a `.` or exponent is an integer, and the output has the same integer;
- a golden number written with one is a float, the output also writes a float, and the two differ by at most `1e-6 + 1e-12`.

The tolerance allows one last-place difference after rounding, since Python and Rust may sum in different orders. A wider gap is a bug. Each run exits 0 with empty standard error. The three table captures match byte for byte.

**Hand-checked values.** Port every assertion of `tests/test_measure.py` classes `Prng`, `Pointer`, `AuditDecide`, `AuditChoose`, `AuditAnnotate`, and the audit half of `LeaningNo` as Rust tests on the core. They include the SplitMix64 first three outputs; the Wilson interval `[0.3000, 0.9032]` for 4 of 6; AUC `7/9`; calibration error `0.375` and `0.45`; the band coverage rows; the key-part cut `0.45`; the `choose` target cut `0.71`; and the held-out Beatles numbers: accuracy 0.630, yes recall 0.311, mean p(yes) 0.397, AUC 0.725, calibration error 0.092, the tuned cut 0.42 on 134 tuning and 138 held-out questions, then accuracy 0.645 and yes recall 0.607 at the cut. Add an `index(3)` range test and a seeded-split repeatability test.

**Replay pipeline.** Replay a committed recording fixture through `thinkthen decide --details --replay DIR` with the key unset, pipe the output to `thinkthen audit` with a small committed key, and pin the exact output. This proves audit reads the current result shape.

**No request, no key.** Bind a counting loopback listener and name it in `THINKTHEN_BASE_URL`. Set `THINKTHEN_API_KEY` to a canary. Make the normal configuration and cache locations unreadable. Run every golden command line, every table line, and every failure case in the table above. The listener accepts zero connections. The canary appears in no standard output, standard error, Debug line, or temporary file. No cache, usage counter, lock, or folder is created or changed. A dry run is no proof and is not used.

**Failures.** Each row of the failure table has a test that pins its exit code, its exact standard-error line, and empty standard output. A secrecy test plants a distinctive id, record text, key value, and path, drives each failure, and finds none of them in standard error.

**Help.** Edit the root inventory assertion of tickets 0082 and 0083 to insert `audit` after `transform`. Keep one assertion. Pin the root row, the short and long introductions, and the two long-help sentences above. The vocabulary check passes with zero unsanctioned hits.

**Red-green.** Write each test first and watch it fail for its stated reason. Then plant each bug below in the finished code, one at a time. Record in the build record which named test turns red. A planted bug that no test catches gets a new test before landing.

1. The Wald interval `p ± Z·sqrt(p(1 − p)/n)` in place of Wilson.
2. The `Z²/(4n²)` term dropped from the Wilson half-width.
3. `false_yes` and `false_no` swapped.
4. AUC ties counted as 1 instead of ½.
5. The last calibration bin left open, so `p = 1.0` falls out.
6. The nearest-rank quantile in place of linear interpolation.
7. `index(n)` taken as `next() mod n`.
8. The seeded split taking `ceil(len / 2)` ids to tune.
9. The ids shuffled in file order without the code point sort.
10. The `decide` tie-break going to the larger cut.
11. The band's low edge counted as no.
12. A tied `choose` answer counted as wrong.
13. One bootstrap generator shared across groups.

**Gates.** Run focused tests, the policy self-test, the fixture checksum test, formatting, Clippy, the exact ratchet, and `git diff --check`. Then run `sdlc/scripts/install`, `lint`, `test`, and `spec` in sequence with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset. No live or paid call runs.

## Specification pages

Add `specification/audit.md` with Status **Settled** once this ticket lands. It states the command line, the inputs, every definition in "The math", the output members, the failure table, and the replay pipeline. It cites the prototype commit. Add its row to `specification/README.md`. Add an executable `spec/audit.md` that runs `audit --table` on `small/decide.jsonl` and pins its first two lines, and pins one refusal with its exit code. Help and pages change in the same commit as the behavior.

## Where this port departs from the prototype

None of these cases appears in a golden file. Each is the agent's decision, and Ian can overturn it.

- **Diagnostics and exit codes.** The prototype exits 2 through `argparse` on its own errors and exits 1 with a Python traceback on a missing file or bad JSON. It echoes ids and up to 80 characters of a record. The port uses the failure table above, exit 5 for a file it cannot read, and echoes nothing a user supplied.
- **The pointer.** The prototype's `pointer` accepts `-1` and leading zeros as array indexes, following Python list indexing. The port uses thinkthen's own RFC 6901 pointer, which refuses both.
- **The rule text.** The prototype parses with Python `float()`. The port uses the settled threshold grammar.
- **The seed.** The prototype masks a negative seed to 64 bits. The port refuses a negative seed.
- **The target.** The prototype accepts any number. The port accepts 0 to 1.
- **A key value for choose.** The prototype would count a number as wrong. The port refuses it.
- **Parts.** The prototype's README says every key line carries a part or none does. Its code checks only the labeled records of each group. The port follows the code, which the goldens pin.

## Budgets and the ratchet

- Production Rust: four new files (`core/measure.rs`, `core/measure/answer.rs`, `core/measure/audit.rs`, `cli/audit.rs`) and at most four existing files touched, likely `core/mod.rs`, `cli/mod.rs`, `cli/args/command.rs`, and one failure-mapping file. At most 950 nonblank production lines.
- Rust tests: at most 800 nonblank lines across at most three new test files and the edited inventory assertion.
- Scripts: `policy.py` and its self-test fixtures, at most 60 nonblank lines.
- Fixtures: the 28 copied files, the three table captures, the new fixture README, and at most two small files for the replay pipeline.
- Prose: `specification/audit.md`, one index row, `spec/audit.md`, and command help.
- Every Rust file stays under the 500-nonblank-line ceiling.
- Dependencies: none. `serde_json`, `sha2`, and `u128` arithmetic cover the work. A new dependency stops the build for a second review that names what it checked.

`sdlc/ratchet.json` rises by the measured Rust increase in the commit that needs it. That commit message says what grew, why it earns its lines, and where the builder looked for duplication first: at least `core/pointer.rs`, `core/threshold.rs`, `core/json.rs`, `cli/table.rs`, and `cli/transform.rs`'s early routing. This ticket raises the ceiling and widens the public surface, so a second agent reviews it and names what it checked.

Stop and re-score if the work exceeds any budget, needs a dependency, changes a golden, or reaches a behavior outside this page.

## Scope and exclusions

Allowed: the audit command and its help, the core math, the fixtures and their checksum test, the policy extension, the specification and executable pages, the inventory assertion edit, and the ratchet.

Excluded: `diff` (ticket 0114); grading `tag`, `score`, `find`, `recognize`, or `relate` answers; reading a recording or cache folder directly; any API in Rust, C, language packages, or databases; a `schema` member; any change to the goldens; edits to the Beatles Bench repository; live or paid calls; release artifacts.

## Dependencies and order

Depends on the ten functions done: ticket 0086 (public Rust API) landed on main, after 0082 (command contract) and 0083 (transform catalog), whose root inventory assertion and early routing this ticket extends. The queue in `sdlc/planning/one-line-plan-2026-09-24.md` places audit after the ten functions and before the release build. Ticket 0114 follows this ticket and reuses `core/measure.rs` and `core/measure/answer.rs`.

## Complexity

Contract 2; State/timing 0; Reach 1; Proof 2; Cost of error 1; Total 6. Final level: 2. Reasons: exact numeric output, a seeded generator matched bit for bit, and goldens from another repository. The runtime holds no state and sends nothing.
