#!/usr/bin/env bash
# The recognize and relate acceptance: the DuckDB calls the product deck's
# `recognize-surfaces.md` page draws are run verbatim in the stock v1.5.5
# CLI against the stand-in, and the two calls that cannot run as drawn are
# pinned with their exact binder errors and the working replacements. The
# deck stays the source of truth; its drawn lines are vendored at
# `tools/drawn-calls/recognize.sql` so this check runs with no private
# deck on disk. The fixture tables hold recorded texts so the replay can
# answer. The "Names become rows" join runs with the usage counters
# around it, and the 255-record refusal is proven.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
DUCKDB="$ROOT/duckdb-bin/duckdb"
EXT="$ROOT/build/release/thinkthen.duckdb_extension"

rm -rf recognize-run
mkdir recognize-run
cd recognize-run
# Everything the run prints also lands in run.log, which check.sh reads
# for the pinned lines.
exec > >(tee run.log) 2>&1

# The calls, verbatim: the vendored copy of the deck's drawn lines, one
# per line, in the deck's order.
grep -v "^--" "$ROOT/tools/drawn-calls/recognize.sql" > calls.sql
echo "the calls as drawn:"
sed 's/^/  /' calls.sql

# The fixtures: recorded texts only, so every answer is a replay. They
# ride in the init file, the CLI runs it before its input, so the calls
# keep saying `FROM tickets` and `FROM alerts` as drawn.
cat > fixtures.sql << 'EOF'
CREATE TABLE tickets(id INTEGER, body VARCHAR);
INSERT INTO tickets VALUES
 (1, 'Maria Chen joined Northwind Freight in Chicago last spring.'),
 (2, 'Amara Okafor founded Kestrel Labs in 2015.'),
 (3, 'Le café 😀 Maria Chen arrived.'),
 (4, 'Northwind Freight is based in Chicago and Denver.');
CREATE TABLE alerts(id INTEGER, body VARCHAR);
INSERT INTO alerts VALUES
 (1, 'Checkout returns 500 at the payment step.'),
 (2, 'Card charges are failing for every customer.'),
 (3, 'The nightly export ran two hours late.'),
 (4, 'The payments database ran out of disk space.');
CREATE TABLE accounts(name VARCHAR, owner VARCHAR);
INSERT INTO accounts VALUES ('Northwind Freight', 'Dana'), ('Kestrel Labs', 'Amara');
EOF
printf '{"recognize": {"relations": [{"name": "based_in", "source": "*", "target": "*"}]}}\n' > names.json

printf "LOAD '%s';\n" "$EXT" > load.sql
cat fixtures.sql >> load.sql

run() {
  echo
  echo "\$ duckdb < $1"
  ENGINE_NULL=1 "$DUCKDB" -unsigned -init load.sql < "$1" 2>&1 | tail -n +2
}

# 1. The recognize call as drawn.
head -1 calls.sql > recognize.sql
run recognize.sql

# 2. The relations call as drawn, then the pinned divergence.
sed -n '2p' calls.sql > relations.sql
run relations.sql || true

# 3. The relate call as drawn, then the pinned divergence.
sed -n '3p' calls.sql > relate.sql
run relate.sql || true

# 4. The working replacements.
cat > working.sql << 'EOF'
-- relate: the query crosses as a string, the shape PostgreSQL takes
SELECT * FROM thinkthen_relate('SELECT id, body FROM alerts', ['caused_by']);

-- relations as rows: the beta shape, a list of structs, unnest makes rows
SELECT t.id, (u.r).name AS rule, (u.r).source AS source, (u.r).target AS target,
       (u.r).probability
  FROM tickets t, unnest(thinkthen_relations(t.body, '@names.json')) AS u(r)
  WHERE t.id IN (1, 2, 4)
  ORDER BY t.id;

-- names become rows: the mentions table, then an ordinary equality join
CREATE TABLE mentions AS
  SELECT t.id AS ticket, (u.n).text AS name, (u.n).kind AS kind
  FROM tickets t, unnest(thinkthen_recognize(t.body, ['person', 'organization'])) AS u(n);
SELECT ticket, name, kind FROM mentions ORDER BY ticket, name;
SELECT 'usage before the join' AS when, metric, value FROM thinkthen_usage();
SELECT a.name, a.owner, count(*) AS mentions
  FROM mentions m JOIN accounts a ON a.name = m.name
  GROUP BY a.name, a.owner ORDER BY a.name;
SELECT 'usage after the join' AS when, metric, value FROM thinkthen_usage();

-- the 255-record refusal
SELECT count(*) FROM thinkthen_relate(
  'SELECT i AS id, ''record '' || i AS body FROM range(256) t(i)', ['caused_by']);
EOF
run working.sql || true

echo
echo "done: the first call ran as drawn; the relations and relate lines are"
echo "pinned divergences with the working replacements above (see NOTES.md)"
