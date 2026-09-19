# 13 Pick a threshold

Status: red

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
  | jq -c '{rows, models: .run.models, replayed: .run.replayed, counts: .checks.decide.counts}' \
  | mustmatch '{"rows":8,"models":["local-decider-3"],"replayed":8,"counts":{"yes":4,"no":4,"unresolved":0}}'
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

jq -S -c '.checks.decide | keys' "$work/sweep.json" \
  | mustmatch '["accuracy","calibration","counts","coverage","f1","precision","recall","sweep"]'
jq -r '.checks.decide.sweep | length' "$work/sweep.json" | mustmatch "8"
jq -S -c '.checks.decide.sweep[0] | keys' "$work/sweep.json" \
  | mustmatch '["accuracy","coverage","f1","precision","recall","threshold","unresolved_accuracy"]'
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
    | jq -r '.checks.decide.sweep | map(select(.coverage == 1)) | max_by(.accuracy) | .threshold'
)

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input holdout.jsonl --replay recording/ \
  > "$work/holdout.jsonl"

thinkthen report "$work/holdout.jsonl" --truth decide=/label --threshold "decide=$cut" \
  | jq -c '.checks.decide | {coverage, accuracy}' \
  | mustmatch '{"coverage":1,"accuracy":1}'
```

`--threshold NAME=RULE` reapplies a rule to the stored probabilities and makes no request. The same rows can be read at any cut for as long as the file is kept.

The recording under `recording/` does not exist yet.

## What this demo decides

- **The two-file procedure is the whole answer and it needs no option.** Sweep one file, take a number, report the other file at that number. ADR 0009 item 7 asks for exactly this, and the shell already has it. No held-out split flag is wanted.
- **Accuracy at coverage is the right headline.** A single accuracy number hides a band that refused half the file. Two numbers beside `coverage` say what the cut bought and what it cost.
- **Naming a check after the verb makes `--threshold decide=0.82` read badly.** The user never wrote the word `decide` as a name. Worse, two runs of two different questions concatenate into one check called `decide`, and the only warning is the list of question texts. The demo asks that a bare-verb run be named after its question text, or that `report` refuse a file whose rows carry two texts.
- **The demo could not fix the shape of a report.** ADR 0008 names the metrics and ADR 0009 names the headline. Neither fixes a key. Every `jq` path on this page is a proposal, `run` and `checks` included.
- **The demo could not read the calibration table.** ADR 0008 asks for one and says only that it sets each probability band beside the share of cases that were truly yes. Eight rows cannot fill bands. The page asserts that the key exists and nothing more.
