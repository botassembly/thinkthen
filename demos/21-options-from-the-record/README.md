# How to choose from a list that differs for every record

Status: green

Verbs: `choose`

Use this when each record carries the options it may be labeled with. A returns desk files every note under a code, and each desk owns its own codes, so no command line can name them. `--options POINTER` reads the list out of each record and asks that record its own question.

`notes.jsonl` holds three notes, each with `id`, `desk`, `note`, and `codes`. The first two hold their codes as a list of names. The third holds them as an object, where each name carries a sentence that says what the code means. `recording/` holds the three exchanges this page replays, and `record.sh` made them once through `sdlc/scripts/live` on 2026-09-19.

## Step 1: label every note with its own codes

```bash
set -eu

thinkthen choose 'Which of these codes fits the note?' \
  --jsonl --field /note --options /codes --raw \
  --input notes.jsonl --replay recording/ \
  | mustmatch "late
wrong_item
double_charge"
```

`--field /note` sends the note text and leaves the rest of the record at home. `--raw` prints the bare label, for a `case` branch or a folder name.

## Step 2: see what each record chose between

`--details` prints the result object, and `question.options` holds the list that record was asked about.

```bash
set -eu

thinkthen choose 'Which of these codes fits the note?' \
  --jsonl --field /note --options /codes --details \
  --input notes.jsonl --replay recording/ \
  | jq -c '{id: .input.id, value, options: .question.options}' \
  | mustmatch '{"id":"N-1","value":"late","options":["late","never_arrived","wrong_address","other"]}
{"id":"N-2","value":"wrong_item","options":["wrong_item","damaged","missing_part","other"]}
{"id":"N-3","value":"double_charge","options":["double_charge","wrong_amount","refund_late","other"]}'
```

The probabilities cover that record's own options and add to one. Each note here names its code in plain words, and the model answered 1.0 on all three.

## Step 3: give each code a sentence

An object under the pointer carries a description per code. The names are the options, and the sentences travel beside them. `--dry-run` sends nothing and prints what would go out.

```bash
set -eu

sed -n '3p' notes.jsonl \
  | thinkthen choose 'Which of these codes fits the note?' \
      --jsonl --field /note --options /codes --dry-run \
  | jq -c '.request.questions.q1.criteria' \
  | mustmatch '{"double_charge":"The same order was paid for more than once.","wrong_amount":"The amount charged is not the amount on the order.","refund_late":"A refund was promised and has not arrived.","other":"None of the three above fits this note."}'
```

A list of names sends the same shape with `null` under each name. The value on standard output is the name in both forms, and a description never becomes an answer.

## What can go wrong

- **A record whose list is missing or too short stops the run, and it sends nothing for that record.** `choose` takes 2 to 255 options. The rows already printed stay printed, and the message names the record by its number.

```bash
set -eu

printf '%s\n' '{"id":"N-8","note":"Where is my order?","tags":["late"]}' \
  | thinkthen choose 'Which of these codes fits the note?' \
      --jsonl --field /note --options /codes --replay recording/ 2>&1 >/dev/null \
  | mustmatch "thinkthen: the record holds nothing at \`/codes\`
thinkthen: stopped at record 1; 0 records finished, 0 from a recording"
```

- **A code holding a control character is refused before any request.** A record comes from somewhere else, and a label holding a line feed would write a line of its own into the caller's output. The message never quotes the label.

```bash
set -eu

printf '%s\n' '{"id":"N-7","note":"Where is my order?","codes":["late","ok\nrm -rf /","other"]}' \
  | thinkthen choose 'Which of these codes fits the note?' \
      --jsonl --field /note --options /codes --replay recording/ 2>&1 >/dev/null \
  | mustmatch "thinkthen: an option or a level is one line of printable text
thinkthen: stopped at record 1; 0 records finished, 0 from a recording"
```

- **`--options` needs `--jsonl`, and it takes no options on the command line.** A pointer needs a JSON record to point into, and the record gives the whole list. Both clashes are exit 2 before any request.
- **Only the pointed values leave the machine.** `--field` and `--options` are the disclosure boundary together. `--details` carries the whole record back in `input` from the local copy.
- **The model handles a short list of codes that exclude one another.** Adding an irrelevant code or changing the order moves the odds. Twenty overlapping codes measure worse than four.
- **No record's answer sets the exit code.** A record run exits 0 when it finished. A script reads the labels from the rows.

## Related how-tos

- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) picks from one fixed list.
- [How to resume a long run that stopped](../12-keep-going/) resumes a record run that failed partway.
- [How to act only when the answer is sure, and send the rest to a person](../04-review-queue/) cuts a queue on a threshold.
