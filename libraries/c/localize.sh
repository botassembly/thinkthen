#!/bin/sh
# Write the release static library from Cargo's archive (ticket 0304 slice 3b).
# Cargo's `libthinkthen_c.a` exports the bundled SQLite's `sqlite3_` names and
# Rust's own runtime names, which clash with a program's own SQLite or a second
# Rust static library. A partial link joins the members the header's functions
# need into one object. Every global name but the header's `thinkthen_`
# functions then turns local, and the object alone becomes the archive.
# Usage: sh localize.sh CARGO_ARCHIVE OUT_ARCHIVE
set -eu
[ "$#" = 2 ] || { echo 'usage: localize.sh CARGO_ARCHIVE OUT_ARCHIVE' >&2; exit 2; }
from=$1
out=$2
if [ "$(uname -s)" = Darwin ]; then
	# One M5 try of `cc -r -Wl,-exported_symbols_list` wrote an object Apple's
	# nm could not read. See sdlc/issues/2026-09-30-static-library-exports-sqlite-symbols.md.
	echo 'localize.sh: macOS keeps the static library unchanged; its sqlite3_ names stay global' >&2
	cp -f -- "$from" "$out"
	exit 0
fi
names=$(nm -g --defined-only -- "$from" 2>/dev/null | awk 'NF == 3 && $2 == "T" && $3 ~ /^thinkthen_/ { print $3 }' | sort -u)
[ -n "$names" ] || { echo "localize.sh: $from defines no thinkthen_ function" >&2; exit 1; }
work=$(mktemp -d "${TMPDIR:-/tmp}/thinkthen-localize.XXXXXX")
trap 'rm -rf -- "$work"' EXIT
set --
for name in $names; do
	set -- "$@" -u "$name"
done
ld -r "$@" -o "$work/joined.o" "$from"
set --
for name in $names; do
	set -- "$@" "--keep-global-symbol=$name"
done
objcopy "$@" "$work/joined.o" "$work/thinkthen.o"
ar rcsD "$work/out.a" "$work/thinkthen.o"
mv -f -- "$work/out.a" "$out"
