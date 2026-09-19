# How to tune a question file and use the same file in the gate

Status: green

Verbs: `decide`

A question worth trusting is measured against labeled cases first. Keeping it in a question file means the wording you measured is the wording the gate runs, byte for byte, and every row says which one it was.

```bash
set -euo pipefail

printf '%s' "$(jq -r 'select(.id == "C-01") | .body' claims.jsonl)" \
  | thinkthen decide @receipt.json --quiet --replay recording/ \
  && printf 'ask for the receipt\n' | mustmatch "ask for the receipt"
```

That is the gate. The rest of this page is how `receipt.json` earned it.

## Input

`claims.jsonl` holds twenty-four made-up expense claims with a trusted answer in `label`. The made-up office rule: a claim needs a receipt over 25 pounds, or for anything from an outside supplier whatever the amount.

`draft.json` is the first wording of the question and `receipt.json` is the tuned one. `recording/` holds the forty-eight live exchanges this page replays, and `record.sh` made them through `sdlc/scripts/live`.

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

Three claims were called yes that the rule calls no, and none went the other way. The draft is too willing, and it says nothing about the 25 pound line or about outside suppliers.

## Step 2: change the file and never the command

The fix goes in the file, under `true` and `false`. The command stays as it was.

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

`--dry-run` says which home each setting came from, so a run that behaves oddly can be read rather than guessed at. A value typed beside `@FILE` wins, and the plan names it.

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
  "$work/tuned.jsonl" | jq -c '{paired, same, flips, mismatched_label}' \
  | mustmatch '{"paired":24,"same":21,"flips":{"yes to no":["C-06","C-12","C-18"]},"mismatched_label":[]}'

jq -s -r '[.[0].meta.question_sha256[0:8]] | join("")' "$work/draft.jsonl" > "$work/a"
jq -s -r '[.[0].meta.question_sha256[0:8]] | join("")' "$work/tuned.jsonl" > "$work/b"
{ cat "$work/a" "$work/b"; } | mustmatch "ad25f6f9
c2c9a714"
```

Twenty-four pairs, the same case ids, the same labels, and three claims that moved from yes to no. Both runs print the same `question.text`, and their digests differ, because the two sentences that were added are part of the question and ride in the digest.

An automatic tuner drives this same loop by rewriting step 2, and every round it produces is traceable by its digest.

## What can go wrong

- **Tuning against the cases you then report on.** Twenty-four made-up claims show the direction. Hold cases back before anyone quotes the accuracy.
- **Editing the wording on the command line.** Then the gate and the measurement are two questions, and no row says so.
- **Reading `changed.question` from the comparison.** It watches the printed text, which did not move here. `meta.question_sha256` is the field that did.
- **A tuned file that misses the recording.** A changed text is a changed request, so `--replay` names the entry it cannot find and the page stops.
- **A gate that reads the exit code loosely.** Word the question so that yes permits the action, and treat every code other than 0 as a refusal.

## Related how-tos

- [How to say what yes and no mean](../40-what-yes-and-no-mean/) is the option this page tunes with.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) tunes the cut instead of the wording.
- [How to know what a run cost](../28-what-a-run-cost/) reads the same rows for tokens.
