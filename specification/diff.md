# Diff

Status: **Settled** by ticket 0114.

`thinkthen diff A [B]` compares two runs over the same records, or two cuts on one run. It lists each answer that changed, says which way it moved, and runs the exact McNemar test. It sends no request and reads no API key.

The definition is the `diff` half of the prototype measurement script at commit `be7cea2e`. `crates/thinkthen/tests/fixtures/measure/README.md` gives the file checksums. Where this page and the prototype disagree, the golden files in that folder decide. diff shares its readers, rule text, and failure rows with [audit](audit.md).

```sh
thinkthen diff runs/before.jsonl runs/after.jsonl --key key.jsonl --table
thinkthen diff runs/before.jsonl --key key.jsonl --compare-threshold 0.4
```

## Command line

```text
thinkthen diff A [B] [--key KEY] [--threshold RULE] [--compare-threshold RULE] [--id POINTER] [--table]
```

- `A` and `B` hold saved result lines. At most one input, `KEY` included, may be `-` for standard input.
- With `B`, two runs are compared. Without `B`, A is compared with itself, and `--compare-threshold` is required. Two cuts on one run cost nothing, because the probabilities are already saved.
- A reads under `--threshold`. B reads under `--compare-threshold`, or under `--threshold` when that is absent. Without a rule an answer stays as printed. Both take the settled threshold grammar.
- `--key KEY` is read as in audit. Parts play no role in diff, and a bad part is refused.
- `--id POINTER` works as in audit, default `/id`.
- `--table` prints the results for a person instead of JSON lines.

diff pairs answers by answer name and record id only. It does not check that the two runs saw the same record text. It compares each pair's question digest and warns when the digests differ, as "Warnings" below says. The `compare` transform checks both the record text and the question.

## The math

The answer under a rule, the key's value, and the outcome (right, wrong, not sure, tied) come from audit. The confidence is `p(yes)` for `decide` and the top probability for `choose`, or null without probabilities.

**Pairing.** Failed answers leave. The pair key is (answer name, record id). `only_a` counts A's pair keys missing from B, and `only_b` the reverse. diff walks A's answers in file order and skips each one without a partner.

**Each pair.** `records` adds one. `from` is A's answer under A's rule, and `to` is B's answer under B's rule. With a key value for the record, `labeled` adds one, and `right_a` and `right_b` count right answers on each side. When `from` differs from `to`, `changed` adds one, the move adds one, and the pair prints a row.

**The effect** of a changed pair with a key value:

| From | To | Effect |
| --- | --- | --- |
| wrong | right | `gained` |
| right | wrong | `lost` |
| tied or not sure | right or wrong | `resolved` |
| right or wrong | tied or not sure | `withdrawn` |
| any other pair | | `changed` |

Without a key value the effect is null. `gained` and `lost` count those effects.

**McNemar.** With a key, the test runs on the discordant pairs of right answers, and `mcnemar_on` is `"right answers"`. One function, `discordant`, counts a pair when it is right on one side and not right on the other. The other answer may be wrong, tied, or not sure. A tied answer that becomes right counts, and so does a right answer that becomes not sure. Quick Fix `qf-diff-warnings` widened the rule from the prototype's wrong-to-right and right-to-wrong. It closed `sdlc/issues/closed/2026-09-25-diff-mcnemar-leaves-out-pairs-that-become-right-from-not-sure.md`. The `diff-choose` golden moved from p 1.0 to p 0.5 (2 to 4 right of 5). Without a key, when every answer of A is `decide` (failed ones included), the test runs on yes-to-no against no-to-yes moves, and `mcnemar_on` is `"yes answers"`. Otherwise both members are null.

For counts `a` and `b`, with `n = a + b`: `p` is 1 when `n` is 0. Otherwise `p = min(1, 2 · Σ C(n, i) / 2ⁿ)` for `i` from 0 to `min(a, b)`. The port sums the terms in log space, and a unit test holds it to exact integer sums up to `n = 120` within a relative `1e-12`.

**The prototype's narrow rule.** The prototype counted only wrong to right and right to wrong. Restoring it changes `discordant`, its unit test, the McNemar edge rows in `tests/diff.rs`, the two `diff-choose` captures, the McNemar block in `spec/diff.md`, and the fixtures README checksums and note in one commit.

## Output

diff builds every line before printing. It prints one JSON object per changed pair in A's order, then a summary line, and exits 0. A warning goes to standard error and changes neither standard output nor the exit code.

A row prints `id`, `name` (the `annotate` answer name, or null), `from`, `to`, `probability` (the two confidences), `key` (`"yes"`, `"no"`, the option, or null), and `effect`.

The last line is `{"summary": S}`. `S` prints `records`, `changed`, `only_a`, `only_b`, `moves`, `labeled`, `right_a`, `right_b`, `gained`, `lost`, `mcnemar_on`, `mcnemar_p`, `compare`, `a`, and `b`. `moves` lists `{from, to, count}` by count, largest first, then by `from` and `to` in code point order. Without a key, the five key counts are null. `compare` is `"runs"` with B and `"cuts"` without. `a` and `b` print `"as run"`, a cut as a number, or a band exactly as typed.

Every float rounds to six places. A reader of this output ignores members it does not know, because a later version may add them.

`--table` prints the prototype's table. A changed row prints `ID[/NAME]  FROM -> TO  p PA -> PB`, and `  key KEY: EFFECT` when the key has a value. Probabilities print with two decimals, and `-` stands for null. The count line starts `A -> B`, or `A -> B (at RULE_A and RULE_B)` when either side has a rule, or `RULE_A -> RULE_B` for two cuts. It goes on with `: C of N changed` and `; FROM -> TO COUNT` for each move. With a key it adds `; gained G, lost L (RA -> RB right of LABELED)`. With a test it adds `; McNemar p P on ON` with three decimals. With unpaired answers it adds `; only in A X, only in B Y`. The table keeps the prototype's words, `unresolved` and `tied` included, as audit's does. An answer inside a band is not sure, and the table calls it `unresolved`.

## Warnings

diff prints at most two warning lines on standard error, after standard output. Quick Fix `qf-diff-warnings` added them from the options in the closed issue `2026-09-25-diff-exits-0-when-nothing-pairs-and-pairs-different-questions-silently.md`. A script that wants to fail on either one reads standard error.

| Case | Standard error |
| --- | --- |
| No answer paired: `records` is 0 | `thinkthen: diff: warning: no answer paired; check that both runs hold the same record ids and answer names` |
| `D` of the `N` pairs carry different question digests | `thinkthen: diff: warning: the question digest differs in D of N paired answers. A different question, threshold, or profile gives a different digest.` |

A line's question digest is its `meta.question_sha256`. An `annotate` line carries `meta.questions_sha256` instead, one digest over all its questions. Each answer on that line takes that one digest, so one changed question flags every answer on the line. The digest check counts a pair only when both lines carry a digest. The digest covers the threshold and the profile too. Two runs of one question at different thresholds or under different profiles also warn. Two cuts on one run compare a line with itself and never warn. With no pair, the second warning cannot print. Failed answers pair with nothing, so a run of only failed answers warns that no answer paired.

## Failures

A failure prints nothing on standard output and one line on standard error. The line never echoes a record, an id, a key value, or a path. `ROLE` is `first run`, `second run`, or `key`, and `N` is a one-based line number. diff prints audit's failure rows with the prefix `thinkthen: diff:` and these roles, and adds these rows:

| Case | Exit | Standard error |
| --- | ---: | --- |
| No B and no `--compare-threshold` | 2 | `thinkthen: diff: diff needs a second run or --compare-threshold` |
| One answer name and record twice in one run | 2 | `thinkthen: diff: ROLE line N repeats a record for one answer` |
| More than one input is `-` | 2 | `thinkthen: diff: only one input may be standard input` |
| A bad `--compare-threshold` | 2 | `thinkthen: diff: --compare-threshold: ` and the threshold refusal |

A rule over answers without probabilities prints audit's `--threshold needs probabilities` sentence, whichever option named the rule.

## What diff never does

diff routes before any setup. It reads the named inputs and nothing else: no API key, environment variable, configuration file, cache, or usage counter. It opens no socket, starts no process, and writes only standard output and standard error.

## Departures

No golden file reaches these. Each is the agent's decision, and Ian can overturn it. audit's departures on failures, bad numbers, JSON, empty input, the pointer, rule text, key values, `choose` values, and state names hold here too.

- **Numbers.** The prototype prints a probability saved as an integer, such as `1`, as `1` in JSON and in the table. The port always prints a probability as a float: `1.0` in JSON and `1.00` in the table.
- **Repeats.** The prototype keeps the last of two answers with one answer name and record id in a run. The port refuses the run, failed answers included.
