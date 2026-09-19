# `annotate`

Status: **Settled** for the file grammar and the output. **Draft** for how questions group into requests.

Asks a saved file of questions about each record and adds one field per question.

```text
thinkthen annotate FILE [--lines|--jsonl] [--field POINTER] [--details] [--dry-run] [BACKEND]
```

## What it reads

`FILE` is the saved question file. `annotate` reads one document on standard input by default, and it reads records under `--lines` or `--jsonl`. `--input FILE` reads the evidence from a file. [records.md](records.md) gives the framing and the pointer rules.

## The file

```json
{
  "version": 1,
  "questions": {
    "unresolved": {"decide": "Does this report a failure that is still unresolved?", "threshold": "0.1:0.9", "on": "/body"},
    "kind": {"choose": "Which kind of request is this?", "options": {"bug": "Reports broken behavior.", "feature": "Asks for new behavior.", "other": "Neither fits."}, "threshold": 0.8},
    "impact": {"score": "How much disruption does this report?", "levels": ["None.", "Work continues with a workaround.", "Work is blocked."]}
  }
}
```

A question has exactly one of `decide`, `choose`, or `score`, and its value is the question text. `options` is a list of labels or a map from label to description. `levels` is a list, lowest first. `threshold` follows the command-line rule for its verb, so a `choose` question takes a single cut alone. A question name uses lowercase letters, digits, and underscores. An unknown key anywhere in the file is an error.

`on` is a JSON Pointer inside the evidence that `--field` selected. It can never reach outside that evidence. A table asks different questions of different columns, and no question should see a column it does not need.

The file holds questions and nothing else. It holds no profile, no output path, and no format.

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
| `--dry-run` | Checks the file and sends nothing | Off |
| Backend options | `--profile` and the advanced flags | The selected profile |

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

## Cautions

A question never sees another question's answer. Work that depends on an earlier answer is a second command.

The questions and the evidence together can pass the backend's token limit for one request. The backend refuses, and the exit code is 4. Fewer questions per file is the answer.

## Open points

- ADR 0007 says that each record is its own request and also that questions with the same `on` share one request. A record with three distinct `on` pointers therefore makes three requests. Recommendation: read the second rule as the specific one. One record makes one request per distinct `on`, and records never share a request.
- Does `--dry-run` print a plan as well as checking the file? Recommendation: it prints the plan for the first record when evidence is available, and it prints only the check when no evidence is given.
