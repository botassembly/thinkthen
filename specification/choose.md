# `choose`

Status: **Settled** for version one, by ADR 0007.

Picks one label from a fixed list.

```text
thinkthen choose QUESTION OPTION... [--threshold T] [--raw] [--quiet] [--details] [RECORD] [BACKEND]
```

## What it reads

One text document on standard input, read to its end as UTF-8. `--input FILE` reads a file instead. `--lines` and `--jsonl` turn the input into records, and [records.md](records.md) gives the rules. An empty document is a usage error.

`QUESTION` comes first and states what decides the pick. Each `OPTION` is one argument. `choose` takes 2 to 255 options. A duplicate option name is a usage error. The tool sends the options in the order the user gave and never reorders them.

## What it prints

A JSON string, or `null` when the answer is unresolved. `--raw` prints the label without its quotation marks and prints nothing for `null`, as `jq -r` does. `--details` prints the object in [result.md](result.md), with an `answer.kind` of `choice`. In record mode one value prints per record, in input order.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--threshold T` | A single cut on the winning option's probability. The band form is a usage error. See [threshold.md](threshold.md) | None. The winning label is returned |
| `--raw` | Prints the label without quotation marks | Off |
| `--quiet` | Prints nothing on standard output | Off |
| `--details` | Prints the full result object | Off |
| `--dry-run` | Prints the plan and sends nothing | Off |
| Record options | `--input`, `--lines`, `--jsonl`, `--field` | One document |
| Backend options | `--profile` and the advanced flags | The selected profile |

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | Single input: a label was returned. Record mode: the run finished |
| 2 | Usage error |
| 3 | Single input only: the answer is unresolved |
| 4, 5, 70 | As [channels.md](channels.md) gives them |

`choose` never exits 1. A pick is not a two-sided decision.

## Unresolved

The answer is unresolved when the winning option's probability falls under the cut, and when the top two options tie exactly. An exact tie is unresolved with or without a threshold, because alphabetical order is no evidence.

## Examples

```sh
thinkthen choose 'Which kind of request is this?' bug feature question other < issue.txt
```

```sh
case "$(thinkthen choose 'Which team owns this request?' billing shipping account --threshold 0.8 --raw < message.txt)" in
  billing) route billing ;;
  "") route triage ;;
esac
```

```sh
thinkthen choose 'Which kind of request is this?' bug feature other --jsonl --field /body --details < issues.jsonl
```

## Cautions

`choose` never runs the option it picks. A label is a string that the next program reads.

Measurement of the first decider model found picking from a fixed list stable. Reversing the option order changed none of fifty picks. The vendor's own documents still say that option order and added irrelevant options shift the odds. A short list of options that exclude one another gives the steadiest answer, and a run with a changed list is a different measurement.

`--raw` prints nothing for an unresolved answer, so a shell `case` matches the empty string. Without `--raw` the four characters `null` reach the shell, and a `case` that matches them is matching a spelling accident.
