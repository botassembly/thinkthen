Status: Open. Filed 2026-09-28 by the marketing lead's UX review, on Ian's instruction. Ian wants the SQL and data frame interfaces' usability settled and improved before 0.1. The main builder designs and builds from this issue. It authorizes no work by itself.

# SQL and data frame interface usability before 0.1

## Summary

A user who writes ThinkThen in SQL today meets three different extensions. Each takes the question in its own form, puts different things in the third argument, returns probabilities, lists and usage in its own shape, and batches rows in its own way. A query written for one database fails on the other two. PostgreSQL refuses a plain-text question outright. SQLite has no probability function. The only way to set a threshold in SQL is hand-built JSON. The site's SQL samples are not run by any check, and one of them calls a function its database does not have.

The data frame bindings share some of these problems. pandas and Polars users call module functions and unwrap `.value`; neither library gets an accessor or an expression it can chain. Python's `decide` takes no `threshold`, while R's does. Python spells the outcome descriptions `true_` and `false_`, and R spells them `true` and `false`. No binding returns a probability per row without `details` or `rank`.

MotherDuck's `prompt_jev()` announcement (https://motherduck.com/blog/motherduck-supports-jev/, 2026-09-21) shows what users will compare us with:

```sql
SELECT conversation_id,
       prompt_jev(transcript, 'Identify the customer''s main complaint',
         choice := [{label: 'billing', description: 'Payments, invoices, and refunds'}, ...]) AS classification
FROM customer_conversations;
```

One scalar call per row takes the text, a sentence of instruction and named options. It returns a typed value that a query can filter, join and aggregate. The database batches the rows. The user writes no JSON.

Good looks like this for ThinkThen:

- The question is plain text. Options, levels, labels, threshold and outcome descriptions go in named arguments with the same names in SQL, pandas, Polars and R. The JSON question and question files stay as escape hatches.
- Each verb has one name and one argument order in all three databases. A query that stays inside the shared subset runs unchanged on SQLite, DuckDB and PostgreSQL.
- Each data frame library gets its own idiom: a pandas Series accessor, a Polars expression namespace, and plain vectors in R for `dplyr::mutate`.
- `decide` returns a boolean with null for not sure. A companion `probability` returns the number without `details` and a JSON path.
- A call over a column or over rows batches under the hood, or the docs name the one batched form a surface needs.
- One page shows the same examples in all three databases and all three data frame libraries, and a check replays each one.

## Survey of main

Read on main at `8033eeac`. Every claim below cites the file and line.

### SQL functions by verb

| Verb | SQLite | DuckDB | PostgreSQL |
| --- | --- | --- | --- |
| decide | `thinkthen_decide(question, text[, deadline[, context]])` returns 1, 0 or NULL (`databases/sqlite/README.md:20`; registered at `databases/sqlite/src/scalars.rs:359-360`) | `thinkthen_decide(question, text[, deadline[, context]])` returns `BOOLEAN` (`databases/duckdb/README.md:9`; `databases/duckdb/cpp/src/thinkthen.cpp:408-421`) | `thinkthen_decide(question, text[, context])` returns `boolean` (`databases/postgresql/README.md:9`; `databases/postgresql/src/lib.rs:132-139`; `databases/postgresql/src/context.rs:13-18`) |
| decide, many texts | none | none | `thinkthen_decide(question, texts text[][, context])` returns rows `(i, decided)` (`databases/postgresql/README.md:10`; `databases/postgresql/src/lib.rs:391-397`) |
| probability | none (`databases/sqlite/README.md:18-32`) | `thinkthen_probability(question, text[, deadline[, context]])` returns `DOUBLE` (`databases/duckdb/README.md:10`) | `thinkthen_probability(question, text[, context])` returns `float8` (`databases/postgresql/src/lib.rs:141-150`) |
| choose | `thinkthen_choose(question, text[, deadline[, context]])`; options live inside a JSON question (`databases/sqlite/README.md:21`) | `thinkthen_choose(question, text, options[, deadline[, context]])`, or `thinkthen_choose(question, text)` for a complete question (`databases/duckdb/cpp/src/thinkthen.cpp:423-440`) | `thinkthen_choose(question, text, options text[][, context])`; the array is required, and `NULL` means the question names them (`databases/postgresql/src/lib.rs:152-163`; `databases/postgresql/README.md:27`) |
| score | same shape as choose, returns REAL (`databases/sqlite/README.md:22`) | same shape as choose, returns `DOUBLE` (`databases/duckdb/README.md:13`) | same shape as choose, returns `float8` (`databases/postgresql/src/lib.rs:165-176`) |
| tag | returns a JSON array as text (`databases/sqlite/README.md:23`) | returns `VARCHAR[]` (`databases/duckdb/README.md:14`) | returns `text[]` (`databases/postgresql/src/lib.rs:178-189`) |
| find | `thinkthen_find(question, units_json[, none[, deadline]])` returns JSON text (`databases/sqlite/README.md:28`) | `thinkthen_find(question, units[, none[, deadline]])` returns a STRUCT (`databases/duckdb/README.md:12`) | `thinkthen_find(question, units text[][, none])` returns `jsonb`, with no deadline argument (`databases/postgresql/README.md:15`, `:29`) |
| annotate | returns JSON text (`databases/sqlite/README.md:26`) | returns `VARCHAR` JSON (`databases/duckdb/README.md:15`) | returns `jsonb` (`databases/postgresql/README.md:16`) |
| details, try_details | JSON text (`databases/sqlite/README.md:24-25`) | JSON text (`databases/duckdb/README.md:16-17`) | `jsonb` (`databases/postgresql/README.md:17-18`) |
| warm | aggregate returning a count (`databases/sqlite/README.md:27`) | aggregate returning a count (`databases/duckdb/README.md:21`) | aggregate returning a count (`databases/postgresql/README.md:19`) |
| recognize | table-valued function; kinds as a comma list, JSON section or `'@name'` (`databases/sqlite/README.md:30`, `:44`; `databases/sqlite/src/ffi.rs:129`) | scalar returning a LIST of structs; kinds as `VARCHAR[]`; the caller must `unnest` (`databases/duckdb/README.md:18`; `databases/duckdb/cpp/src/nested.cpp:165-169`) | set-returning function; kinds as `text[]` or a spec (`databases/postgresql/README.md:20`; `databases/postgresql/src/lib.rs:295-330`) |
| relations from one text | none; `thinkthen_recognize_document(text, spec)` returns JSON instead (`databases/sqlite/README.md:31`, `:46`) | scalar `thinkthen_relations(text, file[, deadline])` returning a LIST (`databases/duckdb/cpp/src/nested.cpp:170-175`) | set-returning `thinkthen_relations(text, spec)` (`databases/postgresql/src/lib.rs:350-356`) |
| relate across rows | table function `thinkthen_relate(table, id_column, name_column, kind_column, rule, ...[, deadline])` with up to four rule slots (`databases/sqlite/README.md:32`, `:48`; `databases/sqlite/src/tables.rs:344`) | table function `thinkthen_relate(query, rules)` (`databases/duckdb/README.md:20`; `databases/duckdb/cpp/src/relate.cpp:252`) | set-returning `thinkthen_relate(query, rules text[])` or `(query, spec)` (`databases/postgresql/src/relate.rs:173-215`) |
| usage | scalar returning a JSON object (`databases/sqlite/README.md:29`) | table function returning rows `(metric, value)` (`databases/duckdb/README.md:22`; `databases/duckdb/cpp/src/usage.cpp:49-52`) | set-returning function with one row of four columns (`databases/postgresql/src/lib.rs:240-253`) |

### How each input reaches a SQL call

| Input | SQLite | DuckDB | PostgreSQL |
| --- | --- | --- | --- |
| Question text | plain text is a decide question; JSON starting with `{`; `'@name'` (`databases/sqlite/README.md:34`) | plain text, JSON or `'@path.json'` (`databases/duckdb/README.md:26`) | JSON or `'@file'` only. Plain text raises `usage` with "bare text is never a path" (`databases/postgresql/src/files.rs:23-33`; `databases/postgresql/README.md:27`) |
| Options, levels, labels | inside the JSON question only | a `VARCHAR[]` third argument, or inside a complete JSON question (`databases/duckdb/cpp/src/thinkthen.cpp:423-440`) | a `text[]` third argument joined into the JSON question, or `NULL` (`databases/postgresql/src/lib.rs:36-43`) |
| Descriptions of options | JSON object in the question | JSON question only; the list form takes bare labels | JSON question only |
| Threshold, `true`, `false` | JSON question only | JSON question only | JSON question only |
| Deadline | third argument in milliseconds (`databases/sqlite/README.md:40`), and `thinkthen_budget_ms(n)` per connection (`databases/sqlite/src/budget.rs:58`) | an argument after the members, in milliseconds (`databases/duckdb/README.md:24`), and `SET thinkthen_query_budget_ms` (`databases/duckdb/cpp/src/thinkthen.cpp:393`) | no argument; `SET thinkthen.deadline_ms` only (`databases/postgresql/README.md:74`) |
| Context | fourth argument, after a deadline of `-1` (`databases/sqlite/README.md:42`) | last argument, after a deadline (`databases/duckdb/README.md:24`) | third argument (`databases/postgresql/README.md:25`; `databases/postgresql/src/context.rs:13-18`) |
| Backend address and key | environment only (`databases/sqlite/README.md:83`) | environment only (`databases/duckdb/README.md:66`) | environment only (`databases/postgresql/README.md:138-139`) |
| Model | `SELECT thinkthen_model('name')` before the engine builds; later calls raise `usage` (`databases/sqlite/README.md:85`, `:95`) | `SET thinkthen_model = 'name'` per session (`databases/duckdb/README.md:74`) | `SET thinkthen.model = 'name'` per role or session (`databases/postgresql/README.md:80`) |
| Batch, cache, throttle, totals | twelve setting functions, process-wide, before the first send (`databases/sqlite/README.md:85-99`) | `SET thinkthen_*`, per session, up to 16 engines (`databases/duckdb/README.md:66-80`) | `SET thinkthen.*`, some superuser-only (`databases/postgresql/README.md:72-87`) |

### Data frame bindings

| Topic | pandas (Python package) | Polars | R |
| --- | --- | --- | --- |
| Call style | module function over a Series: `tt.decide(question, df["body"]).value` (`libraries/python/README.md:56-59`) | the same module function over a Python Polars Series (`libraries/python/README.md:47`); Rust has the `PolarsEngine` trait with `decide_series` and siblings (`libraries/polars/README.md:9`, `:21-29`) | `tt_decide(question, evidence, ...)` over a vector, used inside `dplyr` (`libraries/r/README.md:3`, `:9`) |
| Accessor or expression | none. The package never imports pandas (`libraries/python/README.md:62`) | none. No expression namespace, so a call cannot sit inside `with_columns` or a lazy frame | not applicable; R vectors work in `mutate` |
| Return wrapper | `Call` with `.value` and `.facts` (`libraries/python/README.md:8-10`) | `Call` in Python; `Call<Series>` in Rust (`libraries/polars/README.md:19`) | `thinkthen_call` list with `$value` (`libraries/r/README.md:3`) |
| decide type | `boolean`, `pd.NA` for not sure (`libraries/python/README.md:56`) | nullable `Boolean` (`libraries/polars/README.md:25`) | logical, `NA` for not sure (`libraries/r/README.md:3`) |
| Options, levels, labels | keywords `options=`, `levels=`, `labels=` (`libraries/python/thinkthen/__init__.pyi:213`) | same keywords | arguments `options`, `levels`, `labels` (`libraries/r/thinkthen/R/thinkthen.R:341`, `:352`, `:362`) |
| Threshold | not on `decide`; only through `tt.question(decide=..., threshold=...)` (`libraries/python/thinkthen/__init__.pyi:157-172`, `:211`) | same as pandas | `threshold =` on `tt_decide`, `tt_choose` and `tt_filter` (`libraries/r/thinkthen/R/thinkthen.R:312`, `:341`, `:372`) |
| `true` and `false` descriptions | `true_=` and `false_=` on `tt.question` (`libraries/python/thinkthen/__init__.pyi:167-168`). Neither word is a Python keyword. | same | `true =` and `false =` on `tt_question` (`libraries/r/thinkthen/R/thinkthen.R:248-251`) |
| Probability per row | none; `rank` reads lists only and reorders them (`libraries/python/README.md:64`), and `details` takes one text (`libraries/python/thinkthen/__init__.pyi:219`) | none in Python or Rust (`libraries/polars/README.md:23-29`) | none; `tt_rank` returns a reordered frame (`libraries/r/thinkthen/R/thinkthen.R:384-397`), and `tt_details` takes one text |
| Batching over a column | one engine call per column; pandas 2 crosses at list speed (`libraries/python/README.md:47`, `:62`) | one engine call per column (`libraries/polars/README.md:9`) | one native call per column for decide; choose, score and tag with structured question text send one record per request (`libraries/r/README.md:13`) |
| Question file | `tt.question(file=...)` (`libraries/python/README.md:80`) | same | `tt_question(file = path)` (`libraries/r/README.md:14`) |
| Deadline | seconds (`libraries/python/README.md:70`) | seconds | seconds (`libraries/r/thinkthen/src/rust/src/ffi.rs:149`) |
| Errors | exception classes by kind (`libraries/python/thinkthen/__init__.pyi:86-99`) | one failed row fails the whole Series call (`libraries/polars/README.md:33`) | R conditions by kind with `retryable` (`libraries/r/README.md:3`) |
| Settings | `tt.Engine(model=..., batch=..., cache=...)` or the environment (`libraries/python/README.md:66`) | same in Python; the caller's own `Engine` in Rust (`libraries/polars/README.md:11`) | `tt_engine(...)` once per session (`libraries/r/README.md:17`) |
| Usage | `tt.usage()` returns a dict (`libraries/python/README.md:82`) | same | `tt_usage()` (`libraries/r/README.md:29`) |

### Batching today

- DuckDB packs each vector of up to 2,048 rows into shared requests (`databases/duckdb/README.md:68`, `:70`). A plain scalar query batches.
- SQLite sends one request per scalar row: "Each scalar row is its own one-record call" (`databases/sqlite/README.md:90`; `databases/sqlite/NOTES.md:51`). Only `thinkthen_warm` packs. The README's own recipe selects `thinkthen_batch(1)` so that a later scalar call can read the warm answers from cache (`databases/sqlite/README.md:7-14`, `:105`). That recipe turns packing off, so the documented fast path sends one request per row.
- PostgreSQL sends one request per scalar row. Only the array form of `thinkthen_decide` and `thinkthen_warm` pack (`databases/postgresql/README.md:25`, `:95`).
- pandas, Polars and R make one engine call per column, and that call packs.

### Errors

All three databases prefix a failure with `thinkthen <kind>:`. The retry hint differs three ways:

- SQLite: `thinkthen backend (retryable): status 503` (`databases/sqlite/src/lib.rs:148`, `:208`).
- DuckDB: `thinkthen backend: ` then ` (a second try could help)` before the message (`databases/duckdb/bridge/src/errors.rs:46-60`).
- PostgreSQL: `thinkthen backend: status 503 (retryable: yes)` (`databases/postgresql/README.md:99`).

All three databases offer `thinkthen_try_details`, so a query can keep good rows after a recoverable row failure (`specification/settings.md:15`). No surface reports cost (`databases/sqlite/README.md:75`; `databases/duckdb/README.md:56`; `databases/postgresql/README.md:62`; `libraries/python/README.md:78`).

### Samples and docs

- `site/examples/functions/recognize/sqlite.sql:16-19` calls `thinkthen_relations`. SQLite registers no such function (`databases/sqlite/src/scalars.rs:352-407`; `databases/sqlite/src/ffi.rs:129-130`).
- `databases/postgresql/README.md:52` calls `thinkthen_choose('Which team owns this?', body, ARRAY[...])`. PostgreSQL refuses that plain-text question (`databases/postgresql/src/files.rs:23-33`). The runnable PostgreSQL examples all wrap the question in JSON (`databases/postgresql/examples.json`).
- `databases/duckdb/README.md:26` says choose, score and tag "take plain text and put their members in the list". The two-argument complete-question form also exists (`databases/duckdb/cpp/src/thinkthen.cpp:427-429`, ticket 0247). The function table at `databases/duckdb/README.md:11-14` omits it.
- DuckDB registers `thinkthen_relations` and `thinkthen_recognize` as scalars (`databases/duckdb/cpp/src/nested.cpp:165-175`). A slide drew DuckDB's relations call in `FROM`, where it cannot run. Talking points claimed PostgreSQL runs the SQLite query as written. The survey above shows it does not.
- The smoke runner skips every `.sql` sample (`site/examples/SKIP:8`). Coverage by database is uneven. DuckDB has no decide, filter, choose or find sample. PostgreSQL has no decide or question-file sample. SQLite has no rank or tag sample (`site/examples/functions/*/`). DuckDB has no `examples.json` beside SQLite's and PostgreSQL's.
- The repository has no top-level `examples/` folder and no single SQL or data frame page.
- The settled type contract keeps SQL returns plain: `TEXT`, `BOOLEAN` and `DOUBLE`, with the caller owning any enum (`specification/types.md:39`, `:41`; ADR 0082).

### Differences with no good reason

1. PostgreSQL refuses plain-text questions. SQLite and DuckDB accept them, and all three mark a file with `@`. The `@` prefix already removes the path ambiguity that PostgreSQL's refusal guards against.
2. The third SQL argument means a deadline in SQLite, an options list in DuckDB's choose, and a context in PostgreSQL's decide.
3. SQLite has no `thinkthen_probability`. The deck had to read `thinkthen_details(...) ->> '$.answer.probability'` instead. No data frame binding has a probability per row.
4. SQLite's choose takes no options argument. PostgreSQL's choose requires one, even as `NULL`.
5. Usage has five shapes: a SQLite JSON scalar, DuckDB rows of `(metric, value)`, one wide PostgreSQL row, a Python dict and an R value.
6. The SQL retry hint reads three ways.
7. SQLite relate takes a table and column names. DuckDB and PostgreSQL take a query.
8. SQLite settings lock once the engine builds. DuckDB and PostgreSQL settings apply per session.
9. PostgreSQL find has no deadline argument. The other two have one.
10. Python's `decide` takes no `threshold`. R's does.
11. Python spells `true_` and `false_`. R and the question file spell `true` and `false`.
12. The text argument is `text` in SQLite and DuckDB, `evidence` in PostgreSQL and R, and `text` or `records` in Python.
13. SQL deadlines count milliseconds. Library deadlines count seconds. Both are named `deadline`.

Some differences follow from each dialect and should stay. SQLite has no BOOLEAN type, no arrays, no named arguments and no `SET` for extensions. PostgreSQL custom settings need a dotted name. DuckDB lists and PostgreSQL arrays have different literal syntax. R has no Python-style accessor, and plain vectors already fit `mutate`.

## Proposals

Each proposal gives the call a user writes today and the call this issue asks for. Each applies to SQLite, DuckDB and PostgreSQL and to pandas, Polars and R wherever the dialect or library allows. Each one says plainly where one cannot. The builder can change any spelling below during design and records why.

### Shared argument names

Every surface uses these names, in this order after the text source:

| Name | Meaning |
| --- | --- |
| `question` | plain text, a built question, or a question file |
| `text` | the text or column judged |
| `options`, `levels`, `labels` | members for choose, score and tag; a list, or a label-to-description map |
| `threshold` | one cut, or a two-cut band for decide such as `'0.3:0.7'` |
| `true`, `false` | descriptions of each decide outcome |
| `model` | the model for this call |
| `context` | shared reference text sent once per packed request |
| `batch` | records per request, or `max` |
| `deadline` | seconds in the libraries; SQL spells it `deadline_ms` so the unit shows in the name |

Python renames `true_` and `false_` to `true` and `false`. PostgreSQL and R rename `evidence` to `text` in docs and in named arguments.

### 1. Named arguments, with the JSON question kept

Today a threshold in SQL needs hand-built JSON. The deck's data slide, SQLite:

```sql
SELECT title, thinkthen_decide(json_object(
  'decide',
  'The text is the title of a song by the Beatles. '
  || 'It appears on the album Abbey Road.',
  'threshold', '0.3:0.7'
), title) AS on_abbey_road
FROM songs;
```

After, in DuckDB and PostgreSQL:

```sql
SELECT title,
       thinkthen_decide(
         'The text is the title of a song by the Beatles. '
         || 'It appears on the album Abbey Road.',
         title,
         threshold := '0.3:0.7') AS on_abbey_road
FROM songs;
```

```sql
SELECT id,
       thinkthen_choose('Which team owns this?', body,
         options := '{"billing": "Payments, invoices, and refunds",
                      "shipping": "Parcels and delivery"}') AS team
FROM tickets;
```

Python today, and after:

```python
q = tt.question(decide=abbey_road, threshold="0.3:0.7")   # before
songs["on_abbey_road"] = tt.decide(q, songs["title"]).value

songs["on_abbey_road"] = songs["title"].tt.decide(abbey_road, threshold="0.3:0.7")   # after
```

R keeps its shape and gains the same names on every verb:

```r
songs |> mutate(on_abbey_road = tt_decide(abbey_road, title, threshold = "0.3:0.7"))
```

`options`, `levels` and `labels` take a native list, array or vector of labels, or a label-to-description map. In SQL the map is JSON text. DuckDB also takes a list of `{label, description}` structs, the shape MotherDuck uses. A JSON question, a built question or a question file in the first slot keeps working. A named argument that repeats a field the question already sets raises `usage`.

Where a surface cannot:

- PostgreSQL takes `name => value` and `name := value` for functions with defaulted parameters. pgrx declares defaults. No limit.
- DuckDB takes `name := value` in table functions and macros. The builder probes whether a C++ `ScalarFunction` in v1.5.5 accepts named parameters. If it does not, the extension registers a macro per verb that forwards to the positional function.
- SQLite has no named arguments. SQLite takes the same fields as a third argument of JSON text: `thinkthen_decide('...', title, '{"threshold": "0.3:0.7"}')`. DuckDB and PostgreSQL take that JSON third argument too, so a shared form exists.
- pandas, Polars and R take keyword arguments natively. No limit.

### 2. One name, one order and one shape per verb

The shared positional form in all three databases:

```sql
thinkthen_<verb>(question TEXT, text TEXT [, settings TEXT])
```

`settings` is JSON text holding any of the shared names above. Deadline and context move out of positional slots into `settings` or named arguments, so the third slot means the same thing everywhere. PostgreSQL accepts plain text as a decide question. No outside user depends on the positional deadline and context slots yet, so 0.1 is the time to change them.

Before, three spellings of one question:

```sql
-- SQLite
SELECT thinkthen_choose('{"choose":"Which team owns this?","options":["billing","shipping"]}', body) FROM tickets;
-- DuckDB
SELECT thinkthen_choose('Which team owns this?', body, ['billing', 'shipping']) FROM tickets;
-- PostgreSQL
SELECT thinkthen_choose('Which team owns this?', body, ARRAY['billing', 'shipping']) FROM tickets; -- raises usage today
```

After, one query for all three:

```sql
SELECT id, thinkthen_choose('Which team owns this?', body,
         '{"options": ["billing", "shipping"]}') AS team
FROM tickets;
```

The same rule sets return shapes. `tag` returns the dialect's list type, and SQLite returns a JSON array. `find` returns a struct in DuckDB, `jsonb` in PostgreSQL and JSON text in SQLite, with the same member names. `usage` returns one row with the same four columns in every database and the same four keys in every library.

The data frame verbs follow the same names and order: `decide`, `probability`, `choose`, `score` and `tag`, question first.

Where a surface cannot: SQLite's decide returns 1 and 0, SQLite's `tag` returns JSON, list literals differ, and `json_object` does not port (PostgreSQL spells it `json_build_object`). String concatenation with `||`, `WHERE`, `ORDER BY` and `count(*) FILTER (WHERE ...)` work in all three. A query that stays inside those and the shared call form runs unchanged.

### 3. Typed returns and a probability without JSON paths

Keep the plain return types that ADR 0082 settled: decide boolean with null for not sure, choose text, score double, tag a list. Add the missing companion everywhere it is missing.

```sql
-- Before, SQLite
SELECT title, thinkthen_details('Is it on Abbey Road?', title) ->> '$.answer.probability' AS p FROM songs;
-- After, all three databases
SELECT title, thinkthen_probability('Is it on Abbey Road?', title) AS p FROM songs ORDER BY p DESC;
```

```python
songs["p"] = songs["title"].tt.probability("Is it on Abbey Road?")                     # pandas
songs.with_columns(p=pl.col("title").tt.probability("Is it on Abbey Road?"))           # Polars
```

```r
songs |> mutate(p = tt_probability("Is it on Abbey Road?", title)) |> arrange(desc(p))
```

`probability` takes the same arguments as its verb. For decide it returns the yes probability. For choose it returns the chosen option's probability, and null when the choice is not sure. For score it returns the probability of the nearest level. The builder confirms that `decide` and `probability` on the same rows in one query or one frame send one request per packed batch, with the second read from cache. Rust Polars gains `probability_series`.

A struct or composite return with `.value` and `.probability` conflicts with ADR 0082 item 6 and with SQLite's lack of structs. This issue recommends companion functions and no struct return for 0.1. Overturning ADR 0082 would need Ian.

The data frame idioms return the plain column, not a `Call`. The accessor, the expression and R's verbs put the call's facts where the caller can still read them: `tt.last_facts()` in Python, and a `facts` attribute on the R vector that `dplyr` drops harmlessly. The builder weighs this against `2026-09-26-every-surface-should-give-back-run-facts.md` and records the choice. The module functions `tt.decide(...)` keep returning `Call`.

Where a surface cannot: SQL has no side channel for facts beyond `thinkthen_usage()` and `thinkthen_details`, which stay.

### 4. Row sources where a verb returns rows

Recognize, relations and relate return rows. Each database registers them as row sources under the same names and columns.

Before:

```sql
-- SQLite: table-valued, and no relations function
SELECT t.id, e.text, e.kind FROM tickets t, thinkthen_recognize(t.body, 'person,organization,place') e;
-- DuckDB: a scalar list that must be unnested
SELECT id, unnest(thinkthen_recognize(body, ['person', 'organization', 'place'])) AS entity FROM tickets;
-- PostgreSQL: set-returning
SELECT t.id, e.text, e.kind FROM tickets t, LATERAL thinkthen_recognize(t.body, ARRAY['person','organization','place']) e;
```

After, in all three:

```sql
SELECT t.id, e.text, e.kind
FROM tickets t, thinkthen_recognize(t.body, '{"kinds": ["person", "organization", "place"]}') e;
```

SQLite gains `thinkthen_relations` as a table-valued function. SQLite's relate takes the same `(query, rules)` form as the other two, through its existing nested read-only SELECT. `usage` becomes a row source in SQLite too. In the data frame libraries, `recognize` over a frame already returns one row per entity in Python (`libraries/python/README.md:47-52`) and one frame per text in R (`libraries/r/README.md:15`). pandas returns a `names` column of lists (`libraries/python/README.md:56`). The page shows the explode or unnest step each library needs.

Where a surface cannot: SQLite and PostgreSQL take a column reference in a `FROM`-clause function, and PostgreSQL treats a function in `FROM` as lateral without the keyword. DuckDB takes a correlated column in a table function only through a table in-out function. The builder probes whether the v1.5.5 C++ API registers one. If it cannot, DuckDB keeps the scalar list form, and the page shows `unnest` for DuckDB as a named dialect limit.

### 5. Question files

All three databases read `'@file'` already. The libraries read `tt.question(file=...)` and `tt_question(file = ...)`, and ticket 0244 keeps a bare `@` string literal in the libraries. The gaps are consistency and docs.

A relative name resolves against the process folder in SQLite (`databases/sqlite/README.md:111`), through the caller's file system settings in DuckDB (`databases/duckdb/README.md:104`), and on the server under `pg_read_server_files` or `thinkthen.file_directory` in PostgreSQL (`databases/postgresql/README.md:137`). `find` treats `@` as literal text in DuckDB and PostgreSQL (`databases/duckdb/README.md:24`; `databases/postgresql/README.md:29`).

```sql
SELECT title, thinkthen_decide('@abbey-road.json', title) AS on_abbey_road FROM songs;
```

```python
q = tt.question(file="abbey-road.json")
songs["on_abbey_road"] = songs["title"].tt.decide(q)
```

```r
q <- tt_question(file = "abbey-road.json")
songs |> mutate(on_abbey_road = tt_decide(q, title))
```

The page states the one rule for each surface and shows that the same file drives the command, SQL, pandas, Polars and R to the same question digest. A named argument beside a file question raises `usage` on conflict, as in proposal 1. A separate load-by-name function adds a second path for the same thing, and this issue recommends none for 0.1.

Where a surface cannot: PostgreSQL reads files on the database server, not the client. The page says so.

### 6. Batching is automatic, or the docs say where it is not

A user who writes one scalar call per row expects the database to batch. DuckDB does. pandas, Polars and R do, because each call takes a whole column. SQLite and PostgreSQL call a scalar once per row with no view of the next row, so the extension cannot pack a plain scalar without look-ahead.

Options for the builder to weigh:

- Make warm pack and let a later scalar read the packed answer. This needs the cache to hold each record's answer under a key that a one-record scalar request also reaches. Today the cache key is the whole request digest, so a packed warm and a singleton scalar never share an entry (`databases/sqlite/README.md:105`). This touches the engine and the batching design (`sdlc/issues/2026-09-26-batching-design.md`).
- Give SQLite the array form PostgreSQL has: a row source that takes a question and a JSON array of texts and returns `(i, value)`. PostgreSQL extends its array form from decide to choose, score and tag.
- Keep one request per row in SQLite and PostgreSQL, and state it on the page with the throttle as the lever.

For the data frame libraries: R's choose, score and tag with structured question text send one record per request (`libraries/r/README.md:13`). They should pack like decide. A Polars expression that runs under the streaming engine may see a column in chunks. Each chunk then makes one packed call, and the page says so.

Whichever option the builder picks, the page states, per surface, how many requests a 1,000-row call sends, and a replayed test proves the count.

### 7. Clear errors, not-sure rows, counts and cost

SQL uses one error format in all three databases: `thinkthen <kind>: <message>`, with the retry hint spelled one way and placed at the end. The SQLSTATE mapping stays per database. Python keeps its exception classes and R its conditions, with the same kind names and the same message text.

Not sure stays null and never means failure. The page shows how to count it on each surface:

```sql
SELECT count(*) FILTER (WHERE on_abbey_road) AS yes,
       count(*) FILTER (WHERE NOT on_abbey_road) AS no,
       count(*) FILTER (WHERE on_abbey_road IS NULL) AS not_sure
FROM (SELECT thinkthen_decide('...', title) AS on_abbey_road FROM songs) AS judged;
SELECT * FROM thinkthen_usage();
```

```python
songs["on_abbey_road"].value_counts(dropna=False)
tt.usage()
```

```r
count(songs, on_abbey_road)
tt_usage()
```

`FILTER` works in all three databases. Usage returns `requests_sent`, `cache_answers`, `input_tokens` and `output_tokens` everywhere. Cost joins them when the run-cost issue lands (`sdlc/issues/2026-09-27-no-run-level-cost-beside-the-score.md`). A missing key, a spent quota and a backend refusal each name the setting or environment variable to fix, in the same words on every surface.

A failed row in a Polars Series call fails the whole call (`libraries/polars/README.md:33`). SQL has `thinkthen_try_details` to keep going. The builder states on the page which surfaces keep good rows after a failed one and how.

### 8. Configuration

The address and key stay in the environment, as ruled (`sdlc/issues/2026-09-26-settings-some-surfaces-cannot-reach.md:11`). Model, cache, batch, throttle and totals are settable per session:

```sql
SELECT thinkthen_model('jev-1');   -- SQLite
SET thinkthen_model = 'jev-1';     -- DuckDB
SET thinkthen.model = 'jev-1';     -- PostgreSQL
```

```python
tt.configure(model="jev-1")   # or tt.Engine(model="jev-1") passed as engine=
```

```r
tt_engine(model = "jev-1")
```

SQLite's settings should apply on the next call, as DuckDB's do, instead of raising once the engine builds. A per-call `model` named argument covers a one-off change on every surface. The pandas accessor and Polars expression take `engine=` for a caller's own engine and use the module engine otherwise. The page gives one settings table with every surface's spelling side by side.

Where a surface cannot: PostgreSQL needs a dotted custom setting name, and SQLite has no `SET` for extensions. The names therefore differ by one character across the three databases.

### 9. Naming

Keep `thinkthen_` in SQL and `tt_` in R for 0.1. Name the pandas accessor and the Polars namespace `tt`, matching `import thinkthen as tt`.

- Plain SQL verbs such as `decide(...)` and `score(...)` collide with user functions and future built-ins. `score` and `rank` are likely names in analytics schemas. PostgreSQL can install into a `thinkthen` schema and call `thinkthen.decide(...)`, but DuckDB and SQLite cannot qualify an extension function the same way, so the portable name would still differ.
- `tt_` in SQL is short but opaque to a reader, and a two-letter prefix collides more often. R already uses `tt_` for its functions. A shared prefix across R and SQL would blur which surface a sample targets.
- Aliases double the documented names. This issue recommends none.

### 10. Docs and replayed samples

One page shows five examples, each in all three databases and all three data frame libraries, each replayed by a check against a recording:

1. Filter with decide.
2. Rank with `probability`.
3. Choose with options and descriptions.
4. Score on levels.
5. Recognize joined back to its table or frame.

The page lists the dialect and library limits named in each proposal. Where the shared SQL form runs everywhere, the page shows one query marked for all three databases. Where a surface needs its own form, the page shows each form beside the others. The check replaces the `.sql` skip in `site/examples/SKIP:8` for these samples. DuckDB gains an `examples.json` like SQLite's and PostgreSQL's.

Fix the wrong samples: the SQLite relations call (`site/examples/functions/recognize/sqlite.sql:16-19`), the PostgreSQL README choose (`databases/postgresql/README.md:52`), and the DuckDB README's complete-question form (`databases/duckdb/README.md:11-14`, `:26`).

## Ideas from sqlite-jev

[sqlite-jev](https://github.com/mgaitan/sqlite-jev) by Martín Gaitán is a SQLite extension that asks yes/no, choice, and score questions about rows. It meets several of the problems above. This section credits it and lists what ThinkThen can take and what it should avoid. Each idea names the proposal it serves. Links point at commit `1ac946c`.

### Ideas to take

1. **A table function that reads a table and joins back by rowid.** `jev_rows` reads the named columns of a table, packs up to 40 rows into one request, and returns one result row per source row. The caller joins on `source_rowid`. Users get batching from one query, with no warm pass first. This shape fits proposal 6's row-source option and proposal 4's `(query, ...)` form.

   ```sql
   SELECT t.id, t.subject, round(j.probability, 3) AS urgency
   FROM jev_rows('tickets', 'The customer explicitly expresses urgency or says work is blocked',
                 'noul', NULL, json_array('subject', 'message')) AS j
   JOIN tickets AS t ON t.rowid = j.source_rowid
   WHERE j.probability >= 0.6;
   ```

   Source: [README.md lines 8–21](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/README.md#L8-L21).

2. **One result shape for every question kind.** `jev_rows` always returns `source_rowid`, `answer`, `probability`, `choice`, `score`, and `confidence`. A column that does not apply to the question is NULL. Users learn one set of column names and can switch a question from decide to choose without rewriting the join. This serves proposal 2. Source: [README.md lines 61–62](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/README.md#L61-L62).

3. **Several columns sent as one labelled record.** The column list turns each row into a JSON object keyed by column name, such as `{"subject": ..., "message": ...}`. The scalar form takes `json_object(...)` for the same result. The model sees field names, and users judge a ticket's subject and body together without string concatenation. ThinkThen takes one text per call. No proposal above covers this.

   ```sql
   SELECT jev_prob(json_object('subject', subject, 'message', message),
                   'The customer explicitly expresses urgency')
   FROM tickets WHERE id = 42;
   ```

   Source: [README.md lines 144–151](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/README.md#L144-L151) and [src/jev.c lines 993–999](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/src/jev.c#L993-L999).

4. **A probability function beside the yes/no function, and a threshold argument.** `jev_prob(state, condition)` returns the yes probability. `jev(state, condition, 0.7)` applies a threshold in the call. The README tells users to keep the threshold in SQL so it can rise with the cost of a false yes. This matches proposal 3, and the threshold argument matches proposal 1. Source: [README.md lines 83–85 and 129–140](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/README.md#L83-L140).

5. **A row limit that refuses before sending and names the fix.** `max_rows` defaults to 500. A larger scan fails before any data leaves, with this message:

   ```text
   jev: table 'tickets' exceeds max_rows=500; prefilter into a smaller table/view or raise jev_config('max_rows', ...)
   ```

   An unknown setting lists the valid names, and a missing key names both ways to set it. Users fix the query from the message alone. This fits proposal 7 and gives `thinkthen_max_requests_total` a model for its message. Source: [src/jev.c line 983](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/src/jev.c#L983), [line 736](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/src/jev.c#L736), and [line 374](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/src/jev.c#L374).

6. **One setting function per connection.** `jev_config(name, value)` sets the model, batch size, row limit, and timeout for one connection, and returns the value it set. A call that sets `api_key` returns `set` and never echoes the key. One function is easier to learn than twelve, and a per-connection setting lets two queries in one process differ. This fits proposal 8. ThinkThen keeps the key in the environment, so only the rule that a setting never echoes a secret carries over. Source: [README.md lines 155–167](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/README.md#L155-L167).

7. **Usage counts that include rows and cache hits.** `jev_stats()` returns requests, input and output tokens, rows judged, and cache hits. ThinkThen's usage lacks rows judged, which proposal 7 can add. The example script ends with `SELECT json(jev_stats())`, so a reader sees what the demo cost. The test suite runs the same batch twice and checks that the request count stays flat. Source: [examples/ticket_triage.sql line 69](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/examples/ticket_triage.sql#L69) and [test/test.sql lines 64–68](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/test/test.sql#L64-L68).

8. **A pip package with a `load` helper.** `uv add sqlite-jev` installs a platform wheel. `sqlite_jev.load(connection)` finds the library, loads it, and turns extension loading off again. A test checks that loading stays off afterwards. ThinkThen's SQLite README asks users to copy `libthinkthen0.so` to `thinkthen.so` by hand. Release archives also ship `SHA256SUMS`. No proposal above covers loading.

   ```python
   import sqlite3, sqlite_jev
   connection = sqlite_jev.load(sqlite3.connect(":memory:"))
   ```

   Source: [README.md lines 169–186](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/README.md#L169-L186), [python/sqlite_jev/\_\_init\_\_.py](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/python/sqlite_jev/__init__.py), and [test/python_package.py lines 11–21](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/test/python_package.py#L11-L21).

9. **A limits list in the README.** The README tells users that row text leaves the machine, that a scan is not an index, that they should filter with plain SQL first, that counting and date math belong in SQL, and that row text can steer the model. Proposal 10's page can carry the same short list. Source: [README.md lines 262–273](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/README.md#L262-L273).

10. **One script as test and example.** `test/test.sql` runs against a local mock server, and `test/expected.out` holds its exact output. `examples/ticket_triage.sql` runs the same shapes live. Proposal 10's page can follow this pattern: one SQL file per example, one saved output, and a check that compares them. Source: [test/test.sql](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/test/test.sql) and [test/expected.out](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/test/expected.out).

### Choices to avoid

- **NULL placeholders in positional arguments.** A yes/no call on selected columns must pass `NULL` for the unused labels slot: `jev_rows('tickets', question, 'noul', NULL, json_array(...))`. Named arguments remove this.
- **The question kind as a string argument with a coined name.** Users must type `'noul'`, `'choice'`, or `'score'`. A typo fails only at run time. Separate function names read better.
- **A yes/no function with no unsure answer.** `jev()` returns 0 when the probability sits below the threshold, so "no" and "not sure" look the same ([src/jev.c line 614](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/src/jev.c#L614)). Keep ThinkThen's NULL for unsure.
- **A table name in place of a query.** `jev_rows` takes a table name, needs a rowid table, and reads every row before it returns the first. Users must copy filtered rows into a temporary table first ([README.md lines 120–122](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/README.md#L120-L122)). Proposal 4's query form lets a WHERE clause choose the rows.
- **A second function for batching.** The README tells users to switch from the scalar form to `jev_rows` for a scan. Proposal 6 should state the request count for the scalar form so users need not learn two forms to control cost.
- **Batches sent one after another.** `evaluate_rows` sends each batch of 40 and waits before the next. ThinkThen's throttle already sends several at once.
- **A cache that ends with the connection.** Answers live in memory and vanish when the connection closes. ThinkThen's disk cache and replay outlive the process.
- **The API key in SQL.** `jev_config('api_key', '...')` puts the key in shell history and query logs. Keep ThinkThen's rule that only the environment holds the key.
- **No cancel.** A request blocks for up to 90 seconds, and the code never checks for an interrupt. Keep ThinkThen's interrupt check.
- **Binary values sent as hex text.** A BLOB column becomes a `"hex:..."` string without a warning ([src/jev.c lines 943–953](https://github.com/mgaitan/sqlite-jev/blob/1ac946cdc0d80fd2d77ba0b64deafb456521fdd4/src/jev.c#L943-L953)). Keep ThinkThen's usage error.

## Acceptance criteria

1. The survey tables above, updated to the new surface, live on the page. Every row cites a registration line or a public signature.
2. `thinkthen_decide`, `thinkthen_probability`, `thinkthen_choose`, `thinkthen_score` and `thinkthen_tag` take `(question, text[, settings])` in all three databases, with a plain-text question accepted everywhere.
3. SQLite registers `thinkthen_probability`. pandas, Polars and R each return a probability column without `details`.
4. The deck's data-slide query runs in DuckDB and PostgreSQL with `threshold := '0.3:0.7'` and no `json_object`.
5. Python renames `true_` and `false_` to `true` and `false`, and Python's `decide` takes `threshold`.
6. This Abbey Road example runs as one replayed test per surface, on SQLite, DuckDB, PostgreSQL, pandas, Polars and R, and the six outputs match row for row:

   ```sql
   SELECT title,
          thinkthen_decide('The text is the title of a song by the Beatles. '
                           || 'It appears on the album Abbey Road.',
                           title, '{"threshold": "0.3:0.7"}') AS on_abbey_road,
          thinkthen_probability('The text is the title of a song by the Beatles. '
                           || 'It appears on the album Abbey Road.',
                           title, '{"threshold": "0.3:0.7"}') AS p
   FROM songs
   ORDER BY p DESC;
   ```

   ```python
   songs["on_abbey_road"] = songs["title"].tt.decide(abbey_road, threshold="0.3:0.7")      # pandas
   songs["p"] = songs["title"].tt.probability(abbey_road, threshold="0.3:0.7")
   songs.with_columns(                                                                      # Polars
       on_abbey_road=pl.col("title").tt.decide(abbey_road, threshold="0.3:0.7"),
       p=pl.col("title").tt.probability(abbey_road, threshold="0.3:0.7"),
   ).sort("p", descending=True)
   ```

   ```r
   songs |>
     mutate(on_abbey_road = tt_decide(abbey_road, title, threshold = "0.3:0.7"),
            p = tt_probability(abbey_road, title, threshold = "0.3:0.7")) |>
     arrange(desc(p))
   ```

7. Each Abbey Road test counts its sends. The counts match the page's stated count for that surface. If SQLite or PostgreSQL keeps one request per row, the page says so and the test pins that count.
8. `recognize` runs as a row source joined to its table in SQLite and PostgreSQL. DuckDB matches, or the page names the `unnest` form as its dialect limit.
9. Usage returns the same four counts on every surface. SQL usage returns one row with four columns in all three databases.
10. SQL errors read `thinkthen <kind>: <message>` with one retry spelling in all three databases.
11. The five page examples run on all six surfaces under a check. The three wrong samples are fixed.
12. A decision record in `sdlc/` states the choices made on proposals 1, 3, 4, 6 and 9 and why.

## Out of scope for 0.1

- A struct or composite return type for any verb. It needs a change to ADR 0082.
- A DuckDB function that returns an ENUM.
- Naming the backend address or key from SQL.
- A per-record answer cache, if the builder picks another batching option in proposal 6.
- Look-ahead batching inside a PostgreSQL custom scan or a SQLite virtual table over a whole query.
- Short SQL aliases such as `tt_decide`, or bare verbs.
- A load-question-by-name function.
- Per-row keep-going in pandas, Polars and R column calls, beyond stating today's behavior on the page.
- Cost in usage beyond what the run-cost issue delivers.

## Preparation at current main

The [2026-09-29 source preparation](../records/2026-09-29-sql-usability-preparation.md) checks this issue against `42b45db6`, classifies each proposal, identifies accepted batching, settings, type and find aliases, and recommends three small release-priority batches. The survey and acceptance criteria above remain the original request. The proposed common SQL slot, named syntax, result shape and host lifetimes are not yet approved API decisions.

## Progress on the two scalar defects

[Ticket 0259](../tickets/0259-sql-scalar-usability.md) was accepted at `d01bc92b` and landed at `e6939251`. It fixes PostgreSQL plain judgment questions, including native choose/score/tag labels, and SQLite decide yes probability. Its [build record](../records/0259-sql-scalar-build.md) pins installed-host outputs and exact local requests. The historical survey above records the pre-fix source. The twelve original acceptance criteria stay open for their remaining work, including common SQL syntax, data frame probability, page parity and batching. A [current-main two-sample check](../records/2026-09-29-sql-site-sample-preparation.md) reproduces the SQLite recognize and relate site SQL failures under the installed 0259 library. Marketing owns both later site corrections. The recognize failure advances this issue's criterion 11; the relate failure belongs to the separate site-samples issue and is not one of criterion 11's named wrong examples.

## Related issues

- `2026-09-26-architect-review-04-libraries-and-databases.md` covers identity and warm-total gaps in the same extensions.
- `2026-09-26-settings-some-surfaces-cannot-reach.md` rules on the address and key.
- `closed/2026-09-27-one-type-contract-for-every-surface.md` and ADR 0082 settle plain SQL return types.
- `2026-09-26-every-surface-should-give-back-run-facts.md` owns the facts that proposal 3 moves off the data frame idioms' return value.
- `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md` owns site sample regeneration. Proposal 10 adds the page to it.
- `2026-09-26-batching-design.md` owns the packing rules that proposal 6 extends.


## Remaining SQL error-format acceptance

Review of DuckDB0286 at2463436c2 found ordinary ThinkThen usage errors without ADR0105 section8’s `(retryable: yes|no)` suffix. The accepted ADR governs this change; its exact plain removal messages remain exceptions. DuckDB0286 owns its renderer correction now. A narrow SQLite follow-up must apply the same format, and PostgreSQL0285 must retain and prove its existing suffix. Host-native SQL syntax and binding errors remain the host’s errors. The shared error catalog and final SQL examples wait for these consistent runtime outputs. Earlier SQLite settings/keyed-call completion did not close this umbrella criterion.
