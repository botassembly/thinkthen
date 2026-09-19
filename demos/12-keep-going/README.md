# How to resume a long run that stopped

Status: green

Verbs: `decide`

Use this when a nightly job judges a queue and one record ends the run. `queue.jsonl` holds four support messages, and a stray exporter left the third one's text under `note` instead of `body`. `--cache DIR` points `--record` and `--replay` at one folder, so the rerun pays for the records that never finished.

`recording/` holds the four exchanges this page replays. `record.sh` made them once through `sdlc/scripts/live`, and every number here is what the model answered on 2026-09-19.

## Step 1: the run stops where the record is

```bash
set -eu

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --input queue.jsonl --cache recording/ 2>/dev/null \
  | mustmatch "true
false"
```

Two records were judged and the third ended the run. One line on standard error names the record by its number, never by its text.

```bash
set -eu

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --input queue.jsonl --cache recording/ 2>&1 >/dev/null \
  | mustmatch "thinkthen: the record holds nothing at \`/body\`
thinkthen: stopped at record 3; 2 records finished, 2 from a recording"
```

## Step 2: read the exit code before the file moves

A stopped run prints a prefix that looks exactly like a finished file. The exit code tells them apart. A script writes to a temporary name and moves that name into place under exit 0 alone.

```bash
set -eu

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --input queue.jsonl --cache recording/ \
  >/dev/null 2>&1 && rc=0 || rc=$?

case $rc in
  0) printf 'clean\n' ;;
  2) printf 'a record is shaped wrong\n' ;;
  4) printf 'the backend failed\n' ;;
  *) printf 'run failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac | mustmatch "a record is shaped wrong"
```

Exit 2 covers a mistyped flag and a refused record alike. The message on standard error says which one happened.

## Step 3: repair the record and run again

The repaired file goes through the same folder. `meta.replayed` is the ledger of what each row cost.

```bash
set -eu

jq -c 'if has("note") then {id, body: .note} else . end' queue.jsonl \
  | thinkthen decide 'Does the message report a payment failure?' \
      --jsonl --field /body --details --cache recording/ \
  | jq -c '{id: .input.id, value, replayed: .meta.replayed}' \
  | mustmatch '{"id":"Q-01","value":true,"replayed":true}
{"id":"Q-02","value":false,"replayed":true}
{"id":"Q-03","value":true,"replayed":true}
{"id":"Q-04","value":false,"replayed":true}'
```

Every row reads `true` here, because the committed recording holds all four exchanges and a gate touches no network. On a real rerun the first two rows read `true` and the last two read `false`, and the bill is two requests instead of four.

## Step 4: send several records at once

`--jobs N` bounds how many requests are in flight, and it defaults to 4. The rows print in input order whatever the number is.

```bash
set -eu

jq -c 'if has("note") then {id, body: .note} else . end' queue.jsonl \
  | thinkthen decide 'Does the message report a payment failure?' \
      --jsonl --field /body --cache recording/ --jobs 8 \
  | mustmatch "true
false
true
false"
```

## What can go wrong

- **Exit 4 is the backend, and nothing printed on standard output.** An error never becomes a result, so a downstream `jq` has nothing to misread. `--max-retries 0` makes the failure arrive once, which is what a health check wants.

```bash
set -eu

THINKTHEN_API_KEY=not-a-real-key thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --input queue.jsonl --url http://127.0.0.1:9/v1 \
  --model local-decider-3 --timeout 2 --max-retries 0 2>&1 >/dev/null \
  | mustmatch like "could not be reached"
```

- **A record that answered after the stop is still recorded.** Several requests are in flight when one fails, and the answers that came back were billed. The rerun does not pay for them twice, and the stop always names the earliest failed record.
- **`--cache` stands beside neither `--record` nor `--replay`, and `--jobs` acts over records alone.** Both are exit 2 before any request.
- **A cache never expires.** A folder answers a request whose question, evidence, model, and address are the ones it recorded. A changed question is a new request. A stale folder is deleted by hand.

## Related how-tos

- [How to act only when the answer is sure, and send the rest to a person](../04-review-queue/) splits a judged queue three ways.
- [How to gate a risky command and fail closed](../19-no-or-could-not-ask/) reads every failure code.
- [How to test a script with no network](../27-test-with-no-network/) explains the recording folder.
- [How to know what a run cost](../28-what-a-run-cost/) reads the token counts these rows carry.
