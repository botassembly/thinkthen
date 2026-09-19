# 11 Split a thread

Status: red

Verbs: `segment`

One customer message holds three separate requests. A desk that files it as one ticket will answer one of them and lose the other two. `segment` finds where a new request starts, and the shell cuts the file.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`thread.txt` is one message, one sentence per line, ten lines long.

## Look at what is sent

`segment` reads one document and sends it once. Every unit carries an id, and one yes/no question per gap rides in that single request. The tool builds each question from the user's, so `--dry-run` is the only way to read the text the model gets.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

env -u TYPESAFE_API_KEY thinkthen segment 'Does a new request from the customer begin here?' \
  --units lines --dry-run --input thread.txt > "$work/plan.json"

jq -r '.request.questions | length' "$work/plan.json" | mustmatch "9"
jq -r '.request.questions.q1.instructions' "$work/plan.json" \
  | mustmatch like "Does a new request from the customer begin here?"
```

Nine gaps in a ten-line thread, nine questions, one request. Nothing in the plan repeats the thread nine times.

## Find the parts

```bash
set -euo pipefail

thinkthen segment 'Does a new request from the customer begin here?' \
  --units lines --threshold 0.9 --input thread.txt --replay recording/ \
  | jq -r '"\(.start_line)-\(.end_line)"' \
  | mustmatch "1-2
3-5
6-10"
```

One JSON object per segment, one per line. The model never computed a line number. It answered nine yes/no questions about whether a gap opens something new, and the tool counted.

Each segment carries both numberings and its own text.

```bash
set -euo pipefail

thinkthen segment 'Does a new request from the customer begin here?' \
  --units lines --threshold 0.9 --input thread.txt --replay recording/ \
  | head -1 | jq -S -c 'keys' \
  | mustmatch '["end_line","end_unit","start_line","start_unit","text"]'
```

A record flag is a usage error here. `segment` reads one document and never a stream.

```bash
set -euo pipefail

thinkthen segment 'Does a new request from the customer begin here?' \
  --units lines --jsonl --input thread.txt --replay recording/ \
  >/dev/null 2>&1 && bad=0 || bad=$?
printf 'bad=%s\n' "$bad" | mustmatch "bad=2"
```

## Cut the file

The line numbers are one-based and inclusive, so `sed -n` is the whole extraction. `text` is already in the output, and a desk that wants the file on disk cuts the original rather than trusting a copy.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen segment 'Does a new request from the customer begin here?' \
  --units lines --threshold 0.9 --input thread.txt --replay recording/ \
  > "$work/parts.jsonl"

n=0
while read -r start end; do
  n=$((n + 1))
  sed -n "${start},${end}p" thread.txt > "$work/part-$n.txt"
done < <(jq -r '"\(.start_line) \(.end_line)"' "$work/parts.jsonl")

printf 'parts=%d\n' "$n" | mustmatch "parts=3"
head -1 "$work/part-3.txt" | mustmatch "One more thing."
grep -c . "$work/part-2.txt" | mustmatch "3"
```

The segments cover the file once, with no gap and no overlap. A desk that files each part as a ticket needs that to hold, and nothing but arithmetic checks it.

```bash
set -euo pipefail

thinkthen segment 'Does a new request from the customer begin here?' \
  --units lines --threshold 0.9 --input thread.txt --replay recording/ \
  | jq -s -r '[.[] | .end_line - .start_line + 1] | add' \
  | mustmatch "10"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **Dropping `--window` made the page shorter and the cost clear.** One document, one request, nine questions, and no option to get wrong. The demo confirms ADR 0009 item 1.
- **The demo could not assert the question the model receives.** The tool adds the two unit ids to the user's question and no document says how. The block above matches the user's words inside a longer string and proves nothing about the ids. The surface has to fix that text, because a user who changes the wording of a question is running a new measurement and needs to see the whole of it.
- **The demo could not reach the refusal for a document too large.** ADR 0009 says a document that does not fit one request is refused before any request. `thread.txt` is ten lines. A fixture large enough to trip the limit would be larger than the whole repository of demos, and the limit is a vendor number that will move. The demo asks that the refusal name the exit code and the limit it measured against.
- **The demo could not exercise `--units paragraphs`.** `thread.txt` has no blank line, so the unit numbering and the line numbering agree on every segment and the page proves nothing about the case the line numbers exist for.
- **An unresolved boundary silently joins.** `segment` takes a single cut only, so a gap that fell just under the cut leaves no trace on the segment it did not open. The demo asks that each segment carry the probability of the gap that opened it, because a part built on a shaky boundary becomes a ticket.
