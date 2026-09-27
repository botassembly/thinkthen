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

A tie that holds the key earns one over the tied options. Two of these three ties hold the key, one among two options and one among four.

```bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"
thinkthen audit given/choose.jsonl given/key.jsonl --table | grep ties | mustmatch "  ties holding the key: 2 of 3, share 0.750"
```

`--by` takes a JSON pointer into each input and splits each value by verb. `--pooled` adds one calibration line over every verb.

```bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"
thinkthen audit given/by.jsonl given/by-key.jsonl --by /category --pooled | jq -r '"\(.group) \(.verb) \(.pooled) \(.answers)"' | mustmatch "lead decide null null
lead choose null null
tail decide null null
3 choose null null
null null every verb 6"
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

`relate` and `recognize` rows grade by precision, recall, and F1 over edges or names. Run with a low cut so audit sees every candidate. Demo 45 at 0.01 says one edge the key holds and one it does not.

```bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/demos/45-map-relationships"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
echo '{"id":1,"value":[{"relation":"calls","source":{"name":"gateway","kind":"service"},"target":{"name":"billing","kind":"service"}}]}' > "$work/key.jsonl"
env -u THINKTHEN_API_KEY thinkthen relate @relations.json --threshold 0.01 --url https://api.typesafe.ai/v1 --details --replay recording < entities.json \
  | thinkthen audit - "$work/key.jsonl" --table | sed -n 2p | mustmatch "  matched 1, extra 1, missed 0: precision 0.500   recall 1.000   f1 0.667"
```

A `recognize` run with no kinds prints every name as `ENTITY`. audit grades such a line with every said name and every key name as `ENTITY`, so a key that gives real kinds still grades it. Demo 44's key names three kinds, and a run with no kinds that finds two of the three names matches both.

```bash
set -euo pipefail
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
echo '{"id":1,"value":{"entities":[{"kind":"PER","start":0,"end":10},{"kind":"ORG","start":18,"end":35},{"kind":"LOC","start":39,"end":46}]}}' > "$work/key.jsonl"
echo '{"input":{"id":1},"value":{"entities":[{"text":"Maria Chen","start":0,"end":10,"length":10,"kind":"ENTITY","strength":0.99},{"text":"Chicago","start":39,"end":46,"length":7,"kind":"ENTITY","strength":0.98}]},"question":{"verb":"recognize","kinds":{},"threshold":0.5,"relation_threshold":0.5}}' \
  | thinkthen audit - "$work/key.jsonl" --table | sed -n 2p | mustmatch "  matched 2, extra 0, missed 1: precision 1.000   recall 0.667   f1 0.800"
```
