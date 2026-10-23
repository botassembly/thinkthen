# ThinkThen for PostgreSQL

ThinkThen exposes ten set-returning SQL functions over a source query. Each invocation owns a read-only source cursor and sends its records through one shared SQL host call. The engine owns question admission, batching, thresholds, retries, ordering and answers.

## Install and configure

Download `thinkthen-postgres-0.2.0-18-linux-x86_64.tar.gz` or `thinkthen-postgres-0.2.0-18-linux-arm64.tar.gz` from the [release assets](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0). The package requires PostgreSQL 18 and glibc 2.28 or newer. Select the archive for the server architecture. PostgreSQL ships as a prebuilt release asset.

An administrator extracts the archive and installs it into the matching server installation:

```sh
tar -xzf thinkthen-postgres-0.2.0-18-linux-x86_64.tar.gz
install -m 755 lib/thinkthen_postgres.so "$(pg_config --pkglibdir)/"
install -m 644 share/extension/* "$(pg_config --sharedir)/extension/"
```

Create the extension and configure access in the target database:

```sql
CREATE EXTENSION thinkthen;
GRANT EXECUTE ON FUNCTION thinkthen_decide(text, text, json, json) TO application_role;
SET thinkthen.allowed_backends = '{"stub":["stub"]}';
SET thinkthen.settings = '{"adapter":"stub","batch":16}';
```

The extension uses PostgreSQL's C extension mechanism and requires administrator installation. Functions use the caller's database privileges. Installation revokes EXECUTE from PUBLIC on every ThinkThen function. An administrator grants each required function to roles allowed to spend the server's model key, as the example grants decide. A role without a grant receives SQLSTATE `42501`. EXECUTE permits a role to select any backend whose key the server holds, subject to `thinkthen.allowed_backends`. The allow-list restricts the backend and model pairs that role can use. Evaluating a function can send evidence to the selected backend. The package ships its engine inside the extension library. Users need neither Rust nor a compiler. Loading retains that library for the backend process lifetime, including after `DROP EXTENSION`. See [native library loading](https://www.postgresql.org/docs/18/xfunc-c.html#XFUNC-C-DYNLOAD) and [PGXS installation](https://www.postgresql.org/docs/18/extend-pgxs.html).

`thinkthen.allowed_backends` is an administrator-only JSON object that maps named backends to lists of permitted model names, for example `'{"systemone":["jev-1.13.0"],"openai":["gpt-6-luna"]}'`. PostgreSQL restricts changes with `PGC_SUSET`; administrators may delegate that permission through PostgreSQL's parameter privileges. Configure it in the server configuration or with `ALTER DATABASE ... SET` for persistent use. With no policy set, only the engine's default backend and default model are allowed. An explicit empty object allows none. The engine resolves omitted names and models before checking the pair. A disallowed pair raises SQLSTATE `22023` before client construction or credential lookup. Changing the policy invalidates the connection's cached client even if `thinkthen.settings` stays the same. Existing calls retain their admitted snapshot. See [parameter privileges](https://www.postgresql.org/docs/18/sql-grant.html) and [configuration settings](https://www.postgresql.org/docs/18/config-setting.html).

The extension reserves the `thinkthen.` setting prefix after registering its settings. Unknown names raise native configuration errors after loading. PostgreSQL removes unknown placeholders set before loading and reports a warning.

`thinkthen.settings` stores the existing [client settings](https://github.com/botassembly/thinkthen/blob/v0.2.0/schemas/client.schema.json) as a session string. The first function invocation lazily admits the current string and administrator policy through the shared SQL client constructor. It defaults to `{}`. Reusing the same settings string and policy reuses the connection's immutable client; a different string creates a replacement after source planning, immediately before opening the call. Planning may execute application functions that change settings and make nested ThinkThen calls; the outer invocation uses the settings effective after planning. Each invocation retains its client snapshot independently. A failed replacement does not fall back to the previous client's configuration.

`SET` stores the string; validation occurs when a function first uses it. Settings can select the named backend, model, mode and batch through the shared grammar. SQL cannot choose an address or a credential variable. The shared Rust constructor rejects `url` and `key_source`, including JSON null, before environment lookup or client creation. It also owns all other settings errors. The extension does not duplicate that admission rule in a GUC hook. Configure keys in the server process environment before starting PostgreSQL. Never put a key in SQL, a GUC, diagnostics or logs.

`SET SESSION` lasts for the session after commit; rollback restores its previous value. `SET LOCAL` applies only through the current transaction. Savepoint rollback restores the applicable setting too. The next invocation uses the effective GUC value after that restoration. A call already opened retains its snapshot. These are [PostgreSQL's native SET semantics](https://www.postgresql.org/docs/18/sql-set.html).

## Read a table in one call

Pass a source SELECT as the first argument. The extension opens, fetches and closes its cursor on the same connection. Callers need no cursor declaration or explicit transaction block.

```sql
CREATE TABLE documents(id bigint PRIMARY KEY, body text NOT NULL);
INSERT INTO documents VALUES
  (1, 'Please refund the extra charge.'),
  (2, 'The meeting is on Monday.');

SELECT * FROM thinkthen_decide(
  'SELECT body AS record FROM documents ORDER BY id',
  'Does the message request a refund?');
```

Every function has the same argument shape:

```sql
thinkthen_decide(source_sql text, question text,
                options json DEFAULT '{}', parameters json DEFAULT '[]')
RETURNS TABLE("position" bigint, status text, answer json,
              p double precision, error json, detail json, kept boolean)
```

The other nine names use that signature with their own existing question and options contract. Supplied SQL NULL arguments are native input errors, rather than an empty result or a default. Questions are plain inline text. If the first non-whitespace character is `{`, the shared Rust helper admits a tagged question JSON object instead. Questions are values, never filenames. Options use the existing [request contract](https://github.com/botassembly/thinkthen/blob/v0.2.0/schemas/request.schema.json); omission supplies `{}`.

The source must contain exactly one read-only SELECT that returns one column named `record` of PostgreSQL type `text` or `json`. A text column supplies raw text evidence directly; it needs no JSON wrapping. Use `json_build_object('id', id, 'body', body)` for structured evidence. Existing `jsonb` data can be cast to `json` in the source query. SQL NULL source cells are native input errors. Filter missing records or replace them explicitly in the source query. JSON null remains a JSON record for Rust admission. The extension copies text bytes or serialized JSON bytes according to the native column type. It never extracts JSON record fields.

The optional `parameters` argument is a JSON array. Its first element binds `$1`, its second binds `$2`, and subsequent elements follow that order. Repeated references to a parameter reuse its bound value. JSON strings become PostgreSQL `text`, booleans become `boolean`, and numbers become exact PostgreSQL `numeric` values through the native numeric input function. JSON null binds SQL NULL with type `text`; use an explicit source cast when another type is required. Arrays and objects are not source parameter values. Parameter strings follow PostgreSQL's native text representation and encoding limits. Supplied outer SQL NULL arguments are distinct from null elements inside the parameter array and are rejected.

```sql
SELECT position, answer, status FROM thinkthen_decide(
  'SELECT body AS record FROM documents WHERE id >= $1::bigint ORDER BY id',
  'Does the message request a refund?', '{}', '[1]');
```

Applications can bind the source query, question, options and parameter array as values in their outer statement. Values never substitute into source SQL text. The driver constructs native parameter types, Datums and null flags and uses `SPI_cursor_parse_open` with `ParamListInfo`, `PARAM_FLAG_CONST`, `read_only = true` and a non-scrollable cursor. PostgreSQL copies the parameter data into the portal; temporary bindings can then be released. The modern API replaces the equivalent `SPI_cursor_open_with_args`. See [parameterized cursor opening](https://www.postgresql.org/docs/18/spi-spi-cursor-parse-open.html) and [native argument types and null flags](https://www.postgresql.org/docs/18/spi-spi-cursor-open-with-args.html).

The function fetches bounded source chunks forward until exhaustion. Positions start at one for each invocation, across chunks. Every invocation owns an independent cursor, including repeated or correlated calls; repeating a call can send the same evidence again. An explicit source `ORDER BY` defines reproducible input positions. Empty, multi-statement, non-SELECT, writing and row-locking sources fail before the first fetch. PostgreSQL parses source SQL; the extension does not implement a SQL parser. The read-only restriction does not sandbox application functions in a SELECT. Supply trusted source SQL.

## Call the ten functions

These examples reuse `documents` above and share the same argument order.

```sql
SELECT position, answer, status FROM thinkthen_choose(
  'SELECT body AS record FROM documents ORDER BY id',
  'Which team owns this?', '{"options":["billing","shipping","account"]}');

SELECT position, answer, status FROM thinkthen_tag(
  'SELECT body AS record FROM documents ORDER BY id',
  'Which topics apply?', '{"labels":["refund","delivery","login"]}');

SELECT position, answer, status FROM thinkthen_score(
  'SELECT body AS record FROM documents ORDER BY id',
  'How urgent is this?', '{"levels":["low","medium","high"]}');

SELECT position, answer, status FROM thinkthen_filter(
  'SELECT json_build_object(''id'', id, ''body'', body) AS record FROM documents ORDER BY id',
  'Does the message request a refund?', '{"on":"/body"}') WHERE kept;

SELECT position, answer, status FROM thinkthen_rank(
  'SELECT body AS record FROM documents ORDER BY id',
  'How urgently does this need a reply?', '{"top":10}');

SELECT position, answer, status FROM thinkthen_find(
  'SELECT body AS record FROM documents ORDER BY id', 'When is the meeting?');

SELECT position, answer, status FROM thinkthen_annotate(
  'SELECT body AS record FROM documents ORDER BY id',
  '{"type":"inline","value":{"questions":{"refund":{"decide":"Does the message request a refund?"}}}}');

SELECT position, answer, status FROM thinkthen_recognize(
  'SELECT body AS record FROM documents ORDER BY id',
  '{"type":"inline","value":["person","organization"]}');

SELECT position, answer, status FROM thinkthen_relate(
  'SELECT body AS record FROM documents ORDER BY id',
  '{"type":"inline","value":[{"name":"works_for","source":"person","target":"organization"}]}');
```

## Read rows and errors

`position` is the original one-based input ordinal. `status`, `answer`, `p`, `error` and `detail` follow the [shared row contract](https://github.com/botassembly/thinkthen/blob/v0.2.0/schemas/row.schema.json). An unsure answer is present JSON null; an absent answer, probability, error or detail is SQL NULL. `kept` is the engine's selection flag. Filter exposes all outcomes; `WHERE kept` selects matches. To retain unsure and failed outcomes too, use `WHERE kept OR status <> 'answered'`. Offsets count Unicode scalar values in answers and detail.

JSON columns retain engine JSON, including escaped zero characters in record data and arbitrary valid payload keys. UTF-8 databases preserve the payload bytes unchanged; other database encodings require native character conversion. PostgreSQL's `json` storage preserves the resulting serialized JSON; `jsonb` rejects `\u0000` and imposes additional numeric bounds. PostgreSQL JSON extraction functions can still reject values they cannot represent as text. Choose extraction deliberately; the extension does not extract payload fields or convert them through `jsonb`. Plain SQL `text` already cannot contain a zero byte; escaped JSON input reaches Rust's ordinary admission. See [JSON type behavior](https://www.postgresql.org/docs/18/datatype-json.html).

Rank produces best-first rows with their original positions. Other functions retain input order. Use an outer `ORDER BY` when the application's SQL result ordering must be explicit; `ORDER BY position` restores source order. Rank without `top` retains its inputs in the engine. The extension adds no Rust result buffer or sorting rule.

Request and settings admission errors abort the statement with SQLSTATE `22023` and the engine's unchanged `code: message`. Backend and local final call errors use SQLSTATE `58000` with the same diagnostic form. Failed execution rows retain their engine error object. A final call error still aborts the whole statement after draining; materialization can prevent any result from reaching the caller. PostgreSQL cursor, column, encoding, allocation and SPI errors retain their native SQLSTATE and message after cleanup. They acquire no invented engine code. Native cancellation uses PostgreSQL's `57014`. See [SQLSTATE definitions](https://www.postgresql.org/docs/18/errcodes-appendix.html).

## Bound work and cancel

PostgreSQL materializes these FROM-function results into its native tuplestore before returning rows to the outer query. An outer `LIMIT` does not cap model calls. Put a `LIMIT` in the source query to bound the evidence sent. Rank's `top` bounds retained ranking results, not the number of records examined or model calls. The tuplestore uses PostgreSQL's memory and spill behavior; bounded Rust staging does not bound total temporary database storage. This follows PostgreSQL's [FunctionScan implementation](https://raw.githubusercontent.com/postgres/postgres/REL_18_STABLE/src/backend/executor/nodeFunctionscan.c) and [table-function execution](https://raw.githubusercontent.com/postgres/postgres/REL_18_STABLE/src/backend/executor/execSRF.c).

Cancel through the driver's normal query cancellation, `pg_cancel_backend`, or `statement_timeout`. The backend thread processes PostgreSQL interrupts between bounded host polls and source fetches. It cancels and releases the shared call and closes its source cursor before raising the native query error. Query errors, transaction abort and executor cleanup release invocation resources too. Rollback does not refund model calls already sent.

## Native implementation boundary

The wrapper uses C11 and PGXS, statically links `thinkthen-ffi`, and consumes its common SQL bridge over `thinkthen-sql`. Jansson reads only the bridge's fixed projection envelope and diagnostic fields. PostgreSQL's native JSON parser translates the source parameter array, preserving numeric lexemes for its native numeric input function. Answer and error payloads remain counted raw JSON bytes from the bridge projection; detail remains separate raw JSON. Payload objects never pass through a C JSON parser that could narrow key or string support. The extension adds no unsafe Rust crate, generated projection, input grammar or engine validator.

C converts question, options, settings and text or serialized JSON record bytes from the database encoding to UTF-8 with `pg_server_to_any(..., PG_UTF8)` before entering Rust. Source SQL and native source bindings remain in PostgreSQL's encoding. C converts result JSON, result text and engine diagnostics from UTF-8 with `pg_any_to_server(..., PG_UTF8)` before constructing PostgreSQL values or errors. Both operations run on the backend thread outside Rust frames. Conversion can return the original pointer instead of a newly allocated, NUL-terminated string. The wrapper retains the supplied byte length on that path, copies before the source owner is released, and frees only a distinct conversion allocation with `pfree`. It never frees a Rust buffer with PostgreSQL's allocator. UTF-8 payload bytes stay unchanged; conversion to an encoding that cannot represent a result raises a native PostgreSQL encoding error. That error releases the active call through the same C cleanup boundary. This follows PostgreSQL 18's [conversion implementation and ownership contract](https://raw.githubusercontent.com/postgres/postgres/REL_18_STABLE/src/backend/utils/mb/mbutils.c).

One C driver serves the ten registrations. All functions are `VOLATILE`, `PARALLEL UNSAFE` and `SECURITY INVOKER`; none is leakproof. Each invocation owns its client snapshot, source cursor, SQL call, current fetched chunk, current row and buffers. It connects to SPI and opens an automatically named, non-holdable portal with the bound parameters. SPI's native read-only plan checks reject writing statements, modifying CTEs and row-locking SELECTs before execution; its cursor checks reject empty and multi-query plans. The driver requires a single SELECT portal and the stated source shape before fetching. PostgreSQL's [SPI implementation](https://raw.githubusercontent.com/postgres/postgres/REL_18_STABLE/src/backend/executor/spi.c) owns query parsing and plan validation. The extension does not sandbox application functions in a source SELECT.

All source operations stay on the backend thread. Normal completion closes the owned portal through `SPI_cursor_close` before `SPI_finish`. Error and cancellation cleanup preserve the original error, cancel and release the engine call, and close any still-live owned portal and SPI scope before rethrowing. PostgreSQL's resource-owner cleanup covers portals already released by transaction abort. Separate invocations own separate SPI scopes and source cursors, including nested calls. No portal or prepared plan remains cached in connection state.

The driver uses materialize mode and writes rows directly into PostgreSQL's result tuplestore. A child invocation memory context holds an idempotent cleanup callback registered before any Rust handle is created. That callback cancels and releases owned handles, rows and buffers on normal completion, error or context reset. The result tuplestore and descriptor live in the executor's result context. A separate resettable fetch context holds one bounded input group. After Rust copies a whole chunk, C frees the fetched SPI tuple table and resets that context. No PostgreSQL Datum, portal, tuple or memory-context pointer enters an engine thread. These lifetimes follow [SPI memory management](https://www.postgresql.org/docs/18/spi-memory.html).

Every PostgreSQL operation runs in C after Rust returns. C uses `PG_TRY`/`PG_CATCH` around source fetching, conversion and output construction; on an error it preserves PostgreSQL's error, releases the call and rethrows. The bridge's interrupt probe only reads native pending flags and never invokes `CHECK_FOR_INTERRUPTS`. The C driver processes interrupts outside Rust frames. No PostgreSQL longjmp crosses Rust or a Rust callback. SPI itself can raise transaction errors rather than return error codes, as its [error contract](https://www.postgresql.org/docs/18/spi.html) explains.

The signal boundary uses native `pthread_sigmask` around every entry into the linked Rust bridge, including client creation and cleanup. C saves the backend thread's mask, blocks all blockable signals with `sigfillset`, enters Rust, stores returned owned handles in the already registered cleanup owner, then restores the saved mask after Rust returns. No PostgreSQL API runs while that temporary mask is installed. Restoration occurs on every C exit path. Engine threads inherit the blocked mask when created; their descendants inherit it too and retain it for their lifetime. The backend's original mask is restored between bounded polls, so PostgreSQL handlers run only on the backend thread. There is no extension signal handler, signal-wait thread or asynchronous pthread cancellation. This design relies on POSIX [thread mask inheritance](https://pubs.opengroup.org/onlinepubs/000095399/functions/pthread_create.html) and [per-thread mask control](https://pubs.opengroup.org/onlinepubs/9799919799/functions/pthread_sigmask.html). The installed held-request check verifies the inherited masks.

## Installed proof

The installed extension runs inside a disposable PostgreSQL 18 container with networking disabled. The stub construction file and shared conformance cases run through SQL, including multiple fetch chunks, every function, raw text and structured JSON evidence, JSON null, arbitrary escaped record data, output ownership, and rank order with source positions. Native cases cover parameter types, nulls and repeated bindings, independent and nested source cursors, cleanup after success and failure, rejected writing or multi-statement sources before fetching, source type errors, session and transaction settings, rejected address/key settings before any request, and database reuse after failure. A source query containing a LIMIT bounds input independently of an outer LIMIT.

An installed LATIN1 database case sends non-ASCII evidence and question/options/settings text through the UTF-8 boundary and returns a non-ASCII answer through the database encoding. It also checks native failure and call cleanup for an unrepresentable result. UTF-8 cases retain the escaped zero-character and arbitrary JSON-key checks so encoding conversion cannot silently narrow the payload contract.

Cancellation waits for a shared loopback request to be held, then issues `pg_cancel_backend` from a second connection. It produces native query cancellation while the reply remains held, closes the engine connection, and permits a subsequent query. A native check verifies that every engine thread blocks all blockable signals, not merely that one cancellation happened to reach the right thread. Statement timeout, abandonment and cleanup use the same driver boundary. Engine behavior remains covered by its existing tests; PostgreSQL tests cover the native translation and resource lifetimes. The manual runtime bundle supports a bounded live check of all ten functions on the default backend.

The public contract defines stable error codes, client settings and execution identity. Shared Rust diagnostics escape control characters before host translation, including NUL as `\u0000`. SQL settings reject `url` and `key_source` with `usage.settings`.
