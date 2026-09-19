# How to check the judge against human labels

Status: green

Verbs: `decide`

A probability is a claim about the world, and the only test of it is a file of cases a person has already answered. Use this page after a run over labeled cases, to learn what the judge gets right, what it gets wrong, and which cases it refused to answer. Nothing here calls a model. Every number comes from rows already on disk.

## Input

`../../transforms/rows/cases.jsonl` is forty made-up support messages with a stable `id`, a `body`, and a `label` a person gave. One case, `C-12`, carries no label, because a real case file has one nobody could settle.

`../../transforms/rows/runs/run-a.jsonl` holds the judged rows. Each row is the `decide --details` object with `input` holding the whole case. The body went to the model and the label never left the machine.

The transforms are `../../transforms/counts/counts.jq` and `../../transforms/score/score.jq`.

## Count the answers before scoring them

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n -f ../../transforms/counts/counts.jq "$rows" | jq -c . \
  | mustmatch '{"rows":40,"yes":18,"no":18,"unresolved":4,"thresholds":["0.2:0.8"],"questions":["Does the message report a payment failure?"]}'
```

Three answers, not two. The run applied the band `0.2:0.8`, so four rows resolved to nothing and the transform holds them in their own count. `questions` and `thresholds` each hold one value, which is how you know the file is one run and not two concatenated.

## Score the run against the labels

`score.jq` takes the cut through `--argjson` and applies it to the stored probability. This is the default cut, so it is what a plain `decide` would have done.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n --argjson cut 0.5 -f ../../transforms/score/score.jq "$rows" | jq -c . \
  | mustmatch '{"cut":0.5,"rows":40,"labeled":39,"unlabeled":["C-12"],"unresolved":0,"true_positive":19,"false_positive":1,"true_negative":19,"false_negative":0,"coverage":1,"accuracy":0.9744,"precision":0.95,"recall":1,"f1":0.9744}'
```

Thirty-nine labeled rows, one miss. The judge found every real payment failure and called one other message a failure as well, so recall is 1 and precision is 0.95.

`unlabeled` names the case nobody scored. It is reported by id and counted in no rate. A case with no label is a case a person has to settle, and folding it into either answer would invent evidence.

## Read the miss

An aggregate says how many. The rows say which, and a wrong answer with its probability beside it is what tells you whether the question is worded badly or the case is genuinely hard.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -c 'select(.input | has("label"))
       | select((.answer.probability >= 0.5) != .input.label)
       | {id: .input.id, label: .input.label, probability: .answer.probability, said: .value}' \
  "$rows" \
  | mustmatch '{"id":"C-15","label":false,"probability":0.51,"said":null}'
```

`C-15` is a checkout that refused an expired discount code. A person called that no payment failure. The model landed on 0.51, which is the model saying it does not know. `said` is `null` because the run's own band already refused the row.

## Count the refusals apart

Score the same rows at the band the run used, and the three labeled refusals leave every rate rather than counting as wrong answers.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n --argjson cut '[0.2,0.8]' -f ../../transforms/score/score.jq "$rows" \
  | jq -c '{labeled, unresolved, coverage, accuracy, precision, recall, f1}' \
  | mustmatch '{"labeled":39,"unresolved":3,"coverage":0.9231,"accuracy":1,"precision":1,"recall":1,"f1":1}'
```

Thirty-nine labeled rows: thirty-six scored and three counted apart. An accuracy of 1 at a coverage of 0.9231 is a different claim from an accuracy of 1, and the second number is the one that keeps the first honest.

## The same lines, kept as files

```bash
set -euo pipefail

sh ../../transforms/counts/example.sh | jq -c '{yes, no, unresolved}' \
  | mustmatch '{"yes":18,"no":18,"unresolved":4}'
sh ../../transforms/score/example.sh | jq -c '{accuracy, f1}' \
  | mustmatch '{"accuracy":0.9744,"f1":0.9744}'
```

## What can go wrong

- **`jq` is missing.** The `install` rung names it.
- **A transform stops with exit 5.** `jq` exits 5 for a line it cannot parse and for an error a transform raises, such as a row carrying no `value`. The message names the file and the line.
- **Treating unresolved as no.** `.value // false` does exactly that, and it turns a refusal into a wrong answer. Every transform here tests true, false, and null explicitly.
- **Scoring a case nobody labeled.** `C-12` is listed and scored nowhere. A rate that included it would be a rate about a guess.
- **One accuracy number.** Accuracy without coverage hides a band that refused half the file, and accuracy alone hides which side the judge errs on. Read precision and recall together.
- **Forty cases.** One case moves accuracy by two and a half points. This run measures the wording of one question against one model version, and nothing more.
- **A high probability on evidence that was never shown.** The judge reads the body and only the body. A case whose answer depends on something outside the message will be answered confidently and wrongly, and no metric on this page can see that.

## Related how-tos

- [How to pick a threshold from labeled cases](../13-pick-a-threshold/)
- [How to see whether a probability means what it says](../38-what-a-probability-means/)
- [How to compare two runs](../24-compare-two-runs/)
- [How to know what a run cost](../28-what-a-run-cost/)
