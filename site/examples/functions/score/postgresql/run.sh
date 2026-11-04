#!/bin/sh
# Run the shown SQL against the same-source staged database extension.
set -eu
exec bash ../../../../../site/examples/db-duckdb-ids/run.sh --database postgres "$PWD/show.sql" ../../../../..
