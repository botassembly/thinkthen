# 07 Judged columns

Status: red

Verbs: `annotate`, `filter`

A buyer keeps a product list and wants two columns that no supplier fills in: whether the listing says a bulb comes with the lamp, and whether the listing says the thing needs assembly. Both are facts a reader can point at in the text. The columns go onto the records first, and the spreadsheet comes out at the end.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`listings.jsonl` holds five product listings with `sku`, `price`, and `body`. `columns.json` holds the two questions, each with its own band and each pointing at `/body`. `collides.json` holds one question named after a field the records already carry.

## Add both columns in one pass

An object record gains one top-level field per question. The two questions share an `on`, so they share one request.

```bash
set -euo pipefail

thinkthen annotate columns.json --jsonl --input listings.jsonl --replay recording/ \
  | jq -r '[.sku, (.bulb_included|tostring), (.needs_assembly|tostring)] | @tsv' \
  | mustmatch "LMP-11	false	null
LMP-12	true	false
CHR-03	false	true
DSK-07	false	false
LMP-14	true	false"
```

`LMP-11` says "ships flat packed" and never says whether anything has to be screwed together. Its band put it at `null`. That is the right place for it. Nothing turned an unresolved answer into `false`.

The record keeps every field it arrived with, and the new fields sit beside them.

```bash
set -euo pipefail

thinkthen annotate columns.json --jsonl --input listings.jsonl --replay recording/ \
  | head -1 | jq -c 'keys_unsorted' \
  | mustmatch '["sku","price","body","bulb_included","needs_assembly"]'
```

## A chain stays flat

`filter` prints its kept records byte for byte and `annotate` adds top-level fields, so two judgments in a row leave the record at one level. The second `jq` reads `.sku` and not `.input.sku`.

```bash
set -euo pipefail
set -o pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen filter 'Is this listing for a lamp?' \
  --jsonl --field /body --threshold 0.9 --input listings.jsonl --replay recording/ \
  | thinkthen annotate columns.json --jsonl --replay recording/ \
  > "$work/judged.jsonl"

jq -r '[.sku, (.bulb_included|tostring)] | @tsv' "$work/judged.jsonl" \
  | mustmatch "LMP-11	false
LMP-12	true
LMP-14	true"
```

## Hand it to a spreadsheet

The columns are already fields, so the CSV step is `jq` and no second judgment.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen annotate columns.json --jsonl --input listings.jsonl --replay recording/ \
  | jq -r '[.sku, .price, .bulb_included, .needs_assembly] | @csv' \
  > "$work/catalogue.tmp"

mv -- "$work/catalogue.tmp" "$work/catalogue.csv"
head -1 "$work/catalogue.csv" | mustmatch '"LMP-11",34,false,'
wc -l < "$work/catalogue.csv" | tr -d ' ' | mustmatch "5"
```

## A name that is already taken

`collides.json` names its question `price`, and every record already carries a `price`. That is an input error for the record, before any request for it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen annotate collides.json --jsonl --input listings.jsonl --replay recording/ \
  > "$work/out.jsonl" 2> "$work/err.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
wc -c < "$work/out.jsonl" | tr -d ' ' | mustmatch "0"
mustmatch like "price" < "$work/err.txt"
```

The file itself is fine, so `--dry-run` passes it. The collision is a fact about the records.

```bash
set -euo pipefail

thinkthen annotate collides.json --dry-run --jsonl --input listings.jsonl \
  > /dev/null && printf 'file ok\n' | mustmatch "file ok"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo confirms flat fields.** Two judgments in a row leave the record at one level, and every `jq` on this page reads `.sku`. The old wrapper nested on every pass and made the third read `.input.input.sku`.
- **The demo could not see which answers were unresolved without reading the values twice.** The old surface put a status field beside each column. ADR 0007 spells unresolved as `null`, so a spreadsheet cell reading `null` and a listing that genuinely says nothing are the same cell. In this page that is correct and legible. In CSV it is an empty field. The demo accepts `null` and asks that the `annotate` help warn about the CSV step.
- **One `on`, one request, and the demo cannot see the count.** `meta.usage` is the sum over a record's requests, so a file with one pointer and a file with two pointers print the same shape and differ only in the numbers. A page that wanted to prove the saving would have to assert a token count, and no page asserts a number a vendor chose. Demo 14 makes the same claim over two pointers and can prove it no better.
- **The collision exits 2 before any request, and the demo confirms the rule.** Code 2 covers a usage error and an input error alike. Every record collides here, so the whole run sends nothing.
- **`annotate --dry-run` checks the file and not the records, and the demo shows why that is a limit.** The dry run above passes a file that fails on the first record. One line in the help closes it.
