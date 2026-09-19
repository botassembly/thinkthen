# 06 Top search hits

Status: red

Verbs: `decide where`, `decide rank`

A keyword search over an internal wiki returns six pages and an engineer wants the three worth opening first. The search engine ranked by words. `rank` reorders by whether a page holds something that helps with the problem in hand, and the cut to three happens after the order exists.

## Input

`hits.jsonl` holds six search hits with `id`, `path`, and `body`.

## Order and cut

`rank` asks one yes/no question of each record and sorts by the yes probability. It never compares two records in one question.

```bash
set -euo pipefail

thinkthen decide rank 'the page names a cause of a slow or failing sign-in' \
  --input jsonl --on /body --id /id --top 3 --replay recording/ \
  < hits.jsonl \
  | jq -r '.path' \
  | mustmatch "notes/2025-11-outage.md
runbooks/database.md
runbooks/login.md"
```

`--top 3` printed three rows and paid for six. Every record was judged, because an order needs the whole set.

## A floor before the order

`rank` selects nothing, so a hit that is irrelevant still gets a place in the order. A floor is a separate verb in front of it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide where 'the page is about signing in' \
  --input jsonl --on /body --min-prob 0.9 --replay recording/ < hits.jsonl \
  | thinkthen decide rank 'the page names a cause of a slow or failing sign-in' \
      --input jsonl --on /body --id /id --top 3 --emit annotated --replay recording/ \
      > "$work/ranked.jsonl"

wc -l < "$work/ranked.jsonl" | tr -d ' ' | mustmatch "3"
jq -r '.result.assessment.status' "$work/ranked.jsonl" | sort -u | mustmatch "unassessed"
jq -r '.input.path' "$work/ranked.jsonl" | head -1 | mustmatch "notes/2025-11-outage.md"
```

Every row is `unassessed`, because `rank` takes no pass mark. The order is a suggestion about reading order and not a claim that any page answers the question.

## Publish the finished list

A ranked stream is not a dataset until the pipeline has finished. The temporary file becomes the real one only when every stage exited zero.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

if thinkthen decide where 'the page is about signing in' \
     --input jsonl --on /body --min-prob 0.9 --replay recording/ < hits.jsonl \
   | thinkthen decide rank 'the page names a cause of a slow or failing sign-in' \
       --input jsonl --on /body --id /id --top 3 --replay recording/ \
       > "$work/reading-list.tmp"
then
  mv -- "$work/reading-list.tmp" "$work/reading-list.jsonl"
  printf 'published\n'
else
  printf 'stage statuses: %s\n' "${PIPESTATUS[*]}" >&2
  exit 1
fi | mustmatch "published"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **Compose `rank` with `where`. Do not give `rank` its own floor.** The pipeline above says out loud that two different policies are running: one about eligibility and one about reading order. A `--min-prob` inside `rank` would have hidden the first policy inside the second verb and saved one process. The draft's recommendation under `rank` holds.
- **Keep the yes/no probability as the score. Do not score with `how` levels.** The criterion here is a visible fact about a page, which is what the model is good at. Levels would ask for a rubric, and the measurement says rubrics are the weakest thing the model does.
- **`--top N` prints N and pays for all, and a reader of the help will get this wrong.** Smallest fix: the `--top` line in the help says "limits what prints, not what is requested", in the option table and not only in the prose below it.
- **Exit 7 from `rank` prints nothing, and there is no way to size the next run.** A capped run over a large file spends the whole budget and leaves the user with no number. Smallest fix: on exit 7, `rank` prints the count of requests it made on standard error. This touches the `--max-requests` row in records.md and costs one line of diagnostics.
- **`--emit annotated` on `rank` wraps the record, and the next stage has to unwrap it.** `jq -r '.input.path'` in the second block against `jq -r '.path'` in the first is the whole cost. That is the right trade and needs no change.
