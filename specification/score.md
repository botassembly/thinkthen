# `score`

Status: **Settled** for version one, by ADR 0007.

Places the evidence on named levels and prints a number.

```text
thinkthen score QUESTION|@FILE [LEVEL...] [--details] [RECORD] [BACKEND]
```

## What it reads

One text document on standard input, read to its end as UTF-8. `--input FILE` reads a file instead. `--lines`, `--jsonl`, `--csv`, and `--tsv` turn the input into records, and [records.md](records.md) gives the rules. An empty document is a usage error.

`QUESTION` comes first and names what is being placed. Each `LEVEL` is one argument. `score` takes 2 to 10 levels, lowest first. A duplicate level is a usage error, and so is a level that is empty, that holds only white space, or that holds a control character, because a level is one line of printable text. `@FILE` reads the question and its levels from a question file instead, where `levels` may be an ordered map from each name to its description; the model then reads the descriptions in order and a result still lists the names. [question-file.md](question-file.md) holds the grammar, the defaults, and the precedence.

## What it prints

On one document, a JSON number. The score runs from 0 at the lowest level to the number of levels minus one at the highest. In record mode each compact JSONL row is `{"input":RECORD,"value":NUMBER}` in input order. `--details` prints the object in [result.md](result.md), with an `answer.kind` of `score`.

## The number

The number is the backend's probability-weighted position on the levels. With K levels, it is the sum of each level's probability times its zero-based index, divided by the measured total of the accepted probabilities. The tool rounds that result at twelve decimal places. Three levels with probabilities 0.05, 0.30, and 0.65 give `(0 × 0.05 + 1 × 0.30 + 2 × 0.65) / 1`. That is 1.6. Every probability here is illustrative.

The tool computes that weighted sum and normalization itself, from the probabilities the backend returned, and never reads the vendor's own score field. Experiment 235 found differences between the two fields on 13 of 20 songs, with a maximum difference of 0.02. The vendor field may use unrounded or differently normalized probabilities. Callers should use the tool-computed value because it is internally consistent with the probabilities in the same row.

The number is a position on the levels the user named. It is not a probability that anything holds, and it is not a confidence in the answer. A script that compares two runs over different level lists is comparing two different scales.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--details` | Prints the full result object | Off |
| `--dry-run` | Prints the plan and sends nothing | Off |
| Record options | `--input`, `--lines`, `--jsonl`, `--csv`, `--tsv`, `--field` | One document |
| Backend options | `--url` in short and long help, and `--model` in long help. See [backends.md](backends.md) | The two variables and `jev-latest` |

`score` takes no `--threshold`, no `--quiet`, and no `--raw`. It has no answer exit code, so quiet output would discard its result. Its result is a JSON number; `choose --raw` is the command that prints a bare label.

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

Measurement of the first System One model showed rubric judgments rejecting 18% to 46% of work that people had accepted.

A live run then placed forty made-up incident reports on five levels. The cases are few and they are made up. The order held well: the value tracked the trusted level with a rank correlation of 0.9703, and no text missed by more than one level, on 40 of 40. The exact level was right on 31 of 40. The absolute level ran one step high on 9 of 40, all of them one step and never more. A cut on the number is therefore tuned on labeled cases before anyone trusts it. The help text for `score` says so.

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
