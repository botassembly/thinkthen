# How to pick a threshold from labeled cases

Status: green

Verbs: `decide`

Every other page types a threshold. This one says where the number comes from. Use it when you have cases a person has already answered, you have judged them once, and you want a cut you can defend. Every step after the judging reads the saved probabilities, so trying another cut costs nothing and asks the model nothing.

## Input

`../../transforms/rows/runs/run-a.jsonl` holds forty judged cases, one row per case. Each row is the `decide --details` object with `input` holding the whole case: the `id`, the `body` that was sent, and the `label` a person gave. `../../transforms/rows/cases.jsonl` is the case file the run was made from, and `../../transforms/rows/record.sh` is the loop that made it.

The recipes are `../../transforms/sweep/sweep.jq`, `../../transforms/score/score.jq`, and `../../transforms/band/band.jq`. Each one states its policies in its header.

## Split the run in two

A sweep over a file flatters that file. Tune on one part and check on the other. The ids run from `C-01` to `C-40` with a fixed width, so a string comparison splits them and any reader can repeat it. The rows are already judged, so the split is a `jq` filter and costs no second run.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
rows=../../transforms/rows/runs/run-a.jsonl

jq -c 'select(.input.id <= "C-24")' "$rows" > "$work/tune.jsonl"
jq -c 'select(.input.id > "C-24")' "$rows" > "$work/holdout.jsonl"

wc -l < "$work/tune.jsonl" | tr -d ' ' | mustmatch "24"
wc -l < "$work/holdout.jsonl" | tr -d ' ' | mustmatch "16"
```

## Ask every cut what it would have done

`sweep.jq` scores the tuning file at the 19 cuts from 0.05 to 0.95 and picks one. The pick is the highest F1, and the middle cut of the cuts that tie, because that one sits farthest from both edges of the gap. An even number of ties has two middles, and the pick is the higher of the two. The rule travels in the output, so nobody has to remember it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
rows=../../transforms/rows/runs/run-a.jsonl

jq -c 'select(.input.id <= "C-24")' "$rows" > "$work/tune.jsonl"
jq -n -f ../../transforms/sweep/sweep.jq "$work/tune.jsonl" > "$work/sweep.json"

jq -c '{rows, labeled, unlabeled}' "$work/sweep.json" \
  | mustmatch '{"rows":24,"labeled":23,"unlabeled":["C-12"]}'
jq -c '.pick' "$work/sweep.json" \
  | mustmatch '{"cut":0.7,"accuracy":1,"f1":1,"tied_cuts":[0.55,0.6,0.65,0.7,0.75,0.8],"rule":"the highest F1, and the middle cut of the cuts that tie"}'
jq -c '.sweep[] | select(.cut == 0.5 or .cut == 0.7 or .cut == 0.9)' "$work/sweep.json" \
  | mustmatch '{"cut":0.5,"coverage":1,"unresolved":0,"accuracy":0.9565,"precision":0.9167,"recall":1,"f1":0.9565}
{"cut":0.7,"coverage":1,"unresolved":0,"accuracy":1,"precision":1,"recall":1,"f1":1}
{"cut":0.9,"coverage":1,"unresolved":0,"accuracy":0.8696,"precision":1,"recall":0.7273,"f1":0.8421}'
```

Twenty-four rows, twenty-three of them labeled. `C-12` carries no label, so the sweep lists it and scores it nowhere. The default cut of 0.5 calls one message a payment failure that a person called something else. Six cuts between 0.55 and 0.8 get every labeled row right, and 0.9 starts missing real failures.

## Take the number onto the file you never swept

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
rows=../../transforms/rows/runs/run-a.jsonl

jq -c 'select(.input.id <= "C-24")' "$rows" > "$work/tune.jsonl"
jq -c 'select(.input.id > "C-24")' "$rows" > "$work/holdout.jsonl"

cut=$(jq -n -f ../../transforms/sweep/sweep.jq "$work/tune.jsonl" | jq -r '.pick.cut')
printf '%s\n' "$cut" | mustmatch "0.7"

jq -n --argjson cut "$cut" -f ../../transforms/score/score.jq "$work/holdout.jsonl" \
  | jq -c '{cut, labeled, unlabeled, accuracy, precision, recall, f1}' \
  | mustmatch '{"cut":0.7,"labeled":16,"unlabeled":[],"accuracy":1,"precision":1,"recall":1,"f1":1}'
```

The holdout agrees with the pick. Sixteen rows are weak evidence, and the honest report is that nothing contradicted the cut rather than that the cut is proven.

## Use a band when a wrong answer costs more than a delay

A single cut answers every row, and `unresolved` is zero at every line of the sweep. A band refuses the middle instead. `band.jq` prints what that bought and what it cost, over the whole run.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n --argjson band '[0.2,0.8]' -f ../../transforms/band/band.jq "$rows" \
  | jq -c '{labeled, resolved, unresolved, coverage, accuracy_resolved, accuracy_unresolved}' \
  | mustmatch '{"labeled":39,"resolved":36,"unresolved":3,"coverage":0.9231,"accuracy_resolved":1,"accuracy_unresolved":0.6667}'

jq -n --argjson band '[0.2,0.8]' -f ../../transforms/band/band.jq "$rows" \
  | jq -c '.refused' \
  | mustmatch '[{"id":"C-12","label":null,"probability":0.58},{"id":"C-15","label":false,"probability":0.51},{"id":"C-16","label":false,"probability":0.34},{"id":"C-29","label":true,"probability":0.79}]'
```

The band gets every row it answers right, and it pays with four rows a person now reads. Three of those four carry a label, and at a plain cut of 0.5 the model would have got two of the three right. That is the trade, in two numbers.

Note what is counted where. The three labeled refusals are `unresolved`, counted apart and scored neither right nor wrong. The unlabeled `C-12` is in `refused` because somebody must read it, and it is in no rate at all.

## The same lines, kept as files

Each recipe folder holds the pipeline line, so a reader of `transforms/` runs one command and sees the shape of the output. These two read the whole run rather than the split.

```bash
set -euo pipefail

sh ../../transforms/sweep/example.sh | jq -c '.pick | {cut, f1}' | mustmatch '{"cut":0.65,"f1":1}'
sh ../../transforms/band/example.sh | jq -c '{coverage, accuracy_resolved}' \
  | mustmatch '{"coverage":0.9231,"accuracy_resolved":1}'
```

## What can go wrong

- **`jq` is missing.** The `install` rung names it, because every recipe here is `jq`.
- **A recipe stops with exit 5.** `jq` exits 5 both for a row it cannot parse and for an error a recipe raises itself, such as a row that carries no `value`. The message names the file and the line.
- **`.value // false` quietly turns unresolved into no.** Every recipe here tests the three answers explicitly, and `band.jq` keeps the refused rows in their own group. A recipe of your own that reaches for `//` is scoring an unresolved row as a wrong no.
- **Sweeping and reporting on one file.** The best cut on the file that chose it is not a measurement. Split first.
- **A cut does not travel.** It belongs to one question text and one model version. Change either and sweep again. The rows carry both under `question.text` and `meta.model`, and `compare.jq` reads them.
- **Forty cases are few.** Every rate here moves by a whole case at a time. A cut chosen on this much evidence is a starting point, not a finding.
- **An unlabeled case is not a no.** `C-12` is in no rate. A recipe that scored it as a no would report a precision that no person ever agreed with.

## Related how-tos

- [How to check the judge against human labels](../25-check-the-judge/)
- [How to see whether a probability means what it says](../38-what-a-probability-means/)
- [How to compare two runs](../24-compare-two-runs/)
- [How to know what a run cost](../28-what-a-run-cost/)
- [How to gate a script step on a yes/no answer](../01-refund-gate/)
