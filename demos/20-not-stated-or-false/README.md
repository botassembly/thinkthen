# How to tell "not stated" from "false" with `decide` and `choose`

Status: green

Verbs: `decide`, `choose`

Use this when a script checks a claim against a document and the difference between "the document denies it" and "the document never mentions it" changes what you do. A yes/no question answers both alike. A pick with labels keeps them apart.

```bash
set -euo pipefail

thinkthen choose 'How does this notice treat the claim that the upgrade window runs longer than the original plan?' \
  supported contradicted not_stated ambiguous \
  --raw --threshold 0.8 --replay recording/ < notices/silent.txt \
  | mustmatch "not_stated"
```

## Input

`notices/` holds three maintenance notices about one upgrade window. `extended.txt` says it runs longer than planned, `shortened.txt` says it closes earlier, and `silent.txt` says nothing about the length. The claim under test never changes.

`recording/` holds the six live exchanges this page replays, one pick and one yes/no answer per notice, and `record.sh` made them through `sdlc/scripts/live`.

## Step 1: see what a yes/no question cannot say

`decide` prints `true` or `false` and exits 1 on a no. The block captures the judgment first and pipes it second, because under `set -o pipefail` that code would end the pipeline.

```bash
set -euo pipefail

claim='the upgrade window runs longer than the original plan'

for notice in notices/extended.txt notices/shortened.txt notices/silent.txt; do
  judgment=$(
    thinkthen decide "Does this notice say that $claim?" \
      --details --replay recording/ < "$notice"
  ) && rc=0 || rc=$?
  printf '%s %s ' "$(basename -- "$notice")" "$rc"
  printf '%s' "$judgment" | jq -r '[.value, (.answer.probability | tostring)] | join(" ")'
done | mustmatch "extended.txt 0 true 0.98
shortened.txt 1 false 0.01
silent.txt 1 false 0.04"
```

Two notices answered `false` and they do not mean the same thing. One denies the claim and one never raises it. The exit code says the same twice, and at 0.01 and 0.04 no mark separates denial from silence. A low probability says the model is confident, and nothing about why.

## Step 2: ask the question that has four answers

`supported`, `contradicted`, `not_stated`, and `ambiguous` cover what a document can do with a claim, and each is a branch a script can write.

```bash
set -euo pipefail

claim='the upgrade window runs longer than the original plan'

for notice in notices/extended.txt notices/shortened.txt notices/silent.txt; do
  label=$(
    thinkthen choose "How does this notice treat the claim that $claim?" \
      supported contradicted not_stated ambiguous \
      --raw --threshold 0.8 --replay recording/ < "$notice"
  ) && rc=0 || rc=$?
  case $rc in
    0) ;;
    3) label=unresolved ;;
    *) printf 'choose failed on %s: %d\n' "$notice" "$rc" >&2; exit "$rc" ;;
  esac
  printf '%s %s\n' "$(basename -- "$notice")" "$label"
done | mustmatch "extended.txt supported
shortened.txt contradicted
silent.txt not_stated"
```

Three notices, three labels. `not_stated` reaches the branch that asks the author, which the yes/no version could not. A gap in the evidence is filled rather than acted on.

## Step 3: read the odds behind a label

`--details` carries a probability for every option, in the order they were sent, so the list the script typed is visibly the list the model saw. A notice that says nothing puts its weight on `not_stated`, and what is left says where the model was tempted.

A mark on the winner is one rule, and a margin over the runner-up is another. The two disagree on real files and both are honest, so a desk writes its margin in `jq`, needs no option for it, and measures the rule it picked.

```bash
set -euo pipefail

thinkthen choose 'How does this notice treat the claim that the upgrade window runs longer than the original plan?' \
  supported contradicted not_stated ambiguous \
  --details --replay recording/ < notices/silent.txt \
  | jq -r '(.answer.probabilities | to_entries | sort_by(-.value)) as $p
           | [(.answer.probabilities | keys_unsorted | join(",")), .value,
              ($p[0].value - $p[1].value | . * 100 | round / 100),
              (if $p[0].value - $p[1].value >= 0.2 then "clear" else "close" end)] | join(" ")' \
  | mustmatch "supported,contradicted,not_stated,ambiguous not_stated 0.81 clear"
```

## What can go wrong

- **Exit 1 belongs to `decide` alone, and covers denial and silence at once.** `choose` never exits 1, and [how to branch on a label](../02-route-a-ticket/) holds the rest of its codes.
- "Does the document establish X?" and "Is X true?" differ. Ask what the document does, and let the labels carry the answers.
- `not_stated` needs to be typed. Nothing adds it, and a list without it forces a pick between `supported` and `contradicted` when neither holds.
- `ambiguous` and an unresolved answer differ. The first is the model naming a hedge; the second is the tool saying no option cleared the mark.
- A high probability says nothing about facts the model never saw. `not_stated` at 0.9 means the notice does not say it, and never that the window is on schedule.
- Under `set -e` a `decide` that says no ends the script, and so does an unresolved `choose`. Capture it with `&& rc=0 || rc=$?`.

## Related how-tos

- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) is the two-`case` shape on a routing question.
- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the yes/no form, with a band for the middle.
- [How to choose the next action from a list that changes at every step](../21-options-from-the-record/) takes the labels from each record.
