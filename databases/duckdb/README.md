# ThinkThen for DuckDB

Run the ten functions over a query's records.

Load the local extension into exactly DuckDB 1.4.3. DuckDB rejects this C++ extension in other releases. Set your backend key in the process environment before starting DuckDB. The query contains no credential. The package is unsigned, so start the official DuckDB command with `-unsigned`, or open a Python connection with `duckdb.connect(config={"allow_unsigned_extensions": "true"})`.

Use the extracted extension's absolute path in `LOAD`; hardened macOS hosts reject relative paths.

```sql
LOAD '/absolute/path/to/thinkthen.duckdb_extension';
SET thinkthen_settings = '{"adapter":"stub","batch":16}';

CREATE TABLE documents(id INTEGER, body VARCHAR);
INSERT INTO documents VALUES
    (1, 'Please refund this invoice.'),
    (2, 'The replacement arrived today.');

SELECT position, status, answer, p
FROM thinkthen_decide(
    (SELECT body AS record FROM documents ORDER BY id),
    'Does the message request a refund?');
```

For the default backend, use `SET thinkthen_settings = '{}'`. Settings are the existing [client settings](https://github.com/botassembly/thinkthen/blob/v0.2.0/schemas/client.schema.json). SQL settings cannot contain `url` or `key_source`, even with a null value. Backend names select process-owned address and credential sources; SQL cannot redirect a server credential. The setting is session-scoped; its callback rejects `SET GLOBAL thinkthen_settings` before changing either scope. Each connection retains its own immutable client. A session mutex protects client creation, replacement and snapshots; calls release that mutex before execution. A settings change admits a replacement client before replacing the old one. Each execution snapshots the connection's current client when opening its call, including each execution of a prepared statement. Preparing a statement does not retain a client for later executions. Later settings changes do not change an active call. An unchanged setting reuses the client. The library resolves backend names, addresses, limits and key sources.

## Questions and records

Every function has the form `thinkthen_NAME(input_table, question, options := '{}')`. Plain question text is inline. Text whose first non-whitespace character is `{` carries the existing tagged question JSON; this shared SQL shorthand also serves the other databases. Options are optional and default to `{}`. The input subquery returns exactly one column named `record`, with type VARCHAR or JSON. VARCHAR supplies raw UTF-8 text; JSON supplies structured evidence. SQL NULL source cells raise a native input error. Filter missing records or replace them explicitly in the source query. JSON null remains a JSON record. Question and options are invocation constants, including prepared-statement parameters. Binding accepts constant expressions and resolves prepared parameters before execution; correlated or per-row questions and options are not accepted. Their JSON follows the existing [call schema](https://github.com/botassembly/thinkthen/blob/v0.2.0/schemas/request.schema.json). Malformed object text raises an admission error. A literal question beginning with `{` uses an explicit tagged inline value, such as `{"type":"inline","value":"{literal question?"}`. No argument is a filename, and no settings belong in a call.

```sql
SELECT position, answer FROM thinkthen_choose(
    (SELECT body AS record FROM documents ORDER BY id),
    'What is the message about?',
    options := '{"options":["billing","shipping","other"]}');

SELECT position, answer FROM thinkthen_tag(
    (SELECT body AS record FROM documents ORDER BY id),
    'Which topics apply?',
    options := '{"labels":["billing","shipping"]}');

SELECT position, answer FROM thinkthen_score(
    (SELECT body AS record FROM documents ORDER BY id),
    'How urgent is the message?',
    options := '{"levels":["not urgent","urgent"]}');

SELECT position, answer FROM thinkthen_filter(
    (SELECT body AS record FROM documents ORDER BY id),
    'Does the message request a refund?', options := '{}')
WHERE kept OR status <> 'answered';

SELECT position, answer FROM thinkthen_rank(
    (SELECT body AS record FROM documents ORDER BY id),
    'How urgent is the message?',
    options := '{"levels":["not urgent","urgent"],"top":10}');

SELECT position, answer FROM thinkthen_find(
    (SELECT body AS record FROM documents ORDER BY id),
    'Which line requests a refund?', options := '{}');

SELECT position, answer FROM thinkthen_annotate(
    (SELECT body AS record FROM documents ORDER BY id),
    '{"type":"toolbox","value":{"questions":{"refund":{"decide":"Does the message request a refund?"}}}}', options := '{}');

SELECT position, answer FROM thinkthen_recognize(
    (SELECT body AS record FROM documents ORDER BY id),
    '{"type":"inline","value":["person","organization"]}', options := '{}');

SELECT position, answer FROM thinkthen_relate(
    (SELECT body AS record FROM documents ORDER BY id),
    '{"type":"inline","value":[{"name":"works_for","source":"person","target":"organization"}]}', options := '{}');
```

Use JSON evidence to retain a caller identifier and choose evidence through the existing options:

```sql
SELECT position, answer FROM thinkthen_decide(
    (SELECT json_object('id', id, 'body', body) AS record FROM documents ORDER BY id),
    'Does the message request a refund?',
    options := '{"on":"/body"}');
```

The library interprets structured records, question definitions, thresholds and options. The extension only assembles the named function's request and translates native values.

## Result columns and lifetime

| Column | DuckDB type | Meaning |
| --- | --- | --- |
| position | UBIGINT | Original one-based input ordinal across every input chunk |
| status | VARCHAR | Shared row status |
| answer | JSON | Shared function answer, including JSON null for unsure |
| p | DOUBLE | Shared top-level probability, or SQL NULL when absent |
| error | JSON | Shared row error, or SQL NULL when absent |
| detail | JSON | Unmodified detail JSON, or SQL NULL when absent |
| kept | BOOLEAN | Shared selection result |

The [row schema](https://github.com/botassembly/thinkthen/blob/v0.2.0/schemas/row.schema.json) defines answers, statuses and errors. No probability or score is recalculated. Detail remains raw JSON until DuckDB or the caller examines it. Text offsets count Unicode scalar values in both answers and detail, matching the SQL host.

The input's order defines positions. Explicit source ordering makes them reproducible. Rank returns best first while retaining original input positions. Other functions retain input order. Filter returns one outcome per input record with `kept` indicating selection. Use `WHERE kept` for matches. Failed rows have `kept = false`; the example retains them and unsure rows with `WHERE kept OR status <> 'answered'`. SQL owns any outer sorting. A caller that needs original source columns retains its source relation and joins by a reproducible ordinal; the extension neither projects nor retains another copy of those records.

Invalid settings or requests raise a native DuckDB error with the library's unchanged code and message. Execution can produce failed rows beside successful rows. A final call failure raises a native query error after draining; observed rows do not make the statement successful. A failure never becomes an unsure answer or SQL NULL. DuckDB source or conversion errors remain native DuckDB errors after call cleanup.

DuckDB interrupt cancels the shared call while it waits for the backend. Closing a result or abandoning execution also releases the statement's call without joining the backend worker. Each invocation owns its call, including separate invocations in one query. Only the connection's client survives a statement. Cancel does not undo requests already sent.

## Native execution and build boundary

The extension uses the approved SQL host design. A table-in/table-out operator receives DuckDB vectors and copies one current chunk into `thinkthen-sql::SqlCall`. One sequential bind-operator wrapper retains one call across all vectors and performs one final drain. A chunk boundary never ends input or flushes a backend batch. The ordinary DuckDB executor can still run an upstream source pipeline in parallel. The native probe and installed extension tests establish this executor boundary with the shared host.

No input columns use `project_input`, which changes DuckDB's callback and finalization behavior. No output filter or limit is pushed into source selection. Limit the input subquery to bound the records sent; an outer LIMIT is not a backend-call budget. The engine owns its bounded queues and ranking retention. The host stages only its current chunk. The DuckDB execution thread reads source vectors, probes its session interrupt flag between bounded host polls, and constructs result vectors. No DuckDB handle enters the engine worker. No process signal handler or detached SQL worker is installed.

The packages are Linux x86-64 with glibc and macOS arm64, built against the exact official DuckDB 1.4.3 headers. This uses DuckDB's internal C++ operator ABI and is not a stable C extension ABI. Packages must match DuckDB's release and platform metadata. The shared [runtime floor contract](../../scripts/runtime-floors.json) sets the Linux libc and Mac deployment targets. The installed package README and requirements list the selected platform floor. The loadable artifact uses DuckDB's C++ extension entry point and metadata footer from that pinned build; an unsigned artifact still needs valid metadata. The installed-load checks run with metadata mismatch checks enabled. A release change requires compiling again and repeating the operator proof. Extension metadata required by DuckDB does not add a ThinkThen version-negotiation API.

The native boundary follows the pinned [table-function interfaces](https://github.com/duckdb/duckdb/blob/v1.4.3/src/include/duckdb/function/table_function.hpp), [table-input execution](https://github.com/duckdb/duckdb/blob/v1.4.3/src/execution/operator/projection/physical_tableinout_function.cpp) and [client context](https://github.com/duckdb/duckdb/blob/v1.4.3/src/include/duckdb/main/client_context.hpp).

The build lives under `databases/duckdb`; it does not replace the workspace's root build. It uses a C++17 compiler, CMake, the pinned DuckDB source and existing Rust toolchain. JSON field translation uses DuckDB's bundled yyjson source from that same pinned archive; it adds no independent parser or question rules. The extension follows the [pinned official C++ extension distribution](https://github.com/duckdb/duckdb/blob/v1.4.3/CMakeLists.txt): it links DuckDB statically inside the extension and hides those symbols. This works when a Python host loads DuckDB locally or omits internal C++ exports. It does not link or ship a separate `libduckdb`. The package contains only the stripped release extension and licenses. Test programs and DuckDB runtimes stay outside it. The extension links a position-independent static build of `thinkthen-ffi` containing the shared SQL bridge and the platform libraries that Rust reports for that archive. Only DuckDB's extension entry point is exported; internal bridge and parser symbols do not become another extension's symbols. Normal linker reachability retains the bridge through its direct C++ calls, without pulling in unused foreign interfaces. Each installed package records and checks its actual operating-system and C runtime requirements; the platform names alone promise no older runtime floor. The loaded extension remains resident for the process lifetime because cancellation does not join every Rust worker. No runtime download, caller-selected native library or runtime Rust toolchain is required. Installed-package tests load the built extension from an empty staging directory on each supported platform. Release publication remains separate.

## Shared SQL C bridge

DuckDB owns implementation of the common foreign bridge in the existing `thinkthen-ffi` crate; SQLite consumes it. The crate gains a dependency on `thinkthen-sql` and a static-library build output. No other Rust crate gains unsafe code. Database client creation uses `thinkthen_sql_client_open`, which calls the shared SQL settings admission before constructing the existing client owner. Owned row/error buffers and cleanup remain the same.

The operations are SQL-call open from an existing immutable client and request JSON, send one counted array of typed text/JSON byte slices, end input, bounded poll, and free. Open snapshots admitted input. Sending a chunk copies its bytes before returning and accepts it only after the shared host requests a chunk. Poll returns wait, needs-chunk, row, successful end or an owned error. Row and error ownership use the existing C boundary types. No thread-local error storage is added.

Poll accepts a native interrupt-probe function and opaque context for that poll only. The probe runs synchronously on the database execution thread and only reads the database interrupt flag. It must never throw, unwind, longjmp or call back into the driver. The probe runs between bounded waits while a backend reply is held and between submitted records, so a blocked reply cannot hide an interrupt. Rust retains no callback or context after poll returns. The host performs the existing cancellation; the wrapper raises its database's native interrupt error after returning through the boundary and releasing the call. Free immediately drops and cancels the call without joining. One driver owns each call, and no operation races free. No database pointer enters a worker. Shared Rust parsing, settings, admission, rows, batching, cancellation and errors remain authoritative.

`thinkthen_row_columns` provides one owned JSON envelope for native parsers with narrower payload limits. It retains `status` and optional `p` as scalars and carries each present `id`, `answer` or `error` as a JSON string containing its original JSON bytes. An absent field remains absent; a present JSON null answer becomes the string `"null"`. The projection preserves `id` without interpreting its value and leaves the original row, raw detail, position and kept unchanged. Its buffer survives the row and uses `thinkthen_buffer_free`. This translation adds no row or answer rules. DuckDB's counted yyjson parser reads the original row directly; SQLite uses the projection to preserve payload keys and integers its native JSON parser cannot represent.

## Verification scope

The manual [all-ten.sql](examples/all-ten.sql) file calls every function on the default backend. Place it beside `all-ten.py` and run `python3 all-ten.py /path/to/installed/package`; the runner uses the official DuckDB 1.4.3 command named `duckdb` on PATH. The extension package contains no executable or DuckDB runtime. `--stub` runs the same SQL without a service. The file contains one complete statement per line, and the runner reports each result and rejects failed or missing rows.

The installed-package checks load the delivered extension in the official DuckDB 1.4.3 command and Python wheel on both supported platforms and run every shared stub case. A separate native test consumer runs the conformance and lifecycle cases through SQL. Native cases cover raw text versus JSON, prepared arguments, empty input, multiple vector boundaries and final drain, ordered positions, independent connection settings and invocations, rejected global settings, prepared-statement execution after a settings change, row/error ownership, coded admission and unavailable-backend failures, interrupt of a held shared loopback request, and abandoned streaming execution followed by connection/database teardown. Held-request interruption separately checks cleanup while a worker waits for the backend. Installed loads prove static-archive linkage and required runtime dependencies on both platforms. Tests use the existing stub and loopback fixture with isolated environments, no live credentials and no external network. Coverage separates the C++ wrapper from the Rust host and includes the shared bridge through SQL. The PM runs the manual default-backend check of all ten functions after fresh code review, within the ticket's call limit.

The pinned native sources define the [constant table-argument binding](https://raw.githubusercontent.com/duckdb/duckdb/v1.4.3/src/planner/binder/tableref/bind_table_function.cpp), [extension-setting scope callback](https://raw.githubusercontent.com/duckdb/duckdb/v1.4.3/src/execution/operator/helper/physical_set.cpp), [connection-owned state](https://raw.githubusercontent.com/duckdb/duckdb/v1.4.3/src/include/duckdb/main/client_context_state.hpp) and [C++ extension loading and metadata checks](https://raw.githubusercontent.com/duckdb/duckdb/v1.4.3/src/main/extension/extension_load.cpp) used by the extension.

The public contract defines stable error codes, client settings and execution identity. Shared Rust diagnostics escape control characters before host translation, including NUL as `\u0000`. SQL settings reject `url` and `key_source` with `usage.settings`.
