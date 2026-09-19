# How to pick a threshold from labeled cases

Status: green

Verbs: `decide`

Every other page types a threshold. This one says where the number comes from. Use it when a person has already answered the cases, you have judged them once, and you want a cut you can defend. A sweep over a file flatters that file, so the rows split in two: tune on the first twenty-four, check on the last sixteen.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n -f ../../transforms/sweep/sweep.jq <(jq -c 'select(.input.id <= "C-24")' "$rows") \
  | jq -c '.pick' \
  | mustmatch '{"cut":0.7,"accuracy":1,"f1":1,"tied_cuts":[0.55,0.6,0.65,0.7,0.75,0.8],"rule":"the highest F1, and the middle cut of the cuts that tie"}'
```

## Input

`../../transforms/rows/runs/run-a.jsonl` holds forty judged cases, one row per case. Each row is the `decide --details` object with `input` holding the whole case: the `id`, the `body` that was sent, and the `label` a person gave. The ids run from `C-01` to `C-40` at a fixed width, so a string comparison splits them and any reader can repeat it.

The transforms are `../../transforms/sweep/sweep.jq`, `../../transforms/score/score.jq`, and `../../transforms/band/band.jq`. Each states its policies in its header, and each reads the saved probabilities, so another cut costs nothing and asks the model nothing.

## Step 1: read what the sweep says

`sweep.jq` scores the tuning rows at the 19 cuts from 0.05 to 0.95. The pick is the highest F1, and the middle cut of the cuts that tie, because that one sits farthest from both edges of the gap. The rule travels in the output, so nobody has to remember it.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n -f ../../transforms/sweep/sweep.jq <(jq -c 'select(.input.id <= "C-24")' "$rows") \
  | jq -c '{rows, labeled, unlabeled} , (.sweep[] | select(.cut == 0.5 or .cut == 0.9))' \
  | mustmatch '{"rows":24,"labeled":23,"unlabeled":["C-12"]}
{"cut":0.5,"coverage":1,"unresolved":0,"accuracy":0.9565,"precision":0.9167,"recall":1,"f1":0.9565}
{"cut":0.9,"coverage":1,"unresolved":0,"accuracy":0.8696,"precision":1,"recall":0.7273,"f1":0.8421}'
```

Twenty-four rows, twenty-three of them labeled. `C-12` carries no label, so the sweep lists it and scores it nowhere. The default cut of 0.5 calls one message a payment failure that a person called something else, and 0.9 starts missing real failures.

## Step 2: take the number onto the file you never swept

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n --argjson cut 0.7 -f ../../transforms/score/score.jq \
  <(jq -c 'select(.input.id > "C-24")' "$rows") \
  | jq -c '{cut, labeled, unlabeled, accuracy, precision, recall, f1}' \
  | mustmatch '{"cut":0.7,"labeled":16,"unlabeled":[],"accuracy":1,"precision":1,"recall":1,"f1":1}'
```

The holdout agrees with the pick. Sixteen rows are weak evidence, and the honest report is that nothing contradicted the cut rather than that the cut is proven.

## Step 3: use a band when a wrong answer costs more than a delay

A single cut answers every row, and `unresolved` is zero at every line of the sweep. A band refuses the middle instead. `band.jq` prints what that bought and what it cost, over the whole run.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n --argjson band '[0.2,0.8]' -f ../../transforms/band/band.jq "$rows" \
  | jq -c '{labeled, resolved, unresolved, coverage, accuracy_resolved, accuracy_unresolved} , .refused' \
  | mustmatch '{"labeled":39,"resolved":36,"unresolved":3,"coverage":0.9231,"accuracy_resolved":1,"accuracy_unresolved":0.6667}
[{"id":"C-12","label":null,"probability":0.58},{"id":"C-15","label":false,"probability":0.51},{"id":"C-16","label":false,"probability":0.34},{"id":"C-29","label":true,"probability":0.79}]'
```

The band gets every row it answers right, and it pays with four rows a person now reads. Three of those four carry a label, and at a plain cut of 0.5 the model would have got two of the three right. That is the trade, in two numbers.

Note what is counted where. The three labeled refusals are `unresolved`, counted apart and scored neither right nor wrong. The unlabeled `C-12` is in `refused` because somebody must read it, and it is in no rate at all.

## Step 4: run the same lines from the transform folders

Each transform folder holds its pipeline line, so a reader of `transforms/` runs one command and sees the shape of the output. These two read the whole run rather than the split, so the sweep picks a different cut.

```bash
set -euo pipefail

sh ../../transforms/sweep/example.sh | jq -c '.pick | {cut, f1}' | mustmatch '{"cut":0.65,"f1":1}'
sh ../../transforms/band/example.sh | jq -c '{coverage, accuracy_resolved}' \
  | mustmatch '{"coverage":0.9231,"accuracy_resolved":1}'
```

## What can go wrong

- **A transform stops with exit 5.** `jq` exits 5 both for a row it cannot parse and for an error a transform raises itself, such as a row that carries no `value`. The message names the file and the line. The `install` rung checks that `jq` is there at all.
- **`.value // false` quietly turns unresolved into no.** Every transform here tests the three answers explicitly, and `band.jq` keeps the refused rows in their own group. A transform of your own that reaches for `//` is scoring an unresolved row as a wrong no.
- **Sweeping and reporting on one file.** The best cut on the file that chose it is not a measurement. Split first.
- **A cut does not travel.** It belongs to one question text and one model version. Change either and sweep again. The rows carry both under `question.text` and `meta.model`, and `compare.jq` reads them.
- **Forty cases are few.** Every rate moves by a whole case at a time, and an unlabeled case is in no rate. A cut chosen on this much evidence is a starting point, not a finding.

## Related how-tos

- [How to check the judge against human labels](../25-check-the-judge/)
- [How to compare two runs](../24-compare-two-runs/)
- [How to know what a run cost](../28-what-a-run-cost/)
- [How to act only when the answer is sure, and send the rest to a person](../04-review-queue/)
