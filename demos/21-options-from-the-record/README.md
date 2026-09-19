# How to choose from a list that differs for every record

Status: green

Verbs: `choose`

Use this when each record carries the options it may be labeled with. A returns desk files every note under a code, and each desk owns its own set of codes. One command line cannot name them, because the list changes from record to record. `--options POINTER` reads the list out of each record and asks that record its own question.

## Input

`notes.jsonl` holds three notes, one JSON object per line, each with `id`, `desk`, `note`, and `codes`. The first two records hold their codes as a list of names. The third one holds them as an object, where each name carries a sentence that says what the code means.

`recording/` holds the three exchanges this page replays, so every command here runs with no network and no key. `record.sh` made them once through `sdlc/scripts/live`. Every probability on this page is what the model answered on 2026-09-19.

## Step 1: label every note with its own codes

`--field /note` sends the note text and leaves the rest of the record at home. `--options /codes` reads the list from the same record. One value prints per record, in input order.

```bash
set -euo pipefail

thinkthen choose 'Which of these codes fits the note?' \
  --jsonl --field /note --options /codes \
  --input notes.jsonl --replay recording/ \
  | mustmatch '"late"
"wrong_item"
"double_charge"'
```

Each record was asked about its own four codes. `--raw` prints the label with no quotes, for a `case` branch or a folder name.

```bash
set -euo pipefail

thinkthen choose 'Which of these codes fits the note?' \
  --jsonl --field /note --options /codes --raw \
  --input notes.jsonl --replay recording/ \
  | mustmatch "late
wrong_item
double_charge"
```

## Step 2: see the options each record was asked about

`--details` prints the result object. `question.options` holds the list that record was asked about, so a row says what it chose between.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen choose 'Which of these codes fits the note?' \
  --jsonl --field /note --options /codes --details \
  --input notes.jsonl --replay recording/ > "$work/judged.jsonl"

jq -c '{id: .input.id, value, options: .question.options}' "$work/judged.jsonl" \
  | mustmatch '{"id":"N-1","value":"late","options":["late","never_arrived","wrong_address","other"]}
{"id":"N-2","value":"wrong_item","options":["wrong_item","damaged","missing_part","other"]}
{"id":"N-3","value":"double_charge","options":["double_charge","wrong_amount","refund_late","other"]}'
```

The probabilities cover the options of that record alone and add to one.

```bash
set -euo pipefail

thinkthen choose 'Which of these codes fits the note?' \
  --jsonl --field /note --options /codes --details \
  --input notes.jsonl --replay recording/ \
  | jq -c '{id: .input.id, p: .answer.probabilities[.value]}' \
  | mustmatch '{"id":"N-1","p":1.0}
{"id":"N-2","p":1.0}
{"id":"N-3","p":1.0}'
```

Every note here names its own code in plain words, and the model answered 1.0 on all three. A note that fits two codes lands lower, and `--threshold` then cuts on the winning probability.

## Step 3: give each code a sentence

An object under the pointer carries a description per code. The names are the options, and the sentences reach the backend beside them. `--dry-run` sends nothing and prints what would go out.

```bash
set -euo pipefail

sed -n '3p' notes.jsonl \
  | thinkthen choose 'Which of these codes fits the note?' \
      --jsonl --field /note --options /codes --dry-run \
  | jq -c '.request.questions.q1.criteria' \
  | mustmatch '{"double_charge":"The same order was paid for more than once.","wrong_amount":"The amount charged is not the amount on the order.","refund_late":"A refund was promised and has not arrived.","other":"None of the three above fits this note."}'
```

A list of names sends the same shape with `null` under each name.

```bash
set -euo pipefail

sed -n '1p' notes.jsonl \
  | thinkthen choose 'Which of these codes fits the note?' \
      --jsonl --field /note --options /codes --dry-run \
  | jq -c '.request.questions.q1.criteria' \
  | mustmatch '{"late":null,"never_arrived":null,"wrong_address":null,"other":null}'
```

The value printed on standard output is the name in both forms. A description steers the model and never becomes an answer.

## Step 4: count the codes across the file

The rows are ordinary JSON, so the tally is `jq`.

```bash
set -euo pipefail

thinkthen choose 'Which of these codes fits the note?' \
  --jsonl --field /note --options /codes --details \
  --input notes.jsonl --replay recording/ \
  | jq -r '[.input.desk, .value] | @tsv' \
  | mustmatch "delivery	late
item	wrong_item
billing	double_charge"
```

## What can go wrong

- **`--options` takes the whole list from the record, and the command line gives none.** A run that names options on the command line and a pointer at once is refused before any request.

```bash
set -euo pipefail

thinkthen choose 'Which of these codes fits the note?' late other \
  --jsonl --field /note --options /codes \
  --input notes.jsonl --replay recording/ 2>&1 && rc=0 || rc=$?

printf 'rc=%s\n' "${rc:-0}" | mustmatch "rc=2"
```

- **`--options` needs `--jsonl`.** A pointer needs a JSON record to point into, and a run over one document has none.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

printf 'The box turned up late.\n' \
  | thinkthen choose 'Which of these codes fits the note?' \
      --options /codes --replay recording/ 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
cat "$work/err.txt" \
  | mustmatch "thinkthen: --options needs --jsonl, because a pointer needs a JSON record to point into"
```

- **A record whose list is missing or too short stops the run, and it sends nothing for that record.** `choose` takes 2 to 255 options. The rows already printed stay printed.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

printf '%s\n%s\n' \
  '{"id":"N-8","note":"Where is my order?","tags":["late"]}' \
  '{"id":"N-9","note":"Where is my order?","codes":["late"]}' \
  > "$work/broken.jsonl"

thinkthen choose 'Which of these codes fits the note?' \
  --jsonl --field /note --options /codes \
  --input "$work/broken.jsonl" --replay recording/ \
  > "$work/out.txt" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
wc -c < "$work/out.txt" | tr -d ' ' | mustmatch "0"
cat "$work/err.txt" | mustmatch "thinkthen: the record holds nothing at \`/codes\`
thinkthen: stopped at record 1; 0 records finished, 0 from a recording"
```

- **A code that holds a control character is refused.** A record comes from somewhere else, and `--raw` prints the label byte for byte. A label holding a newline or an escape sequence would inject a line into the caller's script. The message never quotes the label.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

printf '%s\n' '{"id":"N-7","note":"Where is my order?","codes":["late","ok\nrm -rf /","other"]}' \
  | thinkthen choose 'Which of these codes fits the note?' \
      --jsonl --field /note --options /codes --replay recording/ 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
cat "$work/err.txt" \
  | mustmatch "thinkthen: an option or a level is one line of printable text
thinkthen: stopped at record 1; 0 records finished, 0 from a recording"
```

- **Every distinct list is a distinct request.** A recording entry is named by the request body, and the body carries the options. Two records with the same text and different codes each pay once.
- **Only the pointed values leave the machine.** `--field` and `--options` are the disclosure boundary together. The rest of the record stays local, and `--details` carries the whole record back in `input` from the copy the tool never sent.

```bash
set -euo pipefail

sed -n '1p' notes.jsonl \
  | thinkthen choose 'Which of these codes fits the note?' \
      --jsonl --field /note --options /codes --dry-run \
  | jq -c '{state: .request.state, keys: (.request | keys)}' \
  | mustmatch '{"state":"The box turned up on Thursday, two days after the date on the confirmation email.","keys":["model","questions","state"]}'
```

- **A short list of options that exclude one another is what the model handles.** Adding an irrelevant code or changing the order moves the odds. A record that offers twenty overlapping codes measures worse than one that offers four.
- **No record's answer sets the exit code.** A record run exits 0 when it finished. A script reads the labels from the rows.

## Related how-tos

- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) picks from one fixed list over one document.
- [How to sort files into folders by label](../05-sort-a-folder/) uses `--raw` for the folder name.
- [How to act only when the answer is sure, and send the rest to a person](../04-review-queue/) cuts a queue on a threshold.
