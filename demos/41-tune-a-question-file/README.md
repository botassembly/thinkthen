# How to tune a question file and use the same file in the gate

Status: green

Verbs: `decide`

A question worth trusting is measured against labeled cases first. A question file makes the wording you measured the wording the gate runs, byte for byte, and every row says so.

```bash
set -euo pipefail

printf '%s' "$(jq -r 'select(.id == "C-01") | .body' claims.jsonl)" \
  | thinkthen decide @receipt.json --quiet --replay recording/ \
  && printf 'ask for the receipt\n' | mustmatch "ask for the receipt"
```

That is the gate. The rest of this page is how `receipt.json` earned it.

## Input

`claims.jsonl` holds twenty-four made-up expense claims with a trusted answer in `label`. The office rule: a claim needs a receipt over 25 pounds, or from an outside supplier whatever the amount.

`draft.json` is the first wording of the question and `receipt.json` is the tuned one. `recording/` holds the forty-eight live exchanges this page replays, and `record.sh` made them through `sdlc/scripts/live`. The repository's `transforms/` tree is an additional input because the commands below read its score transform.

## Step 1: judge the claims with the draft

```bash
set -euo pipefail
work=$(mktemp -d); trap 'rm -rf -- "$work"' EXIT

thinkthen decide @draft.json --jsonl --field /body --details \
  --replay recording/ < claims.jsonl > "$work/draft.jsonl"

jq -n --argjson cut 0.5 -f ../../transforms/score/score.jq "$work/draft.jsonl" \
  | jq -c '{rows, accuracy, false_positive, false_negative}' \
  | mustmatch '{"rows":24,"accuracy":0.875,"false_positive":3,"false_negative":0}'
```

Three claims were called yes that the rule calls no, and none the other way. The draft says nothing about the 25 pound line or outside suppliers.

## Step 2: change the file and never the command

The fix goes in the file, under `true` and `false`. The command does not move.

```bash
set -euo pipefail
work=$(mktemp -d); trap 'rm -rf -- "$work"' EXIT

jq '. + {
  true: "The claim is for more than 25 pounds, or it is for anything bought from an outside supplier whatever the amount.",
  false: "The claim is mileage at the fixed rate per mile, a per-day subsistence allowance at the fixed rate, or anything under 25 pounds that is not from an outside supplier."
}' draft.json > "$work/receipt.json"

jq -s -e '.[0] == .[1]' "$work/receipt.json" receipt.json > /dev/null \
  && printf 'the tuned file is the committed one\n' \
  | mustmatch "the tuned file is the committed one"

thinkthen decide @"$work/receipt.json" --jsonl --field /body --details \
  --replay recording/ < claims.jsonl > "$work/tuned.jsonl"

jq -n --argjson cut 0.5 -f ../../transforms/score/score.jq "$work/tuned.jsonl" \
  | jq -c '{rows, accuracy, false_positive, false_negative}' \
  | mustmatch '{"rows":24,"accuracy":1,"false_positive":0,"false_negative":0}'
```

`--dry-run` names the home each setting came from, so an odd run is read rather than guessed at. A value typed beside `@FILE` wins.

```bash
set -euo pipefail

head -1 claims.jsonl \
  | thinkthen decide @receipt.json --jsonl --field /body --threshold 0.8 --dry-run \
  | jq -c '.from' \
  | mustmatch '{"question":"file","true":"file","false":"file","threshold":"command line","on":"command line","model":"default"}'
```

## Step 3: compare the two runs and keep the digest

```bash
set -euo pipefail
work=$(mktemp -d); trap 'rm -rf -- "$work"' EXIT

thinkthen decide @draft.json --jsonl --field /body --details \
  --replay recording/ < claims.jsonl > "$work/draft.jsonl"
thinkthen decide @receipt.json --jsonl --field /body --details \
  --replay recording/ < claims.jsonl > "$work/tuned.jsonl"

jq -n --slurpfile before "$work/draft.jsonl" -f ../../transforms/compare/compare.jq \
  "$work/tuned.jsonl" | jq -c '{paired,compared,same,changed_values,flips,
    value_changes:[.changes[]|select(.before_value != .after_value)|{id,before_value,after_value,probability_delta_over_tolerance}],
    yes_no_probability,mismatched_label,changed}' \
  | mustmatch '{"paired":24,"compared":24,"same":21,"changed_values":3,"flips":{"yes to no":["C-06","C-12","C-18"]},"value_changes":[{"id":"C-06","before_value":true,"after_value":false,"probability_delta_over_tolerance":true},{"id":"C-12","before_value":true,"after_value":false,"probability_delta_over_tolerance":true},{"id":"C-18","before_value":true,"after_value":false,"probability_delta_over_tolerance":true}],"yes_no_probability":{"tolerance":0.08,"compared":24,"changed":24,"summarized_same_value":2,"largest_summarized_delta":0.06},"mismatched_label":[],"changed":{"question":true,"question_by":"digest","model":false,"threshold":false}}'

jq -s -r '[.[0].meta.question_sha256[0:8]] | join("")' "$work/draft.jsonl" > "$work/a"
jq -s -r '[.[0].meta.question_sha256[0:8]] | join("")' "$work/tuned.jsonl" > "$work/b"
{ cat "$work/a" "$work/b"; } | mustmatch "ad25f6f9
c2c9a714"

sh ../../transforms/compare/example.sh | jq -c '{paired, same}' | mustmatch '{"paired":40,"same":36}'
```

Twenty-four rows compare, and three values moved from yes to no. `changes` shows those values and same-value probability movements above 0.08. Both runs print the same question text, but their digests differ. The last line runs the transform.

A fair pair is two live runs or two replays over the same cases. Never mix a live run with a replay. The 0.08 default is the largest movement observed in one repeated yes-or-no run over one model, question, set, and day. It is a reporting tolerance, not a regression boundary. A move just above it does not prove a regression.

## Step 4: let audit write the cut and the model

`audit --write` grades the draft's rows against the labels and writes its steady cut into the file.

```bash
set -euo pipefail
work=$(mktemp -d); trap 'rm -rf -- "$work"' EXIT

cp draft.json "$work/draft.json"
thinkthen decide @"$work/draft.json" --jsonl --field /body --details \
  --replay recording/ < claims.jsonl > "$work/draft.jsonl"
jq -c '{id, value: .label}' claims.jsonl > "$work/key.jsonl"

thinkthen audit "$work/draft.jsonl" "$work/key.jsonl" --write "$work/draft.json" 2>&1 >/dev/null \
  | mustmatch "thinkthen: audit: wrote threshold 0.57 for the question; it was 0.5"
jq -c '{threshold, model}' "$work/draft.json" | mustmatch '{"threshold":0.57,"model":"jev-1.13.0"}'
```

A file tuned by `audit --write` names the model its bar was tuned on. Rerun the labeled set, and tune again, whenever that model changes.

## What can go wrong

- **Tuning against reported cases.** These claims show direction. Hold cases back before quoting accuracy.
- **Editing wording on the command line.** Then the gate and measurement ask different questions.
- **Reading `changed.question` alone.** `changed` also names model and cut. Rerun when two move. Modern rows use the complete question digest; legacy rows name their text fallback.
- **Reading the flips before the pairing.** `paired`, `only_in_before`, `only_in_after`, `repeated_ids`, and the two mismatch lists say the runs measured the same cases. A run that stopped early is listed there, not passed off as a smaller run.
- **Folding unresolved into no.** The six directions stay separate.
- **A gate that reads exit codes loosely.** Word the question so yes permits the action. Treat every other code as refusal.

## Related how-tos

- [How to say what yes and no mean](../40-what-yes-and-no-mean/) is the option this page tunes with.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) tunes the cut instead of the wording.
- [How to know what a run cost](../28-what-a-run-cost/) reads the same rows for tokens.
