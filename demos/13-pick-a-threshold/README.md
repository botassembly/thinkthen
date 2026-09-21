# How to pick a threshold from labeled cases

Status: green

Verbs: `decide`, `choose`, `score`

Every other page types a threshold. This one says where the number comes from. Use it when a person has already answered the cases, you have judged them once, and you want a cut you can defend. A sweep over a file flatters that file, so the rows split in two: tune on the first twenty-four, check on the last sixteen.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n -f ../../transforms/sweep/sweep.jq <(jq -c 'select(.input.id <= "C-24")' "$rows") \
  | jq -c '.pick' \
  | mustmatch '{"cut":0.7,"accuracy":1,"f1":1,"tied_cuts":[0.55,0.6,0.65,0.7,0.75,0.8],"rule":"the highest F1, and the middle cut of the cuts that tie"}'
```

## Input

`../../transforms/rows/runs/run-a.jsonl` holds forty judged cases. Each `decide --details` row keeps the case id, body, and human label under `input`. Fixed-width ids make the split repeatable.

The transforms are `../../transforms/sweep/sweep.jq`, `../../transforms/score/score.jq`, and `../../transforms/band/band.jq`. Each states its policies in its header, and each reads the saved probabilities, so another cut costs nothing and asks the model nothing.

## Step 1: read what the sweep says

`sweep.jq` follows the judgment kind. A decision uses 19 cuts from 0.05 to 0.95 and picks the middle cut among those with the highest F1. A choice shows coverage and accuracy at the same cuts. A score shows each boundary between named levels. Choice and score make no automatic pick because their cuts express different trades or questions.

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

Current backend probabilities have two decimal places. A cut finer than 0.01 adds no resolution to these rows, and neighboring cuts often tie. This observed precision is not a backend promise.

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

## Step 4: fit separate record groups only when they differ

One global cut is the simpler default. Separate cuts make sense when record groups have different operating needs and enough labeled cases of their own. Here the example adds a sample document type, then fits each type independently.

```bash
set -euo pipefail

sh ../../transforms/sweep/example.sh \
  | jq -c '{rows,groups:[.groups[]|{value,rows,labeled,cut:.pick.cut}]}' \
  | mustmatch '{"rows":40,"groups":[{"value":"chat","rows":20,"labeled":20,"cut":0.4},{"value":"email","rows":20,"labeled":19,"cut":0.7}]}'
```

The report has no pooled pick. A large group cannot choose the cut for a small group.

## What can go wrong

- **A transform stops with exit 5.** `jq` uses 5 for bad input and transform errors. The message names the file and line.
- **`.value // false` quietly turns unresolved into no.** Every transform here tests the three answers explicitly, and `band.jq` keeps the refused rows in their own group. A transform of your own that reaches for `//` is scoring an unresolved row as a wrong no.
- **Sweeping and reporting on one file.** A cut tested on the file that chose it is not a measurement. Split first.
- **A cut does not travel.** It belongs to one question text and one model version. Change either and sweep again. The rows carry both under `question.text` and `meta.model`, and `compare.jq` reads them.
- **Forty cases are few.** Each case moves the rates. Treat the cut as a starting point.

## Related how-tos

- [How to check the judge against human labels](../25-check-the-judge/)
- [How to tune a question file](../41-tune-a-question-file/)
- [How to know what a run cost](../28-what-a-run-cost/)
- [How to build a triage pipeline that drafts, blocks, or asks a person](../16-triage-pipeline/)
