# 03 Grep for meaning

Status: red

Verbs: `decide where`

A maintainer has a file of issue reports and wants the ones that describe a bug somebody could reproduce. `grep` cannot do it, because the words that matter are not in the text. `where` sits between two ordinary commands and keeps the records that pass, and every field of every kept record survives, because only the pointed value ever left the machine.

## Input

`issues.jsonl` holds five issue reports, one JSON object per line, with `id`, `opened`, and `body`.

## Filter in the middle of a pipeline

`jq` narrows the file by date, `where` narrows it by meaning, and `jq` projects the result. `--on /body` sends the body and nothing else. `--id /id` names the field that identifies a record in a result row.

```bash
set -euo pipefail

jq -c 'select(.opened >= "2026-03-02")' issues.jsonl \
  | thinkthen decide where 'the report gives steps that would reproduce a defect' \
      --input jsonl --on /body --id /id \
      --min-prob 0.9 --jobs 2 --replay recording/ \
  | jq -r '.id' \
  | mustmatch "ISS-101
ISS-104"
```

Two records pass. The feature request, the vague report, and the how-to question do not. `where` counts what it left out and says so on standard error.

```bash
set -euo pipefail

jq -c '.' issues.jsonl \
  | thinkthen decide where 'the report gives steps that would reproduce a defect' \
      --input jsonl --on /body --id /id \
      --min-prob 0.9 --replay recording/ \
  2>&1 >/dev/null | mustmatch like "unsure"
```

## The kept records are whole

Nothing is rewritten. The `opened` field never left the machine and it is still there on the way out.

```bash
set -euo pipefail

thinkthen decide where 'the report gives steps that would reproduce a defect' \
  --input jsonl --on /body --id /id \
  --min-prob 0.9 --replay recording/ < issues.jsonl \
  | jq -c 'keys_unsorted' \
  | head -1 \
  | mustmatch '["id","opened","body"]'
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **`--id` does nothing under `--emit input`, and the tool accepts it in silence.** `--emit input` is the default for `where`, and the demo above passes `--id /id` on every line out of habit and gets no result rows to put an id into. A flag that is inert should say so. Smallest fix: `--id` without `--emit result` or `--emit annotated` is a usage error, exit 2, with a message naming the mode to pass. The same rule catches `--unknown drop` under `--emit annotated`, where the draft already says the flag changes nothing.
- **Require `--min-prob` for `--emit input`.** This demo is a filter, and a filter with no policy would have to invent one. The draft's recommendation holds.
- **Keep `--unknown drop` as the default.** It reads like `grep`, and the dropped count on standard error is what tells the user to look again. The count is easy to lose: a pipeline that writes `2>/dev/null` silently discards the only evidence that anything was unsure. Demo 04 routes the unsure rows to a file instead, which is the honest answer for work that matters.
- **Four flags before the question is a lot.** `--input jsonl --on /body --id /id` appears on every stream command in every demo. No smaller spelling is safe, because the draft rules out guessing the framing, and guessing is the only thing that would shorten it. The demos accept the length and ask that the help show the four together as one unit.
- **`--jobs` has a real use here and a default of 4 is fine.** Five records did not need it. The flag went in because a real issue file is thousands of lines and the rate limit is the reason the lever exists.
