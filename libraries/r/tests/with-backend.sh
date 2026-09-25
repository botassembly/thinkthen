#!/usr/bin/env bash
# Run one R test file against its own 0092 loopback backend (ticket 0108,
# decisions 17 and 18). Usage: with-backend.sh BACKEND FILE.R [BASE_URL]
#
# The child gets a fresh THINKTHEN_CACHE folder, a scratch HOME,
# XDG_CACHE_HOME, and XDG_CONFIG_HOME, the exported absolute R_LIBS, and
# no R_LIBS_USER. The fake key rides only beside the loopback address, and
# the script refuses any other host before an R process starts. The child
# drives the backend through the TT_BACKEND_IN fifo and reads its lines
# from TT_BACKEND_OUT. The final count must equal the child's
# `expect count N` line.
set -euo pipefail
backend=$1 file=$2
dir=$(mktemp -d)
trap 'exec 3>&- 2>/dev/null || true; rm -rf -- "$dir"' EXIT
mkfifo "$dir/in"
"$backend" <"$dir/in" >"$dir/out" &
served=$!
exec 3>"$dir/in"
for _ in $(seq 1 100); do [ -s "$dir/out" ] && break; sleep 0.05; done
port=$(head -n 1 "$dir/out")
[ -n "$port" ] || { echo "with-backend: the backend printed no port" >&2; exit 1; }
base=${3:-http://127.0.0.1:$port/generic/v1}
host=${base#*://}; host=${host%%/*}; host=${host%:*}
case $host in
127.0.0.1 | localhost | "[::1]") ;;
*) echo "with-backend: refused $host, which is not a loopback address" >&2; exit 2 ;;
esac
mkdir -p "$dir/cache" "$dir/home" "$dir/xdg-cache" "$dir/xdg-config"
set +e
env -u R_LIBS_USER -u THINKTHEN_API_KEY \
    HOME="$dir/home" XDG_CACHE_HOME="$dir/xdg-cache" XDG_CONFIG_HOME="$dir/xdg-config" \
    THINKTHEN_CACHE="$dir/cache" THINKTHEN_BASE_URL="$base" THINKTHEN_API_KEY=tt-test-not-a-key \
    TT_BACKEND_ORIGIN="http://127.0.0.1:$port" TT_BACKEND_IN="$dir/in" TT_BACKEND_OUT="$dir/out" \
    TT_TESTS="$(cd "$(dirname "$0")" && pwd)" \
    timeout 600 Rscript "$file" >"$dir/log" 2>&1 3>&-
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
