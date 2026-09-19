# `decide`

Status: **Settled** for version one, by ADR 0007.

Answers a yes/no question about the evidence and sets the exit code.

```text
thinkthen decide QUESTION [--threshold T|LOW:HIGH] [--quiet] [--details] [RECORD] [BACKEND]
```

## What it reads

One text document on standard input, read to its end as UTF-8. `--input FILE` reads a file instead. `--lines` and `--jsonl` turn the input into records, and [records.md](records.md) gives the rules.

`QUESTION` is one argument. It states a fact that is true or false of the evidence. The decider model reads it as the question. A question that is empty or holds only white space is a usage error. An empty document is a usage error, because a judgment about nothing is a mistake in the pipeline.

## What it prints

`true`, `false`, or `null`. `null` is an unresolved answer, and it arises only under a band. `--details` prints the object in [result.md](result.md) instead, with an `answer.kind` of `yes_no`. In record mode one value prints per record, in input order.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--threshold T\|LOW:HIGH` | The rule in [threshold.md](threshold.md). `decide` is the one verb that takes both forms | `0.5` |
| `--quiet` | Prints nothing on standard output. The exit code still carries the answer | Off |
| `--details` | Prints the full result object in place of the bare value | Off |
| `--dry-run` | Prints the plan and sends nothing. See [channels.md](channels.md) | Off |
| Record options | `--input`, `--lines`, `--jsonl`, `--field`. See [records.md](records.md) | One document |
| Backend options | `--url` and `--model`, in the long help alone. See [backends.md](backends.md) | The two variables and `jev-latest` |

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | Single input: the answer is yes. Record mode: the run finished |
| 1 | Single input only: the answer is no |
| 2 | Usage error |
| 3 | Single input only: the answer is unresolved |
| 4, 5, 70 | As [channels.md](channels.md) gives them |

In record mode the exit code reports the run, and no record's answer sets it.

## Examples

```sh
thinkthen decide 'The customer explicitly requests a refund.' < message.txt
```

```sh
if thinkthen decide 'The customer explicitly requests a refund.' --threshold 0.1:0.9 --quiet < message.txt; then
  route refunds
fi
```

```sh
thinkthen decide 'Does this report a payment failure?' --jsonl --field /body --details < tickets.jsonl
```

## Reading the exit code

The help shows a `case $?` block. It separates a no from an unresolved answer and from a failure, and a script that acts on the answer reads all four outcomes.

```sh
thinkthen decide 'The customer explicitly requests a refund.' --threshold 0.1:0.9 --quiet < message.txt
case $? in
  0) route refunds ;;
  1) route support ;;
  3) route triage ;;
  *) printf 'the judge failed\n' >&2; exit 4 ;;
esac
```

The help shows one piece of advice beside that block. Word the question in the form where yes permits the action. A failure then never permits anything, because every outcome other than 0 leaves the action undone.

## Cautions

`decide` exits 1 on a no and 3 on an unresolved answer. Under `set -e` or `set -o pipefail` that ends a script. Put the command in an `if`, a `case`, or a `||` list. [channels.md](channels.md) says more.

Writing a good question matters more than any option. A question works when it names one fact that is visible in the evidence. "Mentions a delivery date" works. "Is a good reply" does not. Measurement of the first decider model showed a narrow question of the first kind catching every planted mismatch while wrongly rejecting 2% to 3% of good work. Outcome questions of the second kind rejected 18% to 46% of work people had accepted.
