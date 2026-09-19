# 11 Split a thread

Status: red

Verbs: `segment`

One customer message holds three separate requests. A desk that files it as one ticket will answer one of them and lose the other two. `segment` finds where a new request starts, and the shell cuts the file.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`thread.txt` is one message, one sentence per line, ten lines long.

## Find the parts

`--units lines` makes each line a unit. `--window 0` sends the whole message once, so the boundary questions travel together instead of one request per line.

```bash
set -euo pipefail

thinkthen segment 'Does a new request from the customer begin on this line?' \
  --units lines --window 0 --threshold 0.9 --input thread.txt --replay recording/ \
  | jq -r '"\(.start_line)-\(.end_line)"' \
  | mustmatch "1-2
3-5
6-10"
```

One JSON object per segment, one per line. The model never computed a line number. It answered nine yes/no questions about whether a line starts something new, and the tool counted.

Each segment carries both numberings and its own text.

```bash
set -euo pipefail

thinkthen segment 'Does a new request from the customer begin on this line?' \
  --units lines --window 0 --threshold 0.9 --input thread.txt --replay recording/ \
  | head -1 | jq -S -c 'keys' \
  | mustmatch '["end_line","end_unit","start_line","start_unit","text"]'
```

## Cut the file

The line numbers are one-based and inclusive, so `sed -n` is the whole extraction. `text` is already in the output, and a desk that wants the file on disk cuts the original rather than trusting a copy.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen segment 'Does a new request from the customer begin on this line?' \
  --units lines --window 0 --threshold 0.9 --input thread.txt --replay recording/ \
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

## The segments cover the file once

Three parts, ten lines, no gap and no overlap. A desk that files each part as a ticket needs that to hold, and nothing but arithmetic checks it.

```bash
set -euo pipefail

thinkthen segment 'Does a new request from the customer begin on this line?' \
  --units lines --window 0 --threshold 0.9 --input thread.txt --replay recording/ \
  | jq -s -r '[.[] | .end_line - .start_line + 1] | add' \
  | mustmatch "10"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo confirms line numbers on every segment.** `sed -n` is the whole extraction, and the `jq` plus `sed` loop above is what every user of `segment` will write. The demo asks that it sit in the `segment` help as the worked example.
- **The demo could not exercise `--units paragraphs`.** `thread.txt` has no blank line, so the unit numbering and the line numbering agree on every segment and the page proves nothing about the case the line numbers were added for. A paragraph fixture would fix the demo and not the surface. The finding is that the page is weaker than the claim it supports.
- **The demo could not say what `--window 0` means.** ADR 0007 lists `--window N` on `segment` and defines neither the default nor the meaning of zero. This page reads it as "one request for the whole document". That reading makes a ten-line thread cost one request instead of nine. The surface has to say it.
- **The demo could not say what happens to an unresolved boundary.** A band is refused on `segment`, and the single cut says a gap is a boundary when p reaches the mark. So a boundary the model is unsure about silently joins, and no field on a segment records that anything was close. The old surface had a flag and a status for it. The demo does not ask for the flag back. It asks that each segment carry the probability of the boundary that opened it, because a part built on a shaky boundary becomes a ticket.
- **The demo could not say what `segment` does with a record flag.** ADR 0007 names which verbs take `--lines` and `--jsonl` and leaves `segment` out of the list. `--units lines` and `--lines` would then be two words for two different ideas on one command line. The surface should say that `segment` reads one document and takes neither flag.
