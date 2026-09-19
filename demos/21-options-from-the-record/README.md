# How to choose the next action from a list that changes at every step

Status: green

Verbs: `choose`

Use this when a script works through a job one step at a time and the actions on offer differ at every step. No command line can name a list that changes, so each record carries its own. `--options POINTER` reads that list out of the record and asks that record its own question.

```bash
set -eu

thinkthen choose 'Which of these actions should be taken next?' \
  --jsonl --field /state --options /actions --raw \
  --input steps.jsonl --replay recording/ \
  | mustmatch "look_up_the_order
open_a_carrier_claim
ship_a_replacement"
```

## Input

`steps.jsonl` holds three steps of one support job, each with `id`, `state`, and `actions`. The state says what has happened so far, and the actions are what a script could do next. The first two records hold their actions as a list of names. The third holds them as an object, where each name carries a sentence that says what the action means.

`--field /state` sends the state and leaves the rest of the record at home, and `--raw` prints the bare name for a `case` branch. `recording/` holds the three exchanges this page replays, and `record.sh` made them through `sdlc/scripts/live` on 2026-09-19.

## Step 1: see what each step chose between

`--details` prints the result object, and `question.options` holds the list that record was asked about. The probabilities cover that record's own options and add to one.

```bash
set -eu

thinkthen choose 'Which of these actions should be taken next?' \
  --jsonl --field /state --options /actions --details \
  --input steps.jsonl --replay recording/ \
  | jq -c '{id: .input.id, value, options: .question.options,
            p: (.answer.probabilities[.value] * 100 | round / 100)}' \
  | mustmatch '{"id":"S-1","value":"look_up_the_order","options":["look_up_the_order","ask_for_the_order_number","open_a_carrier_claim","close_the_ticket"],"p":1}
{"id":"S-2","value":"open_a_carrier_claim","options":["open_a_carrier_claim","send_the_tracking_link","ask_for_the_order_number","close_the_ticket"],"p":0.69}
{"id":"S-3","value":"ship_a_replacement","options":["ship_a_replacement","refund_the_order","open_a_carrier_claim","close_the_ticket"],"p":0.89}'
```

The first step is plain and the model gave it every point. The second is not: the parcel has stopped moving, and 0.69 on a carrier claim against 0.30 on the tracking link is a step a loop should put to a person. `--threshold` cuts on that number and exits 3 when the winner falls short.

## Step 2: give each action a sentence

An object under the pointer carries a description per action. The names are the options, and the sentences travel beside them. `--dry-run` sends nothing and prints what would go out.

```bash
set -eu

sed -n '3p' steps.jsonl \
  | thinkthen choose 'Which of these actions should be taken next?' \
      --jsonl --field /state --options /actions --dry-run \
  | jq -c '.request.questions.q1.criteria' \
  | mustmatch '{"ship_a_replacement":"Send the same item again at no charge.","refund_the_order":"Give the money back and send nothing.","open_a_carrier_claim":"Ask the carrier to investigate the parcel.","close_the_ticket":"End the conversation with no further action."}'
```

A list of names sends the same shape with `null` under each name. The value on standard output is the name in both forms, and a description never becomes an answer.

## What can go wrong

- **A record whose list is missing or too short stops the run, and it sends nothing for that record.** `choose` takes 2 to 255 options. The rows already printed stay printed, and the message names the record by its number.

```bash
set -eu

printf '%s\n' '{"id":"S-8","state":"Nothing has happened yet.","moves":["look_up_the_order"]}' \
  | thinkthen choose 'Which of these actions should be taken next?' \
      --jsonl --field /state --options /actions --replay recording/ 2>&1 >/dev/null \
  | mustmatch "thinkthen: the record holds nothing at \`/actions\`
thinkthen: stopped at record 1; 0 records finished, 0 from a recording"
```

- **An action name holding a control character is refused before any request.** A record comes from somewhere else, and a name holding a line feed would write a line of its own into the caller's output. The message never quotes the name.

```bash
set -eu

printf '%s\n' '{"id":"S-7","state":"Nothing has happened yet.","actions":["look_up_the_order","ok\nrm -rf /","close_the_ticket"]}' \
  | thinkthen choose 'Which of these actions should be taken next?' \
      --jsonl --field /state --options /actions --replay recording/ 2>&1 >/dev/null \
  | mustmatch "thinkthen: an option or a level is one line of printable text
thinkthen: stopped at record 1; 0 records finished, 0 from a recording"
```

- **Nothing here takes an action.** The tool printed a name, and the script that reads it is the only thing that could act. A name it does not know belongs in a default branch that stops.
- **`--options` needs `--jsonl`, and it takes no options on the command line.** A pointer needs a JSON record to point into, and the record gives the whole list. Both clashes are exit 2 before any request.
- **Only the pointed values leave the machine.** `--field` and `--options` are the disclosure boundary together, and `--details` carries the whole record back in `input` from the local copy.
- **The model handles a short list of actions that exclude one another.** Adding an irrelevant action or changing the order moves the odds. Twenty overlapping actions measure worse than four.
- **No record's answer sets the exit code.** A record run exits 0 when it finished, and the names come from the rows.

## Related how-tos

- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) picks from one fixed list.
- [How to resume a long run that stopped](../12-keep-going/) resumes a record run that failed partway.
- [How to act only when the answer is sure, and send the rest to a person](../04-review-queue/) cuts a queue on a threshold.
