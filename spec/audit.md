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
```

Each `--optimize` measure picks its own bar. On the Abbey Road rows, with every record in the tuning part, accuracy picks 0.85, precision 0.95, recall 0.75, and f1 0.73.

```bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"

for measure in accuracy precision recall f1; do
  thinkthen audit abbey/rows.jsonl abbey/key-tune.jsonl --id /input --optimize "$measure" | jq -c '.suggested.cut'
done | mustmatch "0.85
0.95
0.75
0.73"
```

`--write` puts the steady bar into the question file the results came from and reports the old value. A second write from the same results refuses, because the digest moved with the threshold.

```bash
set -euo pipefail
root="$(git rev-parse --show-toplevel)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
cp "$root/crates/thinkthen/tests/fixtures/measure/write/decide.json" "$work/decide.json"
cd "$work"

env -u THINKTHEN_API_KEY thinkthen decide @decide.json --jsonl --details \
  --replay "$root/transforms/rows/recording" --input "$root/transforms/rows/cases.jsonl" > rows.jsonl
key="$root/crates/thinkthen/tests/fixtures/measure/write/key.jsonl"
thinkthen audit rows.jsonl "$key" --write decide.json 2>&1 >/dev/null \
  | mustmatch "thinkthen: audit: wrote threshold 0.59 for the question; it was 0.9"
grep -c '"\\u0074hreshold": 0.59,' decide.json | mustmatch "1"

set +e
thinkthen audit rows.jsonl "$key" --write decide.json >/dev/null 2>&1
code=$?
set -e
test "$code" -eq 2
```
