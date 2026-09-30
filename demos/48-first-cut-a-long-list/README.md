# How to make a first cut before a long list

Status: green

Verbs: `find`

Use this when the list you want to search holds more than 255 lines. `find` takes 2 to 255 units, and `relate` takes at most 255 entities. A cheap first cut with `grep`, a query or an index keeps the lines that can matter. `find` then picks the answer from the survivors.

```bash
set -euo pipefail
grep '^customer: ' handbook.txt | cut -d ' ' -f 2- \
  | env -u THINKTHEN_API_KEY thinkthen find 'When does a refund reach the customer?' \
      --lines --model jev-1.13.0 --replay recording/ \
  | mustmatch 'Refunds reach the original payment method within five working days.'
```

## Input

`handbook.txt` is a made-up staff handbook with 312 lines. Each line starts with its section: warehouse, payroll, facilities, it, security or customer. Twelve lines belong to the customer section.

```bash
set -euo pipefail
printf 'lines=%s customer=%s\n' "$(wc -l < handbook.txt | tr -d ' ')" \
  "$(grep -c '^customer: ' handbook.txt)" | mustmatch 'lines=312 customer=12'
```

## The whole list is refused

Sent whole, the handbook is too long for one `find`. The command stops before any request and exits 2.

```bash
set -euo pipefail
err=$(env -u THINKTHEN_API_KEY thinkthen find 'When does a refund reach the customer?' \
  --lines --model jev-1.13.0 --input handbook.txt --replay recording/ \
  2>&1 >/dev/null) && rc=0 || rc=$?
printf 'rc=%s %s\n' "$rc" "$err" | mustmatch 'rc=2 thinkthen: `find` takes 2 to 255 units'
```

The first cut is the section prefix. `grep` keeps the customer lines, and `cut` drops the prefix, so `find` sees each rule as plain text. A database query or a search index can do the same job.

## A cut can drop the answer

A first cut is only as good as its rule. The handbook says nothing about a manufacturer warranty, and neither do the customer lines. `--none` lets `find` say that none of the survivors answers. The command prints nothing and exits 3.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
grep '^customer: ' handbook.txt | cut -d ' ' -f 2- \
  | thinkthen find 'How long is the manufacturer warranty?' \
      --lines --none --model jev-1.13.0 --replay recording/ \
  > "$work/hit.txt" && rc=0 || rc=$?
printf 'rc=%s bytes=%s\n' "$rc" "$(wc -c < "$work/hit.txt" | tr -d ' ')" \
  | mustmatch 'rc=3 bytes=0'
```

Exit 3 tells a script to widen the cut or ask a person.

## What can go wrong

- More than 255 survivors, or fewer than 2, exit 2 with no request. Tighten or loosen the cut.
- A cut that drops the right line leaves `find` to pick the best of the wrong lines. Use `--none` so a miss exits 3 instead.
- A different cut sends a different request. The replay then misses and exits 5.

The twelve customer lines match the policy in how-to 15 byte for byte. So `recording/thinkthen.jsonl` holds the two answers that page recorded, copied here. `record.sh` records the same two requests from the handbook. Run it only through `sdlc/scripts/live`.

## Related how-tos

- [Find the line that answers a question](../15-find-the-line/)
- [Keep only the records that match a meaning](../03-grep-for-meaning/)
- [Map relationships in a complete entity set](../45-map-relationships/)
- [Test a script with no network](../27-test-with-no-network/)
