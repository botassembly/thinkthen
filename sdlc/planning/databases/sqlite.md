# The SQLite extension: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to SQLite. Nobody has shipped a judgment extension for SQLite.

## As built (ticket 0109, 2026-09-25)

The port onto the public Rust API settles several points below. Where this section and a later one differ, this section holds. `databases/sqlite/README.md` is the user's page, and `databases/sqlite/NOTES.md` holds the builder's findings.

- The floor is SQLite 3.50.0, not 3.41. Below 3.50.0 a CHECK constraint in an untrusted database reaches a volatile function. The load refuses below the floor and names the host.
- Every function and both table-valued functions are direct-only and volatile. No schema object in a database file can call them.
- Two table-valued functions ship: `thinkthen_recognize(text, kinds)` and `thinkthen_relate(table, id, name, kind, rule, …)`. Relate reads rows from a named table (ADR 0047 item 9). It is the one function that opens the database, through a nested read-only SELECT.
- Every call that can send runs on its own worker thread. The calling thread polls `sqlite3_is_interrupted` every 50 ms, so an interrupt returns at once. A detached worker is the known exception to ADR 0017's rule that no thread outlives a call.
- The engine's disk cache holds the answers. `thinkthen_usage()` answers JSON totals. Five setting functions spell the engine settings: `thinkthen_throttle`, `thinkthen_max_requests`, `thinkthen_max_requests_total`, `thinkthen_cache`, and `thinkthen_cache_bytes`. That answers open question 4.
- `thinkthen_warm` takes decide questions only and flushes every 256 rows. The question cache is the extension's own, keyed by argument text and bounded at 4,096. It replaces the auxiliary-data plan.
- The file is `libthinkthen0.so`, loaded as `.load ./thinkthen` once copied to `thinkthen.so`.

## What really good looks like

A user downloads one file, runs `.load`, and asks a question of a column.

```sql
.load ./thinkthen0

SELECT thinkthen_decide('The message asks for a refund.', 'I want my money back');

-- Many rows: the warming pass hands the engine the whole column at the full throttle,
-- and the query that follows answers from the cache.
SELECT thinkthen_warm('The message asks for a refund.', body) FROM messages;
SELECT id, body FROM messages WHERE thinkthen_decide('The message asks for a refund.', body);

-- the rows nobody is sure about, for a person to read
SELECT id FROM messages WHERE thinkthen_decide('The message asks for a refund.', body) IS NULL;

-- SQLite has no list and no struct, so these two answer JSON text
SELECT m.id, t.value FROM messages m,
     json_each(thinkthen_tag('Which topics?', m.body, '["billing","shipping"]')) t;

SELECT id, json_extract(a, '$.spam.value'), json_extract(a, '$.folder.value')
  FROM (SELECT id, thinkthen_annotate(:questions, body) AS a FROM messages);
```

## Goals

- One shared library per platform, downloaded from a GitHub release and loaded with `.load`. The entry-point rule: the file is `libthinkthen0.so` exporting `sqlite3_thinkthen_init`, the name SQLite's loader derives from the file's base (207).
- **The bulk form is an aggregate that warms the cache.** `thinkthen_warm(question, text)` takes the rows SQLite already feeds it, runs them at the throttle `thinkthen_throttle` sets, fills the engine's cache on disk, and returns the count. The default flush is 256 rows: it bounds memory, the throttle sets the speed, and it is the responsiveness lever, not the chunk bound of the interrupt story (207; the plan page's parked answer). The scalars then answer from the cache. An aggregate is the one bulk shape that reads nothing: SQLite does the scan, and the caller's own `WHERE` and `JOIN` pick the rows. The name and the meaning carry to the other two databases. A band on a warm question is never an error: the threshold does not enter the request, `warm` caches the reply, and the band applies when a query reads it (the plan page's parked answer).
- `thinkthen_score` answers the number — the specification's position from 0 to K−1 — and the nearest level's name rides in `thinkthen_details` (ADR 0017 pick 6). The JSON shapes stay with `tag` and `annotate`, which have no scalar to carry the level.
- Equal pairs of question and text inside one warm pass are asked once.
- A constant question is parsed once per statement. `sqlite3_set_auxdata` on the first argument holds the parsed value, and SQLite hands it back through `sqlite3_get_auxdata` while that argument keeps its value. `rusqlite` spells the pair `Context::set_aux` and `Context::get_aux`.
- `tag` and `annotate` answer JSON text, and the engine writes that text. SQLite 3.45 added a binary JSONB, and its documentation publishes the format to keep it stable rather than to invite other programs to write it. This extension writes none.
- The key comes from `THINKTHEN_API_KEY` and nowhere else. No function takes it and no `PRAGMA` sets it. SQLite has no secret store and no privileged setting.
- The support floor is SQLite 3.41 with visible symbols: polling the interrupt needs 3.41.0, `rusqlite`'s bundled headers stop at 3.34 and force a direct link, and the macOS system SQLite is the holdout (207). `rusqlite` stays: the alternative cannot register an aggregate at all.
- The answers outlive the process. A shell user quits, runs the same query again, and pays nothing.
- The functions are volatile. `SQLITE_DETERMINISTIC` stays off, and the cache stops the second bill.

## Anti-goals

- No function takes a table name, a column name, or a query as text, and none opens the database.
- No background thread, no write to any table, no temporary table of answers, no second cache.
- No virtual table in the first release unless the experiment shows the aggregate cannot warm.
- No pointer handed between functions. `sqlite3_value_pointer` exists, and its rule is that a pointer flows straight from one function into the next with no operator in between. A batch handle would not survive a real query.

## Where this database wastes time

- **One row at a time.** SQLite has no chunk, so `WHERE thinkthen_decide(...)` makes one request per row and waits for each. `sqlite-rembed` admits the same of itself, and 1,000 rows there make 1,000 requests. Fix: warm first, and show the warming pass before the query on every page.
- **A table-valued function reads ahead over nothing.** A virtual table can do all its work in `xFilter` and let `xNext` walk a materialized answer, and `sqlite-vec` does exactly that. A table-valued function can also take a column of a preceding table through an ordinary join. That join behaves as a lateral join. Neither helps. `xFilter` sees one outer row's text and no other row's, so a cursor has nothing to read ahead over. The aggregate is the only shape that sees the whole column.
- **The same call twice.** SQLite evaluates the function in `WHERE` and again in `SELECT`, and no SQLite page documents an optimization that folds the two. Fix: the cache answers the second call.
- **A question parsed per row.** Without auxiliary data the shim parses the same constant question once for every row. SQLite describes the retention as conditional rather than guaranteed, so the shim parses again whenever `get_aux` comes back empty.
- **A short process.** The pool dies with the shell. Fix: the cache on disk carries across runs.
- **JSON built twice.** Fix: the engine emits the text for `tag` and `annotate`, and the shim copies it once. `sqlite3_result_subtype` marks that text as valid JSON so a consumer skips a second validation. Measured (207): the flag exists since 3.45, costs nothing, `json_each` ignores it, and its value is third-party consumers only — keep it. A subtype does not survive a subquery or a view.
- **A question parsed per row.** The parsed-question door is the engine's: a question value built from parts crosses whole (ADR 0017 pick 2 and section 8), and the shim's auxiliary data holds it for the statement's life.
- **A blocked connection.** `sqlite3_interrupt` aborts at the next step of the bytecode and never inside a function, and the progress handler does not run there either. Fix: the wait polls `sqlite3_is_interrupted`, added in SQLite 3.41.0 on 2023-02-21. Measured (207, round two): a warm stops one record deep through the engine's token, and a scalar waits out its in-flight request, which is the exact cancel promise of ADR 0017 section 2 — no new request starts, sent requests finish. The chunk bound stays the memory lever, not the responsiveness lever. Older builds get a timeout and nothing better.

## How little code

`rusqlite` has a maintained `loadable_extension` feature since 0.30.0, and its own example registers a scalar function. Its `functions` feature carries `create_aggregate_function`, and a maintainer discussion says a loadable extension needs neither `modern_sqlite` nor `vtab` to use it. `sqlite-loadable-rs` carries table functions, and its README calls itself beta, very unstable, and unsafe. Its newest published version is an alpha from 2023. Start with `rusqlite` and let the experiment prove it can carry an aggregate.

The shim registers the functions, converts text and numbers, holds the parsed question in auxiliary data, maps a failure to a SQL error, and writes the entry point. No threshold math, no JSON building, no retry, no HTTP. The ceiling goes in `sdlc/ratchet.json` at about 240 code lines for this surface (207).

The file ships from a GitHub release first, the way `sqlite-vec` does. A pip package and an npm package carrying the same file come later. The install page opens with the check. A CPython built without `--enable-loadable-sqlite-extensions` has no `enable_load_extension` method at all, and the macOS system SQLite omits extension loading.

## Tests only this surface needs

- A warmed query makes zero requests, counted on the stub. The unwarmed query makes one per row.
- The warming aggregate sees every row of the column and no row twice.
- A thousand rows holding one hundred distinct texts leave one hundred requests.
- A constant question is parsed once over a thousand rows, counted by the shim.
- A query cancelled from another thread stops during the wait and returns an interrupt error.
- `json_each` over `thinkthen_tag` and `json_extract` over `thinkthen_annotate` give the conformance answers.
- A failure is a SQL error, and no row comes back as a no.

## Open questions for the ADR

1. Does the aggregate cover enough, or does a `LIMIT` query need a bulk form that stops early? `pg-jev` reads ahead over a table, and the anti-goals forbid that here.
2. Answered by the plan page's parked answer and 207's measurements: the aggregate flushes every 256 rows by default. It bounds memory, and the throttle sets the speed.
3. Can a question be checked at plan time? SQLite gives a scalar function no plan-time hook. The first-row parse fails before any request at zero cost, as in DuckDB.
4. Answered by ticket 0109: the throttle comes from the setting function `thinkthen_throttle`.
5. Is `thinkthen_usage()` a virtual table, or JSON text from a scalar that skips the module machinery? The experiments shipped a `thinkthen_reset()` as instrumentation; the product shape stays open.
6. Answered by 207: 3.41.0 is the floor, for the interrupt poll and the visible symbols; the direct link sidesteps `rusqlite`'s 3.34 headers.
7. Does the aggregate stay the only bulk form here? **The recommendation is yes, and the other two databases adopt the same name.** SQLite has no array type, so the array overloads PostgreSQL offers cannot be spelled here, and the aggregate is the only bulk shape all three databases share. ADR 0017 pick 10 ships `thinkthen_warm` on all three. Ian can overturn this and leave each database its own best form, at the price of a comparison that does not line up.
