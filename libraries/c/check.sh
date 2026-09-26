#!/bin/sh
# The C surface. The surface rung passes its loopback port as $1.
set -eu
cd -- "$(dirname -- "$0")"

command -v cc >/dev/null 2>&1 || { echo 'libraries/c: no C compiler' >&2; exit 77; }
cache=$(mktemp -d)
trap 'rm -rf -- "$cache"' EXIT
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
fi
[ -n "${THINKTHEN_ARTIFACT:-}" ] || {
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline
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
