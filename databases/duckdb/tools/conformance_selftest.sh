#!/usr/bin/env bash
# The driver's own check: it must be able to fail. A copy of the
# conformance file with one expected answer corrupted must exit nonzero
# and print a FAILED line, the real file run from the repository root must
# still exit zero, and a path passed as an argument must be the file the
# driver reads - the corruption test proves the argument is honored,
# because the driver would otherwise pass by reading the good file.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
CASES="$ROOT/../../conformance/conformance.json"
CORRUPT=$(mktemp /tmp/thinkthen-conformance-XXXXXX.json)

python3 - "$CASES" "$CORRUPT" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1]))
for case in data["cases"]:
    expect = case.get("expect", {})
    if case["verb"] == "decide" and isinstance(expect.get("answer"), bool):
        expect["answer"] = not expect["answer"]
        print(f"corrupted {case['id']}")
        break
else:
    raise SystemExit("no decide case to corrupt")
json.dump(data, open(sys.argv[2], "w"))
PY

set +e
OUT=$(python3 "$ROOT/tools/conformance.py" "$CORRUPT" 2>&1)
RC=$?
set -e
rm -f "$CORRUPT"
if [ "$RC" -eq 0 ]; then
  echo "FAILED   the corrupted cases file exited 0"
  exit 1
fi
if ! grep -q "^FAILED" <<<"$OUT"; then
  echo "FAILED   the corrupted cases file printed no FAILED line"
  exit 1
fi
echo "ok       the corrupted expectation fails the driver (exit $RC)"

# The real file, invoked from the repository root, still passes and the
# driver reads it through its own path resolution, not the caller's cwd.
(cd "$ROOT/../.." && python3 "$ROOT/tools/conformance.py" >/dev/null)
echo "ok       the real conformance file passes from the repository root"
