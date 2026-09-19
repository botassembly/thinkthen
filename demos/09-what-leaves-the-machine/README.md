# 09 What leaves the machine

Status: red

Verbs: `filter`

An operations team has order records that hold an email address and the last four digits of a card. They want to judge the free-text note and nothing else. Before the first request goes out, somebody has to be able to see exactly what would be sent. `--dry-run` prints it and stops.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`orders.jsonl` holds three order records with `order`, `email`, `card_last4`, and `note`.

## Look at the request before sending one

`--dry-run` calls no backend and needs no key, so it runs with the key variable removed from the environment.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

env -u THINKTHEN_API_KEY thinkthen filter 'Does the customer ask for money back?' \
  --jsonl --field /note --threshold 0.9 --dry-run \
  --input orders.jsonl > "$work/plan.json"

jq -S -c 'keys' "$work/plan.json" \
  | mustmatch '["input","key_env","model","request","url"]'
jq -c '.input' "$work/plan.json" | mustmatch '{"framing":"jsonl","field":["/note"]}'
jq -r '.url' "$work/plan.json" | mustmatch "https://api.typesafe.ai/v1/systemone"
jq -r '.key_env' "$work/plan.json" | mustmatch "THINKTHEN_API_KEY"
jq -r '.request.state' "$work/plan.json" \
  | mustmatch "The blender arrived with a cracked jug. Please put the money back on my card."
```

The plan names the key variable and never holds a key. In record mode the plan gains `input`, naming the framing and the pointers that decide what every other record will send. `field` is a list, because a check may name several pointers.

## The pointer is the boundary

Only the value at `/note` reaches the backend. The email address and the card digits stay on the machine, and the plan is the proof.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY thinkthen filter 'Does the customer ask for money back?' \
  --jsonl --field /note --threshold 0.9 --dry-run --input orders.jsonl \
  | mustmatch not like "example.net"
```

Drop `--field` and the whole record becomes the evidence. The plan shows that too, and that is the reason to run it.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY thinkthen filter 'Does the customer ask for money back?' \
  --jsonl --threshold 0.9 --dry-run --input orders.jsonl \
  | mustmatch like "card_last4"
```

The kept records still carry every field. `--field` narrows what is sent and never what is printed.

```bash
set -euo pipefail

thinkthen filter 'Does the customer ask for money back?' \
  --jsonl --field /note --threshold 0.9 --input orders.jsonl --replay recording/ \
  | jq -r '.card_last4' \
  | mustmatch "4113"
```

## A plan a reviewer can keep

A change to the question or the pointer changes the plan, so a saved plan is something a reviewer compares against next week's.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

env -u THINKTHEN_API_KEY thinkthen filter 'Does the customer ask for money back?' \
  --jsonl --field /note --threshold 0.9 --dry-run \
  --input orders.jsonl > "$work/plan.tmp"
mv -- "$work/plan.tmp" "$work/plan.json"

wc -l < "$work/plan.json" | tr -d ' ' | mustmatch "1"
```

One plan for a file of three records. The plan shows the first request and stops.

The recording under `recording/` does not exist yet. `--dry-run` needs none, because it sends nothing.

## What this demo decides

- **The demo confirms `--field` as the boundary and `--dry-run` as the proof.** Two blocks, one showing what goes and one showing what does not, and the third block shows the field coming back on the kept record. A JSON Pointer is a thing a reviewer can read.
- **The `input` object closes the hole the old plan had.** A reviewer comparing two saved plans now sees that `--field /note` became `--field /` even when the first record reads the same. The demo confirms the two members it needed and never reached for a third.
- **The demo could not prove the plan is complete for a run.** `--dry-run` reads the first record and stops, so a pointer that is missing on record two is invisible to the reviewer. The demo does not ask for a whole-file dry run. It asks that the plan name the record it came from, because `input` says which pointer was asked for and not which record answered.
- **The plan has no threshold and the demo did not miss one.** A threshold changes no request and no byte on the wire. A reviewer of what leaves the machine has no use for it. That is the line the plan should hold.
- **A plan holds the evidence, so a plan deserves the care a request deserves.** The first block writes it into a scratch directory under `trap`. A plan redirected to a shared file leaks what the request would have leaked. One sentence in the `--dry-run` help closes it.
- **`--dry-run` with no key in the environment has to work, and the demo depends on it.** `env -u` is how a reviewer proves the command sent nothing. The surface says `--dry-run` needs no key, and this page makes it a test rather than a promise.
