# `filter`

Status: **Settled** for version one, by ADR 0007.

Keeps the records whose evidence reaches the mark and prints them unchanged.

```text
thinkthen filter QUESTION (--lines|--jsonl) [--threshold T] [--field POINTER] [--details] [BACKEND]
```

## What it reads

A stream of records. `filter` requires `--lines` or `--jsonl`, because one document is not a stream. `--input FILE` reads a file instead of standard input. [records.md](records.md) gives the framing, the pointer rules, and the order.

`QUESTION` is one argument. It states a fact that is true or false of each record.

## What it prints

Each kept record, byte for byte as it arrived, in input order. A record that did not reach the mark prints nothing. The count of records left out prints on standard error at the end of the run.

`--details` prints the object in [result.md](result.md) for every record, kept or not, in input order. A pipeline that must keep every record uses `--details` and splits with `jq`.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--threshold T` | A single cut on the probability of yes. The band form is a usage error. See [threshold.md](threshold.md) | `0.5` |
| `--lines` or `--jsonl` | The framing. One of the two is required | None. Its absence is a usage error |
| `--field POINTER` | The part of each record the model sees. See [records.md](records.md) | The whole record |
| `--details` | Prints one result object per record in place of the kept records | Off |
| `--input FILE` | Reads the records from a file | Standard input |
| `--dry-run` | Prints the plan for the first record and sends nothing | Off |
| Backend options | `--profile` and the advanced flags | The selected profile |

`filter` takes no `--quiet` and no `--raw`.

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
