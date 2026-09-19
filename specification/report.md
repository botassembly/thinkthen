# `report`

Status: **Draft**. ADR 0007 fixes what `report` reads and what it counts, and it leaves the options open until a demo drives them.

Counts answers and calls no model.

```text
thinkthen report [--truth POINTER]
```

## What it reads

`--details` rows on standard input, one JSON object per line, as [result.md](result.md) gives them. Rows from `decide`, `choose`, `score`, `filter`, `rank`, and `annotate` are all accepted. `report` opens no connection, reads no key, and needs no profile.

A row that is not valid JSON, and a row that is not a result object, are local failures at exit 5.

## What it prints

One JSON object. It holds counts of yes, no, and unresolved for each question, label counts for each `choose` question, and a summary for each `score` question. A single-command run has one question. An `annotate` run has one entry per question name.

```json
{"questions":{"unresolved":{"kind":"yes_no","yes":128,"no":61,"unresolved":11},"kind":{"kind":"choice","labels":{"bug":94,"feature":71,"other":24},"unresolved":11},"impact":{"kind":"score","count":200,"mean":1.24,"min":0,"max":2}}}
```

Every count here is illustrative.

## The threshold sweep

Given a pointer to a labeled truth in `input`, `report` also prints how each threshold would have scored against those labels. The sweep costs no model call, because the probabilities are already in the rows.

```json
{"questions":{"unresolved":{"kind":"yes_no","yes":128,"no":61,"unresolved":11,"sweep":[{"threshold":0.5,"true_positive":120,"false_positive":14,"true_negative":58,"false_negative":8},{"threshold":0.9,"true_positive":101,"false_positive":3,"true_negative":69,"false_negative":27}]}}}
```

A sweep is the only honest test of a threshold. A threshold is a measurement for one model version, and a number carried over from another model version is a guess.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--truth POINTER` | A JSON Pointer into each row's `input` naming the labeled answer. It turns on the sweep | None, and no sweep prints |

`report` takes no `--threshold`, no `--details`, no `--dry-run`, and no backend option.

## Exit codes

0 when the run finished, and 2, 5, and 70 as [channels.md](channels.md) gives them. Codes 1, 3, and 4 cannot arise, because `report` judges nothing and sends nothing. An empty input exits 0 and prints an object with no questions.

## Examples

```sh
thinkthen decide 'This reports a payment failure.' --jsonl --field /body --details < tickets.jsonl > judged.jsonl
thinkthen report < judged.jsonl
```

```sh
thinkthen report --truth /label < judged.jsonl | jq '.questions.unresolved.sweep'
```

## Open points

- What are the options? `--truth POINTER` is the only one this page proposes. A demo drives the rest.
- Which thresholds does the sweep try? Recommendation: every cut from 0.05 to 0.95 in steps of 0.05, and bands wait for a demand.
- What does a `score` summary hold? Recommendation: `count`, `mean`, `min`, and `max`.
- What happens to a row whose `question.text` differs from the others under the same name? Recommendation: `report` counts it anyway and names the distinct texts on standard error, because a rerun with an edited question is a different measurement.
