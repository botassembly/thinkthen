# How to compare two runs

Status: green

Verbs: `decide`

You reworded a question, or a model version changed under you, and you want to know what moved. Use this page to line two runs up by case id, see which cases flipped in each direction, see which cases only one run holds, and read from the rows themselves whether the question, the model, or the threshold is what changed. Nothing here calls a model.

## Input

`../../transforms/rows/runs/run-a.jsonl` and `../../transforms/rows/runs/run-b.jsonl` hold the same forty cases judged with two wordings of one question. `../../transforms/rows/question.txt` asks "Does the message report a payment failure?" and `../../transforms/rows/question-b.txt` asks "Does the customer report that a payment or a payout did not go through?". Both runs applied the band `0.2:0.8`, and both were answered by `jev-1.13.0`.

The transform is `../../transforms/compare/compare.jq`. The earlier run arrives through `--slurpfile` and the later one on the command line.

## Line the runs up

```bash
set -euo pipefail
before=../../transforms/rows/runs/run-a.jsonl
after=../../transforms/rows/runs/run-b.jsonl

jq -n --slurpfile before "$before" -f ../../transforms/compare/compare.jq "$after" \
  | jq -c '{paired, same, flips}' \
  | mustmatch '{"paired":40,"same":36,"flips":{"yes to unresolved":["C-11","C-39"],"unresolved to yes":["C-12"],"unresolved to no":["C-16"]}}'
```

Four cases moved out of forty. Every flip names both ends, so a move into the unresolved band and a move to no are different lines and are never added together. The reworded question sends two cases that were yes into the band, and it settles two the first wording had refused.

## Ask what changed before you read the flips

A flip has four possible causes: the evidence, the question, the model, or the cut. Three of the four are in the rows.

```bash
set -euo pipefail
before=../../transforms/rows/runs/run-a.jsonl
after=../../transforms/rows/runs/run-b.jsonl

jq -n --slurpfile before "$before" -f ../../transforms/compare/compare.jq "$after" \
  | jq -c '{changed, mismatched_input, mismatched_label}' \
  | mustmatch '{"changed":{"question":true,"model":false,"threshold":false},"mismatched_input":[],"mismatched_label":[]}'
```

`question` is true and the other two are false, so the wording is the only candidate. `mismatched_input` and `mismatched_label` are both empty, which says the pairs really are the same cases with the same trusted answers. Had either list carried an id, the flips below it would be about two different measurements and worth nothing.

## Prove the checks with a doctored file

An empty list is easy to believe and easy to get wrong. This block changes one label and drops one case, and the transform names both.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
before=../../transforms/rows/runs/run-a.jsonl
after=../../transforms/rows/runs/run-b.jsonl

jq -c 'if .input.id == "C-03" then .input.label = false else . end
       | select(.input.id != "C-40")' "$after" > "$work/doctored.jsonl"

jq -n --slurpfile before "$before" -f ../../transforms/compare/compare.jq "$work/doctored.jsonl" \
  | jq -c '{paired, only_in_before, mismatched_label, mismatched_input}' \
  | mustmatch '{"paired":39,"only_in_before":["C-40"],"mismatched_label":["C-03"],"mismatched_input":[]}'
```

A case in one run alone is listed rather than dropped, so a run that stopped early cannot pass as a smaller run that finished. A changed label is named, because it means somebody moved the target between the two measurements.

## The same line, kept as a file

```bash
set -euo pipefail

sh ../../transforms/compare/example.sh | jq -c '{paired, same}' \
  | mustmatch '{"paired":40,"same":36}'
```

## What can go wrong

- **`jq` is missing.** The `install` rung names it.
- **A transform stops with exit 5.** `jq` exits 5 for a line it cannot parse and for an error the transform raises, such as a row carrying no `value`.
- **Pairing on the id alone.** Two runs can share every id and measure different things. The transform checks the evidence and the label of every pair for that reason.
- **Two rows under one id.** They are repeated trials, and averaging them is a different transform. This one lists them in `repeated_ids` and pairs them in nothing, so a duplicate can never be counted twice.
- **Folding unresolved into no.** Then a case that moved into the band reads as a regression. The six directions stay apart here.
- **Reading flips before reading `changed`.** If the model version moved as well as the question, the comparison cannot tell you which one did it, and the answer is to rerun with one of the two held still.
- **Four flips out of forty.** That is a count, not a rate with a confidence behind it. It says where to look.

## Related how-tos

- [How to check the judge against human labels](../25-check-the-judge/)
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/)
- [How to know what a run cost](../28-what-a-run-cost/)
