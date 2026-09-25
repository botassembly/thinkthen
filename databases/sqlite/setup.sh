#!/bin/sh
# One-time setup of the SQLite 3.50.0 amalgamation this surface tests on.
# `setup.sh FOLDER` copies sqlite3.c, shell.c, sqlite3.h, and sqlite3ext.h
# from FOLDER. With no argument it fetches the zip from sqlite.org once.
# Either way it refuses unless both pinned hashes match, then builds the host.
set -eu
here=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
target=${SQLITE_AMALGAMATION:-$HOME/.cache/thinkthen-toolchains/sqlite-amalgamation-3500000}
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
if [ $# -ge 1 ]; then
	from=$1
else
	curl -fsSL -o "$work/amalgamation.zip" https://sqlite.org/2025/sqlite-amalgamation-3500000.zip
	unzip -q "$work/amalgamation.zip" -d "$work"
	from=$work/sqlite-amalgamation-3500000
fi
mkdir -p -- "$work/checked"
cp -- "$from/sqlite3.c" "$from/shell.c" "$from/sqlite3.h" "$from/sqlite3ext.h" "$work/checked/"
if ! (cd -- "$work/checked" && sha256sum --check --quiet) <"$here/amalgamation.sha256"; then
	echo "setup: the amalgamation in $from does not match the pinned hashes" >&2
	exit 1
fi
mkdir -p -- "$target"
cp -- "$work/checked/"* "$target/"
SQLITE_AMALGAMATION=$target sh "$here/tests/host_sqlite.sh"
