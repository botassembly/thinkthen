#!/usr/bin/env bash
# The DuckDB surface's slide acceptance: the sample from the marketing
# repository runs exactly as drawn, in the stock v1.5.5 CLI, against the
# stand-in. The slide text is extracted verbatim from surfaces.md into
# slide.sql; nothing is retyped here. The LOAD rides in the init file the
# CLI runs before its input, so the slide file stays as drawn. The
# fixture parquet is built with the same CLI, so no tool beyond the
# folder is needed.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)

rm -rf slide-run
mkdir slide-run
cd slide-run

"$ROOT/duckdb-bin/duckdb" -c "
CREATE TABLE tickets(id INTEGER, body VARCHAR);
INSERT INTO tickets VALUES
 (1, 'I was charged twice and I want a refund today.'),
 (2, 'Please refund my shipping label fee.'),
 (3, 'Thanks, the fix worked.'),
 (4, 'Maybe look at my billing question later.');
COPY tickets TO 'tickets.parquet' (FORMAT parquet);
"

printf "LOAD '%s';\n" "$ROOT/build/release/thinkthen.duckdb_extension" > load.sql

"$ROOT/duckdb-bin/duckdb" -unsigned -init load.sql < ../slide.sql
