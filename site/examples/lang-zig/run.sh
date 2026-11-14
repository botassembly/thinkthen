#!/bin/sh
# Build this cell's unchanged shown source once with the staged Zig package.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared Zig executable" >&2; exit 2; }
root=$(CDPATH= cd "$(dirname "$0")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
zig=$THINKTHEN_BUILD_TOOLS/zig/0.17.0/zig
if [ "${1:-}" = --prepare ]; then
  work=$2
  mkdir -p "$work"
else
  work=$(mktemp -d)
fi
mkdir -p "$work/src"
cp "$root/site/examples/lang-zig/build.zig" "$root/site/examples/lang-zig/build.zig.zon" "$work/"
cp show.zig "$work/src/main.zig"
export ZIG_GLOBAL_CACHE_DIR="$packages/../site-zig-cache"
cp "$packages/zig/thinkthen-zig-linux-x64.tar.gz" "$work/thinkthen.tar.gz"
if [ -f normalize.sh ]; then
  cp normalize.sh "$work/normalize.sh"
elif [ -f ../c/normalize.sh ]; then
  cp ../c/normalize.sh "$work/normalize.sh"
fi
prepare=${1:-}
cd "$work"
"$zig" fetch --save=thinkthen "$work/thinkthen.tar.gz"
"$zig" build -j2
cat > "$work/run.sh" <<'RUN'
#!/bin/sh
set -eu
program=$(CDPATH= cd "$(dirname "$0")" && pwd)
if [ -f "$program/normalize.sh" ]; then
  output=$(mktemp)
  status=0
  "$program/zig-out/bin/example" > "$output" || status=$?
  cat "$output"
  [ "$status" -eq 0 ] || exit "$status"
  sh "$program/normalize.sh" "$output"
else
  exec "$program/zig-out/bin/example"
fi
RUN
[ "$prepare" = --prepare ] || sh "$work/run.sh"
