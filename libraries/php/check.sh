#!/bin/sh
# Exercise the installed Composer package; shared parity is release-only.
set -eu
cd -- "$(dirname -- "$0")"
repo=$(cd ../.. && pwd)
profile=${THINKTHEN_TEST_PROFILE:-routine}
case "$profile" in routine|full|smoke) ;; stress) echo 'php: not run: no stress gate'; exit 77 ;; *) echo 'php: unknown profile' >&2; exit 2 ;; esac
[ "$(uname -s)" = Linux ] && [ "$(uname -m)" = x86_64 ] || { echo 'php: not run: Linux x86_64 required'; exit 77; }
php_bin=${THINKTHEN_PHP_BIN:-/usr/bin/php8.3}
python_bin=${THINKTHEN_PYTHON_BIN:-/usr/bin/python3}
composer=${THINKTHEN_COMPOSER_BIN:-$(command -v composer || true)}
for tool in "$php_bin" "$python_bin" "$composer"; do
 case "$tool" in /*) ;; *) echo 'php: not run: tool needs an absolute path'; exit 77 ;; esac
 [ -f "$tool" ] || { echo 'php: not run: tool unavailable'; exit 77; }
done
"$php_bin" -n -d extension=ffi -d ffi.enable=1 -r 'exit(class_exists("FFI") ? 0 : 1);' || exit 77
. "$repo/sdlc/scripts/scratch.sh"
usage_home
scratch_dir output
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
 archive=$THINKTHEN_ARTIFACT
else
 "$python_bin" "$repo/sdlc/generators/results/generate.py" --target php --check
 node "$repo/sdlc/scripts/ratchet.mjs" ratchet.php.json
 node "$repo/sdlc/scripts/ratchet.mjs" ratchet.py.json
 for file in autoload.php src/session/*.php examples/*.php fixtures/*.php; do "$php_bin" -n -l "$file" >/dev/null; done
 library=${THINKTHEN_COMPLETE_LIBRARY:-$repo/libraries/c/target/debug/libthinkthen_c.so}
 if [ -z "${THINKTHEN_COMPLETE_LIBRARY:-}" ]; then
  lock=${THINKTHEN_HEAVY_LOCK:-/tmp/thinkthen-heavy.lock}
  if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
   export THINKTHEN_HEAVY_LOCK_HELD="$lock"
   exec flock -w 180 -o "$lock" sh "$repo/libraries/php/check.sh" "$@"
  fi
  CARGO_NET_OFFLINE=true cargo build --locked --offline --manifest-path "$repo/libraries/c/Cargo.toml" --lib -j2
 fi
 "$python_bin" fixtures/package.py --library "$library" --out "$output/package"
 archive=$output/package.tar.gz
fi
"$python_bin" fixtures/session_installed.py --archive "$archive" --composer "$composer"
if [ "$profile" = full ]; then
 # The shared suite consumes the installed package and its bundled native library.
 THINKTHEN_TEST_PROFILE=full "$python_bin" fixtures/session_installed.py --archive "$archive" --composer "$composer" --parity
fi
echo 'php: pass'
