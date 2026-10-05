set -euo pipefail
duckdb -csv -header :memory: < files/queries.sql
