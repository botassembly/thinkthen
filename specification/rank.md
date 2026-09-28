# `rank`

Status: **Settled** for version one, by ADR 0007.

Prints the records in order of the probability of yes.

```text
thinkthen rank QUESTION [--lines|--jsonl|--csv|--tsv] [--top N] [--field POINTER] [--details] [BACKEND]
```

## What it reads

A stream of records. With no framing flag it reads lines, or JSON Lines when a pointer names part of each record. The pointer comes from `--field` or from a question file's `on`. `--lines`, `--jsonl`, `--csv`, or `--tsv` names the framing outright. `--input FILE` reads a file instead of standard input. A blank text line is skipped, as [records.md](records.md) gives. That page also gives the pointer rules.

`QUESTION` is one argument. The tool asks it of each record as a yes/no question. It is the question text, or `@` and the path of a question file holding one `decide` question, and `--true TEXT` and `--false TEXT` say what a yes and a no mean, exactly as they do on `decide`.

By default a stream of records shares requests, filling each to the smaller of the request-size setting and a backend profile limit. `--max-request-bytes N` sets that size for this command, with `THINKTHEN_MAX_REQUEST_BYTES` next and 96,000 bytes at every address by default; see [settings.md](settings.md). The evidence of a batch is one fixed sentence, and each record appears once, inside its own question. Every run moves a few answers, and batching moves a few more. ADR 0055 records local experiment 275, which asked four yes/no questions over the 306 Beatles songs. It ran each batched form three times on the same bytes and once on each of three shuffled record orders. The batched form stayed within 4 right answers across repeats and orders on every task. It never fell more than 3 right answers below one song a request. On "It appears on the album Abbey Road" it scored 283 to 287 right of 306, where one title a request scored 286. It sent 11,468 input tokens for the 306 titles, where one title a request sent 88,933. Its one measured loss came on "It was released before 1965", against the earlier batch form, which listed every record in the evidence. That form scored 276 to 283 right in the table's own order and 258 to 270 over shuffled orders. The quoted form scored 255 to 259, and one title a request scored 253. `--batch 1` asks one record a request and sends the requests the tool sent before batching.

## What it prints

Each line or JSONL record as it arrived, and each CSV or TSV row as a compact JSON object, most likely yes first. Ties keep input order. `--details` prints the object in [result.md](result.md) for the same records in the same order.

`rank` holds every record until the input ends, because a final order needs the whole set. An endless stream has to be cut into windows upstream.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--top N` | Prints the first `N` records of the order. It saves no requests, because every record is judged before anything is sorted | All records |
| `--lines`, `--jsonl`, `--csv`, or `--tsv` | The framing | `--lines`, or `--jsonl` when a pointer is given |
| `--field POINTER` | The part of each record the model sees | The whole record |
| `--details` | Prints one result object for each record it prints | Off |
| `--context FILE` | Uses the file's text once as shared evidence in each record batch; see [records.md](records.md) | None |
| `--input FILE` | Reads the records from a file | Standard input |
| `--true TEXT`, `--false TEXT` | What a yes and a no mean, sent beside the question | No text |
| `--dry-run` | Prints the plan for the first record and sends nothing | Off |
| Record options | `--jobs N`, `--record DIR`, `--replay DIR`, `--cache DIR`, as [records.md](records.md) and [recording.md](recording.md) give them | `--jobs 4` |
| Backend options | `--url` in short and long help, and `--model` in long help. See [backends.md](backends.md) | The two variables and `jev-1.13.0` |

`rank` takes no `--threshold`, no `--quiet`, and no `--raw`. Each is refused by name, and the message says which command carries it. A rule is refused in both homes, so a question file holding a `threshold` is refused too, at exit 5.

`--top N` takes a whole number of 1 or more. `--top 0` prints nothing and is a usage error.

A line or JSONL record is written back as it arrived: nothing is re-encoded, and the line ending is written as a line feed. A CSV or TSV row is written as a compact JSON object in header order. A run that stops at a failed record has printed nothing at all, and the line on standard error says so.

The request, the result object, and the recording entry are those of `decide`, so a recording made by `decide` over the same records replays here. Every ranked row carries `threshold: null` and `value: null`, because `rank` reads no rule and makes no selection. The probability the order came from is under `answer`.

## Exit codes

0 when the run finished, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. An empty line or JSONL stream exits 0 with no output and no request. Empty CSV and TSV inputs exit 2 because the required header is missing.

## Examples

```sh
thinkthen rank 'This helps diagnose the login timeout.' --jsonl --field /body --top 5 < passages.jsonl
```

```sh
thinkthen rank 'It appears on the album Abbey Road.' --context catalog.txt < songs.txt
```

Ticket 0172's [live record](../sdlc/records/2026-09-27-0172-shared-context-build.md) used `decide` to expose all 306 answers for audit under this question and a 0.7 cut. Each of three shared-catalog runs sent one request, scored 306 right with no false yeses or misses, and reported 19,634 input and 5,710 output tokens. This measures the shared request form, not a separate `rank` order.

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
