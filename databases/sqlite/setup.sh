#!/bin/sh
# One-time setup of the SQLite 3.50.0 amalgamation this surface tests on.
# `setup.sh [FOLDER [OLD_FOLDER]]` copies checked amalgamations or fetches
# them from sqlite.org once. macOS also needs a real 3.49.0 host for the floor.
set -eu
here=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
target=${SQLITE_AMALGAMATION:-$HOME/.cache/thinkthen-toolchains/sqlite-amalgamation-3500000}
old_target=${SQLITE_OLD_AMALGAMATION:-$HOME/.cache/thinkthen-toolchains/sqlite-amalgamation-3490000}
if [ "$(uname -s)" = Darwin ]; then
	command -v shasum >/dev/null 2>&1 || { echo 'setup: shasum is required on macOS' >&2; exit 77; }
	hash_check() { shasum -a 256 -c "$1" >/dev/null; }
else
	command -v sha256sum >/dev/null 2>&1 || { echo 'setup: sha256sum is required on Linux' >&2; exit 77; }
	hash_check() { sha256sum --check --quiet "$1"; }
fi
. "$here/../../sdlc/scripts/scratch.sh"
scratch_dir work
if [ $# -ge 1 ]; then
	from=$1
else
	curl -fsSL -o "$work/amalgamation.zip" https://sqlite.org/2025/sqlite-amalgamation-3500000.zip
	unzip -q "$work/amalgamation.zip" -d "$work"
	from=$work/sqlite-amalgamation-3500000
fi
mkdir -p -- "$work/checked"
cp -- "$from/sqlite3.c" "$from/shell.c" "$from/sqlite3.h" "$from/sqlite3ext.h" "$work/checked/"
if ! (cd -- "$work/checked" && hash_check "$here/amalgamation.sha256"); then
	echo "setup: the amalgamation in $from does not match the pinned hashes" >&2
	exit 1
fi
mkdir -p -- "$target"
cp -- "$work/checked/"* "$target/"
if [ "$(uname -s)" = Darwin ]; then
	if [ $# -ge 2 ]; then
		old_from=$2
	elif [ -f "$old_target/sqlite3.c" ] && [ -f "$old_target/shell.c" ]; then
		old_from=$old_target
	else
		curl -fsSL -o "$work/old-amalgamation.zip" https://sqlite.org/2025/sqlite-amalgamation-3490000.zip
		unzip -q "$work/old-amalgamation.zip" -d "$work"
		old_from=$work/sqlite-amalgamation-3490000
	fi
	mkdir -p -- "$work/old-checked"
	cp -- "$old_from/sqlite3.c" "$old_from/shell.c" "$old_from/sqlite3.h" "$old_from/sqlite3ext.h" "$work/old-checked/"
	if ! (cd -- "$work/old-checked" && hash_check "$here/amalgamation-3490000.sha256"); then
		echo "setup: the 3.49.0 amalgamation in $old_from does not match the pinned hashes" >&2
		exit 1
	fi
	mkdir -p -- "$old_target"
	cp -- "$work/old-checked/"* "$old_target/"
fi
SQLITE_AMALGAMATION=$target SQLITE_OLD_AMALGAMATION=$old_target sh "$here/tests/host_sqlite.sh"
