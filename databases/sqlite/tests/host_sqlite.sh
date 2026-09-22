#!/usr/bin/env bash
# Print an LD_LIBRARY_PATH directory holding a SQLite at or above this
# surface's floor (3.50.0), for the Python tests. Some hosts carry an
# older stock library — Ubuntu 24.04 carries 3.45.1, which the extension
# refuses to load — so the suite runs against an amalgamation kept in
# .runtimes instead. The library is built once, then reused.
#
# Fetch a floor-or-newer amalgamation once (nothing else is downloaded):
#
#   curl -O https://sqlite.org/2025/sqlite-amalgamation-3500000.zip
#   unzip -q sqlite-amalgamation-3500000.zip -d .runtimes
#
# Then this script compiles .runtimes/sqlite-amalgamation-*/sqlite3.c into
# .runtimes/host/libsqlite3.so.0. check.sh calls it only when the host's
# own Python carries an older SQLite; a host already at the floor (macOS
# 26 carries 3.51.0) needs nothing.
set -euo pipefail
cd "$(dirname "$0")/.."

dir=.runtimes/host
if [ -f "$dir/libsqlite3.so.0" ]; then
    echo "$PWD/$dir"
    exit 0
fi

source_file=$(ls .runtimes/sqlite-amalgamation-*/sqlite3.c 2>/dev/null | head -1 || true)
if [ -z "$source_file" ]; then
    echo "no SQLite amalgamation under .runtimes; fetch one per tests/host_sqlite.sh" >&2
    exit 1
fi

mkdir -p "$dir"
cc -O2 -fPIC -shared -o "$dir/libsqlite3.so.0" "$source_file" -lpthread -ldl
echo "$PWD/$dir"
