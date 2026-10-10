#!/usr/bin/env bash
# Run one R test file against its own 0092 loopback backend (ticket 0108,
# decisions 17 and 18). Usage: with-backend.sh BACKEND FILE.R [BASE_URL]
#
# The child gets the shared helper's owned home, configuration, cache and
# state folders, the exported absolute R_LIBS, and no ambient settings or
# startup files. The fake key rides only beside the loopback address, and
# the script refuses any other host before an R process starts. The child
# drives the backend through the TT_BACKEND_IN fifo and reads its lines
# from TT_BACKEND_OUT. The final count must equal the child's
# `expect count N` line.
set -euo pipefail
backend=$1 file=$2
. "$(dirname "$0")/../../../sdlc/scripts/scratch.sh"
scratch_dir dir
trap 'exec 3>&- 2>/dev/null || true; scratch_clean' EXIT
mkfifo "$dir/in"
markers=${TT_TEST_MARKERS:-'{}'}
THINKTHEN_TEST_MARKERS="$markers" "$backend" <"$dir/in" >"$dir/out" &
served=$!
exec 3>"$dir/in"
for _ in $(seq 1 600); do [ -s "$dir/out" ] && break; sleep 0.05; done
port=$(head -n 1 "$dir/out")
[ -n "$port" ] || { echo "with-backend: the backend printed no port" >&2; exit 1; }
base=${3:-http://127.0.0.1:$port/generic/v1}
host=${base#*://}; host=${host%%/*}; host=${host%:*}
case $host in
127.0.0.1 | localhost | "[::1]") ;;
*) echo "with-backend: refused $host, which is not a loopback address" >&2; exit 2 ;;
esac
mkdir -p "$dir/cache" "$dir/home"
set +e
python3 - "$(cd "$(dirname "$0")/../../.." && pwd)" "$dir" "$base" "$port" "$file" >"$dir/log" 2>&1 3>&- <<'PY'
import os
from pathlib import Path
import sys

root, directory, base, port, file = sys.argv[1:]
sys.path.insert(0, str(Path(root) / 'conformance/children'))
from children import child_env

env = child_env(('R_LIBS', 'LANG', 'LC_ALL', 'TMPDIR', 'TT_NAMED_BACKEND'),
                home=Path(directory) / 'home',
                THINKTHEN_CACHE=str(Path(directory) / 'cache'),
                THINKTHEN_BASE_URL=base, THINKTHEN_API_KEY='tt-test-not-a-key',
                TT_BACKEND_ORIGIN=f'http://127.0.0.1:{port}',
                TT_BACKEND_IN=str(Path(directory) / 'in'),
                TT_BACKEND_OUT=str(Path(directory) / 'out'),
                TT_TESTS=str(Path(root) / 'libraries/r/tests'))
for name in ('THINKTHEN_TEST_PROFILE', 'THINKTHEN_CONFORMANCE_IDS'):
    if name in os.environ:
        env[name] = os.environ[name]
os.execvpe('sh', ['sh', str(Path(root) / 'sdlc/scripts/time-limit'),
                 '600', 'Rscript', '--vanilla', file], env)
PY
code=$?
set -e
cat -- "$dir/log"
exec 3>&-
wait "$served"
final=$(grep -v '^wait ' "$dir/out" | tail -n 1)
expected=$(sed -n 's/^expect count //p' "$dir/log" | tail -n 1)
if [ "$code" -ne 0 ]; then
  echo "with-backend: FAIL $file exited $code" >&2
  exit 1
fi
if [ -z "$expected" ] || [ "$final" != "$expected" ]; then
  echo "with-backend: FAIL $file: the backend counted $final, and the file expected ${expected:-no count}" >&2
  exit 1
fi
echo "with-backend: ok $file ($final requests)"
