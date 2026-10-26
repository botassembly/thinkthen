#!/bin/sh
set -eu
mkdir -p /work/home /work/build /work/package/lib /work/package/share/extension
cp /src/extensions/postgres/*.c /src/extensions/postgres/*.h /src/extensions/postgres/Makefile \
    /src/extensions/postgres/*.sql /src/extensions/postgres/*.control /work/build/
cd /work/build
python3 /src/extensions/postgres/generate.py /src/schemas/contract.json thinkthen--0.2.0.sql
route_object=
if [ -n "${THINKTHEN_SQL_TEST_BUILD:-}" ]; then
    cc -std=c11 -Wall -Wextra -Werror -fPIC -fvisibility=hidden \
        -DTHINKTHEN_TEST_BUILD -DTHINKTHEN_TEST_FIXTURE \
        -I/src/crates/thinkthen-ffi/include -I/src/conformance/native \
        -c /src/databases/duckdb/tests/routes.c -o routes.o
    route_object=routes.o
fi
make THINKTHEN_TEST_ROUTE_OBJECT="$route_object" PG_CPPFLAGS='-I/src/crates/thinkthen-ffi/include' \
    THINKTHEN_ARCHIVE=/native/libthinkthen_ffi.a
if [ -z "${LLVM_PROFILE_FILE:-}" ]; then
    strip -S thinkthen_postgres.so
fi
# Static Jansson symbols must stay local, so PostgreSQL's same-named builtins
# cannot interpose on their calls when the extension is loaded.
if nm -D thinkthen_postgres.so | awk '{print $NF}' | grep '^json_'; then
    echo 'Jansson symbols cross the PostgreSQL module boundary' >&2
    exit 1
fi
cp thinkthen_postgres.so /work/package/lib/
if [ -n "${THINKTHEN_SQL_TEST_BUILD:-}" ]; then
    cc -std=c11 -Wall -Wextra -Werror -fPIC -c /src/extensions/postgres/tests/abi-mismatch.c -o abi-mismatch.o
    rm thinkthen_postgres.so
    make THINKTHEN_TEST_ROUTE_OBJECT="routes.o abi-mismatch.o" PG_CPPFLAGS='-I/src/crates/thinkthen-ffi/include' \
        THINKTHEN_ARCHIVE=/native/libthinkthen_ffi.a THINKTHEN_PG_CFLAGS="$THINKTHEN_PG_CFLAGS -Wl,--wrap=thinkthen_abi_version"
    mv thinkthen_postgres.so thinkthen_postgres_mismatch.so
fi
python3 /src/scripts/package_inventory.py stage postgres /work/package /work/build
tar -czf /work/thinkthen-postgres-18-linux-x86_64.tar.gz -C /work/package .
printf '%s\n' 'PostgreSQL 18 Linux package built'
