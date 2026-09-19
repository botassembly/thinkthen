# `choose`

Status: **Settled** for version one, by ADR 0007 and ADR 0010.

Picks one label from a fixed list.

```text
thinkthen choose QUESTION OPTION... [--threshold T] [--raw] [--quiet] [--details] [RECORD] [BACKEND]
```

## What it reads

One text document on standard input, read to its end as UTF-8. `--input FILE` reads a file instead. `--lines` and `--jsonl` turn the input into records, and [records.md](records.md) gives the rules. An empty document is a usage error.

`QUESTION` comes first and states what decides the pick. Each `OPTION` is one argument. `choose` takes 2 to 255 options. A duplicate option name is a usage error, and so is an option that is empty or holds only white space. An option holding a control character is a usage error too, because `--raw` prints a label byte for byte and a label with a line feed in it would write a line of its own into the caller's output. The tool sends the options in the order the user gave and never reorders them.

## What it prints

A JSON string, or `null` when the answer is unresolved. `--details` prints the object in [result.md](result.md), with an `answer.kind` of `choice`. In record mode one value prints per record, in input order.

`--raw` prints the label without its quotation marks, as `jq -r` does. On one document an unresolved answer prints nothing. Under `--lines` or `--jsonl` an unresolved answer prints an empty line, so one line still stands for one record. A blank label is a usage error, so an empty line never means a label.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--threshold T` | A single cut on the winning option's probability. The band form is a usage error. See [threshold.md](threshold.md) | None. The winning label is returned |
| `--raw` | Prints the label without quotation marks | Off |
| `--options POINTER` | Takes the options from each record. Requires `--jsonl`. See below | None. The options come from the arguments |
| `--quiet` | Prints nothing on standard output | Off |
| `--details` | Prints the full result object | Off |
| `--dry-run` | Prints the plan and sends nothing | Off |
| Record options | `--input`, `--lines`, `--jsonl`, `--field` | One document |
| Backend options | `--url` and `--model`, in the long help alone. See [backends.md](backends.md) | The two variables and `jev-latest` |

## Options from the record

Settled by ADR 0009 item 4, accepted in ADR 0010. `--options POINTER` names a list of labels, or a map from label to description, inside each record. Every rule above holds for the labels a record supplies, and a record that breaks one is exit 2 for that record before any request for it. A record whose candidate list differs from the next record's needs it. `--options` and positional options together are a usage error. `--options` requires `--jsonl`, because a pointer needs a JSON record to point into. A line of text holds no pointer, so `--options` under `--lines` is a usage error, and so is `--options` on one document.

```sh
thinkthen choose 'Which of these codes fits the note?' --jsonl --field /note --options /codes < notes.jsonl
```

On one document the shell already does this.

```sh
thinkthen choose 'Which team owns this request?' $(jq -r '.[]' teams.json) < message.txt
```

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | Single input: a label was returned. Record mode: the run finished |
| 2 | A usage error or an input error |
| 3 | Single input only: the answer is unresolved |
| 4, 5, 70 | As [channels.md](channels.md) gives them |

`choose` never exits 1. A pick is not a two-sided decision.

## Unresolved

The answer is unresolved when the winning option's probability falls under the cut, and when the top two options tie exactly. An exact tie is unresolved with or without a threshold, because alphabetical order is no evidence. `--details` still names the option that led, in `answer.pick`.

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

Measurement of the first decider model found picking from a fixed list stable. A live run then judged sixty made-up support messages over five labels. The cases are few and they are made up. Reversing the list changed 2 of 60 picks and shuffling it changed 1 of 60. Every change landed on the catch-all `other`. Keep the option order fixed once a cut is tuned, because a run with a reordered list is a different measurement. Put the catch-all last.

The same run added a sixth label that fits nothing. It changed 0 of 60 picks, and the model gave it a probability of 0.0 on all 60 rows. That measures one kind of added option, on sixty made-up cases. A label that overlaps a real one is untested, and the vendor's own documents warn about it. Word the options so that they exclude one another, and keep the list short.

The cut falls on the winning option's probability and never on the backend's `confidence`. [backends.md](backends.md) says why.
