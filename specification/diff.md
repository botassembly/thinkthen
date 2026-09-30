# Diff

Status: **Settled** by ticket 0114. Ticket 0165 added `recognize` and `relate`.

`thinkthen diff A [B]` compares two runs over the same records, or two cuts on one run. It lists each answer that changed, says which way it moved, and runs the exact McNemar test. For `recognize` and `relate` it lists the names or edges each record gained, lost, or changed in kind, as "Names and edges" says. It sends no request and reads no API key.

The definition is the `diff` half of a prototype measurement script. The script's history was removed, so no commit holds it now. `crates/thinkthen/tests/fixtures/measure/README.md` gives the file checksums, and they are the record. Where this page and the prototype disagree, the golden files in that folder decide. diff shares its readers, rule text, and failure rows with [audit](audit.md).

```sh
thinkthen diff runs/before.jsonl runs/after.jsonl --key key.jsonl --table
thinkthen diff runs/before.jsonl --key key.jsonl --compare-threshold 0.4
```

## Command line

```text
thinkthen diff A [B] [--key KEY] [--threshold RULE] [--compare-threshold RULE] [--id POINTER] [--match strict|overlap] [--table]
```

- `A` and `B` hold saved result lines. At most one input, `KEY` included, may be `-` for standard input.
- With `B`, two runs are compared. Without `B`, A is compared with itself, and `--compare-threshold` is required. Two cuts on one run cost nothing, because the probabilities are already saved.
- A reads under `--threshold`. B reads under `--compare-threshold`, or under `--threshold` when that is absent. Without a rule an answer stays as printed. Both take the settled threshold grammar.
- `--key KEY` is read as in audit. Parts play no role in diff, and a bad part is refused.
- `--id POINTER` works as in audit, default `/id`.
- `--match strict|overlap` says how a `recognize` name on one side pairs with a name on the other, with audit's meaning and audit's default, `strict`. It applies to `recognize` and `relate` only.
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

**The prototype's narrow rule.** The prototype counted only wrong to right and right to wrong. Restoring it changes `discordant`, its unit test, the McNemar edge rows in `tests/backend/diff.rs`, the two `diff-choose` captures, the McNemar block in `spec/diff.md`, and the fixtures README checksums and note in one commit.

## Output

diff builds every line before printing. It prints one JSON object per changed pair in A's order, then a summary line, and exits 0. A warning goes to standard error and changes neither standard output nor the exit code.

A row prints `id`, `name` (the `annotate` answer name, or null), `from`, `to`, `probability` (the two confidences), `key` (`"yes"`, `"no"`, the option, or null), and `effect`.

The last line is `{"summary": S}`. `S` prints `records`, `changed`, `only_a`, `only_b`, `moves`, `labeled`, `right_a`, `right_b`, `gained`, `lost`, `mcnemar_on`, `mcnemar_p`, `compare`, `a`, and `b`. `moves` lists `{from, to, count}` by count, largest first, then by `from` and `to` in code point order. Without a key, the five key counts are null. For `recognize` and `relate` pairs, `key_items`, `items_gained`, `items_lost`, `items_changed_kind`, `extra_a`, and `extra_b` print after `mcnemar_p` and before `compare`, as "Names and edges" says. A summary of other verbs does not print them. `compare` is `"runs"` with B and `"cuts"` without. `a` and `b` print `"as run"`, a cut as a number, or a band exactly as typed.

Every float rounds to six places. A reader of this output ignores members it does not know, because a later version may add them.

`--table` prints the port's table. A changed row prints `ID[/NAME]  FROM -> TO  p PA -> PB`, and `  key KEY: EFFECT` when the key has a value. Probabilities print with two decimals, and `-` stands for null. The count line starts `A -> B`, or `A -> B (at RULE_A and RULE_B)` when either side has a rule, or `RULE_A -> RULE_B` for two cuts. It goes on with `: C of N changed` and `; FROM -> TO COUNT` for each move. With a key it adds `; gained G, lost L (RA -> RB right of LABELED)`. With a test it adds `; McNemar p P on ON` with three decimals. With unpaired answers it adds `; only in A X, only in B Y`. "Names and edges" gives the rows and the count line for `recognize` and `relate`. The port prints `unsure` where the prototype used its older word for a not sure answer. An answer inside a band is not sure, and the table calls it `unsure`.

## Names and edges

`recognize` and `relate` say a set of items per record. diff reads them with audit's readers and matches them with audit's rules. Ticket 0165 settled this section.

**Reading.** Every input is a `--details` line, because diff reads the kinds, the relations, and the run cut from `question`. A line saved without it is refused with the cannot-grade row. A `recognize` or `relate` line with no `input` takes its one-based line number as its record id, as audit does. Pairing is by answer name and record id, as for other verbs. A `relate` line whose `meta.failed_questions` is above 0 counts as failed and leaves. A `recognize` or `relate` answer pairs only with an answer of the same verb. One diff holds only `recognize` pairs, only `relate` pairs, or only pairs of other verbs.

**The cut.** Each side's items are its names or edges at or above that side's cut. A takes `--threshold` or the run cut. B takes `--compare-threshold`, else `--threshold`, else the run cut. A cut is one number at or above the line's run cut. A band, or a cut below the run cut, is refused with audit's sentence. A diff reads a `recognize` line's names only, and leaves its `relations` out.

**Matching.** `--match strict` pairs an A name with a B name of the same `start`, `end`, and `kind`. `--match overlap` pairs names of the same kind whose places overlap. Edges pair as audit pairs them: the same relation, source name and kind, and target name and kind. An edge of a relation that the A item's question marks `either` also pairs with its endpoints swapped. `--match` changes nothing for edges. Pairing is one to one. Each side's items are taken by `strength` or `probability`, high to low, ties in output order. Each A item takes the first unpaired B item, in that order, that it pairs with.

- **Lost** is an A item left unpaired.
- **Gained** is a B item left unpaired.
- **Changed kind** is an unpaired A name and an unpaired B name at places that match under `--match`, with different kinds. They count once, not as a loss and a gain. They pair one to one in the same order. Edges have no changed kind.

**The key.** The key is read as audit reads it. Each side reads it under its own question, so a side with no kinds reads every key name as the kind `ENTITY`. A key item whose kind or relation a side's question lacks is refused with audit's row. Each side is matched to the key by audit's "One match each" rule. A key item is matched on a side when that rule pairs it. A side's extras are its items the rule leaves unmatched. McNemar runs on the key items matched only in B against those matched only in A, and `mcnemar_on` is `"key names"` or `"key edges"`. Items matched on both sides or on neither are concordant, and extras stay out of the test.

A record changes when it has a lost, gained, or changed-kind item, or a key item matched on one side only. A changed record prints one row, in A's order, with `id`, `name`, `lost`, `gained`, `changed_kind`, and `key`. `lost` and `gained` hold items in the command's own shape, whole, in their line's output order. Each `changed_kind` entry is `{"from": A item, "to": B item}`, in its A item's order. `key` is `{"matched": [A, B], "extra": [A, B]}`, or null without a key value for the record.

```json
{"id":"r1","name":null,"lost":[{"text":"Revolver Paul McCartney","start":3,"end":26,"length":23,"kind":"person","strength":0.81}],"gained":[{"text":"Revolver","start":3,"end":11,"length":8,"kind":"work","strength":0.9},{"text":"Paul McCartney","start":12,"end":26,"length":14,"kind":"person","strength":0.97}],"changed_kind":[],"key":{"matched":[0,2],"extra":[1,0]}}
```

The summary keeps every member other verbs print. `moves` is an empty list. `labeled` counts records with a key value. `right_a` and `right_b` count key items matched on each side. `gained` and `lost` count key items matched only in B and only in A. `key_items` counts the key items of the labeled records. `items_gained`, `items_lost`, and `items_changed_kind` count items over every changed record. `extra_a` and `extra_b` count each side's extras. Without a key, `key_items`, `extra_a`, and `extra_b` are null, and so is the test.

`--table` prints a changed record as `ID[/NAME]  +G -L ~K`. Then it prints one line per item: all `+` lines, then all `-` lines, then all `~` lines. A name prints as `  + TEXT [START,END) KIND strength S`, with four decimals. An edge prints as `  + RELATION SOURCE (KIND) -> TARGET (KIND) p P`, with two decimals. A changed kind prints as `  ~ TEXT [START,END) KIND -> KIND`, or with both places when they differ: `  ~ TEXT [S,E) KIND -> TEXT [S,E) KIND`. The count line goes on after `: C of N changed` with `; items gained G, lost L, changed kind K`. With a key it adds `; key names matched MA -> MB of T; extras XA -> XB`, or `key edges` for `relate`, in place of the `gained` clause. The McNemar and only-in clauses follow as for other verbs.

```text
A -> B: 4 of 4 changed; items gained 4, lost 3, changed kind 1; key names matched 0 -> 3 of 4; extras 3 -> 1; McNemar p 0.250 on key names
```

diff walks the pairs in A's order. The pairing row fires on the first pair whose two verbs differ when one of them is `recognize` or `relate`. Two other verbs that differ, such as `decide` and `choose`, pair and print a row. Otherwise the mix rows fire on the first pair whose verb class differs from the first pair's class. The classes are `recognize`, `relate`, and every other verb. ROLE is `second run`, and N is the pair's B line. With no second run, ROLE is `first run`, and N is the A line.

| Case | Exit | Standard error |
| --- | ---: | --- |
| A `recognize` or `relate` answer paired with an answer of another verb | 2 | `thinkthen: diff: ROLE line N pairs recognize or relate with another verb` |
| `recognize` or `relate` pairs beside pairs of other verbs | 2 | `thinkthen: diff: ROLE line N mixes recognize or relate with other verbs` |
| `recognize` pairs beside `relate` pairs | 2 | `thinkthen: diff: ROLE line N mixes recognize with relate` |
| `--match` given over `decide` or `choose` pairs | 2 | `thinkthen: diff: --match applies to recognize and relate` |

## Warnings

diff prints at most three warning lines on standard error, after standard output. Quick Fix `qf-diff-warnings` added the first two from the options in the closed issue `2026-09-25-diff-exits-0-when-nothing-pairs-and-pairs-different-questions-silently.md`. A script that wants to fail on any of them reads standard error.

| Case | Standard error |
| --- | --- |
| No answer paired: `records` is 0 | `thinkthen: diff: warning: no answer paired; check that both runs hold the same record ids and answer names` |
| `D` of the `N` pairs carry different question digests | `thinkthen: diff: warning: the question digest differs in D of N paired answers. A different question, threshold, or profile gives a different digest.` |
| The union of the runs' batch settings holds more than one value | `thinkthen: diff: warning: the runs used different batch settings (1 and max); batching moves answers, so some changes may come from it` |

A line's question digest is its `meta.question_sha256`. An `annotate` line carries `meta.questions_sha256` instead, one digest over all its questions. Each answer on that line takes that one digest, so one changed question flags every answer on the line. The digest check counts a pair only when both lines carry a digest. The digest covers the threshold and the profile too. Two runs of one question at different thresholds or under different profiles also warn. Two cuts on one run compare a line with itself and never warn. With no pair, the second warning cannot print. Failed answers pair with nothing, so a run of only failed answers warns that no answer paired.

Each side's batch setting is the set of `meta.batch.setting` values its lines carry. A line without `meta.batch` adds no setting, so a side whose lines name none adds nothing to the union and cannot cause the warning. A present invalid `meta.batch.setting` is refused with its line number. The third warning appears when the union of both sides' sets holds more than one value, with numbers in ascending order before `max`. Two cuts on one run therefore add no batch warning unless that run itself mixed settings.

## Failures

A failure prints nothing on standard output and one line on standard error. The line never echoes a record, an id, a key value, or a path. `ROLE` is `first run`, `second run`, or `key`, and `N` is a one-based line number. diff prints audit's failure rows with the prefix `thinkthen: diff:` and these roles, and adds these rows:

| Case | Exit | Standard error |
| --- | ---: | --- |
| No B and no `--compare-threshold` | 2 | `thinkthen: diff: diff needs a second run or --compare-threshold` |
| One answer name and record twice in one run | 2 | `thinkthen: diff: ROLE line N repeats a record for one answer` |
| More than one input is `-` | 2 | `thinkthen: diff: only one input may be standard input` |
| A bad `--compare-threshold` | 2 | `thinkthen: diff: --compare-threshold: ` and the threshold refusal |
| A line diff cannot grade, such as a `tag` line or a `recognize` line saved without `--details` | 2 | `thinkthen: diff: ROLE line N holds an answer diff cannot grade; diff grades decide, choose, recognize and relate` |
| `--match` given over `decide` or `choose` pairs | 2 | `thinkthen: diff: --match applies to recognize and relate` |

A rule over answers without probabilities prints audit's `--threshold needs probabilities` sentence, whichever option named the rule.

## What diff never does

diff routes before any setup. It reads the named inputs and nothing else: no API key, environment variable, configuration file, cache, or usage counter. It opens no socket, starts no process, and writes only standard output and standard error.

## Departures

No golden file reaches these. Each is the agent's decision, and Ian can overturn it. audit's departures on failures, bad numbers, JSON, empty input, the pointer, rule text, key values, `choose` values, and state names hold here too.

- **Numbers.** The prototype prints a probability saved as an integer, such as `1`, as `1` in JSON and in the table. The port always prints a probability as a float: `1.0` in JSON and `1.00` in the table.
- **Repeats.** The prototype keeps the last of two answers with one answer name and record id in a run. The port refuses the run, failed answers included.
