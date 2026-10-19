#!/bin/sh
# The C surface. The surface rung passes its loopback port as $1.
set -eu
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "libraries/c: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
python3 "$(dirname -- "$0")/../../sdlc/scripts/check_verdict.py" platform libraries/c
tools_group=c
. "$(dirname -- "$0")/../../sdlc/scripts/resolved-tools"
cd -- "$(dirname -- "$0")"
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS

command -v cc >/dev/null 2>&1 || { echo 'libraries/c: no C compiler' >&2; exit 1; }
python3 ../../conformance/children/privacy.py --sources . --public --artifact-env
. ../../sdlc/scripts/scratch.sh
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
scratch_dir cache
if [ "$profile" = stress ]; then
    . ../../sdlc/scripts/installed.sh
    native_install "$(cd ../.. && pwd)" "$cache/native"
    python3 - "$cache/native" "$cache" <<'SESSIONSTRESS'
from pathlib import Path
import sys
sys.path.insert(0, "tests")
from session_wait import run
run(Path(sys.argv[1]), Path(sys.argv[2]), rounds=12)
SESSIONSTRESS
    exit
fi
if [ "$profile" = smoke ]; then
    smoke_guard
    . ../../sdlc/scripts/installed.sh
    native_install "$(cd ../.. && pwd)" "$cache/native"
    python3 ../../sdlc/scripts/check-c-exports.py --self-test "$cache/native/include/thinkthen.h"
    cc -std=c11 -Wall -Wextra -Werror -I "$cache/native/include" tests/c/request_preview.c \
        -L "$cache/native/lib" -lthinkthen -Wl,-rpath,"$cache/native/lib" -o "$cache/preview"
    env -i THINKTHEN_USAGE=off HOME="$cache/home" THINKTHEN_BASE_URL=http://127.0.0.1:9/v1 \
        THINKTHEN_CACHE="$cache/cache" "$cache/preview" >"$cache/plan.json"
    python3 - "$cache/native" "$cache" <<'PY'
from pathlib import Path
import sys
sys.path.insert(0, "tests")
from installed_cases import smoke
smoke(Path(sys.argv[1]), Path(sys.argv[2]))
PY
    exit
fi
python3 tests/test_installed_consumers.py
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    python3 tests/installed-session.py "$THINKTHEN_ARTIFACT"
    exit
fi
[ -n "${THINKTHEN_ARTIFACT:-}" ] || {
	python3 ../../sdlc/scripts/generate-c-header.py --check
	python3 ../../sdlc/scripts/check-c-exports.py --self-test include/thinkthen.h
	# The remaps keep the builder's home out of this build. release-pack --reuse rebuilds the library it packs (ticket 0128).
	if [ "${THINKTHEN_FULL_COMPILER:-}" != 1 ]; then
	export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build" CFLAGS="${CFLAGS:+$CFLAGS }-ffile-prefix-map=$HOME=/build"
	fi
	cargo fmt --check
	cargo clippy --locked --offline --all-targets -- -D warnings
	cargo test --locked --offline -- --nocapture
	if [ "$profile" = full ]; then
		cargo test --locked --offline release_only_ -- --ignored --nocapture
	fi
	# The rung's own backend answers the session example.
	cargo build --quiet --locked --offline --lib
	target=${CARGO_TARGET_DIR:-target}/debug
	# Lay the door out as a release archive does, so the soname resolves.
	cp -- "$target/libthinkthen_c.so" "$cache/libthinkthen.so"
	ln -s libthinkthen.so "$cache/libthinkthen.so.0"
	cc -std=c11 -Wall -Wextra -Werror -I include examples/session.c -L "$cache" -lthinkthen \
	    -Wl,-rpath,"$cache" -o "$cache/session"
}
env -i THINKTHEN_CACHE="$cache/cache" THINKTHEN_API_KEY=sk-examples-loopback \
    THINKTHEN_BASE_URL="http://127.0.0.1:$1/generic/v1" \
    "$cache/session" >"$cache/out" 2>"$cache/err"
[ ! -s "$cache/err" ] || { cat -- "$cache/err" >&2; exit 1; }
python3 - "$cache/out" <<'CALL'
import json, sys
call = json.load(open(sys.argv[1]))
assert call['facts']['records'] == 1 and call['facts']['requests_sent'] == 1, call
CALL
