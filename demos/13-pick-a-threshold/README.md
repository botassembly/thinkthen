# 13 Pick a threshold

Status: red

Verbs: `decide`, `report`

Every other demo types a threshold. Nobody has said where the number comes from. A team with a few labelled examples judges them once, then asks what each cut would have done against the labels, and picks. `report` reads the saved rows and calls no model, so the sweep is free and repeatable.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

**The options of `report` are Draft in ADR 0007. Everything this page passes to `report` is a proposal, and the findings list it as one.**

## Input

`labeled.jsonl` holds eight support messages with `id`, `body`, and `label`. `label` is what a person answered. Four are true and four are false.

## Judge the labelled file once

The labels never leave the machine. `--field /body` sends the body, and `--details` in record mode keeps the whole record in `input`, the label with it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input labeled.jsonl --replay recording/ \
  > "$work/judged.jsonl"

wc -l < "$work/judged.jsonl" | tr -d ' ' | mustmatch "8"
jq -r '.input.label | tostring' "$work/judged.jsonl" | sort | uniq -c | tr -s ' ' \
  | mustmatch " 4 false
 4 true"
```

## Count, then sweep

`report` calls no model, so it runs with no key in the environment. With no options it prints counts. `--truth POINTER` names the recorded answer inside each row's `input` and turns the counts into a sweep over every distinct probability the rows carry.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
q='Does the message report a payment failure?'

thinkthen decide "$q" \
  --jsonl --field /body --details --input labeled.jsonl --replay recording/ \
  > "$work/judged.jsonl"

env -u TYPESAFE_API_KEY thinkthen report < "$work/judged.jsonl" \
  | jq -c --arg q "$q" '{rows, counts: .questions[$q].counts}' \
  | mustmatch '{"rows":8,"counts":{"yes":4,"no":4,"unresolved":0}}'

thinkthen report --truth /label < "$work/judged.jsonl" > "$work/sweep.json"
jq -r --arg q "$q" '.questions[$q].sweep | length' "$work/sweep.json" | mustmatch "8"
jq -S -c --arg q "$q" '.questions[$q].sweep[0] | keys' "$work/sweep.json" \
  | mustmatch '["false_no","false_yes","right","threshold"]'
```

Counts alone say nothing about whether the cut was right. Four yes and four no is what a perfect run looks like and also what a coin looks like. The sweep carries four numbers per cut: the cut, how many it got right, how many it called yes when the label said no, and how many it called no when the label said yes. A desk that must never miss a payment failure reads `false_no` and ignores the rest.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input labeled.jsonl --replay recording/ \
  > "$work/judged.jsonl"

cut=$(
  thinkthen report --truth /label < "$work/judged.jsonl" \
    | jq -r '
        .questions["Does the message report a payment failure?"].sweep
        | map(select(.false_no == 0))
        | max_by(.right)
        | .threshold'
)

thinkthen decide 'Does the message report a payment failure?' \
  --threshold "$cut" --input labeled.jsonl --jsonl --field /body --replay recording/ \
  | sort | uniq -c | tr -s ' ' \
  | mustmatch " 4 false
 4 true"
```

The number came back out of the report and went straight onto the command line. That works because a threshold is a decimal fraction in both places and nothing else.

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo proposes the whole `report` surface, and the proposal is one option.** `--truth POINTER` names the label inside `input`. With it, `report` prints a sweep. Without it, `report` prints counts. Nothing else is needed: the candidate cuts are the observed probabilities, and a grid step, a metric flag, and an output format flag would all be levers nobody on this page pulled.
- **The demo could not name a question.** `--details` rows carry `question.text` and no name, so `report` has only the question text to group by and every `jq` on this page carries a sentence as a key. A question with a typo fixed halfway through a file becomes two questions. The surface should give a result object a question name, or `report` should number its questions and print the text beside.
- **The demo could not sweep a band.** Two numbers make a two-dimensional sweep, and nothing on this page asks for one. The proposal covers single cuts only, and the page says so. A team that wants a band reads the sweep and sets `LOW` and `HIGH` by hand.
- **`report` reading `--details` rows and nothing else is the right shape.** One verb makes the rows and one verb reads them, and a saved file of rows can be swept again next month with no model call. That is the argument for `input` carrying the whole record.
- **Nothing stops a label reaching the backend.** `--field /body` keeps `label` on the machine, and a user who forgets it sends the answer with the question and poisons the measurement. One line in the `report` help closes it.
