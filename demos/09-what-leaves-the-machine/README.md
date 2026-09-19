# 09 What leaves the machine

Status: red

Verbs: `decide where`

An operations team has order records that hold an email address and the last four digits of a card. They want to judge the free-text note and nothing else. Before the first request goes out, somebody has to be able to see exactly what would be sent. `--plan` prints it and stops.

## Input

`orders.jsonl` holds three order records with `order`, `email`, `card_last4`, and `note`.

## Look at the request before sending one

`--plan` calls no backend and needs no key, so it runs with the key variable removed from the environment.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

env -u TYPESAFE_API_KEY thinkthen decide where 'the customer asks for money back' \
  --input jsonl --on /note --id /order --min-prob 0.9 --plan \
  < orders.jsonl > "$work/plan.json"

jq -r '.adapter' "$work/plan.json" | mustmatch "systemone"
jq -r '.key_env' "$work/plan.json" | mustmatch "TYPESAFE_API_KEY"
jq -r '.request.state' "$work/plan.json" \
  | mustmatch "The blender arrived with a cracked jug. Please put the money back on my card."
```

The plan names the key variable and never holds a key.

## The pointer is the boundary

Only the value at `/note` reaches the backend. The email address and the card digits stay on the machine, and the plan is the proof.

```bash
set -euo pipefail

env -u TYPESAFE_API_KEY thinkthen decide where 'the customer asks for money back' \
  --input jsonl --on /note --id /order --min-prob 0.9 --plan \
  < orders.jsonl \
  | mustmatch not like "example.net"
```

Drop `--on` and the whole record becomes the evidence. The plan shows that too, which is the reason to run it.

```bash
set -euo pipefail

env -u TYPESAFE_API_KEY thinkthen decide where 'the customer asks for money back' \
  --input jsonl --min-prob 0.9 --plan \
  < orders.jsonl \
  | mustmatch like "card_last4"
```

## A plan a reviewer can keep

A change to the question or the pointer changes the plan, so a saved plan is something a reviewer can compare against next week's.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

env -u TYPESAFE_API_KEY thinkthen decide where 'the customer asks for money back' \
  --input jsonl --on /note --id /order --min-prob 0.9 --plan \
  < orders.jsonl > "$work/plan.tmp"
mv -- "$work/plan.tmp" "$work/plan.json"

jq -S -c 'keys' "$work/plan.json" \
  | mustmatch '["adapter","backend","key_env","model","request","url"]'
```

`--plan` needs no recording, because it sends nothing.

## What this demo decides

- **The plan for a stream verb shows one request and hides the run.** decide.md says `--plan` prints the request for the first record and stops. A reviewer wants the framing, the pointer, the identifier, and the request cap as well, because those decide what the other records will send. Smallest fix: the plan carries an `input` object holding `framing`, `on`, `id`, and `max_requests`, and the request body stays exactly as it is. This touches the `--plan` section of channels.md, a settled page.
- **`key_env` must be present and `null` when no key will be sent.** channels.md says the plan holds "the name of the key variable" and says nothing about a run with no key. Demo 10 needs to assert on that case. Smallest fix: `key_env` is always a member, and it is `null` when no key variable applies. This touches a settled page.
- **The plan holds the evidence, so the plan is as sensitive as the request.** channels.md says the plan never holds the key and stops there. A plan redirected to a shared file leaks exactly what the request would have leaked. Smallest fix: one sentence in the `--plan` section saying the plan carries the evidence and deserves the same care as the request.
- **`--plan` with no key in the environment has to work, and the demo depends on it.** `env -u` is how a reviewer proves the command sent nothing. The draft already says `--plan` needs no key, and this demo makes that a test rather than a promise.
- **Without `--on` the whole record goes out, and nothing warns the user.** The third block is the hazard, written down. No flag can fix it, because sending the whole record is a real use. Smallest fix: the `--on` row in records.md says what the default sends, in the option table and not only in the prose.
