# 04 Review queue

Status: red

Verbs: `decide`

A team acts on cancellation messages automatically and cannot afford a wrong guess. The sure rows flow on to the job that acts, and the unsure rows go to a file a person reads. Measurement says answers inside the unresolved band flip between identical runs, so a row that landed there is a row to look at again.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`messages.jsonl` holds five customer messages with `id`, `from`, and `body`.

## Judge once, split in code

`decide --jsonl` prints one answer per record in input order. `--details` prints the result object instead of the bare value, and in record mode that object carries `input`, the whole record as it arrived. Nothing is dropped, so the split is a `jq` filter and not a second model call.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --threshold 0.1:0.9 --details \
  --input messages.jsonl --replay recording/ > "$work/judged.jsonl"

wc -l < "$work/judged.jsonl" | tr -d ' ' | mustmatch "5"
jq -r '.value | tostring' "$work/judged.jsonl" | sort | uniq -c | tr -s ' ' \
  | mustmatch " 2 false
 1 null
 2 true"
```

Two rows cleared the high mark, two fell to or under the low one, and one landed between them.

## Publish three files

Each file is written to a temporary name and moved into place, so a reader never sees half a queue. The `act` file holds the original records, unchanged.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
mkdir -p "$work/out"

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --threshold 0.1:0.9 --details \
  --input messages.jsonl --replay recording/ > "$work/judged.jsonl"

jq -c 'select(.value == true)  | .input' "$work/judged.jsonl" > "$work/act.tmp"
jq -c 'select(.value == false) | .input' "$work/judged.jsonl" > "$work/keep.tmp"
jq -c 'select(.value == null)  | {id: .input.id, from: .input.from, p: .answer.probability}' \
  "$work/judged.jsonl" > "$work/review.tmp"

for pile in act keep review; do
  mv -- "$work/$pile.tmp" "$work/out/$pile.jsonl"
done

jq -r '.id' "$work/out/act.jsonl" | mustmatch "MSG-01
MSG-04"
jq -r '.id' "$work/out/review.jsonl" | mustmatch "MSG-03"
```

`select(.value == null)` is the third pile. The band put it there, and `null` is the only spelling of unresolved anywhere in the surface.

## A second mark costs nothing

The saved rows hold the probability the backend gave, so a stricter policy is a `jq` filter and not another request.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --threshold 0.1:0.9 --details \
  --input messages.jsonl --replay recording/ > "$work/judged.jsonl"

jq -c 'select(.answer.probability >= 0.97) | .input.id' "$work/judged.jsonl" \
  | wc -l | tr -d ' ' | mustmatch "1"
```

A run with no threshold at all saves the same numbers, and a user exploring a new question has no mark to invent.

```bash
set -euo pipefail

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --details \
  --input messages.jsonl --replay recording/ \
  | jq -r 'select(.value == null) | .input.id' \
  | wc -l | tr -d ' ' | mustmatch "0"
```

With no threshold the cut is 0.5, nothing is unresolved, and the row still carries the number. `threshold` reads `0.5` on every row.

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo confirms the band and the three piles.** One rule on the command line and three `jq` filters is less machinery than the old status field, and `null` means unresolved in the bare output, in the detail object, and in the exit code table alike.
- **The row guarantee is what makes this page possible.** One row per record, input order kept, no row dropped. ADR 0007 states it for `decide`, `choose`, and `score`, and the demo depends on it.
- **The demo could not name a record without `--details`.** `decide --jsonl` on its own prints `true`, `false`, and `null` with nothing tying a line to a record. Zipping with `paste` works and is a trap, because a run that stops at a failed record prints a short file and the zip silently shifts. Every honest record-mode script on this page reaches for `--details`. The demo argues the `--jsonl` help say so.
- **`input` holding the whole record is the right call and it doubles a large file.** The review file above carries three fields and the act file carries the record. A user with megabyte records pays for `input` on every row to get `answer.probability` on any row. The demo does not ask for a projection flag. It asks that the `--details` help name the cost.
- **Nothing on the run says how many rows landed in the band.** The counts come from `jq` after the fact. That is the right division of work and the demo confirms it.
