# How to check the judge against human labels

Status: green

Verbs: `decide`

A probability makes a testable claim. Use labeled cases to learn what the judge gets right, where it fails, and whether 0.8 means eight in ten. Nothing here calls a model.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n --argjson cut 0.5 -f ../../transforms/score/score.jq "$rows" | jq -c . \
  | mustmatch '{"cut":0.5,"rows":40,"labeled":39,"unlabeled":["C-12"],"unresolved":0,"true_positive":19,"false_positive":1,"true_negative":19,"false_negative":0,"coverage":1,"accuracy":0.9744,"precision":0.95,"recall":1,"f1":0.9744}'
```

## Input

`../../transforms/rows/runs/run-a.jsonl` holds forty `decide --details` rows. Each `input` has an `id`, the shown `body`, and a person's local `label`. The transforms live under `../../transforms/`.

Thirty-nine rows have labels. `C-12` has none, so it appears under `unlabeled` and enters no rate.

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

The rows name each mistake and show how close its probability was.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -c 'select(.input | has("label"))
       | select((.answer.probability >= 0.5) != .input.label)
       | {id: .input.id, label: .input.label, probability: .answer.probability, said: .value}' \
  "$rows" \
  | mustmatch '{"id":"C-15","label":false,"probability":0.51,"said":null}'
```

`C-15` concerns an expired discount code. A person called it no payment failure. Its 0.51 probability fell inside the run's unresolved band, so `said` is `null`.

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

The two ends match their claims. The 0.5 to 0.6 band holds two rows, one labeled, and that one was no. An empty band yields null because no cases supplied evidence. Thirty-six rows sit at or below 0.2 or at or above 0.8, leaving most bands sparse.

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
sh ../../transforms/monitor/example.sh \
  | jq -c '{rows,reviewed,review_coverage,overturned,changes}' \
  | mustmatch '{"rows":6,"reviewed":6,"review_coverage":1,"overturned":0,"changes":[]}'
```

For a running policy, review the uncertain queue and sample automated draft and block rows. Store the person's decision as `reviewed_action`. The monitor reports coverage per action and lists changes. It cannot prove the sample was random.

For `tag` or `annotate`, point `sweep.jq` at the human fields instead of renaming them to `input.label`. A tag gets one decision sweep per label. An annotation gets one established report per mapped question. Each question or label chooses independently, and the report prints no combined score.

## What can go wrong

- **A transform stops with exit 5.** The input is malformed or lacks a required field. The error names the file and line.
- **Treating unresolved as no.** `.value // false` turns a refusal into a wrong answer. Every transform here tests true, false, and null explicitly, and a refused row keeps its probability and stays in its calibration band.
- **Scoring a case nobody labeled.** `C-12` is scored nowhere. A rate that included it would be a rate about a guess.
- **One accuracy number.** Accuracy without coverage hides a band that refused half the file, and accuracy alone hides which side the judge errs on. Read precision and recall together.
- **Calling a model calibrated from the ends alone.** Forty rows leave several bands empty or holding one noisy case. Calibration needs hundreds.
- **A high probability on evidence that was never shown.** The judge reads the body and only the body. A case whose answer lies outside the message is answered confidently and wrongly, and no number here can see that.

## Related how-tos

- [How to build a triage pipeline that drafts, blocks, or asks a person](../16-triage-pipeline/)
- [How to tune a question file](../41-tune-a-question-file/)
- [How to know what a run cost](../28-what-a-run-cost/)
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/)
