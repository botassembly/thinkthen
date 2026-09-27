# How to pick a threshold from labeled cases

Status: green

Verbs: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize`, `relate`

Use a complete labeled population after judging it once. This page shows replay-only tuning, labeled reruns, and the boundary that filtered outputs cannot cross. A sweep over one file flatters that file, so tune on the first twenty-four and check on the last sixteen.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n -f ../../transforms/sweep/sweep.jq <(jq -c 'select(.input.id <= "C-24")' "$rows") \
  | jq -c '.pick' \
  | mustmatch '{"cut":0.7,"accuracy":1,"f1":1,"tied_cuts":[0.55,0.6,0.65,0.7,0.75,0.8],"rule":"the highest F1, and the middle cut of the cuts that tie"}'
```

## Input and replay

`../../transforms/rows/runs/run-a.jsonl` holds forty committed detailed `decide` rows. Each keeps the case id, body, and human label under `input`. The sweep reads complete saved details, so another cut asks no model request. Replay answers a request from a committed recording with no network; it does not collect new labels.

## Step 1: read what the sweep says

`sweep.jq` uses 19 cuts from 0.05 through 0.95 for `decide` and `choose`, and integer boundaries for `score`. A choice has no automatic pick because its cuts express a coverage trade. A score has no automatic pick because each boundary asks a different question.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl

jq -n -f ../../transforms/sweep/sweep.jq <(jq -c 'select(.input.id <= "C-24")' "$rows") \
  | jq -c '{rows, labeled, unlabeled} , (.sweep[] | select(.cut == 0.5 or .cut == 0.9))' \
  | mustmatch '{"rows":24,"labeled":23,"unlabeled":["C-12"]}
{"cut":0.5,"coverage":1,"unsure":0,"accuracy":0.9565,"precision":0.9167,"recall":1,"f1":0.9565}
{"cut":0.9,"coverage":1,"unsure":0,"accuracy":0.8696,"precision":1,"recall":0.7273,"f1":0.8421}'
```

`C-12` has no label, so the sweep lists it and scores it nowhere. The holdout is weak evidence, not proof. Current two-decimal probabilities do not promise finer resolution.

## Step 2: map every function

Each cut belongs to one question, model, output signal, labeled population, and collection boundary. A changed question, model, or incomplete candidate set requires another measured run.

| Function | Rows required | Signal and boundary |
| --- | --- | --- |
| `decide` | Complete detailed rows | `answer.probability`; existing cuts 0.05 through 0.95 |
| `choose` | Complete detailed rows | winning `answer.probabilities[answer.pick]`; existing cuts 0.05 through 0.95, with ties not sure |
| `tag` | Complete detailed rows plus one human-label pointer | each label's probability independently; existing cuts 0.05 through 0.95 |
| `score` | Complete detailed rows with trusted numeric levels | `value`; integer boundaries 1 through K minus 1 |
| `filter` | Full labeled input joined to one recording-backed output per tested cut | output membership at each explicit tested cut; never infer records omitted at a lower cut |
| `rank` | Complete detailed ranked rows | `answer.probability`; cuts 0.05 through 0.95 as a downstream review policy, not a command threshold |
| `find` | One complete detailed result and trusted winner per labeled set | winning option probability; cuts 0.05 through 0.95 as a downstream coverage policy, not a command threshold |
| `annotate` | Complete detailed rows and mapped truth | existing mapped decide, choose, and tag support; project a named score to ordinary score rows before integer boundaries 1 through K minus 1 |
| `recognize` | Complete labeled name candidates, or one recording-backed rerun per tested cut | entity `strength`; explicit tested cuts only, never reconstruct omitted names |
| `relate` | Complete labeled candidate edges, or one recording-backed rerun per tested cut | edge `probability`; explicit tested cuts only, never reconstruct omitted edges |

The existing sweep accepts scalar `yes_no`, `choice`, and `score` rows, standalone `tag` rows with `--arg truth`, and mapped `annotate` `yes_no`, `choice`, or `tag` answers. Mapped `annotate` score is unsupported. Project one named score into ordinary score rows while preserving its question, value, levels, input id, and trusted numeric label before using the integer boundaries.

## Step 3: rerun filtered cuts honestly

The filtered result contains only kept records. For every explicit cut, rerun from the committed recording and join its membership to the full labeled input. Never derive a lower cut from an already filtered result.

```bash
set -euo pipefail
rows=../../transforms/rows/runs/run-a.jsonl
full=$(mktemp)
trap 'rm -f "$full"' EXIT
jq -c 'select(.input.id <= "C-24") | .input' "$rows" > "$full"

for cut in 0.5 0.9; do
  thinkthen filter 'Does the message report a payment failure?' --jsonl --field /body --batch 1 \
    --threshold "$cut" --replay ../../transforms/rows/recording/ < "$full" |
    jq -c -s --argjson cut "$cut" --slurpfile all "$full" '. as $kept | {cut:$cut,ids:($all | map(select(.id as $id | $kept | any(.[]; .id == $id)) | .id))}'
done | mustmatch '{"cut":0.5,"ids":["C-01","C-03","C-05","C-07","C-09","C-11","C-12","C-14","C-15","C-17","C-19","C-21","C-23"]}
{"cut":0.9,"ids":["C-01","C-03","C-05","C-09","C-14","C-17","C-19","C-21"]}'
```

## What can go wrong

- **`.value // false` turns not sure into no.** Test the three answers explicitly.
- **A cut does not travel.** It belongs to one question and model. Change either and measure again.
- **Forty cases are few.** Treat the cut as a starting point.

## Related how-tos

- [How to check the judge against human labels](../25-check-the-judge/)
- [How to tune a question file](../41-tune-a-question-file/)
- [How to know what a run cost](../28-what-a-run-cost/)
- [How to build a triage pipeline that drafts, blocks, or asks a person](../16-triage-pipeline/)
