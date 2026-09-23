# How to put the best matches first

Status: green

Verbs: `rank`

A keyword search brings back six wiki pages and an engineer has time for three. The search engine ordered them by words. `rank` orders them by whether the page answers the question in hand, and the cut to three happens after the order exists.

```bash
set -euo pipefail

jq -c '{query: "Why is signing in slow or failing?", path, passage: .body}' hits.jsonl \
  | thinkthen rank 'The passage answers the query.' \
      --jsonl --field /query --field /passage --top 3 --replay recording/ \
  | jq -r '.path' \
  | mustmatch "runbooks/database.md
notes/2025-11-outage.md
runbooks/login.md"
```

The page about replica lag came first, ahead of the runbook whose name is `login.md`. Words alone would have reversed those two.

## Input

`hits.jsonl` holds six made-up wiki hits, one JSON object each, with `id`, `path`, and `body`. `recording/` holds the six live exchanges the page replays, and `record.sh` made them through `sdlc/scripts/live`.

## Step 1: put the query in every record

The question is fixed for a run, so a query that changes per run belongs in the record. `jq` builds one record carrying both, and `--field` given twice sends an object of the two, keyed by the last part of each pointer.

```bash
set -euo pipefail

jq -c '{query: "Why is signing in slow or failing?", path, passage: .body}' hits.jsonl \
  | env -u THINKTHEN_API_KEY thinkthen rank 'The passage answers the query.' \
      --jsonl --field /query --field /passage --dry-run \
  | jq -c '.input, {state: (.request.state | keys_unsorted)}' \
  | mustmatch '{"framing":"jsonl","field":["/query","/passage"]}
{"state":["query","passage"]}'
```

`path` is in every record and in no request, because no pointer names it. The question stays the same for all six, so one run is one measurement.

## Step 2: read the order and what it does not claim

`rank` asks one yes/no question of each record and sorts by the probability of yes. It never compares two records in one question, and it never runs a tournament. `--details` shows the number the order came from.

```bash
set -euo pipefail

jq -c '{query: "Why is signing in slow or failing?", path, passage: .body}' hits.jsonl \
  | thinkthen rank 'The passage answers the query.' \
      --jsonl --field /query --field /passage --details --replay recording/ \
  | jq -c '{path: .input.path, p: .answer.probability, value, threshold}' \
  | mustmatch '{"path":"runbooks/database.md","p":0.91,"value":null,"threshold":null}
{"path":"notes/2025-11-outage.md","p":0.87,"value":null,"threshold":null}
{"path":"runbooks/login.md","p":0.83,"value":null,"threshold":null}
{"path":"notes/onboarding.md","p":0.38,"value":null,"threshold":null}
{"path":"runbooks/deploy.md","p":0.04,"value":null,"threshold":null}
{"path":"runbooks/backup.md","p":0.02,"value":null,"threshold":null}'
```

`threshold` and `value` are `null` on every row, because `rank` reads no rule and picks nothing. A ranked list is a suggestion about reading order and not a claim about any page. The backup runbook still has a place in the order, and nothing in the run says it answers anything.

`rank` orders and never selects, so a floor is a separate command in front of it. [How-to 03](../03-grep-for-meaning/) is that command.

## What can go wrong

- **It holds the whole stream.** An order needs every record, so `rank` prints nothing until the last answer is in. A run that stops prints nothing at all, and its line on standard error says so. An endless stream is cut into windows upstream.
- **`--top` saves no request.** Every record is judged before anything is sorted, so `--top 3` over a million-line file is a million paid requests. Cut the file before `rank`, not after.
- **`--top 0`.** A run that prints none of its order is a usage error, because a run that pays for six answers and shows none of them is a mistake in the pipeline.
- **A ranked stream is not a dataset.** Write it to a temporary file and move it into place only when every stage of the pipeline exited 0.
- **A query left out of the records.** Put it in the question instead and the question moves with the query, so two runs are two measurements and no digest ties them.

## Related how-tos

- [How to keep only the records that match a meaning](../03-grep-for-meaning/) is the floor that goes in front.
- [How to resume a long run that stopped](../12-keep-going/) pays once for a file that did not finish.
- [How to know what a run cost](../28-what-a-run-cost/) reads the same rows for tokens.
- [How to test a script with no network](../27-test-with-no-network/) is how this page runs in a gate.
