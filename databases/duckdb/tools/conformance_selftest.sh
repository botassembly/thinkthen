#!/usr/bin/env bash
# The driver's own check: it must be able to fail. Copies of the
# conformance file with one expectation corrupted - a decide answer, the
# NULL row's expectation, and one annotate per-record value - must each
# exit nonzero and print a FAILED line; the real file run from the
# repository root must still exit zero; and a path passed as an argument
# must be the file the driver reads, because the corruption tests would
# otherwise pass by reading the good file.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
CASES="$ROOT/../../conformance/conformance.json"
SCRATCH=$(mktemp -d /tmp/thinkthen-conformance-XXXXXX)
trap 'rm -rf "$SCRATCH"' EXIT

python3 - "$CASES" "$SCRATCH" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1]))
names = ("answer", "null-row", "annotate-row")
for name in names:
    copy = json.loads(json.dumps(data))
    cases = {case["id"]: case for case in copy["cases"]}
    if name == "answer":
        case = cases["01-decide-yes-cut"]
        case["expect"]["answer"] = not case["expect"]["answer"]
    elif name == "null-row":
        case = cases["81-decide-many-null-text-passes-through"]
        case["expect"]["answers"][1] = False
        case["expect"]["rows"][1]["value"] = False
    else:
        case = cases["82-annotate-over-repeated-texts"]
        case["expect"]["rows"][1]["value"]["spam"] = True
    json.dump(copy, open(f"{sys.argv[2]}/{name}.json", "w"))
    print(f"corrupted {name}")
PY

for variant in answer null-row annotate-row; do
  set +e
  OUT=$(python3 "$ROOT/tools/conformance.py" "$SCRATCH/$variant.json" 2>&1)
  RC=$?
  set -e
  if [ "$RC" -eq 0 ]; then
    echo "FAILED   the corrupted $variant file exited 0"
    exit 1
  fi
  if ! grep -q "^FAILED" <<<"$OUT"; then
    echo "FAILED   the corrupted $variant file printed no FAILED line"
    exit 1
  fi
  echo "ok       the corrupted $variant expectation fails the driver (exit $RC)"
done

# The real file, invoked from the repository root, still passes and the
# driver reads it through its own path resolution, not the caller's cwd.
(cd "$ROOT/../.." && python3 "$ROOT/tools/conformance.py" >/dev/null)
echo "ok       the real conformance file passes from the repository root"
