# `filter`

Status: **Settled** for version one, by ADR 0007.

Keeps the records whose evidence reaches the mark.

```text
thinkthen filter QUESTION [--lines|--jsonl|--csv|--tsv] [--threshold T] [--field POINTER] [--details] [BACKEND]
```

`QUESTION` is the question text, or `@` and the path of a question file holding one `decide` question. [question-file.md](question-file.md) gives the grammar and the precedence, and `--true TEXT` and `--false TEXT` say what a yes and a no mean, exactly as they do on `decide`.

## What it reads

A stream of records. With no framing flag it reads lines, or JSON Lines when a pointer names part of each record. The pointer comes from `--field` or from a question file's `on`. `--lines`, `--jsonl`, `--csv`, or `--tsv` names the framing outright. `--input FILE` reads a file instead of standard input. A blank text line is skipped, as [records.md](records.md) gives. That page also gives the pointer rules and the order.

`QUESTION` is one argument. It states a fact that is true or false of each record.

By default a stream of records shares requests, filling each to the backend's limits. The evidence of a batch is one fixed sentence, and each record appears once, inside its own question. Every run moves a few answers, and batching moves a few more. ADR 0055 records local experiment 275, which asked four yes/no questions over the 306 Beatles songs. It ran each batched form three times on the same bytes and once on each of three shuffled record orders. The batched form stayed within 4 right answers across repeats and orders on every task. It never fell more than 3 right answers below one song a request. On "It appears on the album Abbey Road" it scored 283 to 287 right of 306, where one title a request scored 286. It sent 11,468 input tokens for the 306 titles, where one title a request sent 88,933. Its one measured loss came on "It was released before 1965", against the earlier batch form, which listed every record in the evidence. That form scored 276 to 283 right in the table's own order and 258 to 270 over shuffled orders. The quoted form scored 255 to 259, and one title a request scored 253. `--batch 1` asks one record a request and sends the requests the tool sent before batching.

## What it prints

Each kept line or JSONL record prints as it arrived, in input order. A kept CSV or TSV row prints as one compact JSON object in header order. A record that did not reach the mark prints nothing.

`--details` prints the object in [result.md](result.md) for each kept record, in input order. It does not change which records `filter` keeps. A pipeline that needs every record with its answer uses `decide --details` and splits with `jq`.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--threshold T` | A single cut on the probability of yes. The band form is a usage error. See [threshold.md](threshold.md) | `0.5` |
| `--lines`, `--jsonl`, `--csv`, or `--tsv` | The framing | `--lines`, or `--jsonl` when a pointer is given |
| `--field POINTER` | The part of each record the model sees. See [records.md](records.md) | The whole record |
| `--details` | Prints one result object per kept record in place of the kept records | Off |
| `--input FILE` | Reads the records from a file | Standard input |
| `--true TEXT`, `--false TEXT` | What a yes and a no mean, sent beside the question | No text |
| `--dry-run` | Prints the plan for the first record and sends nothing | Off |
| Record options | `--jobs N`, `--record DIR`, `--replay DIR`, `--cache DIR`, as [records.md](records.md) and [recording.md](recording.md) give them | `--jobs 4` |
| Backend options | `--url` in short and long help, and `--model` in long help. See [backends.md](backends.md) | The two variables and `jev-1.13.0` |

`filter` takes no `--quiet`, no `--raw`, and no `--top`. Each is refused by name. `--top` belongs to `rank`, because `filter` keeps records and has no order to cut.

A band is refused wherever it came from. A band typed on the command line is a usage error at exit 2, and a band a question file holds is a local failure at exit 5, which is the rule [question-file.md](question-file.md) already fixes for every value.

A line or JSONL record that `filter` keeps is written back as it arrived: nothing is re-encoded, odd spacing and a trailing space survive, and the line ending is written as a line feed. A CSV or TSV row is written as a compact JSON object in header order. A run that stops at a failed record has already printed a prefix of its output.

The request, the result object, and the recording entry are those of `decide`, so a recording made by `decide` over the same records replays here.

## Exit codes

0 when the run finished, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. No record's answer sets the exit code. An empty line or JSONL stream exits 0 with no output and no request. Empty CSV and TSV inputs exit 2 because the required header is missing.

## Examples

```sh
thinkthen filter 'This describes a reproducible bug.' --jsonl --field /body < issues.jsonl
```

```sh
thinkthen filter 'This mentions an unresolved action.' --threshold 0.9 < notes.txt
```

```sh
thinkthen decide 'This reports a payment failure.' --jsonl --field /body --details < tickets.jsonl |
  jq -c 'select(.answer.probability < 0.9)'
```

## Cautions

`filter` takes a single cut only. A band would force a third pile and a flag to steer it. A user who wants three piles runs `decide --jsonl --details` and splits with `jq`.

A record that `filter` leaves out failed to reach the mark. It is not a record the model called false. [threshold.md](threshold.md) says more.

A per-record judgment is a paid request. A loop over a folder of files sits outside any budget this tool keeps. `find -print0 | xargs -0 -n 1 -P 4` runs one process per file, and each process has its own run.
