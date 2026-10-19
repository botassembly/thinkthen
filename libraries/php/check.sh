#!/bin/sh
# Exercise the installed Composer package; shared parity is release-only.
set -eu
profile=${THINKTHEN_TEST_PROFILE:-routine}
case "$profile" in routine|full|smoke) ;; stress) echo 'php: not run: no stress gate'; exit 1 ;; *) echo 'php: unknown profile' >&2; exit 2 ;; esac
python3 "$(dirname -- "$0")/../../sdlc/scripts/check_verdict.py" platform libraries/php
tools_group=php
. "$(dirname -- "$0")/../../sdlc/scripts/resolved-tools"
cd -- "$(dirname -- "$0")"
repo=$(cd ../.. && pwd)
php_bin=$(command -v php || true)
python_bin=$(command -v python3 || true)
composer=$(command -v composer || true)
for tool in "$php_bin" "$python_bin" "$composer"; do
 case "$tool" in /*) ;; *) echo 'php: not run: tool needs an absolute path'; exit 1 ;; esac
 [ -f "$tool" ] || { echo 'php: not run: tool unavailable'; exit 1; }
done
"$php_bin" -n -d extension=ffi -d ffi.enable=1 -r 'exit(class_exists("FFI") ? 0 : 1);' || exit 1
CARGO_NET_OFFLINE=true cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --target-dir "${CARGO_TARGET_DIR:-$repo/target}" --package conformance-backend -j2
. "$repo/sdlc/scripts/scratch.sh"
usage_home
scratch_dir output
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
 archive=$THINKTHEN_ARTIFACT
else
 "$python_bin" "$repo/sdlc/generators/results/generate.py" --target php --check
 for file in autoload.php src/session/*.php examples/*.php fixtures/*.php; do "$php_bin" -n -l "$file" >/dev/null; done
 library=${THINKTHEN_COMPLETE_LIBRARY:-${CARGO_TARGET_DIR:-$repo/target}/debug/libthinkthen_c.so}
 if [ -z "${THINKTHEN_COMPLETE_LIBRARY:-}" ]; then
  env CARGO_NET_OFFLINE=true RUSTC_WRAPPER= cargo build --locked --offline --manifest-path "$repo/libraries/c/Cargo.toml" --lib --target-dir "${CARGO_TARGET_DIR:-$repo/target}" -j2
 fi
 "$python_bin" fixtures/package.py --library "$library" --out "$output/package"
 archive=$output/package.tar.gz
fi
if [ "$profile" = smoke ]; then
 smoke_guard
 "$python_bin" fixtures/session_installed.py --archive "$archive" --composer "$composer" --smoke
 exit
elif [ "$profile" = full ]; then
 # The shared suite consumes the installed package and its bundled native library.
 THINKTHEN_TEST_PROFILE=full "$python_bin" fixtures/session_installed.py --archive "$archive" --composer "$composer" --parity
else
 "$python_bin" fixtures/session_installed.py --archive "$archive" --composer "$composer"
fi
echo 'php: pass'
