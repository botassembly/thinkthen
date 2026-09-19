# How to act only when the answer is sure, and send the rest to a person

Status: green

Verbs: `decide`

Use this when a job acts on records automatically and a wrong guess costs more than a slow one. A team reads cancellation messages and cancels accounts without a person in the loop. With one cut every message lands in one of two piles and nothing says which ones were close. A band adds a third answer, and the records that land in it go to a file a person reads. One run over the whole file gives all three piles.

## Input

`messages.jsonl` holds five customer messages, one JSON object per line, each with `id`, `from`, and `body`.

`recording/` holds the five exchanges this page replays, so every command here runs with no network and no key. `record.sh` made them once through `sdlc/scripts/live`. Every probability on this page is what the model answered on 2026-09-19.

## Step 1: judge every record in one run

`--jsonl` makes each line one record, `--field /body` sends the message text and leaves the rest of the record at home, and `--threshold 0.1:0.9` leaves the middle unresolved. One value prints per record, in input order.

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

`null` is the third answer. It means the probability landed inside the band, and it is the only spelling of unresolved in the bare output, in the result object, and in the exit code table alike.

## Step 2: name the record each answer belongs to

The bare values above tie no line to a record. Counting lines and zipping them back with `paste` is a trap, because a run that stops at a failed record prints a short file and the zip silently shifts. `--details` prints the result object instead, and in record mode that object carries `input`, the whole record as it arrived.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --threshold 0.1:0.9 --details \
  --input messages.jsonl --replay recording/ > "$work/judged.jsonl"

jq -c '{id: .input.id, value}' "$work/judged.jsonl" \
  | mustmatch '{"id":"MSG-01","value":true}
{"id":"MSG-02","value":false}
{"id":"MSG-03","value":null}
{"id":"MSG-04","value":false}
{"id":"MSG-05","value":false}'
```

`input` holds the whole record, `from` and `id` included, and only the `body` ever left the machine. It repeats every record in the output, so a run over large records pays for that on every row.

## Step 3: publish three files

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

jq -r '.id' "$work/out/act.jsonl" | mustmatch "MSG-01"
jq -r '.id' "$work/out/keep.jsonl" | mustmatch "MSG-02
MSG-04
MSG-05"
jq -c '.' "$work/out/review.jsonl" \
  | mustmatch '{"id":"MSG-03","from":"noor","p":0.28}'
```

MSG-04 reads "Cancel the second seat, keep mine." The model answered 0.07, so it went to the keep pile. A message that cancels part of an account is not a message that ends a subscription, and the question as worded says so. MSG-05 asks about pausing a plan and answered 0.08. Only MSG-01 cleared the high mark.

## Step 4: count the run

The rows are the input the `jq` transforms read. `transforms/counts` counts the three answers and reports what the file holds, so a file that concatenated two runs could not pass as one.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --threshold 0.1:0.9 --details \
  --input messages.jsonl --replay recording/ > "$work/judged.jsonl"

jq -n -f ../../transforms/counts/counts.jq "$work/judged.jsonl" \
  | jq -c '.' \
  | mustmatch '{"rows":5,"yes":1,"no":3,"unresolved":1,"thresholds":["0.1:0.9"],"questions":["Does the customer ask to end their subscription?"]}'
```

## Step 5: change the mark without asking again

The saved rows hold the probability the backend gave, so a stricter policy is a `jq` filter and not another request.

```bash
set -euo pipefail

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --threshold 0.1:0.9 --details \
  --input messages.jsonl --replay recording/ \
  | jq -r 'select(.answer.probability >= 0.97) | .input.id' \
  | mustmatch "MSG-01"
```

A run with no threshold at all saves the same numbers. The cut is then 0.5, nothing is unresolved, and a user exploring a new question has no mark to invent.

```bash
set -euo pipefail

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --details \
  --input messages.jsonl --replay recording/ \
  | jq -r 'select(.value == null) | .input.id' \
  | wc -l | tr -d ' ' | mustmatch "0"
```

## What can go wrong

- **No record's answer sets the exit code.** A record run exits 0 when it finished, whatever the answers were. Exit 1 and exit 3 belong to a run over one document. A script reads the values from the rows.
- **Exit 2 is a record the tool refused, and it sent nothing for that record.** A pointer that finds nothing is one. The run stops there, and the rows already printed stay printed.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

printf '%s\n%s\n' \
  '{"id":"MSG-01","body":"Please cancel my account at the end of the month. I do not want to be charged again."}' \
  '{"id":"MSG-99","note":"no body at all"}' \
  | thinkthen decide 'Does the customer ask to end their subscription?' \
      --jsonl --field /body --threshold 0.1:0.9 --replay recording/ \
      > "$work/out.txt" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
cat "$work/out.txt" | mustmatch "true"
cat "$work/err.txt" | mustmatch "thinkthen: the record holds nothing at \`/body\`
thinkthen: stopped at record 2; 1 records finished, 1 from a recording"
```

- **The output after a stop is a prefix and not a finished dataset.** Count the rows against the records before acting on them. The line on standard error says how many finished.
- **The standard-error line names the record by its number and never by its text.** A record is evidence, and a diagnostic is read by a person over someone's shoulder.
- **Only the pointed value leaves the machine.** `--field` is the disclosure boundary. `--details` still carries the whole record in `input`, and that copy stays local.

```bash
set -euo pipefail

thinkthen decide 'Does the customer ask to end their subscription?' \
  --jsonl --field /body --input messages.jsonl --dry-run \
  | jq -c '{input, state: .request.state}' \
  | mustmatch '{"input":{"framing":"jsonl","field":["/body"]},"state":"Please cancel my account at the end of the month. I do not want to be charged again."}'
```

- **A band is not a confidence.** The middle says the model did not reach either mark on the evidence it was shown. Measurement says answers inside the band flip between runs, so a record that landed there is a record to look at again.
- **The model judges the question as worded.** "Cancel the second seat, keep mine." answered 0.07 here. If a partial cancellation has to reach the act pile, that is a different question and a different measurement.

## Related how-tos

- [How to gate a script step on a yes/no answer](../01-refund-gate/) does the same judgment over one document.
- [How to tell "no" from "could not ask"](../19-no-or-could-not-ask/) reads the failure codes in full.
- [How to check the judge against human labels](../25-check-the-judge/) reads rows like these with the `jq` transforms.
