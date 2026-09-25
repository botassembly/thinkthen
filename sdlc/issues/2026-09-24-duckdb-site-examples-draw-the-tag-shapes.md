# The DuckDB site examples draw the tag's recognize and relate shapes

Status: Open

Filed by ticket 0110 on 2026-09-25. Severity: low.

## What was observed

`site/scripts/pull-examples.mjs` copies every `site/src/data/examples/*__duckdb.json` from a private deck, so a hand edit here would vanish on the next pull. Two of those examples draw shapes the port changed:

- `recognize__duckdb.json` calls `thinkthen_relations((SELECT body FROM tickets), '@names.json')` as a table function and reads rows `(name, source id, target id)`. Ticket 0110 decision 11 makes `thinkthen_relations(body, spec)` a scalar that returns a `LIST` of `(relation, source, source_kind, target, target_kind, probability)`. `unnest()` makes rows. `thinkthen_recognize` now returns `(name, kind, start, end, strength)`.
- `relate__duckdb.json` draws the tag's `thinkthen_relate(query, rules)`. Ticket 0118 owns relate on the caller's database and states its shape.

## The new shapes

```sql
SELECT id, unnest(thinkthen_recognize(body, ['person', 'organization', 'place'])) AS name FROM tickets;
SELECT id, unnest(thinkthen_relations(body, '@names.json')) AS relation FROM tickets;
```

## What to do

The deck's owner redraws both examples in the deck, and the next pull brings them here. `databases/duckdb/tools/site_examples.py` runs every drawn DuckDB example and prints one divergence line for each of these two pages that cites this issue. Its divergence list is closed. When the redrawn examples arrive, the builder removes their lines from that list, and the check then runs them like the rest.
