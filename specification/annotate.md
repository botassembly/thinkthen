# `annotate`

Status: **Settled** for the file grammar, the output, and several pointers on `on`. ADR 0010 accepted the pointers and struck structured question values from version one.

Asks a saved question set about each record and adds one field per question.

```text
thinkthen annotate FILE [--lines|--jsonl] [--field POINTER] [--details] [--dry-run] [BACKEND]
```

## What it reads

`FILE` is the saved question set. It holds several named questions, and each entry has the shape of a question file. `annotate` reads one document on standard input by default, and it reads records under `--lines` or `--jsonl`. `--input FILE` reads the evidence from a file. [records.md](records.md) gives the framing and the pointer rules.

## The question set

```json
{
  "version": 1,
  "threshold": "0.1:0.9",
  "questions": {
    "unresolved": {"decide": "Does this report a failure that is still unresolved?", "true": "The report names a failure that is still happening.", "false": "Anything else.", "threshold": "0.1:0.9", "on": "/body"},
    "kind": {"choose": "Which kind of request is this?", "options": {"bug": "Reports broken behavior.", "feature": "Asks for new behavior.", "other": "Neither fits."}, "threshold": 0.8},
    "impact": {"score": "How much disruption does this report?", "levels": ["None.", "Work continues with a workaround.", "Work is blocked."]}
  }
}
```

Each entry has the shape of one question file, and [question-file.md](question-file.md) holds that grammar once. A question has exactly one of `decide`, `choose`, or `score`, and its value is the question text. A `decide` question takes `true` and `false`, the two texts that say what each side means. `options` is a list of labels or a map from label to description. `levels` is a list, lowest first. `threshold` follows the command-line rule for its verb, so a `choose` question takes a single cut alone. A question name uses lowercase letters, digits, and underscores. An unknown key anywhere in the file is an error.

The top-level `threshold` applies to every `decide` question that names none. It is the only key allowed beside `version` and `questions`. The set holds questions and nothing else. It holds no backend, no output path, and no format. An exact check beyond equality is a `jq` field on the record, by ADR 0008 item 6.

`on` is a JSON Pointer inside the evidence that `--field` selected. It can never reach outside that evidence. A table asks different questions of different columns, and no question should see a column it does not need.

### Several pointers on `on`

Settled by ADR 0008 item 2, accepted in ADR 0010. `on` takes one pointer or several. Several pointers build an evidence object, each member keyed by the last part of its pointer, and two members that would share one key are an error in the file. A correctness check names `/input`, `/gold`, and `/output`. A grounding check beside it names `/context` and `/output` and never sees the gold answer.

```json
{"grounded": {"decide": "Is every claim in the output supported by the context?", "on": ["/context", "/output"]}}
```

## What it prints

One JSON object per record. An object record gains one top-level field per question, so a chain of judgments stays flat. Any other record, a text document included, yields an object of the named answers alone.

An unresolved answer is `null`. A backend failure is never `null`, because a failure ends the run.

```json
{"id":"T-91","body":"Payouts have failed for 3 days.","unresolved":true,"kind":"bug","impact":1.6}
```

`--details` prints `input`, `value`, `answers`, and `meta`, as [result.md](result.md) gives them.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--lines` or `--jsonl` | Reads a record stream in place of one document | One document |
| `--field POINTER` | The part of each record the questions see. An `on` pointer works inside it | The whole record |
| `--details` | Prints the full result object per record | Off |
| `--input FILE` | Reads the evidence from a file | Standard input |
| `--dry-run` | Checks the file, prints the plan, and sends nothing. See below | Off |
| Backend options | `--url` and `--model`, in the long help alone. See [backends.md](backends.md) | The two variables and `jev-latest` |

`annotate` takes no `--threshold`, no `--quiet`, and no `--raw`. A question carries its own threshold.

## Exit codes

0 when the run finished, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. A question name that the record already holds is an input error for that record, at exit 2, before any request for it. An unreadable or invalid file is exit 5.

## Examples

```sh
thinkthen annotate triage.json < issue.txt
```

```sh
thinkthen annotate triage.json --jsonl --field /body < issues.jsonl | jq -c 'select(.kind == "bug")'
```

```sh
thinkthen annotate triage.json --dry-run
```

## `--dry-run`

`--dry-run` validates the file and sends nothing. It needs no key. Empty evidence succeeds and prints nothing. With evidence it prints the first request and an `on` object that names the normalized pointers for every question.

One record makes one request per distinct `on`, and the plan shows one request. It is the first record's first `on` set, taking the questions in file order. The plan's `input` object names the framing and, under `on`, the pointers of every question, so a reviewer sees what each check would see and not only the check that the plan printed.

```json
{"framing":"jsonl","on":{"correct":["/input","/gold","/output"],"grounded":["/context","/output"]}}
```

`--dry-run` reads the question set and the first record. A name that collides with a field on record two is invisible to it, as [demo 07](../demos/07-judged-columns/) shows.

## Requests

One record makes one request for each distinct `on`. Every question with the same evidence rides in that one request, and the evidence is billed once. The project measured forty clear yes-or-no questions over 120 cases. Packed requests changed no answer and used 20.8 times fewer billed input tokens than separate requests. A second measurement packed one decision, one choice, and one score. The values stayed the same, and billed input fell from 915 tokens across three requests to 371 in one request. Neither measurement found a lower question-count limit.

Records never share a request. A question never sees another question's answer. Work that depends on an earlier answer is a second command.

`--details` reports `meta.usage` as the sum over the record's requests, and each answer carries the digest of the request that produced it. [result.md](result.md) gives the shape.

## Cautions

The questions and the evidence together can pass the backend's token limit for one request. The backend refuses, and the exit code is 4. Fewer questions per file is the answer.

The cache key covers the whole request group. Adding or changing one question asks the whole group again for every record. An answer near its threshold can move when neighboring questions change. In six deliberately borderline cases, one answer moved from `false` to unresolved when neighboring questions joined it. The largest probability shift was 0.04. Keep the group fixed while comparing runs and retain `--details` probabilities. Use a narrower, distinct `on` group when the record permits it and the questions need separate stability.

## An eval is `annotate` and a saved run

Settled by ADR 0008 item 1. A case is one flat JSON object with any field names. A definition is a question set. A run is the file of `--details` rows. `jq` reads the run and counts it. No eval engine enters this tool.

```sh
thinkthen annotate checks.json --jsonl --record runs/v1 --replay runs/v1 < cases.jsonl > run-v1.jsonl
jq -s '[.[] | select(.value.correct)] | length' run-v1.jsonl
```

ADR 0010 holds the `report` command out of version one and puts the `jq` transforms first. `specification/roadmap.md` records the five outcomes and the test between them.

A missing field and a failed request stop the run, so a completed run holds a judgment for every case. A rerun on the same recording folder answers the finished cases from disk.
