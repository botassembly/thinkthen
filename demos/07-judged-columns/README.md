# 07 Judged columns

Status: red

Verbs: `decide where`

A buyer keeps a product list and wants two columns that no supplier fills in: whether the listing says a bulb comes with the lamp, and whether the listing says the thing needs assembly. Both are facts a reader can point at in the text. The columns go onto the records first, and the spreadsheet comes out at the end.

## Input

`listings.jsonl` holds five product listings with `sku`, `price`, and `body`.

## Add one column

`--as NAME` keeps the record and appends the judgment as a field, with its status in a field beside it. Nothing is selected and nothing is dropped.

```bash
set -euo pipefail

thinkthen decide where 'the listing states that a bulb is included' \
  --input jsonl --on /body --as bulb_included \
  --min-prob 0.9 --replay recording/ < listings.jsonl \
  | jq -r '[.sku, (.bulb_included|tostring), .bulb_included_status] | @tsv' \
  | mustmatch "LMP-11	false	accepted
LMP-12	true	accepted
CHR-03	false	accepted
DSK-07	false	accepted
LMP-14	true	accepted"
```

The status field is what keeps a column honest. An unsure row carries `null` in the value and `unsure` in the status, and no row is quietly turned into `false`.

## Add a second column

Each pass is its own command, and the record that comes out of the first is the record that goes into the second.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide where 'the listing states that a bulb is included' \
  --input jsonl --on /body --as bulb_included \
  --min-prob 0.9 --replay recording/ < listings.jsonl \
  | thinkthen decide where 'the listing states that assembly is needed' \
      --input jsonl --on /body --as needs_assembly \
      --min-prob 0.9 --replay recording/ \
  > "$work/judged.jsonl"

wc -l < "$work/judged.jsonl" | tr -d ' ' | mustmatch "5"
jq -r 'select(.needs_assembly_status == "unsure") | .sku' "$work/judged.jsonl" \
  | mustmatch "LMP-11"
jq -r 'select(.needs_assembly == true) | .sku' "$work/judged.jsonl" \
  | mustmatch "CHR-03"
```

`LMP-11` says "ships flat packed" and never says whether anything has to be screwed together. It lands in the unsure band, which is the right place for it.

## Hand it to a spreadsheet

The columns are already on the records, so the CSV step is `jq` and no second judgment.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide where 'the listing states that a bulb is included' \
  --input jsonl --on /body --as bulb_included \
  --min-prob 0.9 --replay recording/ < listings.jsonl \
  | thinkthen decide where 'the listing states that assembly is needed' \
      --input jsonl --on /body --as needs_assembly \
      --min-prob 0.9 --replay recording/ \
  | jq -r '[.sku, .price, .bulb_included, .bulb_included_status, .needs_assembly, .needs_assembly_status] | @csv' \
  > "$work/catalogue.tmp"

mv -- "$work/catalogue.tmp" "$work/catalogue.csv"
head -1 "$work/catalogue.csv" | mustmatch '"LMP-11",34,false,"accepted",false,"accepted"'
grep -c ',' "$work/catalogue.csv" | mustmatch "5"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **Define `--as NAME` for `jsonl` now, and not only for CSV.** records.md puts `--as` in the CSV paragraph and defers the whole thing. This demo needed it for JSONL, where there is no quoting dialect to settle and no collision rule to invent beyond one line. Every alternative was worse: `--emit annotated` makes the next `jq` read `.input.sku` and the one after that read `.input.input.sku`, because the wrapper nests on every pass. Flat columns chain and nested wrappers do not. Proposed rule: `--as NAME` implies `--emit input`, appends `NAME` and `NAME_status`, and a record that already carries either field fails as a record error.
- **The status field must sit beside the value and must be named.** Without it, `false` and "the model was not sure" are the same cell in a spreadsheet, which is the error the four outcomes exist to prevent. Proposed rule: the value is `null` when the status is not `accepted`, and `NAME_status` carries `accepted`, `unsure`, or `unassessed`.
- **`--as` and `--emit` are two answers to one question, and one of them should win.** Proposed rule: `--as` with any explicit `--emit` other than `input` is a usage error, exit 2. Two flags that both decide the output shape is one flag too many.
- **Chaining two stream commands doubles the process count and stays readable.** Two passes over five records is two commands, and the pipeline says which column came from which question. No combined verb is wanted.
- **CSV can stay deferred.** Once the columns are fields on a JSON record, `jq -r '@csv'` is the whole CSV feature, and the quoting dialect belongs to `jq`. records.md can drop the promise of a CSV framing and name the `@csv` line instead.
