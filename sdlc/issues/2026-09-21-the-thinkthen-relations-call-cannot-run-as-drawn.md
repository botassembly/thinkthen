# The thinkthen_relations call cannot run as drawn on any database

Status: Open. Filed on the `surfaces` branch per ruling 9.

The marketing page `repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/recognize-surfaces.md`
draws the beta relations call for all three databases as

```sql
SELECT * FROM thinkthen_relations(body, '@names.json');
```

No public door can run that line as drawn. The three database lanes each
found it and recorded the working shape beside the drawn one:

- **DuckDB.** The stable C API registers no table-in-out function and its
  table functions take literal parameters only, so a function cannot take
  a text column as its argument. The working form is the scalar
  `thinkthen_relations('@names.json', body)` returning a LIST of structs
  that `unnest()` turns into rows — the same shape `thinkthen_recognize`
  takes. Evidence: `databases/duckdb/NOTES.md` (the relate acceptance and
  the C-API limitation), commits `ab360f4`/`4aa35b2`.
- **PostgreSQL.** The drawn form needs the set-returning function called
  with `LATERAL`, not `SELECT * FROM` alone, because the function reads
  the body per row. Working shape: `FROM tickets t, LATERAL
  thinkthen_relations('@names.json', t.body) r`. Evidence:
  `databases/postgresql/NOTES.md`.
- **SQLite.** Not run; the lane invented nothing for the beta line and
  said so. A table-valued function taking a per-row body is the open
  question there. Evidence: `databases/sqlite/NOTES.md` ("Not run,
  honestly").

Nothing bent: the beta function exists and is proven in DuckDB and
PostgreSQL in the shapes above; the deck's single line covers three
different argument rules, and one drawn line cannot. The product side
redraws the page once this finding says what shape can run.

What Ian can overturn: the redraw, or the beta function's shape itself.
