# 13 Pick a threshold

Status: red

`report` left the plan under ADR 0010. This demo is rewritten over the `jq` recipes of the plan's recipes slice, and it stays red until they exist.

Verbs: `decide`, `report`

Every other demo types a threshold. Nobody has said where the number comes from. A team with a few labelled examples judges them once, asks what each cut would have done against the labels, picks one, and then checks the pick on a second file it never tuned against. `report` reads the saved rows and calls no model, so every step after the first is free.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`labeled.jsonl` holds eight support messages with `id`, `body`, and `label`. `holdout.jsonl` holds six more, kept back. `label` is what a person answered.

## Judge the tuning file once

The labels never leave the machine. `--field /body` sends the body, and `--details` in record mode keeps the whole record in `input`, the label with it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input labeled.jsonl --replay recording/ \
  > "$work/tune.jsonl"

wc -l < "$work/tune.jsonl" | tr -d ' ' | mustmatch "8"
jq -r '.input.label | tostring' "$work/tune.jsonl" | sort | uniq -c | tr -s ' ' \
  | mustmatch " 4 false
 4 true"
```

## Read the run, then score it

`report` takes the run as an argument. With no option it prints the counts and the run's own facts. A run made by a bare verb is one check named after the verb, so this one is called `decide`.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input labeled.jsonl --replay recording/ \
  > "$work/tune.jsonl"

env -u TYPESAFE_API_KEY thinkthen report "$work/tune.jsonl" \
  | jq -c '{rows, models, replayed, warnings, counts: (.checks.decide | {yes, no, unresolved})}' \
  | mustmatch '{"rows":8,"models":["jev-1.13.0"],"replayed":8,"warnings":[],"counts":{"yes":4,"no":4,"unresolved":0}}'
```

Four yes and four no is what a perfect run looks like and also what a coin looks like. `--truth NAME=POINTER` compares the check with the trusted label and says which one this is.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input labeled.jsonl --replay recording/ \
  > "$work/tune.jsonl"

thinkthen report "$work/tune.jsonl" --truth decide=/label > "$work/sweep.json"

jq -c '.checks.decide | keys' "$work/sweep.json" \
  | mustmatch '["accuracy_resolved","accuracy_unresolved","calibration","coverage","f1","false_negative","false_positive","kind","no","precision","recall","sweep","true_negative","true_positive","unresolved","yes"]'
jq -r '.checks.decide.sweep | length' "$work/sweep.json" | mustmatch "19"
jq -c '.checks.decide.sweep[0] | keys' "$work/sweep.json" \
  | mustmatch '["accuracy_resolved","accuracy_unresolved","coverage","f1","precision","recall","threshold"]'
jq -r '.checks.decide.calibration | length' "$work/sweep.json" | mustmatch "10"
```

`coverage` leads: the share of rows that resolved, the accuracy among them, and the accuracy among the rest. A cut that resolves three rows out of eight at 100 percent is not better than one that resolves eight at 90 percent, and only the pair of numbers says so.

## Pick the cut, then check it on the other file

The sweep is over the file it was tuned on, so its best number flatters itself. The pick goes onto a file that was never swept.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input labeled.jsonl --replay recording/ \
  > "$work/tune.jsonl"

cut=$(
  thinkthen report "$work/tune.jsonl" --truth decide=/label \
    | jq -r '.checks.decide.sweep | map(select(.coverage == 1)) | max_by(.accuracy_resolved) | .threshold'
)

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input holdout.jsonl --replay recording/ \
  > "$work/holdout.jsonl"

thinkthen report "$work/holdout.jsonl" --truth decide=/label --threshold "decide=$cut" \
  | jq -c '.checks.decide | {coverage, accuracy_resolved}' \
  | mustmatch '{"coverage":1,"accuracy_resolved":1}'
```

The run holds one check, so `decide=` may be left out of both options. The page writes the name to show what it is.

`--threshold NAME=RULE` reapplies a rule to the stored probabilities and makes no request. The same rows can be read at any cut for as long as the file is kept.

The recording under `recording/` does not exist yet.

## What this demo decides

- **The two-file procedure is the whole answer and it needs no option.** Sweep one file, take a number, report the other file at that number. ADR 0009 item 7 asks for exactly this, and the shell already has it. No held-out split flag is wanted.
- **Accuracy at coverage is the right headline.** A single accuracy number hides a band that refused half the file. Two numbers beside `coverage` say what the cut bought and what it cost.
- **Naming a check after the verb makes `--threshold decide=0.82` read badly.** The user never wrote the word `decide` as a name. Leaving `decide=` out works here, because the run holds one check, so the word is only forced on a file that holds two. Worse, two runs of two different questions concatenate into one check called `decide`, and the only sign is the `warnings` list. The demo asks that a bare-verb run be named after its question text, or that `report` refuse a file whose rows carry two texts.
- **The shape is fixed and the demo reads it.** `report.md` now names every key, so the `jq` paths on this page are assertions and no longer proposals. That was the one change the page could not be written without.
- **The demo could not fill the calibration table.** `report.md` fixes ten bands of 0.1 and says an empty band reports `null`. Eight rows leave most of them empty, so the page asserts the band count and nothing about the numbers inside.
