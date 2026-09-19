# `report`

Status: **Draft**. The options come from Proposed ADR 0008, and the headline table comes from Proposed ADR 0009. Every option stays Draft until demo 13 and demo 14 drive it.

Interprets a saved run and calls no model.

```text
thinkthen report [RUN] [--truth NAME=POINTER]... [--threshold NAME=RULE]... [--baseline RUN] [--id POINTER]
```

## What it reads

`--details` rows, one JSON object per line, as [result.md](result.md) gives them. `RUN` names a file, and without it the rows arrive on standard input. Rows from `decide`, `choose`, `score`, `filter`, `rank`, and `annotate` are all accepted.

An `annotate` run holds one check per question name. A run from `decide`, `choose`, or `score` is one check named after the verb. Rows under one name that carry more than one question text are listed as a warning, because a rerun with an edited question is a different measurement.

`report` opens no connection, reads no key, and needs no profile. A row that is not a result object is a local failure at exit 5.

## What it prints with no option

One JSON object. For each check it holds the counts of yes, no, and unresolved, the label counts, and the score summary. It also holds the run's facts: the models that answered, the definition digests, the tool versions, and how many rows were replayed. A run that mixes two model versions says so.

Every count on this page is illustrative.

```json
{"rows":200,"replayed":200,"models":["jev-1.13.0"],"tool":["thinkthen 0.4.0"],"questions_sha256":["9ad3...7e"],"checks":{"unresolved":{"kind":"yes_no","yes":128,"no":61,"unresolved":11},"kind":{"kind":"choice","labels":{"bug":94,"feature":71,"other":24},"unresolved":11},"impact":{"kind":"score","count":200,"mean":1.24,"min":0,"max":2}}}
```

## Accuracy at coverage

`--truth NAME=POINTER` compares a judged check with a trusted label inside the case. The headline table is accuracy at coverage: the share of rows that resolved, the accuracy among them, and the accuracy among the rest.

```json
{"checks":{"unresolved":{"coverage":0.945,"accuracy_resolved":0.962,"accuracy_unresolved":0.545,"true_positive":120,"false_positive":7,"true_negative":62,"false_negative":0,"precision":0.945,"recall":1.0,"f1":0.972}}}
```

A yes/no check also gets the four counts, accuracy, precision, recall, and F1. Unresolved rows are counted apart and are never scored as right or wrong. The same table prints for each cut in a sweep, and a calibration table sets each probability band beside the share of cases that were truly yes. A `choose` check gets precision, recall, and F1 for each label, and the macro average.

`--truth /POINTER=/POINTER` compares two fields of the case exactly, with no judgment involved. A benchmark where a candidate's own label meets a gold label uses that form.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `RUN` | The file of rows | Standard input |
| `--truth NAME=POINTER` | Scores the check `NAME` against a label in the case. Repeatable | None, and no scoring prints |
| `--threshold NAME=RULE` | Applies another rule to the stored probabilities. No request is made | The rule each row was judged at |
| `--baseline RUN` | Compares this run with an earlier one | None |
| `--id POINTER` | Matches cases between the two runs | `/id` |

`report` takes no `--details`, no `--dry-run`, and no backend option.

`--baseline` prints the change in every metric, the cases that flipped in each direction, and the cases present in only one run. Rows that share an id inside one run are repeated trials. A metric averages within a case first, so a case tried five times weighs the same as a case tried once.

## Picking a cut and reporting on held-out cases

`report` has no option for a held-out split. Two files do the job, and the split stays visible in the shell.

```sh
thinkthen annotate checks.json --jsonl --record runs/dev --replay runs/dev < dev.jsonl > run-dev.jsonl
thinkthen report run-dev.jsonl --truth unresolved=/gold | jq '.checks.unresolved.sweep'
```

Read the cut off that sweep. Then judge the held-out cases and report at the cut alone.

```sh
thinkthen annotate checks.json --jsonl --record runs/test --replay runs/test < test.jsonl > run-test.jsonl
thinkthen report run-test.jsonl --truth unresolved=/gold --threshold unresolved=0.1:0.9
```

A cut chosen on the same cases it is reported against is not a measurement. A threshold is a measurement for one model version, and a number carried over from another version is a guess.

## Exit codes

0 when the run finished, and 2, 5, and 70 as [channels.md](channels.md) gives them. Codes 1, 3, and 4 cannot arise, because `report` judges nothing and sends nothing. An empty input exits 0 and prints an object with no checks.

## Open points

- Which cuts does a sweep try? Recommendation: every cut from 0.05 to 0.95 in steps of 0.05. Bands wait for a demand.
- Does a sweep also try the backend's `confidence`? Proposed ADR 0009 asks for both to be swept against labels. Recommendation: sweep both and name each column, then look at the `choose` rule again.
- What does a `score` summary hold? Recommendation: `count`, `mean`, `min`, and `max`.
