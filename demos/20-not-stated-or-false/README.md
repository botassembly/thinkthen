# How to tell "not stated" from "false"

Status: green

Verbs: `choose`, `decide`

Use this when a script checks a claim against a document and the difference between "the document denies it" and "the document never mentions it" would change what you do. A yes/no question answers one of those two the same way it answers the other. A pick with labels such as `supported`, `contradicted`, and `not_stated` keeps them apart.

The recording under `recording/` holds the six live exchanges this page replays: one pick and one yes/no answer for each of three notices.

## Input

`notices/` holds three maintenance notices about the same upgrade window. `extended.txt` says the window runs longer than planned. `shortened.txt` says it closes earlier. `silent.txt` says nothing at all about the length.

The claim under test is the same in every run: the upgrade window runs longer than the original plan.

## What a yes/no question can say

`decide` prints `true` or `false`. Run it on all three notices.

```bash
set -euo pipefail

claim='the upgrade window runs longer than the original plan'

for notice in notices/extended.txt notices/shortened.txt notices/silent.txt; do
  answer=$(
    thinkthen decide "Does this notice say that $claim?" \
      --replay recording/ < "$notice"
  ) && rc=0 || rc=$?
  printf '%s %s %s\n' "$(basename -- "$notice")" "$answer" "$rc"
done | mustmatch "extended.txt true 0
shortened.txt false 1
silent.txt false 1"
```

Two notices answered `false` and they do not mean the same thing. One denies the claim and one never raises it. The exit code says the same thing twice, so a script branching on `$?` alone cannot tell them apart.

The probabilities are close together too. One is 0.01 and the other is 0.04, and no mark on that scale separates a denial from a silence.

```bash
set -euo pipefail

claim='the upgrade window runs longer than the original plan'

for notice in notices/shortened.txt notices/silent.txt; do
  judgment=$(
    thinkthen decide "Does this notice say that $claim?" \
      --details --replay recording/ < "$notice"
  ) && rc=0 || rc=$?
  printf '%s' "$judgment" \
    | jq -r '[.value, (.answer.probability | tostring)] | join(" ")'
done | mustmatch "false 0.01
false 0.04"
```

A low probability means the model is confident the notice does not say the claim. It says nothing about why.

`decide` exits 1 on a "no", and under `set -o pipefail` that code ends a pipeline that `jq` would otherwise have finished. Capture the judgment first and pipe it second, as the block does.

## Ask the question that has three answers

`choose` takes the labels the job needs. `supported`, `contradicted`, `not_stated`, and `ambiguous` cover the four things a document can do with a claim, and each one is a separate branch.

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

Three notices, three different labels, and each one is a branch a script can write.

## Branch on the three answers

```bash
set -euo pipefail

claim='the upgrade window runs longer than the original plan'

verdict() {
  label=$(
    thinkthen choose "How does this notice treat the claim that $claim?" \
      supported contradicted not_stated ambiguous \
      --raw --threshold 0.8 --replay recording/ < "$1"
  ) && rc=0 || rc=$?
  case ${rc:-0} in
    0) ;;
    3) label=unresolved ;;
    *) return "$rc" ;;
  esac
  case $label in
    supported)   printf 'publish\n' ;;
    contradicted) printf 'correct the draft\n' ;;
    not_stated)  printf 'ask the author\n' ;;
    ambiguous|unresolved) printf 'send to a person\n' ;;
    *) printf 'unknown label: %s\n' "$label" >&2; return 2 ;;
  esac
}

for notice in notices/extended.txt notices/shortened.txt notices/silent.txt; do
  printf '%s: %s\n' "$(basename -- "$notice")" "$(verdict "$notice")"
done | mustmatch "extended.txt: publish
shortened.txt: correct the draft
silent.txt: ask the author"
```

"Ask the author" is the branch the yes/no version could not reach. A missing statement is a gap in the evidence, and a gap is fixed by getting more evidence rather than by acting on a denial that was never made.

## Read the odds behind a label

`--details` carries a probability for every option sent. A notice that says nothing puts its weight on `not_stated`, and what is left over says where the model was tempted.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

claim='the upgrade window runs longer than the original plan'

thinkthen choose "How does this notice treat the claim that $claim?" \
  supported contradicted not_stated ambiguous \
  --details --replay recording/ < notices/silent.txt \
  > "$work/silent.json"

jq -r '.answer.probabilities | keys_unsorted | join(",")' "$work/silent.json" \
  | mustmatch "supported,contradicted,not_stated,ambiguous"
jq -r '.value' "$work/silent.json" | mustmatch "not_stated"
jq -e '.answer.probabilities.not_stated > .answer.probabilities.contradicted' \
  "$work/silent.json" > /dev/null && printf 'silence beats denial\n' \
  | mustmatch "silence beats denial"
```

The keys come back in the order the options were sent, so a reader can see that the list the script typed is the list the model saw.

Every `thinkthen` line carries `--replay recording/`, so the page touches no network and reads no key. `record.sh` made the six exchanges once, through `sdlc/scripts/live`.

## What can go wrong

| Exit code | What happened | What to do |
| --- | --- | --- |
| 0 | A label was returned | Branch on it |
| 1 | `decide` only: the answer is no | Remember that "no" covers denial and silence at once |
| 2 | A usage error: a blank option, one option, a repeat, or a band on `--threshold` | Fix the command line. Nothing was sent |
| 3 | The winner fell under the mark, or the top two tied exactly | Send the notice to a person |
| 4 | The backend failed, or the adapter refused the reply | Retry or stop |
| 5 | A local failure: the recording folder, standard input | Fix the machine |

- "Does the document establish X?" and "Is X true?" are different questions. Word the question so that it asks what the document does, and let the labels carry the answers.
- `not_stated` needs to be typed. Nothing adds it, and a list without it forces the model to pick between `supported` and `contradicted` when neither is true.
- `ambiguous` and an unresolved answer are different. The first is the model naming a hedge in the document; the second is the tool saying no option cleared the mark. Both usually end at a person, and a page that counts them should keep them apart.
- A high probability says nothing about facts the model was never shown. `not_stated` at 0.9 means the notice does not say it, and never that the window is on schedule.
- Under `set -e` a `decide` that says no ends the script, and so does an unresolved `choose`. Capture the code with `&& rc=0 || rc=$?`.

## Related how-tos

- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) is the same two-`case` shape on a routing question.
- [Refund gate](../01-refund-gate/) is the yes/no form, with a band for the middle.
- [How to sort files into folders by label](../05-sort-a-folder/) runs a pick over a whole folder.
