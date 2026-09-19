# 04 Review queue

Status: red

Verbs: `decide where`

A team acts on cancellation messages automatically and cannot afford a wrong guess. The sure rows flow on to the job that acts, and the unsure rows go to a file a person reads. Measurement says answers inside the unsure band flip between identical runs, so a row that landed there is a row to look at again.

## Input

`messages.jsonl` holds five customer messages with `id`, `from`, and `body`.

## Judge once, split in code

`--emit annotated` prints one row per input record, so nothing is lost and the split is a `jq` filter rather than a second model call.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide where 'the customer asks to end their subscription' \
  --input jsonl --on /body --id /id \
  --min-prob 0.9 --emit annotated --replay recording/ \
  < messages.jsonl > "$work/judged.jsonl"

wc -l < "$work/judged.jsonl" | tr -d ' ' | mustmatch "5"
jq -r '.result.assessment.status' "$work/judged.jsonl" | sort | uniq -c | tr -s ' ' \
  | mustmatch " 3 accepted
 2 unsure"
```

Three rows cleared the mark in one direction or the other and two did not.

## Publish two files

Each file is written to a temporary name and moved into place, so a reader never sees half a queue. The `act` file holds the accepted yes rows, unchanged.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
mkdir -p "$work/out"

thinkthen decide where 'the customer asks to end their subscription' \
  --input jsonl --on /body --id /id \
  --min-prob 0.9 --emit annotated --replay recording/ \
  < messages.jsonl > "$work/judged.jsonl"

jq -c 'select(.result.assessment.status == "accepted" and .result.assessment.value == true) | .input' \
  "$work/judged.jsonl" > "$work/act.tmp"
jq -c 'select(.result.assessment.status == "unsure") | {id: .input.id, from: .input.from, body: .input.body}' \
  "$work/judged.jsonl" > "$work/review.tmp"

mv -- "$work/act.tmp" "$work/out/act.jsonl"
mv -- "$work/review.tmp" "$work/out/review.jsonl"

jq -r '.id' "$work/out/act.jsonl" | mustmatch "MSG-01
MSG-04"
jq -r '.id' "$work/out/review.jsonl" | mustmatch "MSG-03"
```

## A second mark costs nothing

`judged.jsonl` holds the probability the backend gave, so a stricter policy is a `jq` filter and not another request.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide where 'the customer asks to end their subscription' \
  --input jsonl --on /body --id /id \
  --min-prob 0.9 --emit annotated --replay recording/ \
  < messages.jsonl > "$work/judged.jsonl"

jq -c 'select(.result.answer.probability >= 0.97) | .input.id' "$work/judged.jsonl" \
  | wc -l | tr -d ' ' | mustmatch "1"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **`--min-prob` should be optional under `--emit annotated`.** The last block re-judged the file at a different mark with no model call, which is the whole argument for saving probabilities. A user exploring a new question has no mark yet and should not have to invent one to see the numbers. The draft's recommendation under `where` holds: require it for `--emit input`, allow its absence for `result` and `annotated`.
- **The row guarantee is what makes this demo possible.** One row per record, input order kept, no row dropped for being unsure. Every split here is a `jq` filter over a stable stream. Nothing in records.md needs to change.
- **`--unknown` is inert under `--emit annotated` and the tool says nothing.** The draft states the fact and then accepts the flag. Demo 03 asks for the same fix: a flag that cannot act in the chosen mode is a usage error.
- **`where --emit annotated` is not a filter, and the name stops fitting.** In this demo `where` selects nothing. It judges every record and hands the selection to `jq`. That is the right shape for work that matters, and the verb still reads as a filter in the help. Smallest fix: the `where` help leads with the annotated form for work that must keep every record, and names the filter form as the shortcut it is.
- **An unsure row needs the probability beside it, and it has one.** `result.answer.probability` is on every annotated row, so the review file can be sorted by how close a row came to the mark. No new flag is needed.
