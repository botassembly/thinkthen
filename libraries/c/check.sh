#!/bin/sh
# The C surface. The surface rung passes its loopback port as $1.
set -eu
cd -- "$(dirname -- "$0")"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "libraries/c: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
if [ "$profile" = stress ]; then
	echo 'libraries/c: not run: no port load campaign'
	exit 77
fi

command -v cc >/dev/null 2>&1 || { echo 'libraries/c: no C compiler' >&2; exit 77; }
. ../../sdlc/scripts/scratch.sh
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
scratch_dir cache
if [ "$profile" = smoke ]; then
	smoke_guard
	command -v pkg-config >/dev/null 2>&1 || { echo 'libraries/c: no pkg-config' >&2; exit 77; }
	. ../../sdlc/scripts/installed.sh
	native_install "$(cd ../.. && pwd)" "$cache"
	# shellcheck disable=SC2046 # pkg-config prints flags to split.
	cc -std=c11 -Wall -Wextra -Werror $(PKG_CONFIG_PATH=$cache/lib/pkgconfig pkg-config --cflags thinkthen) \
		examples/smoke.c $(PKG_CONFIG_PATH=$cache/lib/pkgconfig pkg-config --libs thinkthen) -Wl,-rpath,"$cache/lib" -o "$cache/smoke"
	"$cache/smoke"
	exit
fi
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
	# The installed-file mode (ticket 0128): the slide builds from the unpacked archive alone,
	# found through its own pkg-config file.
	command -v pkg-config >/dev/null 2>&1 || { echo 'libraries/c: no pkg-config' >&2; exit 77; }
	tar -xzf "$THINKTHEN_ARTIFACT" -C "$cache"
	PKG_CONFIG_PATH=$cache/lib/pkgconfig
	export PKG_CONFIG_PATH
	pkg-config --exists thinkthen || { echo 'libraries/c: the archive holds no pkg-config file for thinkthen' >&2; exit 1; }
	libdir=$(pkg-config --variable=libdir thinkthen)
	case $libdir in "$cache"/*) ;; *) echo "libraries/c: pkg-config names $libdir, outside the archive" >&2; exit 1 ;; esac
	# shellcheck disable=SC2046 # pkg-config prints flags to split.
	cc -std=c11 -Wall -Wextra -Werror $(pkg-config --cflags thinkthen) examples/slide.c \
		$(pkg-config --libs thinkthen) -Wl,-rpath,"$libdir" -o "$cache/slide"
	# Ticket 0374: the shipped library keeps its own panic hook, and the token cap variable
	# refuses a call before it sends, counted at a backend this check starts.
	. ../../sdlc/scripts/installed.sh
	case $(uname -s) in Darwin) own_panic_hook "$libdir/libthinkthen.dylib" ;; *) own_panic_hook "$libdir/libthinkthen.so" ;; esac
	# shellcheck disable=SC2046 # pkg-config prints flags to split.
	cc -std=c11 -Wall -Wextra -Werror $(pkg-config --cflags thinkthen) tests/c/driver.c \
		$(pkg-config --libs thinkthen) -Wl,-rpath,"$libdir" -o "$cache/driver"
	cd ../.. && backend_start && cd libraries/c
	base=http://127.0.0.1:$port/generic/v1 question='{"decide":"asks for a refund"}' text='Refund me.'
	ask() {
		folder=$1
		shift
		printf 'decide 3\n%s\n%s\n%s\n%s\n%s\n%s\n' ${#base} "$base" ${#question} "$question" ${#text} "$text" |
			env -i THINKTHEN_CACHE="$cache/$folder" THINKTHEN_API_KEY=sk-examples-loopback "$@" "$cache/driver"
	}
	refusal="max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request"
	[ "$(ask capped THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL=10)" = "$(printf '1 %s\n%s' ${#refusal} "$refusal")" ] ||
		{ echo 'libraries/c: the token cap did not refuse with code 1 and its sentence' >&2; exit 1; }
	[ "$(backend_count)" = 0 ] || { echo 'libraries/c: the refused call reached the backend' >&2; exit 1; }
	[ "$(ask open | head -n 1 | cut -d' ' -f1)" = 0 ] && [ "$(backend_count)" = 1 ] ||
		{ echo 'libraries/c: the uncapped call did not send exactly one request' >&2; exit 1; }
	echo 'libraries/c: the token cap refused before sending, installed'
fi
[ -n "${THINKTHEN_ARTIFACT:-}" ] || {
	# The remaps keep the builder's home out of this build. release-pack --reuse rebuilds the library it packs (ticket 0128).
	export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build" CFLAGS="${CFLAGS:+$CFLAGS }-ffile-prefix-map=$HOME=/build"
	cargo fmt --check
	cargo clippy --locked --offline --all-targets -- -D warnings
	cargo test --locked --offline -- --nocapture
	# The rung's own backend answers the slide as the test's backends do.
	cargo build --quiet --locked --offline --lib
	target=${CARGO_TARGET_DIR:-target}/debug
	# Lay the door out as a release archive does, so the soname resolves.
	cp -- "$target/libthinkthen_c.so" "$cache/libthinkthen.so"
	ln -s libthinkthen.so "$cache/libthinkthen.so.0"
	cc -std=c11 -Wall -Wextra -Werror -I include examples/slide.c -L "$cache" -lthinkthen \
	    -Wl,-rpath,"$cache" -o "$cache/slide"
}
env -i THINKTHEN_CACHE="$cache/cache" THINKTHEN_API_KEY=sk-examples-loopback \
    THINKTHEN_BASE_URL="http://127.0.0.1:$1/generic/v1" \
    "$cache/slide" >"$cache/out" 2>"$cache/err"
[ ! -s "$cache/err" ] || { cat -- "$cache/err" >&2; exit 1; }
diff -u examples/slide.txt "$cache/out"
