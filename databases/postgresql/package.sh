#!/bin/sh
# package.sh [--reuse] MAJOR FOLDER (ticket 0128): stage the selected PostgreSQL release files in FOLDER:
# lib/ holds the loadable module and extension/ holds the control file and the SQL. A user
# copies them into `pg_config --pkglibdir` and `pg_config --sharedir`/extension.
# Without --reuse it packages the shipped build through the locked pgrx wrapper against
# PG_CONFIG, or /usr/bin/pg_config. With --reuse it builds nothing and stages the shipped
# tree check.sh kept. pgrx lays the tree out under pg_config's own folders, so this reads
# them from pg_config and names no host's paths or library suffix.
set -eu
reuse=
[ "${1:-}" != --reuse ] || { reuse=1; shift; }
[ $# -eq 2 ] || { echo 'usage: package.sh [--reuse] MAJOR FOLDER' >&2; exit 2; }
PG_MAJOR=$1
case $PG_MAJOR in 16|17|18) ;; *) echo "package.sh: unsupported PostgreSQL major $PG_MAJOR" >&2; exit 2 ;; esac
shift
mkdir -p -- "$1"
out=$(CDPATH='' cd -- "$1" && pwd)
cd -- "$(dirname -- "$0")"
# check.sh reads the same pg_config and target folder.
. ./runtime.sh
runtime_target
if [ "$(uname -s)" = Darwin ]; then
	PG_CONFIG=${PG_CONFIG:-$(brew --prefix postgresql@$PG_MAJOR)/bin/pg_config}
	clang=$(xcrun --find clang)
	LIBCLANG_PATH=${LIBCLANG_PATH:-${clang%/bin/clang}/lib}
	export LIBCLANG_PATH
fi
EXT=${CARGO_TARGET_DIR:-target}/release/thinkthen-pg$PG_MAJOR
if [ -n "$reuse" ]; then
	tree=$EXT-shipped
else
	# The builder's home stays out of the module, as in check.sh.
	package_flags=${RUSTFLAGS:-}
	[ "${THINKTHEN_FULL_COMPILER:-}" = 1 ] || package_flags="${package_flags:+$package_flags }--remap-path-prefix=$HOME=/build"
	RUSTFLAGS="$package_flags" \
		./pgrx-package-locked.sh --pg-config "$PG_CONFIG" --no-default-features --features "pg$PG_MAJOR" >/dev/null
	tree=$EXT
fi
[ -d "$tree" ] || { echo "package.sh: no packaged tree at $tree; run check.sh or package.sh without --reuse" >&2; exit 1; }
set -- "$tree$("$PG_CONFIG" --pkglibdir)"/thinkthen.*
[ $# -eq 1 ] && [ -f "$1" ] || { echo "package.sh: want one thinkthen module in $tree, found: $*" >&2; exit 1; }
# The tree can keep the SQL of an earlier version, so this packs only the control file's version.
share=$tree$("$PG_CONFIG" --sharedir)/extension
version=$(sed -n "s/^default_version = '\(.*\)'$/\1/p" thinkthen.control)
cmp -s thinkthen.control "$share/thinkthen.control" || { echo "package.sh: $share/thinkthen.control differs from thinkthen.control; rebuild the tree" >&2; exit 1; }
[ -f "$share/thinkthen--$version.sql" ] || { echo "package.sh: no thinkthen--$version.sql in $share; rebuild the tree" >&2; exit 1; }
mkdir -p "$out/lib" "$out/extension"
cp -- "$1" "$out/lib/"
cp -- "$share/thinkthen.control" "$share/thinkthen--$version.sql" "$out/extension/"
