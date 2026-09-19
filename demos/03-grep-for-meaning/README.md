# How to keep only the records that match a meaning

Status: green

Verbs: `filter`

`grep` keeps the lines that hold a word. `filter` keeps the records that mean something, and it prints them unchanged, so it sits between two ordinary commands in a pipeline and the pipeline never learns it is there.

```bash
set -euo pipefail

thinkthen filter 'Does the report give steps that would reproduce a defect?' \
  --jsonl --field /body --threshold 0.9 --replay recording/ < issues.jsonl \
  | jq -r '.id' \
  | mustmatch "ISS-101
ISS-104"
```

Two of the five reports pass. The feature request, the vague one, and the how-to question do not.

## Input

`issues.jsonl` holds five made-up issue reports, one JSON object each, with `id`, `opened`, `reporter`, and `body`. `recording/` holds the five live exchanges the page replays, and `record.sh` made them through `sdlc/scripts/live`.

## Step 1: see what leaves the machine

`--field` is the disclosure boundary: only the value it names is sent. `--dry-run` prints the request that would go out, reads no key, and opens no connection, so a reviewer can read it before anyone pays for one.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY thinkthen filter 'Does the report give steps that would reproduce a defect?' \
  --jsonl --field /body --threshold 0.9 --dry-run --input issues.jsonl \
  | jq -S -c 'keys, .input, {state: (.request.state | .[0:20])}' \
  | mustmatch '["input","key_env","model","request","url"]
{"field":["/body"],"framing":"jsonl"}
{"state":"Export to CSV writes"}'
```

The plan names the key variable and never a key. The reporter's address is in every record and in no request.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY thinkthen filter 'Does the report give steps that would reproduce a defect?' \
  --jsonl --field /body --threshold 0.9 --dry-run --input issues.jsonl \
  | mustmatch not like "example.net"
```

Drop `--field` and the whole record becomes the evidence. Run the plan again and the addresses are in it, which is the reason to run it.

## Step 2: check that the kept records are whole

`--field` narrows what is sent and never what is printed. A kept record comes back byte for byte, so `diff` against the source file shows no work nobody asked for.

```bash
set -euo pipefail

thinkthen filter 'Does the report give steps that would reproduce a defect?' \
  --jsonl --field /body --threshold 0.9 --replay recording/ --input issues.jsonl \
  | head -1 \
  | mustmatch '{"id":"ISS-101","opened":"2026-03-02","reporter":"sam.okafor@example.net","body":"Export to CSV writes an empty file. Steps: open any report, choose Export, pick CSV, save. The file is 0 bytes every time on build 4.2.1."}'
```

A filter that reprinted its records through a JSON encoder would reorder the keys and reformat the numbers.

## What can go wrong

- **Three records went and nothing said so.** A single cut keeps or drops, and a run that finished says nothing on standard error, so two kept out of five and two out of two read alike. To see what went, ask `decide --jsonl --details` and split in `jq`, which is the shape [how-to 04](../04-review-queue/) uses.
- **A band.** `filter` takes one cut. A third pile needs a flag to steer it, and `jq` already steers piles, so a band is a usage error that names `decide --details`.
- **A pointer that finds nothing.** A record with no `/body` is an input error for that record at exit 2, before any request for it, and the run stops there with a prefix already printed.
- **A paid request for every record.** `filter` judges each record, whether it keeps it or not, so a file of a million lines is a million paid requests. Cut the file with `grep` or `jq` before `filter`, and leave `filter` the records a word cannot separate.
- **A cut nobody measured.** `0.9` here was read off five made-up reports. Tune one on cases a person judged, the way [how-to 13](../13-pick-a-threshold/) does.

## Related how-tos

- [How to lint a change by meaning and fail the build](../43-lint-a-change/) makes a build step out of the same command.
- [How to put the best matches first](../06-top-search-hits/) orders the records instead of cutting them.
- [How to resume a long run that stopped](../12-keep-going/) pays once for a file that did not finish.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) earns the cut.
