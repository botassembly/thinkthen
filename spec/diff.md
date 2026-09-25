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

thinkthen diff 249/control.jsonl 249/soft.jsonl 2>&1 >/dev/null | mustmatch "thinkthen: diff: warning: the question digest differs in 272 of 272 paired answers. A different question or threshold gives a different digest."
```
