# How to build a triage pipeline that drafts, blocks, or asks a person

Status: red

Verbs: `annotate`

Use this when one record needs several judgments before ordinary policy code can choose an action. Six fictional support tickets become complete audit rows in three files. The reviewed recording is still pending, so this page stays red.

```bash
./run --replay recording/ | mustmatch "draft=2
block=1
review=3"
```

## Input

`tickets.tsv` holds six made-up messages with an id, subject, body, and `reviewed_action`. That last field is a person's recorded decision. The policy uses the same `draft`, `block`, and `review` values, which lets the test compare them directly.

`questions.json` asks whether the message requests a credential, which queue owns it, and how urgent it is. Every question points at `/body`. The id, subject, and `reviewed_action` stay local. `--details` restores the complete TSV row under `input`, and streamed table results remain JSONL.

## Step 1: judge once, then apply policy once

The script's complete flow is short enough to read as one pipeline. `annotate` packs the three questions into one request per ticket. The tested `jq` policy adds `policy.action` and `policy.reason` without removing any result field.

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

The rules run in this order: an unresolved answer goes to `review`; a credential request goes to `block`; the `other` queue goes to `review`; an urgent request goes to `review`; everything else goes to `draft`. The exact reasons are `unresolved`, `credential_request`, `out_of_scope`, `urgent`, and `routine`.

## Step 2: keep the audit row

Each output row retains `input`, the three values and answer details, the model metadata, and the policy result. The source row stays flat under `input`; no shell zip can shift a short result onto the wrong ticket.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

./triage "$work/triage" --replay recording/
jq -c '{id: .input.id, values: .value, policy}' "$work/triage/block.jsonl" \
  | mustmatch '{"id":"SUP-1044","values":{"credential_request":true,"queue":"account","urgency":2},"policy":{"action":"block","reason":"credential_request"}}'
```

The script reserves the output name before it judges, then builds inside a hidden staging directory. A failed judgment, malformed policy input, failed file write, or catchable interruption removes the directory. A caller reads it only after the script returns successfully. An unresolved value remains JSON `null` and fails closed into the review file.

## What can go wrong

- The output directory must not exist. This prevents a second run from mixing old and new rows.
- A question name may not collide with a field already in the input row. `annotate` refuses that record before sending it. Keep names such as `queue` and `urgency` out of the TSV header.
- `/body` is the disclosure boundary. `--dry-run` shows the exact request without sending it. Detailed output still contains the full local row, including `reviewed_action`, so treat the three files as audit data.
- Adding, removing, or changing one question changes the whole packed request and its cache digest. A rerun asks and pays for all three judgments again, and an answer near its cut can move.
- The policy refuses missing, malformed, and unknown values. It never silently drafts them.

## Related how-tos

- [How to gate a risky command and fail closed](../19-no-or-could-not-ask/) explains unresolved decisions.
- [How to grade an assistant's answers with a rubric](../14-grade-a-batch/) asks several question types together.
- [How to resume a long run that stopped](../12-keep-going/) reuses completed recordings.
