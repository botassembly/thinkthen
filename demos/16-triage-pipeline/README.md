# How to build a triage pipeline that drafts, blocks, or asks a person

Status: green

Verbs: `annotate`

Use this when one record needs several answers before ordinary policy code can choose an action. Six fictional support tickets become complete audit rows in three files.

```bash
./run --batch 1 --replay recording/ | mustmatch "SUP-1042	Duplicate renewal charge	draft	routine
SUP-1043	Package needed for a flight	review	urgent
SUP-1044	Locked out after changing phones	block	credential_request
SUP-1045	Request for a product integration	review	out_of_scope
SUP-1046	Delivery address may be wrong	review	urgent
SUP-1047	Remove an old shipping address	draft	routine
draft=2
block=1
review=3
agreement=6/6"
```

## Input

`tickets.tsv` holds six made-up messages with an id, subject, body, and `reviewed_action`. That last field is a person's recorded decision. The policy uses the same `draft`, `block`, and `review` values, which lets the test compare them directly. The first six lines show the page-local TSV view: id, subject, action, and reason. The saved audit files remain JSONL.

`questions.json` asks whether the message requests a credential, which queue owns it, and how urgent it is. Every question points at `/body`. The id, subject, and `reviewed_action` stay local. `--details` restores the complete TSV row under `input`, and streamed table results remain JSONL.

The six recording files hold one request per ticket. Pass `--batch 1` through the scripts to replay those exact requests; the default packs records differently. The three answers in each replayed row name the same request digest, which proves that the questions rode together.

## Step 1: judge once, then apply policy once

The idea is `thinkthen annotate ... | jq -f triage.jq`: the tool judges, and the rules decide. The complete safe script below stages its JSONL audit files before publishing them.

The script's complete flow is short enough to read as one pipeline. With `--batch 1`, `annotate` packs the three questions into one request per ticket. The tested `jq` policy adds `policy.action` and `policy.reason` without removing any result field.

```sh
mkdir "$output"
tmp=$(mktemp -d "$output/.staging.XXXXXX")
thinkthen annotate questions.json --tsv --details --jobs 4 --input tickets.tsv --max-retries 0 "$@" \
  | jq -c -f ../../transforms/triage/triage.jq > "$tmp/all.jsonl"
for action in draft block review; do
  jq -c --arg action "$action" 'select(.policy.action == $action)' "$tmp/all.jsonl" > "$tmp/$action.jsonl"
done
rm -- "$tmp/all.jsonl"
for action in draft block review; do
  mv -- "$tmp/$action.jsonl" "$output/$action.jsonl"
done
rmdir -- "$tmp"
```

The rules run in this order: a not sure answer goes to `review`; a credential request goes to `block`; the `other` queue goes to `review`; an urgent request goes to `review`; everything else goes to `draft`. The exact reasons are `unsure`, `credential_request`, `out_of_scope`, `urgent`, and `routine`.

## Step 2: keep the audit row

Each output row retains `input`, the three values and answer details, the model metadata, and the policy result. The source row stays flat under `input`; no shell zip can shift a short result onto the wrong ticket.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

./triage "$work/triage" --batch 1 --replay recording/
jq -s -e 'all(.[]; ([.answers[].request] | unique | length) == 1)' "$work/triage"/*.jsonl >/dev/null
jq -c '{id: .input.id, values: .value, policy}' "$work/triage/block.jsonl" \
  | mustmatch '{"id":"SUP-1044","values":{"credential_request":true,"queue":"account","urgency":1.02},"policy":{"action":"block","reason":"credential_request"}}'
```

The script reserves the output name before it judges, then builds inside a hidden staging directory. A failed question, malformed policy input, failed file write, or catchable interruption removes the directory. A caller reads it only after the script returns successfully. A not sure value remains JSON `null` and fails closed into the review file.

## Step 3: fail not sure answers closed

The policy checks null before every automated rule. A null in any of the three answer kinds goes to a person.

```bash
printf '%s\n' '{"value":{"credential_request":false,"queue":null,"urgency":0}}' \
  | jq -c -f ../../transforms/triage/triage.jq \
  | jq -c '.policy' \
  | mustmatch '{"action":"review","reason":"unsure"}'
```

## What can go wrong

- The output directory must not exist. This prevents a second run from mixing old and new rows.
- A question name may not collide with a field already in the input row. `annotate` refuses that record before sending it. Keep names such as `queue` and `urgency` out of the TSV header.
- `/body` is the disclosure boundary. `--dry-run` shows the exact request without sending it. Detailed output still contains the full local row, including `reviewed_action`, so treat the three files as audit data.
- Adding, removing, or changing one question changes the whole packed request and its cache digest. A rerun asks and pays for all three questions again, and an answer near its cut can move.
- The policy refuses missing, malformed, and unknown values. It never silently drafts them.

## Related how-tos

- [How to gate a risky command and fail closed](../19-no-or-could-not-ask/) explains not sure decisions.
- [How to grade an assistant's answers with a rubric](../14-grade-a-batch/) asks several question types together.
- [How to resume a long run that stopped](../12-keep-going/) reuses completed recordings.
