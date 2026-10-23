# ThinkThen for SQLite

ThinkThen exposes ten table functions through a loadable SQLite extension. Each invocation reads a source query and passes its records through one shared SQL host call. The engine owns questions, batching, thresholds, retries and answers.

## Load and configure

The packages target Linux x86-64 with glibc and macOS arm64. SQLite must support extension loading and provide SQLite 3.41.0 or newer. The examples also use SQLite's JSON functions. A host built with `SQLITE_OMIT_LOAD_EXTENSION` cannot load this package. Applications must enable extension loading through their SQLite API before loading; the shell already enables it. The entry point checks the runtime minimum before using newer dispatch-table entries.

Unpack the package. In the SQLite shell:

```sql
.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
SELECT thinkthen_configure('{"adapter":"stub","batch":16}');
```

`thinkthen_configure(settings_json)` admits the existing [client settings](https://github.com/botassembly/thinkthen/blob/v0.2.0/schemas/client.schema.json) and returns SQL integer `1` on success. Configuration belongs to this connection. A failed configuration leaves its previous client intact. An unchanged settings string reuses its client. Every invocation snapshots its client, so replacing settings does not alter an active cursor. Without configuration, the first invocation admits the engine's default settings.

For a hosted backend, set the engine's key environment variable before starting SQLite. SQL settings select a backend by name and may set model, mode and batch through the shared client grammar. SQL cannot choose a backend address or key variable; `url` and `key_source` are rejected before a client is created. Pass settings as a bound parameter in applications. Do not place keys in SQL, diagnostics or logs. Calls may send evidence to the selected backend.

Load the extension separately on each connection. Its native libraries stay loaded until process exit so cancellation can release a cursor while the engine finishes cleanup. Connection closure still frees clients and cursors. Loading does not register functions automatically on future connections.

## Read a table in one call

```sql
CREATE TABLE documents(id INTEGER PRIMARY KEY, tenant INTEGER, body TEXT NOT NULL);
INSERT INTO documents VALUES
  (1, 7, 'Please refund the extra charge.'),
  (2, 7, 'Thanks for the quick help.');

SELECT position, status, json_extract(answer, '$') AS refund, p
FROM thinkthen_decide(
  'SELECT body AS record FROM documents ORDER BY id',
  'Does the message request a refund?');
```

Every table function takes two required arguments followed by two optional arguments:

| Argument | Value |
| --- | --- |
| `source_sql` | One caller-authored, read-only statement that returns exactly one column named `record` |
| `question` | Plain inline question text, or a JSON object containing a tagged question from the [request contract](https://github.com/botassembly/thinkthen/blob/v0.2.0/schemas/request.schema.json) |
| `options` | Optional existing options JSON object; defaults to `{}` |
| `parameters` | Optional JSON array of source parameter values in SQLite parameter-index order; defaults to `[]` |

Applications can bind all four arguments to their outer statement. The extension prepares the source on the same connection and binds each parameter with SQLite's binding APIs. It never substitutes values into SQL text. Use contiguous `?1`, `?2`, and subsequent slots. The parameter array supplies every slot exactly once; repeated uses of the same numbered slot reuse its value. JSON strings, integers, real numbers, booleans and null become SQLite text, integer, real, integer `0` or `1`, and NULL. Arrays and objects are not source parameter values. Source parameters do not accept blobs. Integers must fit SQLite's signed 64-bit integer type; translation never silently rounds them to a real value. Supplied outer arguments require SQL TEXT; SQL NULL, blobs and other storage classes raise native input errors rather than being coerced.

The source must prepare to one row-producing statement. Empty input and an additional statement after its SQL tail are rejected before stepping; trailing whitespace and comments are permitted. A source cell contains UTF-8 TEXT. Plain TEXT is a text record, even when its bytes look like JSON. A TEXT value carrying SQLite's JSON subtype, such as the result of `json()`, `json_object()` or `json_array()`, is a JSON record. SQL NULL or a non-text source cell is a database input error. Filter missing records or replace them explicitly in the source query. JSON null remains a JSON record. JSON admission and evidence selection belong to the engine.

```sql
SELECT body AS record FROM documents; -- plain text
SELECT json_object('id', id, 'body', body) AS record FROM documents; -- JSON object
```

SQLite does not preserve JSON subtypes in stored table values. Wrap a stored JSON column when reading it: `SELECT json(document) AS record FROM saved_documents`. Subqueries and sorts can also discard SQLite subtypes; apply the JSON function in the final source projection. After a sort, a TEXT value without a JSON subtype raises an error if its bytes form a JSON object or array. This includes literal text that looks like an object or array, because its origin cannot be recovered. Ordinary sorted text stays text. A retained JSON subtype still admits structured evidence.

If the query raises this error, stage its ordered records and apply `json()` after staging:

```sql
CREATE TEMP TABLE ordered_input(position INTEGER PRIMARY KEY, document TEXT);
INSERT INTO ordered_input(document)
SELECT json_object('id', id, 'body', body) FROM documents ORDER BY body, id;

SELECT position, answer FROM thinkthen_decide(
  'SELECT json(document) AS record FROM ordered_input ORDER BY position',
  'Does the message request a refund?', '{"on":"/body"}');
```

The INTEGER PRIMARY KEY supplies the final order without another sort. To retain literal JSON-looking text, stage `body` and read it directly in the same indexed order. Sorts within nested source queries also count. A later source failure can occur after earlier chunks have sent requests; completed requests cannot be refunded.

The source statement has its own cursor inside the outer statement. It reads bounded chunks only when the shared host requests input. A chunk boundary does not end the call or flush a backend batch. Values are copied before the next source step. Separate scans, including self-joins, own separate source cursors and calls. SQLite can restart a scan or evaluate a correlated invocation for each outer row. Each restart creates a new call and can send evidence again; identical invocations do not share answers. Source SQL can invoke other application functions or virtual tables, including nested calls, on the execution thread. Such nested work has independent cursor ownership and follows SQLite's own resource limits; it is not a way to batch an outer table.

The read-only check prevents direct database writes. It does not sandbox SQL: application functions and other virtual tables can have side effects. Supply trusted source SQL. The extension does not install an authorizer or replace a progress handler. Its table functions and configuration function are direct-only and cannot be invoked through stored views or triggers.

## Call the ten functions

The following examples reuse `documents`. They use the same argument order. Plain text is inline; text whose first non-whitespace character is `{` must be a tagged question JSON object. The shared SQL host admits both forms. Questions are values, never filenames.

```sql
SELECT position, answer, status FROM thinkthen_choose(
  'SELECT body AS record FROM documents ORDER BY id',
  'Which team owns this?',
  '{"options":["billing","shipping","account"]}', '[]');

SELECT position, answer, status FROM thinkthen_tag(
  'SELECT body AS record FROM documents ORDER BY id',
  'Which topics apply?',
  '{"labels":["refund","delivery","login"]}', '[]');

SELECT position, answer, status FROM thinkthen_score(
  'SELECT body AS record FROM documents ORDER BY id',
  'How urgent is this?',
  '{"levels":["low","medium","high"]}', '[]');

SELECT position, answer, status FROM thinkthen_filter(
  'SELECT json_object(''id'', id, ''body'', body) AS record FROM documents ORDER BY id',
  'Does the message request a refund?',
  '{"on":"/body"}', '[]') WHERE kept = 1;

SELECT position, answer, status FROM thinkthen_rank(
  'SELECT body AS record FROM documents ORDER BY id',
  'How urgently does this need a reply?',
  '{"top":10}', '[]');

SELECT position, answer, status FROM thinkthen_find(
  'SELECT body AS record FROM documents ORDER BY id',
  'When is the meeting?');

SELECT position, answer, status FROM thinkthen_annotate(
  'SELECT body AS record FROM documents ORDER BY id',
  '{"type":"inline","value":{"questions":{"refund":{"decide":"Does the message request a refund?"}}}}',
  '{}', '[]');

SELECT position, answer, status FROM thinkthen_recognize(
  'SELECT body AS record FROM documents ORDER BY id',
  '{"type":"inline","value":["person","organization"]}', '{}', '[]');

SELECT position, answer, status FROM thinkthen_relate(
  'SELECT body AS record FROM documents ORDER BY id',
  '{"type":"inline","value":[{"name":"works_for","source":"person","target":"organization"}]}',
  '{}', '[]');
```

## Read results and failures

All ten functions expose these columns:

| Column | SQLite value |
| --- | --- |
| `position` | INTEGER, the original one-based input ordinal across source chunks |
| `status` | TEXT from the shared row |
| `answer` | TEXT containing a present JSON answer; JSON null is the text `null`, while an absent answer is SQL NULL |
| `p` | REAL when the shared row supplies a probability, otherwise SQL NULL |
| `error` | TEXT containing the shared error JSON, otherwise SQL NULL |
| `detail` | Unparsed JSON TEXT from the shared row, otherwise SQL NULL |
| `kept` | INTEGER `0` or `1` from the shared selection result |

Answer shapes and probability rules remain the [shared row contract](https://github.com/botassembly/thinkthen/blob/v0.2.0/schemas/row.schema.json). Offsets count Unicode scalar values in answers and detail. Use SQLite JSON functions to extract fields when needed. Detail is not decoded merely to read another column.

The engine emits rank results best first with their original positions. Other functions retain source order. SQL result order requires an outer `ORDER BY` when an application depends on it; ordering by `position` restores source order. Rank without `top` retains its inputs in the engine. The extension adds no second rank buffer.

Filter returns every record's outcome and its `kept` flag. `WHERE kept = 1` selects matches. Failed rows have `kept = 0`; use `WHERE kept = 1 OR status <> 'answered'` to retain unsure and failed outcomes as well.

Request admission failures abort the SQLite statement with `SQLITE_ERROR`. The diagnostic contains the engine's dotted code and unchanged message, formatted as `code: message`. Failed execution rows retain their error objects alongside good rows. After draining, a final call failure aborts the statement with the same diagnostic form. A caller that already received rows must still check the final SQLite step result. Failure never becomes an unsure answer or SQL NULL.

SQLite source preparation, binding, stepping, type and allocation failures remain native SQLite errors after call cleanup. Copy the original diagnostic and retain its result code before finalizing a source statement, because cleanup can replace the connection's last error. Report allocation failure as `SQLITE_NOMEM`. SQLite owns published result bytes or receives a copy; neither a result nor a diagnostic borrows freed call or cursor storage. They do not acquire invented engine codes. An outer filter or `LIMIT` does not change which evidence the engine has already admitted or sent. Restrict the source query to bound the work. Closing or resetting the outer cursor cancels unread work; it cannot undo completed backend requests.

## Cancel and release

Call `sqlite3_interrupt(connection)` through the host application's normal cancellation API. The extension polls `sqlite3_is_interrupted` on the SQLite execution thread while waiting for host events and while fetching input. The shared bridge uses bounded event waits so a held reply cannot hide an interrupt. On interruption the extension cancels the shared call, finalizes its source cursor and returns `SQLITE_INTERRUPT`. It does not clear or latch SQLite's flag: SQLite clears the interrupt after all running statements finish, and an interrupt with no running statement does not cancel a future invocation. The interrupt affects running statements on that connection according to SQLite's native rules. Keep the connection alive until the interrupt call returns.

All SQLite operations stay on the SQLite execution thread. Only owned record bytes reach the engine worker. Cursor close, query errors and statement abandonment release the cursor's call without joining a backend worker. No process signal handler or detached SQL worker is installed.

## Native implementation boundary and proof

The wrapper uses C11, SQLite's `sqlite3ext.h` dispatch table, and Jansson for host JSON translation. It statically links `thinkthen-ffi` and consumes the common SQL C bridge owned there. That bridge wraps `thinkthen-sql`; this extension adds no Rust crate with unsafe code, second SQL host or copy of the engine's validators. Jansson decodes source parameter arrays and the bridge's fixed column envelope. Answers and errors remain raw JSON text, so arbitrary JSON keys and values retain the engine's encoding. Request and record admission still run in Rust. The package contains one extension library with its native engine and Jansson linked in. Users need neither Rust nor a compiler to load the package.

One eponymous-only virtual-table module implementation registers the ten function names. Its four hidden columns carry invocation arguments. The planner requires usable equality constraints for the two required arguments and every supplied optional argument and leaves output filters, ordering and limits to SQLite. A temporarily unusable required constraint returns `SQLITE_CONSTRAINT` so the planner can choose another join order; an absent argument returns a diagnostic and `SQLITE_ERROR`. Hidden columns have no coercing affinity and expose their invocation values if queried. Each `xFilter` invocation owns its client snapshot, prepared source, current chunk, current result and host call. Repeating `xFilter` cleans up the previous scan first. `xNext` advances this state, `xColumn` exposes the current result, and `xClose` releases it. Registration uses `SQLITE_VTAB_DIRECTONLY`; configuration uses `SQLITE_DIRECTONLY` without a deterministic flag. Connection state is shared by the registered modules and configuration function through explicit ownership; replacing a client or destroying one registration cannot free another cursor's snapshot. The entry point returns `SQLITE_OK_LOAD_PERMANENTLY` after successful registration and does not call `sqlite3_auto_extension`. A registration failure removes registrations installed by that attempt and frees its state before returning an error, so SQLite cannot unload code that still has registered callbacks.

The installed checks exercise same-connection source reentry, two active cursors, scan restarts, configuration replacement, source cleanup and interruption during held replies. They verify native source and argument errors, retained diagnostics, JSON null versus an absent answer, and connection reuse after interruption. Shared construction and conformance cases run through the installed extension. A lazy source with an outer limit checks bounded input staging and cursor abandonment. Platform package checks run on Linux x86-64 and macOS arm64 using both SQLite 3.41.0 and the pinned current SQLite host. The shared pre-call cancellation fixture maps to a native held-statement interrupt: SQLite deliberately treats an interrupt with no running statement as a no-op.

The source-query and shared-host shape follows the SQL host design.

SQLite documents [table-valued virtual tables](https://sqlite.org/vtab.html#tabfunc2), [extension loading and permanent library lifetime](https://sqlite.org/loadext.html), [interrupt behavior](https://sqlite.org/c3ref/interrupt.html), [source binding](https://sqlite.org/c3ref/bind_blob.html), [read-only statement limitations](https://sqlite.org/c3ref/stmt_readonly.html), [required virtual-table arguments](https://sqlite.org/vtab.html#enforcing_required_parameters_on_table_valued_functions), [finalization](https://sqlite.org/c3ref/finalize.html), and [diagnostic lifetime](https://sqlite.org/c3ref/errcode.html). Its [3.41.0 release notes](https://sqlite.org/releaselog/3_41_0.html) introduce the interrupt inspection API this extension requires.

The public contract defines stable error codes, client settings and execution identity. Shared Rust diagnostics escape control characters before host translation, including NUL as `\u0000`. SQL settings reject `url` and `key_source` with `usage.settings`.
