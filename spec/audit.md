# Audit

`audit` grades saved answers against an answer key and sends no request. The table opens with the group, its verb, and its counts, then the agreement with its 95% interval.

```bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"

thinkthen audit small/decide.jsonl small/decide-key.jsonl --table | head -2 | mustmatch "Is it red?  (decide, 6 rows, 6 labeled, 0 failed, rule as run)
  agreement 0.667 (95% 0.300 to 0.903): 4 right, 2 wrong, 0 unresolved, 0 tied"
```

A band on a `choose` answer prints nothing on standard output and exits 2.

```bash
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"

set +e
thinkthen audit small/choose.jsonl small/choose-key.jsonl --threshold 0.4:0.6 >/dev/null 2>&1
code=$?
set -e
test "$code" -eq 2
test -z "$(thinkthen audit small/choose.jsonl small/choose-key.jsonl --threshold 0.4:0.6 2>/dev/null)"
thinkthen audit small/choose.jsonl small/choose-key.jsonl --threshold 0.4:0.6 2>&1 >/dev/null | mustmatch "thinkthen: audit: choose takes a single cut; a band applies to decide"
