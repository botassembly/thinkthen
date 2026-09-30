#!/bin/sh
# PHP package gate. The surface rung supplies $1; fixtures start their own counted loopback backend.
set -eu
cd -- "$(dirname -- "$0")"
repo=$(cd ../.. && pwd)
profile=${THINKTHEN_TEST_PROFILE:-routine}
case "$profile" in routine|full) ;; stress) echo 'php: not run: no stress gate'; exit 77 ;; *) echo "php: unknown profile $profile" >&2; exit 2 ;; esac
if [ "$(uname -s)" != Linux ] || [ "$(uname -m)" != x86_64 ]; then
    echo 'php: not run: this package gate is proven on Linux x86_64'; exit 77
fi
php_bin=${THINKTHEN_PHP_BIN:-/usr/bin/php8.3}
python_bin=${THINKTHEN_PYTHON_BIN:-/usr/bin/python3}
bwrap_bin=${THINKTHEN_BWRAP_BIN:-/usr/bin/bwrap}
flock_bin=${THINKTHEN_FLOCK_BIN:-/usr/bin/flock}
git_bin=${THINKTHEN_GIT_BIN:-/usr/bin/git}
for named in "$php_bin" "$python_bin" "$flock_bin"; do
    case "$named" in /*) ;; *) echo "php: not run: tool path is not absolute: $named" >&2; exit 77 ;; esac
    [ -x "$named" ] || { echo "php: not run: tool is unavailable: $named" >&2; exit 77; }
done
"$php_bin" -v | grep -q '^PHP 8\.3\.' || { echo 'php: not run: PHP 8.3 is required' >&2; exit 77; }
"$php_bin" -n -d extension=ffi -d ffi.enable=1 -r 'exit(class_exists("FFI") ? 0 : 1);' ||
    { echo 'php: not run: PHP 8.3 ext-ffi is unavailable' >&2; exit 77; }
"$flock_bin" --version >/dev/null 2>&1 || { echo 'php: not run: flock is unavailable' >&2; exit 77; }
for tool in cargo cc nm readelf; do
    command -v "$tool" >/dev/null 2>&1 || { echo "php: not run: no $tool" >&2; exit 77; }
done
export THINKTHEN_PHP_BIN="$php_bin" THINKTHEN_PYTHON_BIN="$python_bin"
export THINKTHEN_BWRAP_BIN="$bwrap_bin" THINKTHEN_FLOCK_BIN="$flock_bin" THINKTHEN_GIT_BIN="$git_bin"
unset THINKTHEN_API_KEY
# Run one command under the named lock. A caller that already holds it, such
# as the surfaces rung, exports THINKTHEN_HEAVY_LOCK_HELD; waiting on it again
# would only time out.
locked() {
    held=$1; shift
    if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" = "$held" ]; then "$@"; else "$flock_bin" -w 180 -o "$held" "$@"; fi
}
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. "$repo/sdlc/scripts/scratch.sh"
usage_home
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'PHP installed: C archive missing' >&2; exit 1; }
    for tool in cargo nm readelf; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
    . "$repo/sdlc/scripts/scratch.sh"
    . "$repo/sdlc/scripts/installed.sh"
    installed_unpack
    package=$scratch
    scratch_dir native
    tar -xzf "$THINKTHEN_C_ARTIFACT" -C "$native"
    "$python_bin" "$repo/sdlc/scripts/check-c-exports.py" "$native/include/thinkthen.h" "$native/lib/libthinkthen.so"
    readelf -d "$native/lib/libthinkthen.so" | grep -q 'Library soname: \[libthinkthen.so.0\]'
    [ "$(readlink "$native/lib/libthinkthen.so.0")" = libthinkthen.so ] || exit 1
    locked "${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-7.lock}" \
        env CARGO_TARGET_DIR="$repo/target" CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= \
        cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
    THINKTHEN_RELEASE_PHP_DIR="$package" THINKTHEN_RELEASE_C_DIR="$native" \
        THINKTHEN_BACKEND_BIN="$repo/target/debug/conformance-backend" "$python_bin" fixtures/portable_batch.py
    "$python_bin" fixtures/release_plants.py "$package" "$native"
    echo 'PHP installed release PASS: five typed rows and three literal requests'
    exit 0
fi
for named in "$bwrap_bin" "$git_bin"; do
    case "$named" in /*) ;; *) echo "php: not run: tool path is not absolute: $named" >&2; exit 77 ;; esac
    [ -x "$named" ] || { echo "php: not run: tool is unavailable: $named" >&2; exit 77; }
done
"$python_bin" -c 'import jsonschema' || { echo 'php: not run: Python jsonschema is unavailable' >&2; exit 77; }
"$bwrap_bin" --version >/dev/null 2>&1 || { echo 'php: not run: bwrap is unavailable' >&2; exit 77; }
"$git_bin" --version >/dev/null 2>&1 || { echo 'php: not run: git is unavailable' >&2; exit 77; }
command -v node >/dev/null 2>&1 || { echo 'php: not run: no node' >&2; exit 77; }
"$python_bin" fixtures/selector_preflight.py
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.php.json
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.py.json
"$python_bin" -c 'import json; p=json.load(open("composer.json")); assert p["name"]=="botassembly/thinkthen" and p["require"]=={"php":">=8.3","ext-ffi":"*"}'
for file in src/*.php examples/*.php fixtures/*.php; do "$php_bin" -d ffi.enable=1 -l "$file" >/dev/null; done
lock=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-6.lock}
locked "$lock" env CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= \
    cargo build --locked --offline --manifest-path "$repo/libraries/c/Cargo.toml" --lib -j2
locked "$lock" env CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= \
    cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
THINKTHEN_PORTABLE_LIBRARY="$repo/libraries/c/target/debug/libthinkthen_c.so" "$python_bin" fixtures/portable_batch.py
"$python_bin" fixtures/abi.py
"$python_bin" fixtures/installed.py
for plant in source header native canary private-key wrong-value; do "$python_bin" fixtures/installed.py "$plant"; done
"$python_bin" fixtures/run_matrix.py
"$python_bin" fixtures/plant.py
"$python_bin" fixtures/type_cases.py
echo 'php: pass'
