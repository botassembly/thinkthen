# `score`

Status: **Settled** for the grammar, the limits, and the warning. **Draft** for the arithmetic of the number.

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

Draft. The number is the weighted position of the levels. With K levels, it is the sum of each level's probability times its zero-based index. Three levels with probabilities 0.05, 0.30, and 0.65 give `0 × 0.05 + 1 × 0.30 + 2 × 0.65`, which is 1.6. Every probability here is illustrative.

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

## Open points

- Is the number the weighted position, or the zero-based index of the most likely level? The weighted form carries the spread and matches the bare value `1.6` that the review names as a `score` output. The index form matches the level a person would name. Recommendation: the weighted position.
- Does the number run from 0 to K−1, or from 0 to 1? ADR 0007 fixes 0 to K−1. Recommendation: keep it, and note that a script divides by K−1 to compare two rubrics of different lengths.
