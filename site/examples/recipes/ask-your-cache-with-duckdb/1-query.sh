set -euo pipefail
# Run from this example folder with the stock DuckDB CLI on PATH.
duckdb -csv -header :memory: < files/queries.sql
