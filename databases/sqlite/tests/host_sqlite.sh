#!/bin/sh
# Build the checked SQLite host once. Linux uses libsqlite3.so.0 for Python;
# macOS uses native load probes for the exact 3.50.0 and 3.49.0 floor cases.
# Both build the pinned sqlite3 CLI. Linux also builds a single-thread
# library in single/ for the thread-mode refusal (ticket 0304 slice 3b).
# check.sh verifies source hashes first.
set -eu
source=${SQLITE_AMALGAMATION:-$HOME/.cache/thinkthen-toolchains/sqlite-amalgamation-3500000}
host=$HOME/.cache/thinkthen-toolchains/sqlite-3500000-host
if [ "$(uname -s)" = Darwin ]; then
	command -v shasum >/dev/null 2>&1 || { echo 'host_sqlite: shasum is required on macOS' >&2; exit 77; }
	stamp=$(cd -- "$source" && shasum -a 256 sqlite3.c shell.c)
	old=${SQLITE_OLD_AMALGAMATION:-$HOME/.cache/thinkthen-toolchains/sqlite-amalgamation-3490000}
	stamp="$stamp
$(cd -- "$old" && shasum -a 256 sqlite3.c shell.c)
$(shasum -a 256 "$(dirname -- "$0")/load_probe.c")"
	if [ -x "$host/load-probe-3500000" ] && [ -x "$host/load-probe-3490000" ] &&
		[ -x "$host/sqlite3" ] && [ "$(cat -- "$host/SOURCE.sha256" 2>/dev/null)" = "$stamp" ]; then
		echo "$host"
		exit 0
	fi
else
	command -v sha256sum >/dev/null 2>&1 || { echo 'host_sqlite: sha256sum is required on Linux' >&2; exit 77; }
	stamp=$(cd -- "$source" && sha256sum sqlite3.c shell.c)
	if [ -f "$host/libsqlite3.so.0" ] && [ -f "$host/single/libsqlite3.so.0" ] && [ -x "$host/sqlite3" ] &&
		[ "$(cat -- "$host/SOURCE.sha256" 2>/dev/null)" = "$stamp" ]; then
		echo "$host"
		exit 0
	fi
fi
. "$(dirname -- "$0")/../../../sdlc/scripts/scratch.sh"
scratch_dir building
if [ "$(uname -s)" = Darwin ]; then
	cc -O2 -std=c11 -Wall -Wextra -Werror -I"$source" -c "$(dirname -- "$0")/load_probe.c" -o "$building/load_probe.o"
	cc -O2 -o "$building/load-probe-3500000" "$building/load_probe.o" "$source/sqlite3.c"
	cc -O2 -o "$building/load-probe-3490000" "$building/load_probe.o" "$old/sqlite3.c"
else
	cc -O2 -fPIC -shared -o "$building/libsqlite3.so.0" "$source/sqlite3.c" -lpthread -ldl -lm
	mkdir -p -- "$building/single"
	cc -O2 -fPIC -shared -DSQLITE_THREADSAFE=0 -o "$building/single/libsqlite3.so.0" "$source/sqlite3.c" -ldl -lm
fi
cc -O2 -I"$source" -o "$building/sqlite3" "$source/shell.c" "$source/sqlite3.c" -lpthread -ldl -lm
printf '%s\n' "$stamp" >"$building/SOURCE.sha256"
mkdir -p -- "$host"
if [ "$(uname -s)" = Darwin ]; then
	mv -f -- "$building/load-probe-3500000" "$building/load-probe-3490000" "$host/"
else
	mv -f -- "$building/libsqlite3.so.0" "$host/"
	mkdir -p -- "$host/single"
	mv -f -- "$building/single/libsqlite3.so.0" "$host/single/"
fi
mv -f -- "$building/sqlite3" "$building/SOURCE.sha256" "$host/"
echo "$host"
