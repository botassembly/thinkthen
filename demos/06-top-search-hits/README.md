# 06 Top search hits

Status: red

Verbs: `rank`, `filter`

A keyword search over an internal wiki returns six pages and an engineer wants the three worth opening first. The search engine ranked by words. `rank` reorders by whether a page holds something that helps with the problem in hand, and the cut to three happens after the order exists.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`hits.jsonl` holds six search hits with `id`, `path`, and `body`.

## Order and cut

`rank` asks one yes/no question of each record and sorts by the probability of yes. It never compares two records in one question.

```bash
set -euo pipefail

thinkthen rank 'Does the page name a cause of a slow or failing sign-in?' \
  --jsonl --field /body --top 3 --input hits.jsonl --replay recording/ \
  | jq -r '.path' \
  | mustmatch "notes/2025-11-outage.md
runbooks/database.md
runbooks/login.md"
```

`--top 3` printed three rows and paid for six. Every record was judged, because an order needs the whole set.

The records come out as they arrived. The reordering is the only thing `rank` does to the stream.

```bash
set -euo pipefail

thinkthen rank 'Does the page name a cause of a slow or failing sign-in?' \
  --jsonl --field /body --top 1 --input hits.jsonl --replay recording/ \
  | mustmatch '{"id":"H3","path":"notes/2025-11-outage.md","body":"Sign-in hung for twenty minutes. The token service had exhausted its connection pool. Raising the pool size cleared it and we added an alert on pool waits."}'
```

## A floor before the order

`rank` selects nothing and takes no threshold, so an irrelevant hit still gets a place in the order. A floor is a separate verb in front of it.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen filter 'Is the page about signing in?' \
  --jsonl --field /body --threshold 0.9 --input hits.jsonl --replay recording/ \
  | thinkthen rank 'Does the page name a cause of a slow or failing sign-in?' \
      --jsonl --field /body --top 3 --replay recording/ \
  > "$work/reading-list.jsonl"

wc -l < "$work/reading-list.jsonl" | tr -d ' ' | mustmatch "3"
jq -r '.path' "$work/reading-list.jsonl" | head -1 | mustmatch "notes/2025-11-outage.md"
```

Two policies run and the pipeline says so: one about eligibility, one about reading order. A threshold inside `rank` would have hidden the first inside the second.

## The order carries no claim

`--details` shows the number the order came from. The page that sorts last still has a probability, and nothing in the run says it answers the question.

```bash
set -euo pipefail

thinkthen rank 'Does the page name a cause of a slow or failing sign-in?' \
  --jsonl --field /body --details --input hits.jsonl --replay recording/ \
  | jq -r '.threshold | tostring' \
  | sort -u \
  | mustmatch "null"
```

`threshold` is `null` on every row, because `rank` takes none. That is the honest reading of a ranked list: it is a suggestion about reading order and not a claim about any page.

## Publish the finished list

A ranked stream is not a dataset until the pipeline has finished. The temporary file becomes the real one only when every stage exited zero.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

if thinkthen filter 'Is the page about signing in?' \
     --jsonl --field /body --threshold 0.9 --input hits.jsonl --replay recording/ \
   | thinkthen rank 'Does the page name a cause of a slow or failing sign-in?' \
       --jsonl --field /body --top 3 --replay recording/ \
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

- **The demo confirms `rank` with no rubric.** One plain question orders six pages, and `rank QUESTION` needs no second file and no levels. Composing with `filter` reads well and keeps the two policies apart.
- **`rank` buffers the whole stream and nothing says so.** An order needs every record, so `rank` cannot print a row until the last request returns. `filter` streams and `rank` does not, and the two look identical on the command line. The demo asks for one line in the `rank` help.
- **The demo could not bound a ranked run.** `rank` makes one request per record, `--top N` limits only what prints, and a run that finishes prints nothing on standard error. So `rank` over a million-line file makes a million requests with no lever and no count. The demo asks that the `--top` row in the help say what it does not limit, and that a finished run print the requests it made on standard error.
- **`--details` on `rank` keeps the record in `input` and stays flat.** The old surface wrapped, and the next stage had to unwrap. Nothing here needs the change.
- **`threshold: null` on a ranked row is the right answer.** The field exists on every result object and a ranked row has no rule to report. The demo confirms the ADR's reading.
