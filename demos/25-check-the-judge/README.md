# How to check the judge against human labels

Status: green

Verbs: `decide`

A probability is a claim about the world, and the only test of it is a file of cases a person has already answered. Use this page after a run over labeled cases, to learn what the judge gets right, what it gets wrong, and whether a probability of 0.8 means what it says. Nothing here calls a model.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n --argjson cut 0.5 -f ../../transforms/score/score.jq "$rows" | jq -c . \
  | mustmatch '{"cut":0.5,"rows":40,"labeled":39,"unlabeled":["C-12"],"unresolved":0,"true_positive":19,"false_positive":1,"true_negative":19,"false_negative":0,"coverage":1,"accuracy":0.9744,"precision":0.95,"recall":1,"f1":0.9744}'
```

## Input

`../../transforms/rows/runs/run-a.jsonl` holds forty judged rows. Each is the `decide --details` object, and `input` holds the whole case: a stable `id`, the `body` that went to the model, and the `label` a person gave, which never left the machine. The transforms are `counts.jq`, `score.jq`, and `calibration.jq` under `../../transforms/`.

Thirty-nine labeled rows, one miss. `C-12` carries no label, because a real case file has one nobody could settle, so it is named in `unlabeled` and counted in no rate.

## Step 1: count the answers, then score them at the band the run used

Three answers, not two. The run applied the band `0.2:0.8`, so four rows resolved to nothing. `questions` and `thresholds` hold one value each, which is how you know the file is one run.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n -f ../../transforms/counts/counts.jq "$rows" | jq -c . \
  | mustmatch '{"rows":40,"yes":18,"no":18,"unresolved":4,"thresholds":["0.2:0.8"],"questions":["Does the message report a payment failure?"]}'

jq -n --argjson cut '[0.2,0.8]' -f ../../transforms/score/score.jq "$rows" \
  | jq -c '{labeled, unresolved, coverage, accuracy, precision, recall, f1}' \
  | mustmatch '{"labeled":39,"unresolved":3,"coverage":0.9231,"accuracy":1,"precision":1,"recall":1,"f1":1}'
```

Thirty-six rows scored and three counted apart. An accuracy of 1 at a coverage of 0.9231 is a different claim from an accuracy of 1, and the second number keeps the first honest.

## Step 2: read the one it got wrong

An aggregate says how many. The rows say which, and a probability beside a wrong answer says whether the question is worded badly or the case is hard.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -c 'select(.input | has("label"))
       | select((.answer.probability >= 0.5) != .input.label)
       | {id: .input.id, label: .input.label, probability: .answer.probability, said: .value}' \
  "$rows" \
  | mustmatch '{"id":"C-15","label":false,"probability":0.51,"said":null}'
```

`C-15` is a checkout that refused an expired discount code, and a person called that no payment failure. The model landed on 0.51, which is it saying it does not know, and `said` is `null` because the run's band refused the row.

## Step 3: ask whether the probabilities mean what they say

A probability of 0.8 claims that about eight cases in ten like it are really yes. `calibration.jq` sets ten bands a tenth wide beside the share of each that was truly yes. Read `mean_probability` beside `share_truly_yes`.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n -f ../../transforms/calibration/calibration.jq "$rows" \
  | jq -c '.bands[] | select(.band == "0-0.1" or .band == "0.5-0.6" or .band == "0.2-0.3" or .band == "0.9-1")' \
  | mustmatch '{"band":"0-0.1","rows":17,"unresolved":0,"labeled":17,"truly_yes":0,"share_truly_yes":0,"mean_probability":0.0171}
{"band":"0.2-0.3","rows":0,"unresolved":0,"labeled":0,"truly_yes":0,"share_truly_yes":null,"mean_probability":null}
{"band":"0.5-0.6","rows":2,"unresolved":2,"labeled":1,"truly_yes":0,"share_truly_yes":0,"mean_probability":0.545}
{"band":"0.9-1","rows":12,"unresolved":0,"labeled":12,"truly_yes":12,"share_truly_yes":1,"mean_probability":0.9842}'
```

At the two ends the number means what it says. The middle is another story: 0.5 to 0.6 holds two rows, one of them labeled, and that one was a no. An empty band yields null and never zero, because zero reads as a finding and null reads as "nobody asked". Thirty-six of the forty rows sit at or below 0.2 or at or above 0.8, so most of the scale is empty.

## Step 4: run the same lines from the transform folders

```bash
set -euo pipefail

sh ../../transforms/counts/example.sh | jq -c '{yes, no, unresolved}' \
  | mustmatch '{"yes":18,"no":18,"unresolved":4}'
sh ../../transforms/score/example.sh | jq -c '{accuracy, f1}' \
  | mustmatch '{"accuracy":0.9744,"f1":0.9744}'
sh ../../transforms/calibration/example.sh \
  | jq -c '.bands[] | select(.band == "0.9-1") | {rows, truly_yes, mean_probability}' \
  | mustmatch '{"rows":12,"truly_yes":12,"mean_probability":0.9842}'
```

## What can go wrong

- **A transform stops with exit 5.** `jq` exits 5 for a line it cannot parse and for an error a transform raises, such as a row with no `value`. It names the file and the line.
- **Treating unresolved as no.** `.value // false` turns a refusal into a wrong answer. Every transform here tests true, false, and null explicitly, and a refused row keeps its probability and stays in its calibration band.
- **Scoring a case nobody labeled.** `C-12` is scored nowhere. A rate that included it would be a rate about a guess.
- **One accuracy number.** Accuracy without coverage hides a band that refused half the file, and accuracy alone hides which side the judge errs on. Read precision and recall together.
- **Calling a model well calibrated from the ends alone.** Answers near 0 and 1 are the easy cases, and forty rows across ten bands leaves three bands empty and three holding one row. A share of 0 or 1 over one row is noise, and a calibration table earns trust at hundreds of cases.
- **A high probability on evidence that was never shown.** The judge reads the body and only the body. A case whose answer lies outside the message is answered confidently and wrongly, and no number here can see that.

## Related how-tos

- [How to pick a threshold from labeled cases](../13-pick-a-threshold/)
- [How to tune a question file](../41-tune-a-question-file/)
- [How to know what a run cost](../28-what-a-run-cost/)
- [How to act only when the answer is sure, and send the rest to a person](../04-review-queue/)
