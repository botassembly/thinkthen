# `report`

Status: **Draft**. The options come from Proposed ADR 0008, and the headline table comes from Proposed ADR 0009. Every option stays Draft until demo 13 and demo 14 drive it.

Interprets a saved run and calls no model.

```text
thinkthen report [RUN] [--truth NAME=POINTER]... [--threshold NAME=RULE]... [--baseline RUN] [--id POINTER]
```

## What it reads

`--details` rows, one JSON object per line, as [result.md](result.md) gives them. `RUN` names a file, and without it the rows arrive on standard input. Rows from `decide`, `choose`, `score`, `filter`, `rank`, and `annotate` are all accepted.

An `annotate` run holds one check per question name. A run from `decide`, `choose`, or `score` is one check named after the verb. Rows under one name that carry more than one question text are listed under `warnings`, because a rerun with an edited question is a different measurement.

A pointer in `--truth` reads the case, which a `--details` row saved as `input`. `/label` names the case's own `label` member. A pointer that names a field the questions never saw is the point of the option, because a truth label is what `--field` and `on` keep off the wire.

`report` opens no connection, reads no key, and needs no profile. A row that is not a result object is a local failure at exit 5.

## The report object

`report` prints one JSON object, and scripts are the only readers of it. This section fixes every key. Every count on this page is illustrative.

With no option the object carries the run's facts and one entry per check.

```json
{"rows":200,"replayed":200,"models":["jev-1.13.0"],"tool":["thinkthen 0.4.0"],"questions_sha256":["9ad3...7e"],"warnings":[],"checks":{"unresolved":{"kind":"yes_no","yes":128,"no":61,"unresolved":11},"kind":{"kind":"choice","labels":{"bug":94,"feature":71,"other":24},"unresolved":11},"impact":{"kind":"score","count":200,"mean":1.24,"min":0,"max":2}}}
```

`models`, `tool`, and `questions_sha256` each hold the distinct values the rows carried, sorted. A run that mixes two model versions therefore shows two entries. `warnings` holds one string per problem the rows show, and it is empty on a clean run.

## Accuracy at coverage

`--truth NAME=POINTER` compares a judged check with a trusted label inside the case. The headline is accuracy at coverage: the share of rows that resolved, the accuracy among them, and the accuracy among the rest. The named check gains these keys beside the counts it already had.

```json
{"kind":"yes_no","yes":128,"no":61,"unresolved":11,"coverage":0.945,"accuracy_resolved":0.962,"accuracy_unresolved":0.545,"true_positive":120,"false_positive":7,"true_negative":62,"false_negative":0,"precision":0.945,"recall":1.0,"f1":0.972,"sweep":[],"calibration":[]}
```

Unresolved rows are counted apart and are never scored as right or wrong. A `choose` check gets `precision`, `recall`, and `f1` for each label under `labels_scored`, and the macro average under `macro`.

`sweep` holds one row per cut, each carrying the same rates at that cut.

```json
{"threshold":0.05,"coverage":1.0,"accuracy_resolved":0.71,"accuracy_unresolved":null,"precision":0.62,"recall":1.0,"f1":0.77}
```

`calibration` holds ten bands of 0.1, lowest first. `rows` counts the rows whose probability fell in the band, and `truly_yes` is the share of those the label called yes. A band with no rows reports `"rows":0` and `"truly_yes":null`.

```json
{"low":0.9,"high":1.0,"rows":84,"truly_yes":0.976}
```

`--truth /POINTER=/POINTER` compares two fields of the case exactly, with no judgment involved. A benchmark where a candidate's own label meets a gold label uses that form. The check is keyed by its left pointer, so `--truth /gold_code=/output_code` prints under `/gold_code`.

```json
{"kind":"exact","matched":5,"rows":6,"accuracy":0.8333333333333334}
```

`--baseline` adds one `baseline` object beside `checks`. `only_baseline` and `only_run` name the cases that one file holds and the other does not, sorted. `flipped` names the cases whose value changed, and `delta` holds this run's metric minus the baseline's for every metric both carry.

```json
{"rows":6,"only_baseline":["E-04"],"only_run":[],"checks":{"correct":{"flipped":{"to_yes":["E-02","E-06"],"to_no":["E-01"]},"delta":{"coverage":0,"accuracy_resolved":-0.1,"f1":-0.08}}}}
```

A case named in `only_baseline` is a case the second file does not hold. The report says that and nothing more. It may have been refused, dropped upstream, or never judged, and none of those is a result.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `RUN` | The file of rows | Standard input |
| `--truth NAME=POINTER` | Scores the check `NAME` against a label in the case. Repeatable | None, and no scoring prints |
| `--threshold NAME=RULE` | Applies another rule to the stored probabilities. No request is made | The rule each row was judged at |
| `--baseline RUN` | Compares this run with an earlier one | None |
| `--id POINTER` | Matches cases between the two runs | `/id` |

`NAME=` may be left out of `--truth` and `--threshold` when the run holds one check, as a run made by `decide --jsonl --details` does. A left side that carries no `=` is the pointer or the rule alone.

`report` takes no `--details`, no `--dry-run`, and no backend option.

Rows that share an id inside one run are repeated trials. A metric averages within a case first, so a case tried five times weighs the same as a case tried once.

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

- A sweep tries the nineteen cuts from 0.05 to 0.95 in steps of 0.05, and it tries no band. Bands wait for a demand.
- Does a sweep also try the backend's `confidence`? Proposed ADR 0009 asks for both to be swept against labels. Recommendation: sweep both and name each column, then look at the `choose` rule again.
- A bare-verb run is one check named after the verb, so `--threshold decide=0.82` names a word the user never typed. Demo 13 asks for the check to be named after its question text instead. Recommendation: keep the verb name, because `NAME=` may be left out on a single-check run, and let the `warnings` entry catch a file that holds two texts.
