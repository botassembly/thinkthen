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
    # Compile outside the checkout with the README's unedited pkg-config flags.
    scratch_dir consumer
    tar -xzf "$THINKTHEN_ARTIFACT" -C "$consumer"
    cp examples/session.c "$consumer/consumer.c"
    (
        cd "$consumer"
        export PKG_CONFIG_PATH="$PWD/lib/pkgconfig"
        case $(uname -s) in
        Darwin) static_selection=-Wl,-dead_strip_dylibs ;;
        *) static_selection=-Wl,--as-needed ;;
        esac
        cc -std=c11 consumer.c -o consumer $(pkg-config --cflags thinkthen) \
            "$PWD/lib/libthinkthen.a" "$static_selection" $(pkg-config --libs --static thinkthen)
        cat >sqlite-consumer.c <<'SQLITE'
#include <thinkthen.h>
#include <sqlite3.h>
int main(void) {
    uint32_t interface_number = 0;
    sqlite3 *database = NULL;
    if (!thinkthen_version(&interface_number) || interface_number != THINKTHEN_INTERFACE_NUMBER)
        return 1;
    if (sqlite3_open(":memory:", &database) != SQLITE_OK) return 2;
    return sqlite3_close(database) != SQLITE_OK;
}
SQLITE
        cc -std=c11 sqlite-consumer.c -o sqlite-consumer \
            $(pkg-config --cflags thinkthen) "$PWD/lib/libthinkthen.a" \
            "$static_selection" $(pkg-config --libs --static thinkthen) -lsqlite3
        env -i ./sqlite-consumer
    )
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
	case $(uname -s) in
	Darwin) library=libthinkthen.dylib source_library=libthinkthen_c.dylib soname=libthinkthen.0.dylib ;;
	*) library=libthinkthen.so source_library=libthinkthen_c.so soname=libthinkthen.so.0 ;;
	esac
	cp -- "$target/$source_library" "$cache/$library"
	ln -s "$library" "$cache/$soname"
	cc -std=c11 -Wall -Wextra -Werror -I include examples/session.c -L "$cache" -lthinkthen \
	    -Wl,-rpath,"$cache" -o "$cache/session"
}
env -i THINKTHEN_CACHE="$cache/cache" THINKTHEN_API_KEY=sk-examples-loopback \
    THINKTHEN_BASE_URL="http://127.0.0.1:$1/generic/v1" \
    "$cache/session" >"$cache/out" 2>"$cache/err"
[ ! -s "$cache/err" ] || { cat -- "$cache/err" >&2; exit 1; }
python3 - "$cache/out" <<'CALL'
import json, sys
answer = json.load(open(sys.argv[1]))
assert answer == {'answer': True, 'probability': 0.9}, answer
CALL
