# How to resume a long run that stopped

Status: green

Verbs: `decide`

Use this when a nightly job judges a queue and one record ends the run. A stray exporter ships a record shaped wrong, the run stops there, and the records after it were never asked. The job then runs again and pays only for the records that never finished. One folder given to `--cache` holds the answers of the first run and answers the second one from disk.

## Input

`queue.jsonl` holds four support messages. The third one keeps its text under `note` instead of `body`.

`recording/` holds the four exchanges this page replays, so every command here runs with no network and no key. `record.sh` made them once through `sdlc/scripts/live`. Every probability on this page is what the model answered on 2026-09-19.

## Step 1: run the queue through a cache folder

`--cache DIR` points `--record` and `--replay` at one folder. A record the folder already holds is answered from disk, and every other record goes to the backend and lands in the folder. The run stops at the third record.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R recording "$work/cache"

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input queue.jsonl \
  --cache "$work/cache" \
  > "$work/out.tmp" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
jq -c '{id: .input.id, value}' "$work/out.tmp" \
  | mustmatch '{"id":"Q-01","value":true}
{"id":"Q-02","value":false}'
cat "$work/err.txt" | mustmatch "thinkthen: the record holds nothing at \`/body\`
thinkthen: stopped at record 3; 2 records finished, 2 from a recording"
```

Two records were judged and the third ended the run. The line on standard error names the record by its number, how many records finished, and how many of those a recording answered.

## Step 2: read the exit code before the file moves

The file goes to a temporary name and moves into place after the code is read. A stopped run prints a prefix that looks exactly like a finished file, and the exit code is the witness that tells them apart.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R recording "$work/cache"

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input queue.jsonl \
  --cache "$work/cache" \
  > "$work/out.tmp" 2> "$work/err.txt" && rc=0 || rc=$?

case $rc in
  0) mv -- "$work/out.tmp" "$work/judged.jsonl"; printf 'clean\n' ;;
  2) printf 'a record is shaped wrong\n' ;;
  4) printf 'the backend failed\n' ;;
  5) printf 'a local failure\n' ;;
  *) printf 'run failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac | mustmatch "a record is shaped wrong"

test ! -f "$work/judged.jsonl" && printf 'nothing moved into place\n' \
  | mustmatch "nothing moved into place"
```

Exit 2 covers a mistyped flag and a refused record alike. The `case` branch cannot tell them apart, and the message on standard error says which one happened.

## Step 3: repair the record and run again

The repaired file goes through the same cache folder. The two records that finished are answered from disk.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R recording "$work/cache"

jq -c 'if has("note") then {id, body: .note} else . end' queue.jsonl > "$work/fixed.jsonl"

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input "$work/fixed.jsonl" \
  --cache "$work/cache" > "$work/out.tmp" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=0"
mv -- "$work/out.tmp" "$work/judged.jsonl"
jq -c '{id: .input.id, value, replayed: .meta.replayed}' "$work/judged.jsonl" \
  | mustmatch '{"id":"Q-01","value":true,"replayed":true}
{"id":"Q-02","value":false,"replayed":true}
{"id":"Q-03","value":true,"replayed":true}
{"id":"Q-04","value":false,"replayed":true}'
```

`meta.replayed` is the ledger of what the run paid for. Every row here reads `true`, because the committed recording holds all four exchanges and a gate touches no network. On a real rerun the first two rows read `true` and the last two read `false`, and the bill is two requests instead of four.

Q-03 reads a payout that bounced twice and answered 0.98. The repair moved its text and changed nothing else.

## Step 4: send several records at once

`--jobs N` bounds how many requests are in flight. The default is 4. The answers print in input order whatever the number is, so a script reads the same file at any speed.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R recording "$work/cache"

jq -c 'if has("note") then {id, body: .note} else . end' queue.jsonl > "$work/fixed.jsonl"

for jobs in 1 8; do
  thinkthen decide 'Does the message report a payment failure?' \
    --jsonl --field /body --input "$work/fixed.jsonl" \
    --cache "$work/cache" --jobs "$jobs" > "$work/rows.$jobs"
done

cat "$work/rows.1" | mustmatch "true
false
true
false"
cmp -s "$work/rows.1" "$work/rows.8" && printf 'same bytes\n' | mustmatch "same bytes"
```

A run at 8 jobs holds at most 8 requests open and buffers at most 8 finished rows, so the memory of a long run stays flat. One process opens its connections once and reuses them across records.

## What can go wrong

- **Exit 4 is the backend, and nothing printed.** Under `set -e` a bare command ends the script, so the code is captured and read. `--max-retries 0` makes the failure arrive once, which is what a health check wants. At the default the run pays three attempts and three timeouts first.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

THINKTHEN_API_KEY=not-a-real-key thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input queue.jsonl \
  --url http://127.0.0.1:9/v1 --model local-decider-3 \
  --timeout 2 --max-retries 0 \
  > "$work/out.jsonl" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=4"
wc -c < "$work/out.jsonl" | tr -d ' ' | mustmatch "0"
mustmatch like "could not be reached" < "$work/err.txt"
mustmatch like "stopped at record 1" < "$work/err.txt"
```

An error never becomes a result, so a downstream `jq` has nothing to misread.

- **A record that answered after the stop is still recorded.** Several requests are in flight when one of them fails, and the requests that came back were billed. The folder keeps them, and the rerun does not pay for them twice. The stop itself always names the earliest failed record, whatever order the answers arrived in.
- **`--cache` stands beside neither `--record` nor `--replay`.** One run keeps one folder.

```bash
set -euo pipefail

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --input queue.jsonl \
  --cache recording/ --replay recording/ 2>&1 && rc=0 || rc=$?

printf 'rc=%s\n' "${rc:-0}" | mustmatch "rc=2"
```

- **`--jobs` acts over records alone.** A run over one document sends one request and has nothing to bound.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

printf 'The card on file expired.\n' \
  | thinkthen decide 'Does the message report a payment failure?' \
      --jobs 4 --replay recording/ 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
cat "$work/err.txt" \
  | mustmatch "thinkthen: --jobs bounds the requests in flight, and one document sends one request"
```

- **A finished run says nothing about what it cost.** The line on standard error appears only when a run stops. `--details` carries `meta.usage` on every row, and [how to know what a run cost](../28-what-a-run-cost/) reads it.
- **A cache never expires.** A folder answers a request whose question, evidence, model, and address are byte for byte the ones it recorded. A changed question is a new digest and a new request. A stale folder is deleted by hand.

## Related how-tos

- [How to act only when the answer is sure, and send the rest to a person](../04-review-queue/) judges a queue and splits it three ways.
- [How to test a script with no network](../27-test-with-no-network/) explains the recording folder in full.
- [How to know what a run cost](../28-what-a-run-cost/) reads the token counts these rows carry.
