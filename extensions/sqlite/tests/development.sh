#!/bin/sh
# Build only the SQLite adapter against an explicitly selected engine archive.
set -eu
. scripts/native-build-env
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
python3 extensions/rules.py "$scratch/rules.h"
sqlite_flags=${THINKTHEN_SQLITE_CFLAGS:-}
sqlite="$THINKTHEN_SQLITE_ROOT/sqlite-3.41.0"
jansson="$THINKTHEN_JANSSON_ROOT"
for source in extension rows source question replay; do
    cc -std=c11 -Wall -Wextra -Werror -O2 -fPIC -fvisibility=hidden $sqlite_flags \
        -I "$scratch" -I "$sqlite" -I "$jansson/include" -I crates/thinkthen-ffi/include \
        -c "extensions/sqlite/src/$source.c" -o "$scratch/$source.o"
done
cc -std=c11 -Wall -Wextra -Werror -O2 -fPIC -fvisibility=hidden $sqlite_flags \
    -DTHINKTHEN_TEST_BUILD -DTHINKTHEN_TEST_FIXTURE -I crates/thinkthen-ffi/include \
    -I conformance/native -c databases/duckdb/tests/routes.c -o "$scratch/routes.o"
case $(uname -s) in
    Darwin) link="-dynamiclib $(scripts/native-frameworks) -liconv"; suffix=dylib ;;
    Linux) link=-shared; suffix=so ;;
esac
cc $link "$scratch"/*.o "$THINKTHEN_SQL_TEST_ARCHIVE" "$jansson/lib/libjansson.a" \
    -lpthread -ldl -lm -o "$scratch/thinkthen_sqlite.$suffix"
cc -std=c11 -Wall -Wextra -Werror -pthread -I "$sqlite" -I "$jansson/include" \
    extensions/sqlite/tests/consumer.c "$sqlite/libsqlite3.a" "$jansson/lib/libjansson.a" \
    -ldl -lm -o "$scratch/consumer"
if [ "${1:-}" = --unknown-only ]; then
    python3 databases/unknown-code-check.py sqlite "$scratch/consumer" "$scratch/thinkthen_sqlite.$suffix"
else
    python3 extensions/sqlite/tests/run.py "$scratch/consumer" "$scratch/thinkthen_sqlite.$suffix" "$THINKTHEN_LOOPBACK" "$PWD"
fi
