# 11 Split a thread

Status: red

Verbs: `decide segment`

One customer message holds three separate requests. A desk that files it as one ticket will answer one of them and lose the other two. `segment` finds where a new request starts, and the shell cuts the file.

## Input

`thread.txt` is one message, one sentence per line.

## Find the parts

`--units lines` makes each line a unit. `--window 0` sends the whole message once, so the boundary questions travel together instead of one request per line.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide segment --boundary 'a new request from the customer begins on this line' \
  --units lines --window 0 --min-prob 0.9 --replay recording/ \
  < thread.txt > "$work/parts.json"

jq -r '.segments | length' "$work/parts.json" | mustmatch "3"
jq -r '.segments[] | "\(.start_unit)-\(.end_unit)"' "$work/parts.json" \
  | mustmatch "1-2
3-5
6-10"
```

The model never computed a line number. It answered nine yes/no questions about whether a line starts something new, and the tool counted the units.

## Cut the file

The ranges are one-based and inclusive, so `sed -n` is the whole extraction.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide segment --boundary 'a new request from the customer begins on this line' \
  --units lines --window 0 --min-prob 0.9 --replay recording/ \
  < thread.txt > "$work/parts.json"

n=0
while read -r start end; do
  n=$((n + 1))
  sed -n "${start},${end}p" thread.txt > "$work/part-$n.txt"
done < <(jq -r '.segments[] | "\(.start_unit) \(.end_unit)"' "$work/parts.json")

printf 'parts=%d\n' "$n" | mustmatch "parts=3"
head -1 "$work/part-3.txt" | mustmatch "One more thing."
grep -c . "$work/part-2.txt" | mustmatch "3"
```

## Every boundary carries its own judgment

A part that was split on a shaky boundary is worth a second look before it becomes a ticket.

```bash
set -euo pipefail

thinkthen decide segment --boundary 'a new request from the customer begins on this line' \
  --units lines --window 0 --min-prob 0.9 --replay recording/ \
  < thread.txt \
  | jq -r '[.segments[].boundary.assessment.status] | unique | join(",")' \
  | mustmatch "accepted"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **`segment` is a convincing everyday job, and `match` was not.** Splitting a message that holds three requests is work a support desk does by hand every day, and the shell can act on a line range without any judgment of quality. A `match` demo would have spent most of its lines building candidate pairs, because the tool never builds them, and the interesting half of a matching job is the half `match` does not do. That is not an argument to cut `match`. It is an argument that `match` is not the verb a stranger meets first.
- **A segment needs line numbers and not only unit indices.** With `--units lines` the two are the same and `sed -n` works. With `--units paragraphs` a unit index is a count of blank-line-separated blocks, and nothing in the result tells a shell where that block starts in the file, so the one thing a user wants to do with a segment cannot be done. Smallest fix: every segment carries `start_line` and `end_line` beside `start_unit` and `end_unit`, for both unit kinds. This is the change this demo argues hardest for.
- **`--window 0` is the right default.** One message, one request, nine boundary questions. A per-boundary window would have cost nine requests for a ten-line file. The help should say to raise it when a document does not fit, which the draft already recommends.
- **Lines and paragraphs are enough. Sentences are not needed.** This thread is one sentence per line because the customer wrote it that way. A thread that is not gets split upstream by whatever the user already trusts, and a sentence splitter inside the tool would take the blame for its own mistakes.
- **`--unknown join` is the right default.** An unsure boundary that split would create a part nobody asked for, and a part is a ticket. Joining leaves the doubt visible inside one part, where a person reading the ticket will see it.
- **Everyone who uses `segment` writes the same `sed` loop.** The tool must not write the files, and it should not. Smallest fix: the `segment` help carries the `jq` plus `sed -n` loop from this demo as its worked example.
