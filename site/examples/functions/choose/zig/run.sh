#!/bin/sh
# Build the shown source with the site's staged Zig package and build recipe.
set -eu
root=$(cd ../../../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
zig=$THINKTHEN_BUILD_TOOLS/zig/0.17.0/zig
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
trap 'exit 1' HUP INT TERM
mkdir -p "$work/src"
cp ../../../lang-zig/build.zig ../../../lang-zig/build.zig.zon "$work/"
cp show.zig "$work/src/main.zig"
export ZIG_GLOBAL_CACHE_DIR="$packages/../site-zig-cache"
cp "$packages/zig/thinkthen-zig-linux-x64.tar.gz" "$work/thinkthen.tar.gz"
cd "$work"
"$zig" fetch --save=thinkthen "$work/thinkthen.tar.gz"
"$zig" build -j2
"$work/zig-out/bin/example" > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0))' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"
