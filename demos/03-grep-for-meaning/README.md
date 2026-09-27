# How to keep only the records that match a meaning

Status: green

Verbs: `filter`

`grep` keeps the lines that hold a word. `filter` keeps the records that mean something. Here it reads CSV and prints JSONL for the next command in the pipeline.

```bash
set -euo pipefail

thinkthen filter 'Does the report give steps that would reproduce a defect?' --batch 1 \
  --csv --field /body --threshold 0.9 --replay recording/ < issues.csv \
  | jq -r '.id' \
  | mustmatch "ISS-101
ISS-104"
```

Two of the five reports pass. The feature request, the vague one, and the how-to question do not.

## Input

`issues.csv` holds five made-up issue reports under the `id`, `opened`, `reporter`, and `body` headers. Every CSV cell becomes a JSON string before the command reads `/body`. `recording/` holds the five live exchanges the page replays, and `record.sh` made them through `sdlc/scripts/live`.

## Step 1: see what leaves the machine

`--field` is the disclosure boundary: only the value it names is sent. `--dry-run` prints the request that would go out, reads no key, and opens no connection, so a reviewer can read it before anyone pays for one.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY thinkthen filter 'Does the report give steps that would reproduce a defect?' \
  --csv --field /body --threshold 0.9 --dry-run --input issues.csv \
  | jq -S -c 'keys, .input, {state: (.request.state | .[0:20])}' \
  | mustmatch '["input","key_env","model","request","url"]
{"field":["/body"],"framing":"csv"}
{"state":"Each question quotes"}'
```

The plan names the key variable and never a key. The reporter's address is in every record and in no request.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY thinkthen filter 'Does the report give steps that would reproduce a defect?' \
  --csv --field /body --threshold 0.9 --dry-run --input issues.csv \
  | mustmatch not like "example.net"
```

Drop `--field` and the whole record becomes the evidence. Run the plan again and the addresses are in it, which is the reason to run it.

## Step 2: read the JSONL output

`--field` narrows what is sent and never what is printed. CSV is an input framing. A kept row prints as one compact JSON object, so the next command gets predictable JSONL.

```bash
set -euo pipefail

thinkthen filter 'Does the report give steps that would reproduce a defect?' --batch 1 \
  --csv --field /body --threshold 0.9 --replay recording/ --input issues.csv \
  | head -1 \
  | mustmatch '{"id":"ISS-101","opened":"2026-03-02","reporter":"sam.okafor@example.net","body":"Export to CSV writes an empty file. Steps: open any report, choose Export, pick CSV, save. The file is 0 bytes every time on build 4.2.1."}'
```

The header fixes the object key order. Every cell stays a string; the tool does not infer dates, numbers, booleans, or JSON from cell text.

## What can go wrong

- **Three records went and nothing said so.** A single cut keeps or drops, and a run that finished says nothing on standard error, so two kept out of five and two out of two read alike. To keep a complete audit row and split it in `jq`, use [the triage pipeline](../16-triage-pipeline/).
- **A band.** `filter` takes one cut. A third pile needs a flag to steer it, and `jq` already steers piles, so a band is a usage error that names `decide --details`.
- **A pointer that finds nothing.** A record with no `/body` is an input error for that record at exit 2, before any request for it, and the run stops there with a prefix already printed.
- **A paid request for every record.** `filter` judges each record, whether it keeps it or not, so a file of a million lines is a million paid requests. Cut the file with `grep` or `jq` before `filter`, and leave `filter` the records a word cannot separate.
- **A cut nobody measured.** `0.9` here was read off five made-up reports. Tune one on cases a person judged, the way [how-to 13](../13-pick-a-threshold/) does.

## Related how-tos

- [How to lint a change by meaning and fail the build](../43-lint-a-change/) makes a build step out of the same command.
- [How to put the best matches first](../06-top-search-hits/) orders the records instead of cutting them.
- [How to resume a long run that stopped](../12-keep-going/) pays once for a file that did not finish.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) earns the cut.
