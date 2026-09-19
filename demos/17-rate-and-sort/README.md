# How to rate on a scale, sort by it, and test it with `jq -e`

Status: green

Verbs: `score`

Use this when a queue of reports needs an order rather than a verdict. `score` places each report on levels you name and prints one number, so `sort -rn` puts the worst first and `jq -e` turns the number into an exit code. Use it to order a review queue a person reads, and never as a gate that has to hold.

The recording under `recording/` holds the four live exchanges this page replays, one per report. The numbers below are the ones the model really returned.

## Input

`reports/` holds four short bug reports: a broken nightly import, an export button that fails in one browser, a search filter that ignores the year, and a footer that sits four pixels low.

## Rate one report

The question comes first and the levels follow it, lowest first. The number runs from 0 at the lowest level to the number of levels minus one at the highest, and it can land between two of them.

```bash
set -euo pipefail

thinkthen score 'How much disruption does this report?' \
  'No disruption; nothing stops working.' \
  'Work continues, because a workaround exists.' \
  'Work is blocked, and no workaround exists.' \
  --replay recording/ < reports/nightly-import.txt \
  | mustmatch "2.0"
```

The import report reaches the top level. The footer report sits at the bottom.

```bash
set -euo pipefail

thinkthen score 'How much disruption does this report?' \
  'No disruption; nothing stops working.' \
  'Work continues, because a workaround exists.' \
  'Work is blocked, and no workaround exists.' \
  --replay recording/ < reports/footer-spacing.txt \
  | mustmatch "0.01"
```

## Sort the queue by the number

One line per report, the number first, and `sort -rn` does the rest.

```bash
set -euo pipefail

for report in reports/*.txt; do
  number=$(
    thinkthen score 'How much disruption does this report?' \
      'No disruption; nothing stops working.' \
      'Work continues, because a workaround exists.' \
      'Work is blocked, and no workaround exists.' \
      --replay recording/ < "$report"
  )
  printf '%s\t%s\n' "$number" "$(basename -- "$report")"
done | LC_ALL=C sort -rn | cut -f2 | mustmatch "nightly-import.txt
export-button.txt
search-filter.txt
footer-spacing.txt"
```

The order is the one a person would give. The number is a position on the levels this page named, so a run over a different list of levels is a different scale and the two orders cannot be mixed.

## Page on a number with `jq -e`

`score` takes no threshold, and no answer of its sets an exit code. `jq -e` cuts on the number in one line and sets one.

```bash
set -euo pipefail

page_oncall() {
  thinkthen score 'How much disruption does this report?' \
    'No disruption; nothing stops working.' \
    'Work continues, because a workaround exists.' \
    'Work is blocked, and no workaround exists.' \
    --replay recording/ < "$1" \
    | jq -e '. >= 2' > /dev/null
}

for report in reports/nightly-import.txt reports/search-filter.txt; do
  if page_oncall "$report"; then
    printf 'page %s\n' "$(basename -- "$report")"
  else
    printf 'queue %s\n' "$(basename -- "$report")"
  fi
done | mustmatch "page nightly-import.txt
queue search-filter.txt"
```

`jq -e` exits 1 when its last output is `false` or `null`, so the `if` reads the way a shell test reads. Put the command in an `if`, because under `set -e` a bare `jq -e` that says no ends the script.

## One number hides the shape

`--details` carries the probability of every level and the backend's own confidence. Two reports can share a number and not share a distribution.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

for report in export-button search-filter; do
  thinkthen score 'How much disruption does this report?' \
    'No disruption; nothing stops working.' \
    'Work continues, because a workaround exists.' \
    'Work is blocked, and no workaround exists.' \
    --details --replay recording/ < "reports/$report.txt" \
    > "$work/$report.json"
done

jq -r '[.value, (.answer.probabilities | to_entries | map(.value) | join(","))] | join(" ")' \
  "$work/export-button.json" | mustmatch "1.01 0.0,0.99,0.01"
jq -r '[.value, (.answer.probabilities | to_entries | map(.value) | join(","))] | join(" ")' \
  "$work/search-filter.json" | mustmatch "1.0 0.0,1.0,0.0"

jq -r '.answer.level' "$work/export-button.json" \
  | mustmatch "Work continues, because a workaround exists."
jq -r '.threshold' "$work/search-filter.json" | mustmatch "null"
```

Those two numbers are a hundredth apart and they mean different things. The search filter sits on one level with nothing anywhere else. The export button leans on the same level and keeps a hundredth on "blocked", because a few people have no other browser. A number of 1 can also come from a split, with half the probability on 0 and half on 2, and nothing in the number says which happened. Read `--details` when that difference would change what you do.

The keys of `answer.probabilities` are the levels as they were typed, in the order they were sent, and `threshold` is `null` because `score` takes no rule.

Every `thinkthen` line carries `--replay recording/`, so the page touches no network and reads no key. `record.sh` made the four exchanges once, through `sdlc/scripts/live`.

## What can go wrong

| Exit code | What happened | What to do |
| --- | --- | --- |
| 0 | A number was printed | Sort on it or cut on it |
| 2 | A usage error: fewer than two levels, more than ten, a blank level, a repeat, or `--threshold` | Fix the command line. Nothing was sent |
| 4 | The backend failed, or the adapter refused the reply | Retry or stop. It is not a rating |
| 5 | A local failure: the recording folder, standard input | Fix the machine |

- `score` takes no `--threshold`, no `--quiet`, and no `--raw`. Each of them is a usage error, and `jq -e` does the cutting.
- No answer of `score` sets the exit code, so a run that printed a number exits 0 whatever the number was. The cut lives in `jq`.
- Rating is the weakest thing a decider model does. Measurement of the first decider model showed rubric judgments rejecting 18% to 46% of work that people had accepted. A number belongs in a queue a person reads. A gate that has to hold belongs in `decide` or `choose`.
- Two runs over different level lists are two different scales. Divide each number by the number of levels minus one before comparing them, and even then say what changed.
- `sort -n` reads the shell's locale. `LC_ALL=C` keeps the decimal point a point.
- The Bash way to branch on levels is `choose` with the levels as ordered labels, because a label matches in a `case` and a number does not.

## Related how-tos

- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) is the way to branch on a level rather than sort by it.
- [How to sort files into folders by label](../05-sort-a-folder/) files a folder instead of ordering it.
- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the gate that `score` is not.
