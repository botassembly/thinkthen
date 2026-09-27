# Transform catalog

The installed tool lists its ten built-in `jq` transforms in byte order.

```bash
thinkthen transform list | mustmatch "band
calibration
compare
cost
counts
monitor
score
sweep
triage
trials"
```

`show` prints one transform exactly as the repository holds it.

```bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)

thinkthen transform show counts | cmp - "$root/transforms/counts/counts.jq"
```

An unknown name prints nothing on standard output, exits 2, and never repeats the name.

```bash
set +e
thinkthen transform show Counts.jq >/dev/null 2>&1
code=$?
set -e
test "$code" -eq 2
test -z "$(thinkthen transform show Counts.jq 2>/dev/null)"
thinkthen transform show Counts.jq 2>&1 >/dev/null | mustmatch "thinkthen: transform: unknown name; run \`thinkthen transform list\` to see the catalog"
```
