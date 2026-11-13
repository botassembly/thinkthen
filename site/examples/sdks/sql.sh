#!/bin/bash
# Runs a database page's SQL in that database's own shell, with no output
# flags, so the page shows what the reader's shell prints.
#
# From an example folder:  bash ../../sql.sh DATABASE [SETUP] SHOWN
#   DATABASE  duckdb, sqlite or postgresql
#   SETUP     the page's first example; its first paragraph (up to its
#             first blank line) runs first and prints nothing, as the
#             reader ran it earlier in the same session
#   SHOWN     the SQL the page shows
set -euo pipefail
database=$1
shift
setup=
if [ $# -eq 2 ]; then
    setup=$(realpath "$1")
    shift
fi
shown=$(realpath "$1")
root=$(cd "$(dirname "$0")/../../.." && pwd)
packages=$(realpath "${SITE_PACKAGES:-$root/target/site-packages}")

first_paragraph() { awk 'NF == 0 { exit } { print }' "$1"; }
session() {
    if [ -n "$setup" ]; then first_paragraph "$setup"; fi
    cat "$shown"
}

# PostgreSQL runs as nobody under root, so its folder must sit outside root's private home.
parent=${TMPDIR:-/tmp}
if [ "$database" = postgresql ] && [ "$(id -u)" = 0 ]; then parent=/tmp; fi
scratch=$(mktemp -d "$parent/thinkthen-site-sql-XXXXXX")
cleanup() { rm -rf "$scratch"; }
trap cleanup EXIT

case $database in
duckdb)
    # Hardened DuckDB builds refuse a relative LOAD, so the page shows an
    # absolute-path placeholder; point it at the staged extension.
    cp "$packages/duckdb/thinkthen.duckdb_extension" "$scratch/"
    cd "$scratch"
    session | sed "s|/absolute/path/to/thinkthen.duckdb_extension|$scratch/thinkthen.duckdb_extension|" |
        "$packages/bin/duckdb" -unsigned -bail
    ;;
sqlite)
    cp "$packages/sqlite/thinkthen_sqlite.so" "$scratch/"
    cd "$scratch"
    session | "$packages/bin/sqlite3" -bail
    ;;
postgresql)
    pg=$(cat "$packages/postgres/bindir")
    as_user=()
    if [ "$(id -u)" = 0 ]; then
        chown nobody "$scratch"
        as_user=(runuser -u nobody --)
    fi
    "${as_user[@]}" "$pg/initdb" -D "$scratch/data" -U postgres --auth=trust \
        --no-locale -E UTF8 >/dev/null
    options="-k $scratch -c listen_addresses=''"
    options+=" -c extension_control_path='$packages/postgres/share:\$system'"
    options+=" -c dynamic_library_path='$packages/postgres/lib:\$libdir'"
    options+=" -c thinkthen.allowed_backends='${THINKTHEN_BACKEND:-systemone}:*'"
    "${as_user[@]}" "$pg/pg_ctl" -D "$scratch/data" -l "$scratch/log" -w -s \
        -o "$options" start >/dev/null
    stop() {
        "${as_user[@]}" "$pg/pg_ctl" -D "$scratch/data" -m immediate -w -s stop \
            >/dev/null || true
        cleanup
    }
    trap stop EXIT
    export PGHOST=$scratch PGUSER=postgres PGDATABASE=postgres
    psql=("$pg/psql" -X -v ON_ERROR_STOP=1)
    # The setup ran earlier in the reader's session; its command tags are not shown again.
    if [ -n "$setup" ]; then first_paragraph "$setup" | "${psql[@]}" -q >/dev/null; fi
    "${psql[@]}" <"$shown"
    ;;
*)
    echo "sql.sh: unknown database $database" >&2
    exit 2
    ;;
esac
