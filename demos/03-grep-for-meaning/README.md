# 03 Grep for meaning

Status: red

Verbs: `filter`

A maintainer has a file of issue reports and wants the ones that describe a defect somebody could reproduce. `grep` cannot do it, because the words that matter are not in the text. `filter` sits between two ordinary commands and keeps the records that pass. Every field of every kept record survives, because only the pointed value ever left the machine.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`issues.jsonl` holds five issue reports, one JSON object per line, with `id`, `opened`, and `body`.

## Filter in the middle of a pipeline

`jq` narrows the file by date, `filter` narrows it by meaning, and `jq` projects the result. `--jsonl` makes each line a JSON record and `--field /body` sends the body and nothing else.

```bash
set -euo pipefail
set -o pipefail

jq -c 'select(.opened >= "2026-03-02")' issues.jsonl \
  | thinkthen filter 'Does the report give steps that would reproduce a defect?' \
      --jsonl --field /body --threshold 0.9 --replay recording/ \
  | jq -r '.id' \
  | mustmatch "ISS-101
ISS-104"
```

Two records pass. The feature request, the vague report, and the how-to question do not.

## The kept records are whole

Nothing is rewritten. `opened` never left the machine and it is still there on the way out, byte for byte.

```bash
set -euo pipefail

thinkthen filter 'Does the report give steps that would reproduce a defect?' \
  --jsonl --field /body --threshold 0.9 --input issues.jsonl --replay recording/ \
  | head -1 \
  | mustmatch '{"id":"ISS-101","opened":"2026-03-02","body":"Export to CSV writes an empty file. Steps: open any report, choose Export, pick CSV, save. The file is 0 bytes every time on build 4.2.1."}'
```

That is the input line, unchanged. A filter that reprinted its records through a JSON encoder would reorder keys and reformat numbers, and a `diff` against the source file would show work nobody asked for.

## Three records went missing and nothing said so

`filter` takes a single cut, so a record is kept or dropped and there is no third pile. The run prints two lines out of five and exits 0.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen filter 'Does the report give steps that would reproduce a defect?' \
  --jsonl --field /body --threshold 0.9 --input issues.jsonl --replay recording/ \
  > "$work/kept.jsonl" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=0"
wc -l < "$work/kept.jsonl" | tr -d ' ' | mustmatch "2"
wc -c < "$work/err.txt" | tr -d ' ' | mustmatch "0"
```

A run that finishes prints nothing on standard error, so two lines out of five and two lines out of two look the same. To see the three that went, judge every record and split in `jq`. Demo 04 does exactly that, and the cost is two more commands in the pipeline.

## A pointer that finds nothing

A record with no `/body` is an input error for that record before any request for it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

printf '%s\n' '{"id":"ISS-900","opened":"2026-03-07"}' > "$work/bad.jsonl"
thinkthen filter 'Does the report give steps that would reproduce a defect?' \
  --jsonl --field /body --threshold 0.9 --input "$work/bad.jsonl" --replay recording/ \
  > /dev/null 2>&1 && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo confirms `filter` and `--field`.** Three words in front of the question replaced four, and `--field` carries the whole boundary story: the body goes out, the rest stays. The record flags read well beside `jq` on either side.
- **The demo could not say how many records were dropped.** A run that stops early now prints a line on standard error, and a run that finishes prints nothing. A `filter` that keeps two of five is a run that finished, so it stays silent about the three. The demo asks that a finished `filter` run print the records read and the records dropped on standard error, in the same line the stopped run already uses.
- **Exit 2 for a bad record is settled and the demo confirms it reads right.** Code 2 covers a usage error and an input error alike, and the failing record sent nothing either way. The block above is a whole file of bad records. Demo 12 shows the case where earlier records were already answered, and the line on standard error is what separates the two.
- **`filter` with a band is refused, and the demo agrees.** A third pile needs a flag to steer it, and `jq` already steers piles. Demo 04 shows the shape.
- **Parallelism left the command line.** The old page passed `--jobs 2`. ADR 0007 kept `jobs` in the configuration file, and ADR 0010 took the file out of version one, so a user with a rate limit and a large file has no lever at all. The demo does not ask for the flag back. It asks that the `filter` help say where `jobs` stands, because nothing on the command line points at it.
