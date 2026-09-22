# `annotate`

Status: **Settled** for the file grammar, the output, and several pointers on `on`. ADR 0010 accepted the pointers and struck structured question values from version one; ADR 0039 amends that exclusion.

Asks a saved question set about each record and adds one field per question.

```text
thinkthen annotate FILE [--lines|--jsonl|--csv|--tsv] [--field POINTER] [--details] [--dry-run] [BACKEND]
```

## What it reads

`FILE` is the saved question set. It holds several named questions, and each entry has the shape of a question file. `annotate` reads one document on standard input by default, and it reads records under `--lines`, `--jsonl`, `--csv`, or `--tsv`. `--input FILE` reads the evidence from a file. [records.md](records.md) gives the framing and the pointer rules.

## The question set

```json
{
  "version": 1,
  "threshold": "0.1:0.9",
  "profile": "jev",
  "questions": {
    "unresolved": {"decide": "Does this report a failure that is still unresolved?", "true": "The report names a failure that is still happening.", "false": "Anything else.", "threshold": "0.1:0.9", "on": "/body"},
    "kind": {"choose": "Which kind of request is this?", "options": {"bug": "Reports broken behavior.", "feature": "Asks for new behavior.", "other": "Neither fits."}, "threshold": 0.8},
    "impact": {"score": "How much disruption does this report?", "levels": ["None.", "Work continues with a workaround.", "Work is blocked."]}
  }
}
```

Each entry has the shape of one question file, and [question-file.md](question-file.md) holds that grammar once. A question has exactly one of `decide`, `choose`, `tag`, or `score`, and its value is the question text, as a string, an object, or a list. A `decide` question takes `true` and `false`, the two values that say what each side means. `options` and `labels` are lists or ordered maps from label to description. `levels` is a list, lowest first, or an ordered map from each name to its description. `threshold` follows the command-line rule for its verb, so a `choose` question takes a single cut alone. A question name is not empty and uses only lowercase letters, digits, and underscores. An unknown key anywhere in the file is an error. A set without the required `questions` object says that the wrapper is missing before it reports any unknown top-level key.

The top-level `threshold` applies to every `decide` question that names none. The optional top-level `profile` names the backend profile used to calibrate the set's thresholds. `version`, `threshold`, `profile`, and `questions` are the only top-level keys. A nested question cannot carry another profile. The set holds no backend address, model, output path, or format. An exact check beyond equality is a `jq` field on the record, by ADR 0008 item 6.

`on` is a JSON Pointer inside the evidence that `--field` selected. It can never reach outside that evidence. A table asks different questions of different columns, and no question should see a column it does not need. The selection follows the `state` rule of [records.md](records.md): an object or a list travels as that JSON value, and several pointers send one ordered object.

### Several pointers on `on`

Settled by ADR 0008 item 2, accepted in ADR 0010. `on` takes one pointer or several. Several pointers build an evidence object, each member keyed by the last part of its pointer, and two members that would share one key are an error in the file. A correctness check names `/input`, `/gold`, and `/output`. A grounding check beside it names `/context` and `/output` and never sees the gold answer.

```json
{"grounded": {"decide": "Is every claim in the output supported by the context?", "on": ["/context", "/output"]}}
```

## What it prints

One JSON object per record. In record mode an object record gains one top-level field per question, so a chain of judgments stays flat. A line, JSON scalar, or JSON array keeps its parsed record under `input` and its named answer object under `value`. CSV and TSV rows are objects and stay flat. On one document, an object still gains the answers and every other JSON shape or text document still yields the named answers alone.

An unresolved answer is `null`. A failed question is a failure marker and never `null`. When the same reply contains a usable answer, the good answer and failed marker both print and the completed run exits 6. A reply with no usable answer ends the run at exit 4.

```json
{"id":"T-91","body":"Payouts have failed for 3 days.","unresolved":true,"kind":"bug","impact":1.6}
```

```json
{"input":"Payouts have failed for 3 days.","value":{"unresolved":true,"kind":"bug","impact":1.6}}
```

`--details` prints `input`, `value`, `answers`, and `meta`, as [result.md](result.md) gives them.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--lines`, `--jsonl`, `--csv`, or `--tsv` | Reads a record stream in place of one document | One document |
| `--field POINTER` | The part of each record the questions see. An `on` pointer works inside it | The whole record |
| `--details` | Prints the full result object per record | Off |
| `--input FILE` | Reads the evidence from a file | Standard input |
| `--dry-run` | Checks the file, prints the plan, and sends nothing. See below | Off |
| `--profile FILE` | Applies explicit local backend limits and names the running calibration profile. See [backends.md](backends.md) | None |
| Backend options | `--url` in short and long help, and `--model` in long help. See [backends.md](backends.md) | The two variables and `jev-latest` |

`annotate` takes no `--threshold`, no `--quiet`, and no `--raw`. A question carries its own threshold.

## Exit codes

0 when the run finished with every question answered, 6 when it finished after one or more logical questions failed, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. A question name that the record already holds is an input error for that record, at exit 2, before any request for it. An unreadable or invalid file is exit 5. A later whole-run failure keeps its own code and stop boundary.

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

`--dry-run` validates the file and sends nothing. It needs no key. An empty document is a usage error. An empty line or JSONL stream succeeds and prints nothing. An empty CSV or TSV input fails because its required header is missing. With evidence it prints the first request and an `on` object that names the normalized pointers for every question.

One record makes one request per distinct `on`, and the plan shows one request. It is the first record's first `on` set, taking the questions in file order. The plan's `input` object names the framing and, under `on`, the pointers of every question, so a reviewer sees what each check would see and not only the check that the plan printed.

```json
{"framing":"jsonl","on":{"correct":["/input","/gold","/output"],"grounded":["/context","/output"]}}
```

`--dry-run` reads the question set and the first record. A name that collides with a field on record two is invisible to it. [How-to 16](../demos/16-triage-pipeline/) keeps the collision caution beside its table-input pipeline.

## Requests

One record makes one request for each distinct `on`. Every question with the same evidence rides in that one request, and the evidence is billed once. The project measured forty clear yes-or-no questions over 120 cases. Packed requests changed no answer and used 20.8 times fewer billed input tokens than separate requests. A second measurement packed one decision, one choice, and one score. The values stayed the same, and billed input fell from 915 tokens across three requests to 371 in one request. Neither measurement found a lower question-count limit.

Records never share a request. A question never sees another question's answer. Work that depends on an earlier answer is a second command.

`--details` reports `meta.usage` as the sum over the record's requests. `meta.requests` lists their recording digests in question-set group order. Each answer also carries the digest of the request that produced it. [result.md](result.md) gives the shape.

A failed bare value is `{"failed":{"kind":"backend","cause":CAUSE}}`. In detailed output its entry carries `question`, `failure`, and `request`, and carries no `value`, `answer`, or `threshold`. `meta.failed_questions` counts failed logical questions. It is always present, including zero. `null` means not sure and never means failed.

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
