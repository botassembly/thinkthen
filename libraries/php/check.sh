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
command -v php >/dev/null 2>&1 || { echo 'php: not run: no PHP 8.3' >&2; exit 77; }
php -v | grep -q '^PHP 8\.3\.' || { echo 'php: not run: PHP 8.3 is required' >&2; exit 77; }
php -d ffi.enable=1 -r 'exit(class_exists("FFI") ? 0 : 1);' || { echo 'php: not run: ext-ffi is unavailable' >&2; exit 77; }
for tool in cargo cc nm readelf bwrap python3 node; do
    command -v "$tool" >/dev/null 2>&1 || { echo "php: not run: no $tool" >&2; exit 77; }
done
unset THINKTHEN_API_KEY
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.php.json
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.py.json
python3 -c 'import json; p=json.load(open("composer.json")); assert p["name"]=="botassembly/thinkthen" and p["require"]=={"php":">=8.3","ext-ffi":"*"}'
for file in src/*.php examples/*.php fixtures/*.php; do php -d ffi.enable=1 -l "$file" >/dev/null; done
lock=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-6.lock}
flock -w 180 -o "$lock" env CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= \
    cargo build --locked --offline --manifest-path "$repo/libraries/c/Cargo.toml" --lib -j2
flock -w 180 -o "$lock" env CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= \
    cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
python3 fixtures/abi.py
python3 fixtures/installed.py
for plant in source header native canary wrong-value; do python3 fixtures/installed.py "$plant"; done
python3 fixtures/run_matrix.py
python3 fixtures/plant.py
python3 fixtures/type_cases.py
echo 'php: pass'
