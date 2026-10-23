# ThinkThen for R

Ask questions about a character vector or a data frame. Filter returns the selected input records. The other functions return ordinary data frames. The Rust library owns batching, thresholds, retries and failure handling.

```r
library(thinkthen)

messages <- data.frame(
  id = c("a", "b"),
  body = c("Please refund my order.", "Where is my delivery?")
)

refunds <- tt_decide(
  messages,
  "Does the customer ask for money back?",
  on = "/body",
  threshold = "0.2:0.8"
)
refunds[c("status", "answer", "p")]
```

Set the selected backend's key in your shell before creating a client. Omit `client` to use the package's shared default client, which is created on first use. Create a client to select another named backend or reuse explicit settings:

```r
client <- tt_client(adapter = "openai", batch = 16)
answers <- tt_choose(
  c("Please refund my order.", "Where is my delivery?"),
  "Which team should handle this message?",
  options = c("billing", "shipping"),
  client = client
)
```

A client admits its settings once and retains its engine and connections. Each function then opens a separate call with its question, question options and records. Backend names are ordinary strings resolved by the Rust library's backend table. The R package contains no backend list. See the [backend documentation](../../README.md#backends).

## Data frame expressions

Use the result helpers to keep answers beside your original data. Existing functions retain the result frames described below.

Use `tt_answer(rows)` when a table expression needs one answer per row. Use `tt_bind(data, rows, prefix)` when the table should retain the answer and its outcome together. Both helpers consume a completed result and make no engine call.

```r
library(thinkthen)

messages <- data.frame(
  id = c("same", "same", "third"),
  body = c("Please refund my order.", "Where is my delivery?", NA_character_),
  row.names = c("order A", "order B", "order C")
)
client <- tt_client(adapter = "stub")

refund <- tt_decide(messages$body, "Is a refund requested?", client = client)
with_refund <- tt_bind(messages, refund, prefix = "refund")
with_refund[c("refund_answer", "refund_p", "refund_status", "refund_error")]

# Base R: one call for the whole column.
messages$refund <- tt_answer(tt_decide(
  messages$body, "Is a refund requested?", client = client
))

# Optional dplyr and tibble packages.
with_team <- tibble::as_tibble(messages) |>
  dplyr::mutate(team = tt_answer(tt_choose(
    body, "Which team should handle this?",
    options = c("billing", "shipping"), client = client
  )))

# Optional data.table package; the assignment follows data.table semantics.
DT <- data.table::as.data.table(messages)
DT[, urgency := tt_answer(tt_score(
  body, "How urgent is this?", levels = c("routine", "urgent"), client = client
))]
```

Each `tt_*` call above opens one engine call for its whole input column. The engine decides its backend batches. Do not use `rowwise()`, `sapply()` or a loop to call once per row. A grouped `mutate()` or a `data.table` expression with `by` evaluates the expression for each group, so it makes one engine call per group. Ungroup first when the intended input is the whole column. These evaluation rules follow [dplyr](https://dplyr.tidyverse.org/reference/mutate.html) and [data.table](https://rdatatable.gitlab.io/data.table/reference/assign.html).

The helper signatures are:

```r
tt_answer(rows)
tt_bind(data, rows, prefix)
```

`tt_answer` accepts a complete ThinkThen result frame from any function except filter. It rejects ordinary input frames and filter output with an ordinary R error, even when they contain an unrelated column named `answer`. Rank answers remain in rank order. `tt_bind` requires exactly one result row per input row and never recycles results. A missing or `NA` prefix raises an ordinary R error.

`tt_answer` returns a logical vector for decide, a character vector for choose, a numeric vector for score, and a list for tag, find, annotate, recognize and relate. The output length equals the completed result's row count. An absent scalar answer is typed `NA`. In the returned list, an absent answer is an `NA` element; a successfully answered JSON null remains `NULL`. Empty tag or entity answers remain empty collections. Named failures inside an annotate answer remain in that answer. No nested outcome is flattened or summarized. The existing result frame retains its documented answer representation. Extracting an answer never replaces the original result frame; retain that frame to inspect `status`, `error`, `p` or `tt_detail(rows)`.

`tt_bind` appends five columns, named `<prefix>_answer`, `<prefix>_p`, `<prefix>_status`, `<prefix>_error` and `<prefix>_detail`. The answer column uses `tt_answer`'s representation. Probability and status come straight from the result; a missing probability stays `NA_real_`. Error stays a list column. Detail stays raw JSON text, and `tt_detail` remains the explicit decoder for a result frame. These columns keep failed rows visible. An unsure answer is `NA`, and its status remains `"unsure"`. A failed row has its original `"failed"` status and its engine error. The helpers never infer a failure, probability or kept decision.

The helper accepts a base data frame or an ungrouped tibble and a complete result from one of the eight functions that retain input order. It appends columns to a copy and returns the same frame kind. It preserves original columns, their classes and attributes, original column names, row order, row names and frame attributes. It neither mutates a data.table by reference nor promises support for arbitrary data-frame subclasses. Use the vector expression above for data.table. A zero-row input produces the same zero-row frame with correctly typed empty outcome columns.

`prefix` is one nonempty string. Generated column names are literal, so `prefix = "refund check"` produces names such as `refund check_answer`. Any generated name already present in the input raises an ordinary R error before the helper changes the frame. It does not overwrite or silently repair names. An input may have duplicate or nonsyntactic column names; their names and order stay unchanged. These host shape checks follow the same explicit collision principle as [tibble column addition](https://tibble.tidyverse.org/reference/add_column.html).

The caller must pass the same unchanged rows used to obtain the result. `tt_bind` checks the row count and the result's producer before attachment. It never matches IDs, row names or an index column. Duplicate IDs, duplicate index-column values and nonstandard row names therefore do not redirect an answer. Base data frames require unique row names; when callers repeat a row index, R supplies repaired row names according to its [subsetting rules](https://stat.ethz.ch/R-manual/R-devel/library/base/html/Extract.data.frame.html). The helper preserves those resulting names. Editing, filtering or rearranging either frame between the call and attachment is the caller's responsibility.

Rank and filter retain their existing APIs. `tt_rank` returns its ordered result frame, and `top` can shorten it. `tt_answer` can extract those integer ranks in result order, but those ranks must not be assigned to an unreordered input column. `tt_bind` rejects rank results and filter output with an ordinary R error. Result frames carry a small private producer attribute for that host check; no engine rule or public row column changes. Filter returns the original selected rows and retains their names and classes. Its call failure still raises, so it never returns a partial selection. To retain every judgement with a status column, use decide and attach its result.

All ten functions remain available in ordinary R table code:

```r
text <- messages$body
questions <- list(questions = list(
  refund = list(decide = "Is a refund requested?"),
  team = list(choose = "Which team should handle this?",
              options = c("billing", "shipping"))
))
relations <- list(list(name = "works_for", source = "person", target = "company"))

answers <- list(
  decide = tt_decide(text, "Is a refund requested?", client = client),
  choose = tt_choose(text, "Which team should handle this?",
                     options = c("billing", "shipping"), client = client),
  tag = tt_tag(text, "Which topics occur?", labels = c("billing", "delivery"), client = client),
  score = tt_score(text, "How urgent is this?", levels = c("routine", "urgent"), client = client),
  find = tt_find(text, "When does the delivery arrive?", client = client),
  annotate = tt_annotate(text, questions, client = client),
  recognize = tt_recognize(text, kinds = c("person", "company"), client = client),
  relate = tt_relate(text, rules = relations, client = client)
)
with_tags <- tt_bind(messages, answers$tag, prefix = "topics")
with_tags$topics_answer[[1]]
complete <- messages[!is.na(messages$body), , drop = FALSE]
selected <- tt_filter(complete, "Is a refund requested?", on = "/body", client = client)
ranked <- tt_rank(complete$body, "How urgent is this?", levels = c("routine", "urgent"),
                  top = 2, client = client)
```

Use a column as `x` for plain text evidence, or the frame with `on = "/body"` for structured records. Existing serialization of dates, factors and nested list columns remains unchanged. Client configuration, tagged questions, thresholds, shared admission errors, row errors, raw detail and interrupts keep the contracts below. A final call failure raises before either helper receives a result. No table helper changes cancellation or performs a second call.

The package continues to import only `jsonlite`. `tibble`, `dplyr` and `data.table` are optional caller tools; they become test dependencies for their examples and are not attached or loaded by ThinkThen. There is no tidy evaluation layer, table backend, formula interface or per-function vector wrapper. Result frames retain private producer metadata for the helpers; their public columns stay unchanged.

## Vectors and tables

A character vector supplies one text record per element. Text stays text even when it looks like JSON. The binding sends present text as raw UTF-8 without JSON wrapping. A data frame supplies one JSON object per row, including list columns for nested JSON values. A list supplies one JSON record per element. Only structured records use JSON serialization. An empty vector, list or data frame supplies no records.

A record's `id` field passes through unchanged; otherwise the library supplies its input position. Vector names and data-frame row names do not become IDs. Missing values become JSON null and retain their position. The library decides whether a record can supply evidence. `on` selects evidence in a structured record with a JSON Pointer.

Except for filter, the result is a base R data frame, including for a one-record input. Scalar answers use logical, character or numeric columns. Nested answers use a list column. `id` and `error` are list columns, and `status` is a character column. An absent scalar answer or probability is typed `NA`; an absent nested value is `NULL` in its list column. Inspect `status` to distinguish unsure and failed rows.

```r
text <- c("Ada works for Acme.", "The delivery arrives Friday.")

tags <- tt_tag(text, "Which topics occur?", labels = c("employment", "delivery"))
urgency <- tt_score(text, "How urgent is this?", levels = c("routine", "urgent"))
delivery <- tt_find(text, "When does the delivery arrive?")
names <- tt_recognize(text, kinds = c("person", "company"))
edges <- tt_relate(text, rules = list(
  list(name = "works_for", source = "person", target = "company")
))
names$answer[[1]]
```

`tt_filter` returns the original records selected by the shared Rust call. A character vector returns a character vector, a data frame returns selected rows with all columns, and a list returns a list. Names, row names and column classes stay intact. Selection uses input positions rather than IDs, so repeated IDs cannot select the wrong originals. The Rust call owns kept decisions and the final failure. Filter returns no partial selection when that call fails. Use `tt_decide` when you want judgement rows for every record. `tt_rank` returns rows in the engine's rank order; `top` limits the retained results.

The `detail` column holds the library's raw JSON text. Decode it only when needed:

```r
details <- tt_detail(refunds)
details[[1]]$source
```

`tt_detail` returns a list of ordinary R JSON values. It does not change the result frame. An absent detail is `NA_character_` in the raw column and `NULL` in the decoded list. The row contract defines the answer and detail fields.

## Questions and toolboxes

A character question is always inline text. It is never interpreted as a filename. Use a named R list for an inline annotate toolbox:

```r
questions <- list(questions = list(
  refund = list(decide = "Does the customer ask for money back?"),
  topic = list(choose = "What is the message about?",
               options = c("billing", "shipping", "other"))
))
annotated <- tt_annotate(messages, questions, on = "/body")
refund <- tt_question(questions, name = "refund")
rows <- tt_decide(messages, refund, on = "/body")
```

`tt_question` explicitly selects a toolbox question. `tt_reference("refund")` explicitly names a reference. These forms map to the shared contract's tagged questions; the Rust library admits them. Reference execution and proxy mode currently return the library's unsupported-operation error. The proxy transport is later work. To read a toolbox file, load its JSON into an R list first; a call never opens an arbitrary filename supplied as question text.

## Errors and interrupts

A failure reported by the shared call raises a `thinkthen_error` condition with the library's `code` and message. It also inherits the condition class for the error family supplied by Rust, such as `thinkthen_backend_error`, plus R's `error` and `condition` classes. R never sorts dotted codes into families or rewrites messages. Ordinary R argument and serialization errors remain R errors.

```r
tryCatch(
  tt_decide("Please refund my order.", "Is a refund requested?",
            threshold = "invalid"),
  thinkthen_error = function(error) {
    message(error$code, ": ", conditionMessage(error))
  }
)
```

Every call drains its rows before Rust reports completion. A failed row makes completion raise a coded condition. An answered annotate row retains failed named entries in its answer. Input length does not change failure handling. An unsure answer remains a result. The binding translates the shared terminal result without scanning rows for failures.

Press Escape in an R front end, or Ctrl-C in a terminal, to interrupt a running call. The binding cancels and drops the shared call, then raises R's ordinary interrupt condition. It installs no process-wide signal handler and does not wait for an active backend reply before returning the interrupt. The engine owns cancellation of its remaining work. Detaching the package retains its native library until process exit, so a finishing worker keeps valid executable code. Do not unload the DLL manually. R objects and interrupt checks stay on R's thread; engine work stays in Rust.

## Test without a service

```r
stub <- tt_client(adapter = "stub")
rows <- tt_decide(c("alpha", "beta"), "Ready?", client = stub)
```

The built-in stub needs no key, server or network. It uses the ordinary engine and follows the [shared construction cases](../../construction/stub.json). It tests a caller's integration and measures no semantic quality.

## Package

The package name is `thinkthen`. A binary package contains its native engine. A source package needs R development headers, a C compiler, make and the Rust toolchain pinned by the package. Install `jsonlite` before installing a supplied source archive:

```sh
R CMD INSTALL /path/to/thinkthen_VERSION.tar.gz
```

The source archive contains engine source and vendored Rust dependencies. Offline installation needs the R dependency and toolchain already installed. The package uses base R and `jsonlite` and requires no table framework. Package numbers remain 0.x; no call or native-library version negotiation exists. The [public API](API.md) describes the complete surface.
