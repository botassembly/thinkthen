set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared Zig executable" >&2; exit 2; }
# Fetches the Zig package archive scripts/prepare-packages.sh staged, the way
# `zig fetch --save=thinkthen URL` fetches the release archive, then builds and runs show.zig.
packages=$(cd ../../../target/site-packages && pwd)
zig=$THINKTHEN_BUILD_TOOLS/zig/0.17.0/zig
export ZIG_GLOBAL_CACHE_DIR="$packages/../site-zig-cache"
work=${2:-$packages/../site-zig-example}
mkdir -p "$work/src"
cp build.zig build.zig.zon "$work/"
# show.zig uses the default settings. This routine-only branch selects
# the test backend explicitly for the compiled local check.
if [ "${THINKTHEN_BACKEND:-}" = stub ]; then
  sed 's/tt.Client.init(.{}, /tt.Client.init(.{ .backend = .stub }, /' show.zig > "$work/src/main.zig"
  grep -q '.backend = .stub' "$work/src/main.zig" || { echo 'run.sh: show.zig has no tt.Client.init(.{}, ...)' >&2; exit 1; }
else
  cp show.zig "$work/src/main.zig"
fi
cp "$packages/zig/thinkthen-zig-linux-x64.tar.gz" "$work/thinkthen.tar.gz"
cd "$work"
"$zig" fetch --save=thinkthen "$work/thinkthen.tar.gz"
"$zig" build -j2
cat > "$work/run.sh" <<'RUN'
exec "$(dirname "$0")/zig-out/bin/example"
RUN
[ "${1:-}" = --prepare ] || bash "$work/run.sh"
