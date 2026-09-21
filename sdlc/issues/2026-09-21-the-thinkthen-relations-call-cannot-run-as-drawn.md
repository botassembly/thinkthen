# The thinkthen_relations call cannot run as drawn on any database

Status: Open. Filed on the `surfaces` branch per ruling 9.

The marketing page `repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/recognize-surfaces.md`
draws the beta relations call for all three databases as

```sql
SELECT * FROM thinkthen_relations(body, '@names.json');
```

No database can run that line as drawn, and each engine needs a different
shape. The three lanes recorded each one:

- **DuckDB.** `body` resolves against no FROM, and the stable C API's table
  functions take literal parameters only, so the correlated form also
  fails (`does not support lateral join column parameters`). The working
  call is the scalar list shape, `unnest` in the SELECT list:
  `SELECT t.id, unnest(thinkthen_relations(t.body, '@names.json')) AS r FROM tickets t;`
  Evidence: `databases/duckdb/NOTES.md` finding 2 and its working-shape
  section; commits `ab360f4`/`4aa35b2`.
- **PostgreSQL.** `body` is an unresolved identifier in the drawn line.
  The working form passes the text: `FROM thinkthen_relations('...',
  '@names.json')`, which the check runs; a per-row body rides a join
  beside the table, like the recognize call. Evidence:
  `databases/postgresql/NOTES.md` and `check.sh`.
- **SQLite.** The beta companion is unbuilt on this surface; no
  `thinkthen_relations` call shape exists there yet, and the lane invented
  none. The working relate shape is the table call
  `SELECT * FROM thinkthen_relate('alerts', 'id', 'body', 'caused_by');`.
  Evidence: `databases/sqlite/NOTES.md`.

Nothing bent: the beta function is proven in DuckDB and PostgreSQL in the
shapes above, the question-file spelling (including the `"*"` any-kind
ends) is verified there, and the one drawn line cannot carry three
argument rules. The product side redraws the page once this finding names
the shapes that run.

What Ian can overturn: the redraw, or whether the beta remains public at
all.
