#!/bin/sh
set -eu
mkdir -p /work/home /work/build /work/package/lib /work/package/share/extension
cp /src/extensions/postgres/*.c /src/extensions/postgres/*.h /src/extensions/postgres/Makefile \
    /src/extensions/postgres/*.sql /src/extensions/postgres/*.control /work/build/
cd /work/build
make PG_CPPFLAGS='-I/src/crates/thinkthen-ffi/include -I/jansson/include' \
    THINKTHEN_ARCHIVE=/native/libthinkthen_ffi.a THINKTHEN_JANSSON=/jansson
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
cp thinkthen.control thinkthen--0.2.0.sql /work/package/share/extension/
cp /src/extensions/postgres/README.md /src/LICENSE /work/package/
tar -czf /work/thinkthen-postgres-18-linux-x86_64.tar.gz -C /work/package .
printf '%s\n' 'PostgreSQL 18 Linux package built'
