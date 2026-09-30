# How to route a request by how hard it is

Status: green

Verbs: `score`

Use this when incoming requests have to reach the right desk and the thing that decides is how much work each one is. `score` places a request on levels you name and prints one number. `jq -e` turns that number into an exit code, and an ordinary `if` routes on it.

```bash
set -euo pipefail

thinkthen score 'How hard is this request to answer?' \
  'A canned reply answers it.' \
  'One person can answer it after a look at the account.' \
  'It needs a specialist and more than one system.' \
  --replay recording/ < requests/tax-split.txt \
  | mustmatch "1.87"
```

## Input

`requests/` holds four short customer messages: a copy of a receipt, a reset link that never arrives, a seat moving to another address, and a quarter of invoices to be reissued under a new name. The question comes first and the levels follow it, lowest first. The number runs from 0 at the lowest level to the number of levels minus one at the highest, and it can land between two of them. The blocks below hold the levels in `levels`, which sends the same bytes.

`recording/thinkthen.jsonl` holds the answers of the four exchanges this page replays, so every command runs with no network and no key. `record.sh` made them through `sdlc/scripts/live`, and the numbers are the ones the model really returned.

## Step 1: send the hard ones to the specialist

`score` takes no threshold, and no answer of its sets an exit code. `jq -e` cuts on the number and sets one: it exits 1 when its last output is `false` or `null`, so the `if` reads like a shell test.

```bash
set -euo pipefail
levels=('A canned reply answers it.'
        'One person can answer it after a look at the account.'
        'It needs a specialist and more than one system.')

specialist() {
  thinkthen score 'How hard is this request to answer?' "${levels[@]}" \
    --replay recording/ < "$1" | jq -e '. >= 1.5' > /dev/null
}

for request in requests/tax-split.txt requests/password.txt; do
  if specialist "$request"; then
    printf 'specialist %s\n' "$(basename -- "$request")"
  else
    printf 'front desk %s\n' "$(basename -- "$request")"
  fi
done | mustmatch "specialist tax-split.txt
front desk password.txt"
```

Put the command in an `if`, because under `set -e` a bare `jq -e` that says no ends the script. The same numbers printed one per line and piped through `LC_ALL=C sort -rn` give a queue worked worst first, which needs no cut at all.

## Step 2: see what one number hides

`--details` carries the probability of every level. Two requests can share a number and not share a distribution, and the difference is what says whether a cut near them would hold.

```bash
set -euo pipefail
levels=('A canned reply answers it.'
        'One person can answer it after a look at the account.'
        'It needs a specialist and more than one system.')

for request in receipt two-seats; do
  thinkthen score 'How hard is this request to answer?' "${levels[@]}" \
    --details --replay recording/ < "requests/$request.txt" \
    | jq -r '[(.value|tostring), (.answer.probabilities | to_entries | map(.value|tostring) | join(",")),
              (.threshold|tostring)] | join(" ")'
done | mustmatch "0.87 0.13,0.87,0.0 null
0.9 0.14,0.82,0.04 null"
```

Those two numbers are three hundredths apart and do not mean the same thing. The receipt sits on one level with nothing at the top. The seat request leans on the same level and keeps four hundredths on "specialist", because moving a seat sometimes touches billing. A number near 1 can also come from a split, half on 0 and half on 2, and nothing in the number says which happened. The keys come back as the levels were typed, and `threshold` is `null` because `score` takes no rule.

## What can go wrong

- **No answer of `score` sets the exit code.** A run that printed a number exits 0 whatever the number was, and the cut lives in `jq`. Exit 2 is a usage error with nothing sent, 4 is the backend, and 5 is local.
- **`score` takes no `--threshold`, no `--quiet`, and no `--raw`.** Each is a usage error. Fewer than two levels, more than ten, a blank level, and a repeat are usage errors too.
- **A score is the weakest thing the model gives.** Measurement of the first System One model showed rubric scores rejecting 18% to 46% of work that people had accepted. A number belongs in a queue a person reads, and a gate that has to hold belongs in `decide` or `choose`.
- **Two runs over different level lists are two different scales.** Divide each number by the number of levels minus one before comparing them, and even then say what changed.
- **`sort -n` reads the locale.** `LC_ALL=C` keeps the decimal point a point.
- **The Bash way to branch on levels is `choose`** with the levels as ordered labels, because a label matches in a `case` and a number does not.

## Related how-tos

- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) branches on a level instead of cutting on a number.
- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the gate that `score` is not.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) measures a cut before it runs over a whole file.
