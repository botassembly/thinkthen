#!/bin/sh
# Build the SQLite 3.50.0 host once: libsqlite3.so.0 for Python through
# LD_LIBRARY_PATH, and the sqlite3 CLI from shell.c. Both land in the
# toolchain folder, keyed by the two pinned source hashes, and the script
# prints that folder. check.sh checks the hashes before it calls this.
set -eu
source=${SQLITE_AMALGAMATION:-$HOME/.cache/thinkthen-toolchains/sqlite-amalgamation-3500000}
host=$HOME/.cache/thinkthen-toolchains/sqlite-3500000-host
stamp=$(cd -- "$source" && sha256sum sqlite3.c shell.c)
if [ -x "$host/sqlite3" ] && [ -f "$host/libsqlite3.so.0" ] && [ "$(cat -- "$host/SOURCE.sha256" 2>/dev/null)" = "$stamp" ]; then
	echo "$host"
	exit 0
fi
. "$(dirname -- "$0")/../../../sdlc/scripts/scratch.sh"
scratch_dir building
cc -O2 -fPIC -shared -o "$building/libsqlite3.so.0" "$source/sqlite3.c" -lpthread -ldl -lm
cc -O2 -I"$source" -o "$building/sqlite3" "$source/shell.c" "$source/sqlite3.c" -lpthread -ldl -lm
printf '%s\n' "$stamp" >"$building/SOURCE.sha256"
mkdir -p -- "$host"
mv -f -- "$building/libsqlite3.so.0" "$building/sqlite3" "$building/SOURCE.sha256" "$host/"
echo "$host"
