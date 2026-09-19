# How to act only when the answer is sure, and send the rest to a person

Absorbed by how-tos 19, 16, and 13: the band goes to 19, the review pile to 16, and the trade between coverage and accuracy to 13. ADR 0018 rules that this folder stays until 16 is green and then leaves.

Status: green

Verbs: `decide`

Use this when a job acts on records automatically and a wrong guess costs more than a slow one. A team reads cancellation messages and cancels accounts with nobody in the loop. `--threshold 0.1:0.9` leaves the middle unresolved, so one run over the whole file gives three piles instead of two.

```bash
set -euo pipefail

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --threshold 0.1:0.9 \
  --input messages.jsonl --replay recording/ \
  | mustmatch "true
false
null
false
false"
```

## Input

`messages.jsonl` holds five customer messages, one JSON object per line, each with `id`, `from`, and `body`. `--jsonl` makes each line one record, `--field /body` sends the message text alone, and one value prints per record in input order.

`null` is the third answer. It means the probability landed inside the band, and it is the only spelling of unresolved in the bare output, in the result object, and in the exit code table alike.

`recording/` holds the five exchanges this page replays, so every command runs with no network and no key. `record.sh` made them once through `sdlc/scripts/live`, and every probability here is what the model answered on 2026-09-19.

## Step 1: name the record each answer belongs to

The bare values above tie no line to a record, and zipping them back with `paste` is a trap: a run that stopped early prints a short file and the zip silently shifts. `--details` prints the result object, and in record mode that object carries `input`, the whole record as it arrived.

```bash
set -euo pipefail

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --threshold 0.1:0.9 --details \
  --input messages.jsonl --replay recording/ \
  | jq -c '{id: .input.id, value}' \
  | mustmatch '{"id":"MSG-01","value":true}
{"id":"MSG-02","value":false}
{"id":"MSG-03","value":null}
{"id":"MSG-04","value":false}
{"id":"MSG-05","value":false}'
```

`input` holds the whole record, `from` and `id` included, and only the `body` ever left the machine. A run over large records repeats them on every row.

## Step 2: publish three files

Each file is written to a temporary name and moved into place, so a reader never sees half a queue. `act` holds the original records, unchanged.

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

jq -r '.id' "$work/out/act.jsonl" | mustmatch "MSG-01"
jq -r '.id' "$work/out/keep.jsonl" | mustmatch "MSG-02
MSG-04
MSG-05"
jq -c '.' "$work/out/review.jsonl" \
  | mustmatch '{"id":"MSG-03","from":"noor","p":0.28}'
```

MSG-04 reads "Cancel the second seat, keep mine." The model answered 0.07, so it went to the keep pile. Cancelling part of an account is not ending a subscription, and the question as worded says so.

## What can go wrong

- **A refused record stops the run at exit 2, and the rows already printed stay printed.** A pointer that finds nothing is one, and nothing was sent for that record. [How to resume a long run that stopped](../12-keep-going/) reads the stop and pays for the rest alone.
- **No record's answer sets the exit code.** A record run exits 0 when it finished, whatever the answers were. Exit 1 and exit 3 belong to a run over one document, and a script reads the values from the rows.
- **The output after a stop is a prefix and not a finished dataset.** Count the rows against the records first. The line on standard error names the record by its number and never by its text.
- **Only the pointed value leaves the machine.** `--field` is the disclosure boundary, and the whole record under `--details` stays local. `--dry-run` prints what would go out and sends nothing.
- **A band is not a confidence.** The middle says the model reached neither mark on the evidence it was shown. Answers inside the band flip between runs, so a record that landed there is one to look at again.
- **The model judges the question as worded.** If a partial cancellation has to reach the act pile, that is a different question and a new measurement.

## Related how-tos

- [How to gate a script step on a yes/no answer](../01-refund-gate/) does the same judgment over one document.
- [How to gate a risky command and fail closed](../19-no-or-could-not-ask/) reads the failure codes in full.
- [How to check the judge against human labels](../25-check-the-judge/) reads rows like these with the `jq` transforms.
- [How to resume a long run that stopped](../12-keep-going/) picks a stopped run back up.
