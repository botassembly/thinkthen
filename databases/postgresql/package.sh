#!/bin/sh
# package.sh [--reuse] FOLDER (ticket 0128): stage the PostgreSQL 16 release files in FOLDER:
# lib/ holds the loadable module and extension/ holds the control file and the SQL. A user
# copies them into `pg_config --pkglibdir` and `pg_config --sharedir`/extension.
# Without --reuse it packages the shipped build through the locked pgrx wrapper against
# PG_CONFIG, or the pg_config on PATH. With --reuse it builds nothing and stages the shipped
# tree check.sh kept. pgrx lays the tree out under pg_config's own folders, so this reads
# them from pg_config and names no host's paths or library suffix.
set -eu
reuse=
[ "${1:-}" != --reuse ] || { reuse=1; shift; }
[ $# -eq 1 ] || { echo 'usage: package.sh [--reuse] FOLDER' >&2; exit 2; }
mkdir -p -- "$1"
out=$(CDPATH='' cd -- "$1" && pwd)
cd -- "$(dirname -- "$0")"
PG_CONFIG=${PG_CONFIG:-$(command -v pg_config)}
built=${CARGO_TARGET_DIR:-target}/release
if [ -n "$reuse" ]; then
	tree=$built/thinkthen-pg16-shipped
else
	# The builder's home stays out of the module, as in check.sh.
	RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build" \
		./pgrx-package-locked.sh --pg-config "$PG_CONFIG" >/dev/null
	tree=$built/thinkthen-pg16
fi
[ -d "$tree" ] || { echo "package.sh: no packaged tree at $tree; run check.sh or package.sh without --reuse" >&2; exit 1; }
set -- "$tree$("$PG_CONFIG" --pkglibdir)"/thinkthen.*
[ $# -eq 1 ] && [ -f "$1" ] || { echo "package.sh: want one thinkthen module in $tree, found: $*" >&2; exit 1; }
mkdir -p "$out/lib" "$out/extension"
cp -- "$1" "$out/lib/"
cp -- "$tree$("$PG_CONFIG" --sharedir)"/extension/thinkthen.control \
	"$tree$("$PG_CONFIG" --sharedir)"/extension/thinkthen--*.sql "$out/extension/"
