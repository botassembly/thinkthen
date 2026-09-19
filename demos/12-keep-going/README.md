# 12 Keep going

Status: red

Verbs: `decide if`, `decide where`

A nightly job judges a queue of support messages. Two things go wrong in real life: the backend is unreachable, and one record in the file is too big to send. A script under `set -euo pipefail` has to survive both on purpose, and the exit codes are what tell it apart.

## Input

`queue.jsonl` holds four messages. The third one has a pasted log inside it and is far bigger than the others.

## The backend is down

Under `set -e` a bare command ends the script, so the exit code is captured and read. `--max-retries 0` makes the failure arrive once instead of three times, which is what a health check wants.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide if 'the message reports a payment failure' \
  --url http://127.0.0.1:9/v1/systemone --adapter systemone \
  --min-prob 0.9 --timeout 2 --max-retries 0 \
  < /dev/null > "$work/out.json" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=4"
wc -c < "$work/out.json" | tr -d ' ' | mustmatch "0"
mustmatch like "127.0.0.1:9" < "$work/err.txt"
```

Nothing printed on standard output. An error never becomes a result, so a downstream `jq` has nothing to misread.

## One record fails and the run stops

`--on-error stop` is the default. The run ends at the bad record and what printed is a prefix of the input.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide where 'the message reports a payment failure' \
  --input jsonl --on /body --id /id --min-prob 0.9 \
  --max-record-bytes 256 --replay recording/ \
  < queue.jsonl > "$work/out.jsonl" 2>/dev/null && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=5"
jq -r '.id' "$work/out.jsonl" | mustmatch "Q-01"
```

Two records were judged, one passed the filter, and the third ended the run. A script that treats `out.jsonl` as the answer has the wrong answer.

## One record fails and the run finishes

`--on-error continue` carries every record to the end and exits 6. It needs a result-bearing mode, because an error row has nowhere to go in `--emit input`.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide where 'the message reports a payment failure' \
  --input jsonl --on /body --id /id --min-prob 0.9 \
  --max-record-bytes 256 --on-error continue --emit annotated --replay recording/ \
  < queue.jsonl > "$work/out.jsonl" 2>/dev/null && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=6"
wc -l < "$work/out.jsonl" | tr -d ' ' | mustmatch "4"
jq -r '[.result.status] | join("")' "$work/out.jsonl" | sort | uniq -c | tr -s ' ' \
  | mustmatch " 1 error
 3 ok"
jq -r 'select(.result.status == "error") | .input.id' "$work/out.jsonl" | mustmatch "Q-03"
```

Every record has a row and the failed one is named. The run is complete and it is not clean, and exit 6 says exactly that.

## Asking for the impossible

`--on-error continue` with the default `--emit input` is a usage error, and it arrives before any request is sent.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide where 'the message reports a payment failure' \
  --input jsonl --on /body --min-prob 0.9 --on-error continue --replay recording/ \
  < queue.jsonl > "$work/out.jsonl" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
wc -c < "$work/out.jsonl" | tr -d ' ' | mustmatch "0"
mustmatch like "--emit" < "$work/err.txt"
```

## Three codes, three branches

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide where 'the message reports a payment failure' \
  --input jsonl --on /body --id /id --min-prob 0.9 \
  --max-record-bytes 256 --on-error continue --emit annotated --replay recording/ \
  < queue.jsonl > "$work/out.jsonl" 2>/dev/null && rc=0 || rc=$?

case $rc in
  0) printf 'clean\n' ;;
  6) printf 'complete with failures\n' ;;
  7) printf 'stopped at a limit\n' ;;
  *) printf 'run failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac | mustmatch "complete with failures"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **Refuse `--on-error continue` with `--emit input`, and name the mode in the message.** The demo shows the refusal arriving at exit 2 before a request goes out, which is the cheapest place for it. The draft's recommendation in records.md holds.
- **Exit 6 says at least one record failed and never says how many.** The script above knows the run is dirty and has to count the error rows itself to find out how dirty. `where` already prints its dropped count on standard error, so the shape exists. Smallest fix: under `--on-error continue`, the count of failed records prints on standard error at the end of the run.
- **An error row needs a `status` field at the top of the row.** records.md says a failed record "gets a row with an error status and a stable code" and never says where. This demo reads `.result.status`, and result.md's `assessment.status` already carries `accepted`, `unsure`, and `unassessed`, which are outcomes of a judgment that happened. Smallest fix: a result row carries `status` of `ok` or `error` beside `question`, `answer`, and `assessment`, and an error row carries `error.code` and no `answer`. This touches result.md, a settled page, and it is the field that lets one `jq` filter split a continue run.
- **A stopped run prints a prefix that looks exactly like a finished file.** Nothing in the output says it was cut short, and only the exit code knows. No flag fixes this, because a stream has to print as it goes. The demos answer it the way demo 06 does: write to a temporary name and `mv` only after the exit code is read.
- **`--max-retries 0` is what a health check wants and the default of 2 is right for a run.** A down backend at the default costs three attempts and three timeouts before exit 4. Both behaviours are correct and the flag is the lever. Smallest fix: the `--max-retries` line in backends.md names the health-check case.
- **A replay miss has no exit code.** These blocks all carry `--replay recording/`, and nothing in the specification says what happens when the recording has no answer for a record. It is a local failure, a backend failure, and a record failure by three different readings, and the demos cannot assert on it. The document that defines `--replay` has to pick one. This demo argues for a local failure, exit 5, because the recording is a file on this machine.
