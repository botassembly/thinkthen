# How to find the line that answers a question

Status: red

Use `find` when one bounded document has several units and the best unit must answer one question. Every unit leaves together in one request and can affect the choice.

```bash
set -euo pipefail
env -u THINKTHEN_API_KEY thinkthen find 'When does a refund reach the customer?' \
  --lines --input policy.txt --replay recording/ \
  | mustmatch 'Refunds reach the original payment method within five working days.'
```

Verbs: `find`

## Input

`policy.txt` is a twelve-line shop policy, one rule per line. The first command returns the original selected line without rewriting it.

## Inspect the one request

A dry run needs no key or connection. It shows one question and all twelve units in one aggregate evidence value.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
env -u THINKTHEN_API_KEY thinkthen find 'When does a refund reach the customer?' \
  --lines --dry-run --input policy.txt > "$work/plan.json"
jq -r '.request.questions | length' "$work/plan.json" | mustmatch '1'
jq -r '.request.state | fromjson | length' "$work/plan.json" | mustmatch '12'
```

Choose `filter` or `rank` when each unit must be judged alone. Choose `find` only when sending the whole set together is appropriate.

## Let nothing fit

The policy says nothing about manufacturer warranties. `--none` lets the result say that. The command prints nothing and exits 3.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
thinkthen find 'How long is the manufacturer warranty?' \
  --lines --none --input policy.txt --replay recording/ \
  > "$work/hit.txt" && rc=0 || rc=$?
printf 'rc=%s bytes=%s\n' "$rc" "$(wc -c < "$work/hit.txt" | tr -d ' ')" \
  | mustmatch 'rc=3 bytes=0'
```

A script can branch on that outcome and send the question to a person.

```bash
set -euo pipefail
if answer=$(thinkthen find 'How long is the manufacturer warranty?' \
              --lines --none --input policy.txt --replay recording/)
then
  printf 'quote: %s\n' "$answer"
else
  printf 'ask a person\n'
fi | mustmatch 'ask a person'
```

## What can go wrong

Exit 2 means the count, aggregate size, framing, pointer, or command line is invalid. `find` accepts 2 to 255 units, or 2 to 254 with `--none`, and at most 16 MiB of original input. Exit 4 means the backend failed. Exit 5 means an input file or recording failed. A strict `none` lead or any top tie involving `none` exits 3. A tie among real units returns the first one.

This page stays red until the two exact requests in `record.sh` have reviewed recordings. Do not run that script outside `sdlc/scripts/live`.

## Related how-tos

- [Put the best matches first](../06-top-search-hits/)
- [Keep only records that match a meaning](../03-grep-for-meaning/)
- [Test a script with no network](../27-test-with-no-network/)
