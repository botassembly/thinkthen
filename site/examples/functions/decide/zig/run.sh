#!/bin/sh
# Build the shown source with the site's staged Zig package and build recipe.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared program" >&2; exit 2; }
root=$(cd ../../../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
zig=$THINKTHEN_BUILD_TOOLS/zig/0.17.0/zig
if [ "${1:-}" = --prepare ]; then
  work=$2; mkdir -p "$work"
else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
trap 'exit 1' HUP INT TERM
mkdir -p "$work/src"
cp ../../../lang-zig/build.zig ../../../lang-zig/build.zig.zon "$work/"
cp show.zig "$work/src/main.zig"
export ZIG_GLOBAL_CACHE_DIR="$packages/../site-zig-cache"
cp "$packages/zig/thinkthen-zig-linux-x64.tar.gz" "$work/thinkthen.tar.gz"
cd "$work"
"$zig" fetch --save=thinkthen "$work/thinkthen.tar.gz"
"$zig" build -j2
cat > "$work/run.sh" <<'RUN'
#!/bin/sh
set -eu
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
"$(dirname "$0")/zig-out/bin/example" > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(fromjson)' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"
RUN
[ "${1:-}" = --prepare ] || sh "$work/run.sh"
