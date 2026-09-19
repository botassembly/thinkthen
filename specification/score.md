# `score`

Status: **Settled** for version one, by ADR 0007.

Places the evidence on named levels and prints a number.

```text
thinkthen score QUESTION LEVEL... [--details] [RECORD] [BACKEND]
```

## What it reads

One text document on standard input, read to its end as UTF-8. `--input FILE` reads a file instead. `--lines` and `--jsonl` turn the input into records, and [records.md](records.md) gives the rules. An empty document is a usage error.

`QUESTION` comes first and names what is being placed. Each `LEVEL` is one argument. `score` takes 2 to 10 levels, lowest first. A duplicate level is a usage error.

## What it prints

A JSON number. The score runs from 0 at the lowest level to the number of levels minus one at the highest. `--details` prints the object in [result.md](result.md), with an `answer.kind` of `score`. In record mode one number prints per record, in input order.

## The number

The number is the backend's probability-weighted position on the levels. With K levels, it is the sum of each level's probability times its zero-based index. Three levels with probabilities 0.05, 0.30, and 0.65 give `0 × 0.05 + 1 × 0.30 + 2 × 0.65`. That is 1.6. Every probability here is illustrative.

The number is a position on the levels the user named. It is not a probability that anything holds, and it is not a confidence in the answer. A script that compares two runs over different level lists is comparing two different scales.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--details` | Prints the full result object | Off |
| `--dry-run` | Prints the plan and sends nothing | Off |
| Record options | `--input`, `--lines`, `--jsonl`, `--field` | One document |
| Backend options | `--profile` and the advanced flags | The selected profile |

`score` takes no `--threshold`, no `--quiet`, and no `--raw`.

## Exit codes

0 on success, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. No answer sets the exit code.

## Examples

```sh
thinkthen score 'How much disruption does this report?' 'None.' 'Work continues with a workaround.' 'Work is blocked.' < ticket.txt
```

```sh
thinkthen score 'How much disruption does this report?' 'None.' 'Work continues with a workaround.' 'Work is blocked.' \
  --jsonl --field /body < tickets.jsonl
```

## The measured warning

Rating is the weakest thing a decider model does. Measurement of the first decider model showed rubric judgments rejecting 18% to 46% of work that people had accepted. The help text for `score` says so.

No other command depends on `score`. A gate that has to hold belongs in `decide` or `choose`. A number from `score` belongs in a review queue that a person reads.

## Cutting on a score

`score` takes no threshold. `jq -e` cuts on the number in one line and sets the exit code.

```sh
thinkthen score 'How much disruption does this report?' 'None.' 'Work continues with a workaround.' 'Work is blocked.' < ticket.txt |
  jq -e '. >= 2' > /dev/null && page oncall
```

`score` takes no threshold at all, and no option on it sets an exit code. ADR 0010 settles that.

The Bash way to branch on levels is `choose` with the levels as ordered labels. The help shows it beside the `jq` line.

```sh
case "$(thinkthen choose 'How much disruption does this report?' none workaround blocked --raw < ticket.txt)" in
  blocked) page oncall ;;
  workaround) queue review ;;
  none) : ;;
  "") queue review ;;
esac
```

A script that compares two runs over different level lists divides each number by K−1 first. The two scales are otherwise different.
