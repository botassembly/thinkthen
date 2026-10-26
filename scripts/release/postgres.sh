#!/bin/sh
set -eu
initdb -D /tmp/pg/data -A trust -U thinkthen --no-locale -E UTF8 >/dev/null
pg_ctl -D /tmp/pg/data -l /tmp/pg/server.log -o '-k /tmp/pg -c listen_addresses=' -w start >/dev/null
trap 'pg_ctl -D /tmp/pg/data -m fast -w stop >/dev/null' EXIT HUP INT TERM
export PGHOST=/tmp/pg PGUSER=thinkthen PGDATABASE=postgres
psql -X -v ON_ERROR_STOP=1 -c 'CREATE SCHEMA thinkthen' -c 'CREATE EXTENSION thinkthen SCHEMA thinkthen' -c "ALTER DATABASE postgres SET thinkthen.allowed_backends = 'stub:*'" >/dev/null
python3 /candidate/checks/sql.py postgres "$(command -v psql)" unused /candidate/checks/stub.json
