# `filter`

Status: **Settled** for version one, by ADR 0007.

Keeps the records whose evidence reaches the mark.

```text
thinkthen filter QUESTION (--lines|--jsonl|--csv|--tsv) [--threshold T] [--field POINTER] [--details] [BACKEND]
```

`QUESTION` is the question text, or `@` and the path of a question file holding one `decide` question. [question-file.md](question-file.md) gives the grammar and the precedence, and `--true TEXT` and `--false TEXT` say what a yes and a no mean, exactly as they do on `decide`.

## What it reads

A stream of records. `filter` requires `--lines`, `--jsonl`, `--csv`, or `--tsv`, because one document is not a stream. `--input FILE` reads a file instead of standard input. [records.md](records.md) gives the framing, the pointer rules, and the order.

`QUESTION` is one argument. It states a fact that is true or false of each record.

## What it prints

Each kept line or JSONL record prints as it arrived, in input order. A kept CSV or TSV row prints as one compact JSON object in header order. A record that did not reach the mark prints nothing.

`--details` prints the object in [result.md](result.md) for every record, kept or not, in input order. A pipeline that must keep every record uses `--details` and splits with `jq`.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--threshold T` | A single cut on the probability of yes. The band form is a usage error. See [threshold.md](threshold.md) | `0.5` |
| `--lines`, `--jsonl`, `--csv`, or `--tsv` | The framing. One of the four is required | None. Its absence is a usage error |
| `--field POINTER` | The part of each record the model sees. See [records.md](records.md) | The whole record |
| `--details` | Prints one result object per record in place of the kept records | Off |
| `--input FILE` | Reads the records from a file | Standard input |
| `--true TEXT`, `--false TEXT` | What a yes and a no mean, sent beside the question | No text |
| `--dry-run` | Prints the plan for the first record and sends nothing | Off |
| Record options | `--jobs N`, `--record DIR`, `--replay DIR`, `--cache DIR`, as [records.md](records.md) and [recording.md](recording.md) give them | `--jobs 4` |
| Backend options | `--url` and `--model`, in the long help alone. See [backends.md](backends.md) | The two variables and `jev-latest` |

`filter` takes no `--quiet` and no `--raw`. Each is refused by name, and the message says which command carries that view. A missing framing is a usage error, because one document is not a stream.

A band is refused wherever it came from. A band typed on the command line is a usage error at exit 2, and a band a question file holds is a local failure at exit 5, which is the rule [question-file.md](question-file.md) already fixes for every value.

A line or JSONL record that `filter` keeps is written back as it arrived: nothing is re-encoded, odd spacing and a trailing space survive, and the line ending is written as a line feed. A CSV or TSV row is written as a compact JSON object in header order. A run that stops at a failed record has already printed a prefix of its output.

The request, the result object, and the recording entry are those of `decide`, so a recording made by `decide` over the same records replays here.

## Exit codes

0 when the run finished, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. No record's answer sets the exit code. An empty record stream exits 0 with no output and no request.

## Examples

```sh
thinkthen filter 'This describes a reproducible bug.' --jsonl --field /body < issues.jsonl
```

```sh
thinkthen filter 'This mentions an unresolved action.' --lines --threshold 0.9 < notes.txt
```

```sh
thinkthen filter 'This reports a payment failure.' --jsonl --field /body --details < tickets.jsonl |
  jq -c 'select(.answer.probability < 0.9)'
```

## Cautions

`filter` takes a single cut only. A band would force a third pile and a flag to steer it. A user who wants three piles runs `decide --jsonl --details` and splits with `jq`.

A record that `filter` leaves out failed to reach the mark. It is not a record the model called false. [threshold.md](threshold.md) says more.

A per-record judgment is a paid request. A loop over a folder of files sits outside any budget this tool keeps. `find -print0 | xargs -0 -n 1 -P 4` runs one process per file, and each process has its own run.
