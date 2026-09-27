# Diff

`diff` compares two cuts on one saved run and sends no request. The count line says what moved, what the key says about it, and the McNemar test.

```bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"

thinkthen diff small/decide.jsonl --key small/decide-key.jsonl --compare-threshold 0.4 --table | tail -1 | mustmatch "as run -> 0.4: 1 of 6 changed; no -> yes 1; gained 1, lost 0 (4 -> 5 right of 6); McNemar p 1.000 on right answers"
```

Without a second run or `--compare-threshold`, diff prints nothing on standard output and exits 2.

```bash
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"

set +e
thinkthen diff small/decide.jsonl >/dev/null 2>&1
code=$?
set -e
test "$code" -eq 2
test -z "$(thinkthen diff small/decide.jsonl 2>/dev/null)"
thinkthen diff small/decide.jsonl 2>&1 >/dev/null | mustmatch "thinkthen: diff: diff needs a second run or --compare-threshold"
```

When nothing pairs, diff still prints its summary and exits 0. It warns on standard error. Here the second run is empty.

```bash
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"

: | thinkthen diff small/decide.jsonl - --table 2>/dev/null | mustmatch "A -> B: 0 of 0 changed; McNemar p 1.000 on yes answers; only in A 6, only in B 0"
: | thinkthen diff small/decide.jsonl - 2>&1 >/dev/null | mustmatch "thinkthen: diff: warning: no answer paired; check that both runs hold the same record ids and answer names"
```

When paired answers carry different question digests, diff warns with the count. These two runs ask differently worded questions.

```bash
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"

thinkthen diff 249/control.jsonl 249/soft.jsonl 2>&1 >/dev/null | mustmatch "thinkthen: diff: warning: the question digest differs in 272 of 272 paired answers. A different question, threshold, or profile gives a different digest."
```

McNemar on right answers counts every pair that becomes right or stops being right. Here `c2` moves from tied to right, and it counts.

```bash
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"

thinkthen diff small/choose.jsonl small/choose-b.jsonl --key small/choose-key.jsonl --table | tail -1 | mustmatch "A -> B: 2 of 5 changed; red -> green 1; tied -> green 1; gained 1, lost 0 (2 -> 4 right of 5); McNemar p 0.500 on right answers"
```

For `recognize` runs, diff lists the names each record gained, lost, or changed in kind. With a key, it counts the key names each side matched and runs McNemar on the key names only one side matched. Under `--match overlap`, record 2's `Abbey Road` pairs with `Abbey Road Studios`, so it prints no row, and `Yesterday` changes kind from `person` to `work`.

```bash
cd "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/measure"

thinkthen diff items/recognize-a.jsonl items/recognize-b.jsonl --key items/recognize-key.jsonl --match overlap --table | tail -1 | mustmatch "A -> B: 3 of 4 changed; items gained 1, lost 0, changed kind 1; key names matched 2 -> 4 of 4; extras 1 -> 0; McNemar p 0.500 on key names"
thinkthen diff items/recognize-a.jsonl items/recognize-b.jsonl --key items/recognize-key.jsonl --match overlap --table | grep '^  ~' | mustmatch "  ~ Yesterday [0,9) person -> work"
```
