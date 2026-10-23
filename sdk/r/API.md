# R public API and packaging

The R package provides a data-first API for character vectors, data frames and lists. R wraps the shared Rust client and pull call directly. It adds host value conversion and native conditions; it adds no engine rules.

## Functions

Arguments after `...` must be named. The functions accept no additional options in `...` and report ordinary R unused-argument errors.

```r
tt_client(adapter = NULL, ..., mode = NULL, url = NULL,
          key_source = NULL, model = NULL, batch = NULL)
tt_question(toolbox, name = NULL)
tt_reference(name)
tt_detail(rows)
tt_answer(rows)
tt_bind(data, rows, prefix)

tt_decide(x, question, ..., threshold = NULL, on = NULL, client = NULL)
tt_choose(x, question, options = NULL, ..., threshold = NULL, on = NULL, client = NULL)
tt_tag(x, question, labels = NULL, ..., threshold = NULL, on = NULL, client = NULL)
tt_score(x, question, levels = NULL, ..., on = NULL, client = NULL)
tt_filter(x, question, ..., threshold = NULL, on = NULL, client = NULL)
tt_rank(x, question, ..., levels = NULL, top = NULL, on = NULL, client = NULL)
tt_find(x, question, ..., threshold = NULL, on = NULL, client = NULL)
tt_annotate(x, toolbox, ..., on = NULL, client = NULL)
tt_recognize(x, kinds = character(), ..., threshold = NULL, on = NULL, client = NULL)
tt_relate(x, rules, ..., threshold = NULL, on = NULL, client = NULL)
```

The `tt_` prefix avoids collisions with base R and table functions. Data comes first for R's native pipe. One named function handles a single record and a collection. Filter returns selected originals in the input's R shape. Other functions collect pulled rows into an ordinary data frame. There are no public call sessions, polling methods or stream variants in this API.

`tt_client` admits configuration once and returns an opaque client handle. `adapter` passes a plain name to the library's adapter setting; R contains no closed selector list or provider defaults. `key_source` names a credential environment variable rather than carrying a credential. NULL client settings are omitted so Rust applies its defaults. A NULL `client` in a function selects the lazily created shared client. Per-call arguments contain no mode, backend, address or credential setting. Proxy mode remains an admitted client choice whose execution currently fails in Rust; the binding implements no proxy transport.

A question string maps to an inline tagged question, including text containing a hash or ending in `.json`. Kinds and relation rules map to the corresponding inline array; annotate maps its toolbox list to the inline toolbox object. `tt_question` supplies a toolbox tagged question with an optional name. `tt_reference` supplies a reference tagged question. These helpers only express the caller's chosen representation. They do not validate, load files or execute references. They can occupy a function's question, kinds, rules or toolbox argument. Rust rejects incompatible forms and currently rejects reference execution.

Thresholds use numeric values or band strings; options, labels, levels and kinds use character vectors. Relation rules use lists of named lists. NULL question options are omitted so Rust applies toolbox inheritance and defaults. The binding does not parse bands, check duplicates or validate function-compatible options. It builds the tagged request once and Rust admits it once.

## Data frame helpers

`tt_answer(rows)` extracts the typed answer vector or list from a complete result frame. Unsure or failed absent scalar answers remain typed NA; absent nested answers become NA list elements. Successfully answered null remains NULL, and empty nested collections remain empty. Ordinary input frames and filter output are rejected with ordinary R errors. Rank answers stay in rank order.

`tt_bind(data, rows, prefix)` attaches answer, probability, status, error and raw detail columns to a copy of a base data frame or an ungrouped tibble. The new names are `<prefix>_answer`, `<prefix>_p`, `<prefix>_status`, `<prefix>_error` and `<prefix>_detail`. `prefix` must be one nonempty nonmissing string. A collision, a row count mismatch, rank output or filter output raises an ordinary R error. Rows are never recycled or joined on IDs. The caller supplies the same unchanged input rows that produced the result. The frame's original names, row names, classes, columns and attributes remain intact. Empty frames receive typed empty columns. Neither helper makes an engine call or reinterprets engine outcomes. The [table examples](README.md#data-frame-expressions) show base R, tibble, dplyr and data.table use.

Result frames have a private producer attribute for the helper admission check. A filter selection clears that private result marker while retaining the original input's ordinary attributes. The package still imports only jsonlite; table packages are optional caller and test dependencies.

## Input and operation

Character vectors provide raw UTF-8 text records, with no JSON quoting or wrapping, including strings that look like JSON. Data frames provide one JSON object per row. Lists provide one JSON record per element. Only structured values are serialized as JSON. A missing atomic record supplies JSON null and keeps its input position; it is not dropped or replaced by an empty string. `NULL` supplies an empty collection; `list(NULL)` supplies one JSON null record.

Atomic fields of length one map to JSON scalars; longer or empty vectors map to arrays. Unnamed lists remain arrays even with one element, and named lists become objects. Nested one-element arrays stay arrays. Empty arrays, empty objects and null remain distinct. Standard factor and date fields follow jsonlite's documented class mappings, using factor labels and full available numeric precision. Unsupported objects raise R serialization errors. No engine admission rule is checked in R. The binding converts R text to UTF-8; the library owns invalid Unicode and evidence errors.

For each function call, the client opens one shared `PullCall` from its admitted request. The host selects `OffsetUnit::CodePoints`. It services the call's input requests on R's thread, sends text or structured JSON through the corresponding shared operation, ends input, and drains rows. Input and output remain bounded by the shared call. R retains the original collection for filter and collects result rows for the other functions. Ranking retains the engine's existing behavior. There is no second worker, private queue protocol or callback into R from an engine thread.

Every accepted input uses the shared completion policy, including one-element vectors and empty inputs. The binding drains all rows, then translates the call's final error and Rust error family into an R condition. A failed row is a final failure. An answered annotate row retains failed named entries in its answer. The binding neither scans rows to create that failure nor reimplements filter selection.

## Results

Except for filter, the same data-frame columns appear for empty and nonempty results:

| Column | R value |
| --- | --- |
| `id` | List of JSON values |
| `status` | Character vector |
| `answer` | Function-specific vector or list column below |
| `p` | Numeric vector, `NA_real_` when absent |
| `error` | List of NULL or a named list with `code` and `message` |
| `detail` | Character vector of raw JSON, `NA_character_` when absent |

| Function | `answer` column |
| --- | --- |
| decide | Logical vector |
| choose | Character vector |
| score | Numeric vector |
| rank | Integer vector |
| tag | List of character vectors |
| find | List of NULL or named span lists |
| annotate | List of named outcome lists |
| recognize | List of lists of named span lists |
| relate | List of lists of named relation lists |

Scalar null answers use typed NA. Nested nulls use NULL. Empty collections stay empty collections. Status preserves the difference between unsure, failed and answered-null. Results use ordinary R copy semantics, with no engine-backed mutable field. The binding parses the shared row's `id`, `status`, `answer`, `p` and `error`, and keeps the separately supplied detail text unparsed. `tt_detail(rows)` decodes only that column on request into a list of ordinary JSON values, without simplification or generated classes. A raw JSON `null` remains distinct from an absent detail in the raw column. The helper returns data; it modifies neither the frame nor the engine.

Span positions follow the code-point convention supplied by the shared call: zero-based Unicode scalar positions with an exclusive end. This applies consistently to spans in answers, nested annotate outcomes and detail. R performs no manual offset conversion and has no separate detail offset convention. Use `substr(text, span$start + 1L, span$end)` to extract a span. A native R one-based inclusive boundary would need a shared Rust capability before a binding could offer it; this proposal does not introduce one in R.

Filter uses the shared row's `position` and `kept` values to subset the original collection. It does not compare IDs, inspect answers or evaluate thresholds. Character vectors retain their names and attributes; data frames retain row names and column classes; lists retain their names and elements. Empty selections preserve the input kind. Duplicate or structured IDs cannot change which original is selected. NULL input returns NULL. The call's final error prevents a partial selection from being returned as success.

R conditions carry `code` and the library's message. The error-family name supplied by Rust selects the corresponding `thinkthen_*_error` condition class, under `thinkthen_error`, `error` and `condition`. The binding never parses a code prefix. Interruption signals the shared cancel handle, drops the call and propagates R's ordinary interrupt condition. Dropping does not join the engine worker or wait for the backend reply. Only R's main thread reads R objects, builds result objects and checks R interrupts between shared-call polls. The engine owns work that was already sent and its attempt timeout. No process-wide signal handler is installed.

## Package and proof

The package lives at `sdk/r/`. DESCRIPTION, NAMESPACE, Rd help and the small named R wrappers are handwritten. One private extendr bridge wraps the public Rust client and pull call directly. It provides host conversion, resource ownership and interrupt integration. It contains no engine implementation or C ABI dependency. Registration glue holds no behavior. No generated wrapper or result source is committed. Namespace detachment does not unload the DLL: the package supplies no unload hook and never calls `library.dynam.unload` or `dyn.unload`. Its native code remains loaded until process exit, because cancelled shared workers may still be completing. Explicit manual DLL unloading is outside this lifetime contract.

Runtime R dependencies remain limited to jsonlite. Extendr is the Rust bridge dependency, not a package the R caller must attach. Source packaging places engine source and locked Rust dependencies in a temporary package build tree and vendors dependencies for offline source installation. R headers, the compiler, make, the pinned toolchain and R dependencies must already be installed. The native library is linked into the package artifact. Package numbers remain 0.x and the release milestone sets the published number. There is no version negotiation, compatibility layer or separate native library installation. Publishing is outside this ticket.

The installed package's base R tests use one small driver over the shared construction and conformance cases through the public named functions. Expected filter answers determine the expected selected originals; no second public judgement mode exists just to reproduce fixture JSON. Other rows compare after the documented R representation changes, decoding detail only when requested. Tests do not reproduce stub arithmetic. Surface cases cover R value conversion, typed columns, original filter identity and attributes, collection error mapping for zero, one and several records, Unicode extraction with consistent answer/detail offsets, lazy detail decoding and interrupting a held loopback request. A backend event supplies readiness; there is no fixed sleep or short assertion timeout. Preparation and checks use the existing job gate and installed artifact. Live calls remain the PM's separate check.
