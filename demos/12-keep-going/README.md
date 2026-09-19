# 12 Keep going

Status: red

Verbs: `decide`

A nightly job judges a queue of support messages. Two things go wrong in real life: the backend is unreachable, and one record is shaped wrong. A script under `set -euo pipefail` has to survive both on purpose. Then the job runs again and does not pay twice for the records that already finished.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`queue.jsonl` holds four messages. The third one keeps its text under `note` instead of `body`. A stray exporter does that.

## The backend is down

Under `set -e` a bare command ends the script, so the exit code is captured and read. `--max-retries 0` makes the failure arrive once. A health check wants exactly that.

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

A stopped run prints one line on standard error: the record it stopped at, how many records it finished, and how many of those came from a recording. That line and the exit code are the only witnesses, so the file goes to a temporary name and moves into place after both are read.

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
wc -l < "$work/err.txt" | tr -d ' ' | mustmatch "1"
mustmatch like "/body" < "$work/err.txt"
mustmatch like "2 finished" < "$work/err.txt"

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
- **One code for two repairs is still a cost, and the line on standard error pays it.** Exit 2 covers a mistyped flag and a bad record alike, and those want different repairs. The `case` branch reading 2 cannot tell them apart, and the script has to read the message to find out. The demo accepts the ruling and asks that the stopped-run line lead with the record, because that word is what a reader greps for.
- **The demo could not show the paid half of the cache.** Every entry exists in the recording, so `meta.replayed` reads `true` on every row and the saving is asserted in prose. A demo cannot prove it without a network. The claim stays untested by demos and belongs in a live check.
- **A finished run says nothing, and the stopped run says everything.** The line above names the records finished and the records replayed. A clean run over ten thousand records prints no such line, so the only way to learn what a good run cost is to break it. The demo asks for the same line at the end of every record run.
- **`--max-retries 0` is what a health check wants.** A down backend at the default costs three attempts and three timeouts before exit 4. Both behaviours are right and the flag is the lever.
