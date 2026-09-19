# 12 Keep going

Status: red

Verbs: `decide`

A nightly job judges a queue of support messages. Two things go wrong in real life: the backend is unreachable, and one record in the file is shaped wrong. A script under `set -euo pipefail` has to survive both on purpose, and the exit codes are what tell them apart. Then the job runs again and does not pay twice for the records that already finished.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`queue.jsonl` holds four messages. The third one keeps its text under `note` instead of `body`. A stray exporter does that.

## The backend is down

Under `set -e` a bare command ends the script, so the exit code is captured and read. `--max-retries 0` makes the failure arrive once instead of three times. A health check wants exactly that.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input queue.jsonl \
  --url http://127.0.0.1:9/v1/systemone --adapter systemone \
  --timeout 2 --max-retries 0 \
  > "$work/out.jsonl" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=4"
wc -c < "$work/out.jsonl" | tr -d ' ' | mustmatch "0"
mustmatch like "127.0.0.1:9" < "$work/err.txt"
```

Nothing printed on standard output. An error never becomes a result, so a downstream `jq` has nothing to misread.

## One record fails and the run stops

The run ends at the bad record, and what printed is a prefix of the input.

A script that treats the output as the answer has the wrong answer, and only the exit code knows. So the file goes to a temporary name and moves into place after the code is read.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input queue.jsonl --replay recording/ \
  > "$work/out.tmp" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
jq -r '.input.id' "$work/out.tmp" | mustmatch "Q-01
Q-02"
mustmatch like "/body" < "$work/err.txt"

case $rc in
  0) mv -- "$work/out.tmp" "$work/judged.jsonl"; printf 'clean\n' ;;
  2) printf 'a record is shaped wrong\n' ;;
  4) printf 'the backend failed\n' ;;
  5) printf 'a local failure\n' ;;
  *) printf 'run failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac | mustmatch "a record is shaped wrong"
```

Two records were judged and the third ended the run.

## Fix the record and run again

One folder given to `--record` and `--replay` together is a cache. A finished record is answered from disk and only the rest goes to the backend.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R recording "$work/cache"

jq -c 'if has("note") then {id, body: .note} else . end' queue.jsonl > "$work/fixed.jsonl"

thinkthen decide 'Does the message report a payment failure?' \
  --jsonl --field /body --details --input "$work/fixed.jsonl" \
  --record "$work/cache" --replay "$work/cache" \
  > "$work/out.jsonl" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=0"
jq -r '.input.id' "$work/out.jsonl" | mustmatch "Q-01
Q-02
Q-03
Q-04"
jq -r '[.meta.replayed] | join("")' "$work/out.jsonl" | sort -u | mustmatch "true"
```

`meta.replayed` is the ledger. Every row here came from the cache, because a gate touches no network and the folder holds every entry. On a real rerun the first two rows would read `true` and the last two `false`, and the bill would be two requests instead of four.

The recording under `recording/` does not exist yet.

## What this demo decides

- **Stopping at the first failed record is the right default and the exit code is the only witness.** A stopped run prints a prefix that looks exactly like a finished file. No flag fixes that, because a stream prints as it goes. The `mv`-after-the-code shape above is the answer, and it belongs in the help.
- **Exit 2 for a bad record contradicts the exit table.** The table says code 2 is a usage error and that nothing was sent. Two requests were paid for above and two rows printed. The `case` branch reading 2 cannot tell a mistyped flag from a bad record, and those want different repairs. This is the strongest finding on the page. The surface needs a separate code for a run that started and then hit a bad record.
- **The demo could not show the paid half of the cache.** Every entry exists in the recording, so `meta.replayed` reads `true` on every row and the saving is asserted in prose. A demo cannot prove it without a network. The claim stays untested by demos and belongs in a live check.
- **Nothing counts what a stopped run finished.** The script counts its own rows with `jq`. This is the third page asking for a count on standard error at the end of a record run.
- **`--max-retries 0` is what a health check wants.** A down backend at the default costs three attempts and three timeouts before exit 4. Both behaviours are right and the flag is the lever.
