#!/bin/sh
# Write the release static library from Cargo's archive (ticket 0304 slice 3b,
# ticket 0351 on macOS). Cargo's `libthinkthen_c.a` exports the bundled
# SQLite's `sqlite3_` names and Rust's own runtime names, which clash with a
# program's own SQLite or a second Rust static library. A partial link joins
# the members the header's functions need into one object. Every global name
# but the header's `thinkthen_` functions then turns local, and the object
# alone becomes the archive.
# Usage: sh localize.sh CARGO_ARCHIVE OUT_ARCHIVE
set -eu
[ "$#" = 2 ] || { echo 'usage: localize.sh CARGO_ARCHIVE OUT_ARCHIVE' >&2; exit 2; }
from=$1
out=$2
# Mach-O names carry a leading underscore.
prefix=
[ "$(uname -s)" != Darwin ] || prefix=_
names=$(nm -g --defined-only -- "$from" 2>/dev/null |
	awk -v p="${prefix}thinkthen_" 'NF == 3 && $2 == "T" && index($3, p) == 1 { print $3 }' | sort -u)
[ -n "$names" ] || { echo "localize.sh: $from defines no thinkthen_ function" >&2; exit 1; }
. "$(dirname -- "$0")/../../sdlc/scripts/scratch.sh"
scratch_dir work
set --
if [ -n "$prefix" ]; then
	# Apple's linker turns every name off the list local. Rust's standard
	# library embeds LLVM bitcode newer than Apple's tools read, and after the
	# partial link Apple's nm and linker read the object as that bitcode. The
	# LLVM objcopy that ships inside every Rust toolchain removes it.
	printf '%s\n' $names >"$work/names"
	for name in $names; do
		set -- "$@" "-Wl,-u,$name"
	done
	cc -r -nostdlib -arch "$(lipo -archs "$from")" -Wl,-exported_symbols_list,"$work/names" "$@" \
		-o "$work/joined.o" "$from" 2>"$work/ld.log" || { cat "$work/ld.log" >&2; exit 1; }
	objcopy=$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/^host: //p')/bin/rust-objcopy
	[ -x "$objcopy" ] || { echo "localize.sh: no rust-objcopy at $objcopy" >&2; exit 1; }
	"$objcopy" --remove-section=__LLVM,__bitcode --remove-section=__LLVM,__cmdline "$work/joined.o" "$work/thinkthen.o"
	ZERO_AR_DATE=1 ar rcs "$work/out.a" "$work/thinkthen.o"
else
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
fi
mv -f -- "$work/out.a" "$out"
