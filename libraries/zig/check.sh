#!/bin/sh
# Exercise the installed owned API; full shared parity is release-only.
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
case ${THINKTHEN_TEST_PROFILE:-routine} in
 routine|full|smoke) ;; stress) echo 'Zig: not run: no stress gate'; exit 77 ;;
 *) echo 'Zig: unknown test profile' >&2; exit 2 ;;
esac
zig=${THINKTHEN_ZIG:-$(command -v zig || true)}
[ -x "$zig" ] || exit 77
export THINKTHEN_ZIG="$zig"
for tool in python3 node cc flock; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
export PATH="$(dirname "$zig"):$PATH"
lock=${THINKTHEN_HEAVY_LOCK:-/tmp/thinkthen-zig.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
    export THINKTHEN_HEAVY_LOCK_HELD="$lock"
    exec flock -w 180 -o "$lock" /bin/sh "$0" "$@"
fi
. "$root/sdlc/scripts/scratch.sh"
usage_home
mkdir -p "$here/target/cache" "$here/target/scratch" "$here/target/artifacts"
python3 "$root/sdlc/generators/results/generate.py" --target zig --inputs --check
python3 "$root/sdlc/generators/results/generate.py" --target zig --check
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.zig.json"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.py.json"
"$zig" fmt --check "$here/src/thinkthen.zig" "$here/src/session.zig" "$here/src/authored.zig" "$here/build.zig" \
    "$here/Tests/session.zig" "$here/Tests/session_consumer.zig" "$here/Tests/request_fixture.zig" "$here/Tests/view_check.zig" \
    "$here/Tests/session-build.zig" "$here/examples/decide.zig" "$here/examples/smoke.zig"
if [ -z "${THINKTHEN_ARTIFACT:-}" ]; then
    . "$root/sdlc/scripts/installed.sh"
    scratch_dir native
    native_install "$root" "$native"
    tar -czf "$here/target/artifacts/c.tar.gz" -C "$native" .
    THINKTHEN_C_ARTIFACT="$here/target/artifacts/c.tar.gz" python3 "$here/Tests/package_local.py"
    version=$(sed -n 's/^[[:space:]]*\.version = "\([^"]*\)",$/\1/p' "$here/build.zig.zon")
    THINKTHEN_ARTIFACT="$here/target/artifacts/thinkthen-zig-$version-x86_64-unknown-linux-gnu.tar.gz"
    export THINKTHEN_ARTIFACT
fi
if [ "${THINKTHEN_TEST_PROFILE:-routine}" = smoke ]; then
    smoke_guard
    . "$root/sdlc/scripts/installed.sh"
    scratch_dir smoke
    tar -xzf "$THINKTHEN_ARTIFACT" -C "$smoke"
    native="$smoke/native/x86_64-unknown-linux-gnu"
    "$zig" build-exe -j2 -fllvm -flld "$native/lib/libthinkthen.a" --cache-dir "$here/target/scratch/smoke-cache" --global-cache-dir "$here/target/cache" \
      --dep thinkthen -Mroot="$here/examples/smoke.zig" -I "$native/include" -Mthinkthen="$smoke/src/thinkthen.zig" \
      -lc -lgcc_s -lutil -lrt -lpthread -lm -ldl -femit-bin="$smoke/consumer"
    "$smoke/consumer"
    exit
fi
python3 "$here/Tests/guard.py" "$THINKTHEN_ARTIFACT"
python3 "$here/Tests/installed.py"
