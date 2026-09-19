# `rank`

Status: **Settled** for version one, by ADR 0007.

Prints the records in order of the probability of yes.

```text
thinkthen rank QUESTION (--lines|--jsonl) [--top N] [--field POINTER] [--details] [BACKEND]
```

## What it reads

A stream of records. `rank` requires `--lines` or `--jsonl`. `--input FILE` reads a file instead of standard input. [records.md](records.md) gives the framing and the pointer rules.

`QUESTION` is one argument. The tool asks it of each record as a yes/no question. It is the question text, or `@` and the path of a question file holding one `decide` question, and `--true TEXT` and `--false TEXT` say what a yes and a no mean, exactly as they do on `decide`.

## What it prints

Each record as it arrived, most likely yes first. Ties keep input order. `--details` prints the object in [result.md](result.md) for the same records in the same order.

`rank` holds every record until the input ends, because a final order needs the whole set. An endless stream has to be cut into windows upstream.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--top N` | Prints the first `N` records of the order. It saves no requests, because every record is judged before anything is sorted | All records |
| `--lines` or `--jsonl` | The framing. One of the two is required | None. Its absence is a usage error |
| `--field POINTER` | The part of each record the model sees | The whole record |
| `--details` | Prints one result object for each record it prints | Off |
| `--input FILE` | Reads the records from a file | Standard input |
| `--true TEXT`, `--false TEXT` | What a yes and a no mean, sent beside the question | No text |
| `--dry-run` | Prints the plan for the first record and sends nothing | Off |
| Record options | `--jobs N`, `--record DIR`, `--replay DIR`, `--cache DIR`, as [records.md](records.md) and [recording.md](recording.md) give them | `--jobs 4` |
| Backend options | `--url` and `--model`, in the long help alone. See [backends.md](backends.md) | The two variables and `jev-latest` |

`rank` takes no `--threshold`, no `--quiet`, and no `--raw`. Each is refused by name, and the message says which command carries it. A rule is refused in both homes, so a question file holding a `threshold` is refused too, at exit 5. A missing framing is a usage error.

`--top N` takes a whole number of 1 or more. `--top 0` prints nothing and is a usage error.

A record is written back as it arrived: nothing is parsed, nothing is re-encoded, and the line ending is written as a line feed. A run that stops at a failed record has printed nothing at all, and the line on standard error says so.

The request, the result object, and the recording entry are those of `decide`, so a recording made by `decide` over the same records replays here. Every ranked row carries `threshold: null` and `value: null`, because `rank` reads no rule and makes no selection. The probability the order came from is under `answer`.

## Exit codes

0 when the run finished, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. An empty record stream exits 0 with no output and no request.

## Examples

```sh
thinkthen rank 'This helps diagnose the login timeout.' --jsonl --field /body --top 5 < passages.jsonl
```

```sh
thinkthen filter 'This describes a reproducible bug.' --jsonl --field /body < issues.jsonl |
  thinkthen rank 'This affects many users.' --jsonl --field /body --top 10
```

## Ranking against a query

The question is fixed for the run, so a query that changes per run belongs in the record. `jq` builds one record per passage carrying both the query and the passage, and `--field` names both. [records.md](records.md) gives the several-pointer form.

```sh
jq -c --arg q "$QUERY" '{query: $q, passage: .}' passages.jsonl |
  thinkthen rank 'The passage answers the query.' --jsonl --field /query --field /passage --top 5
```

The question stays the same for every record, so one run is one measurement.

## Cautions

The method is fixed and printed in the help. The tool asks one yes/no question of each record, sorts the records by the probability of yes, and breaks exact ties by input order. It never compares two records in one question, and it never runs a tournament.

`rank` takes no rubric. Ordering by the probability of yes follows the vendor's own reranking method.

`rank` orders and never selects. A user who wants a floor runs `filter` first, as the second example shows.
