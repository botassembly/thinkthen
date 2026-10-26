# ThinkThen in R

All calls return a tibble, including one-record calls. Pass vectors or tables for collections and close the owned client with `tt_close()`. An R list is the collection; place a JSON array record inside that list when it is one record.

Check `thinkthen/README.md` inside the R archive for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

ThinkThen judges evidence with ten functions. Install the supplied 0.2.0 R package with `R CMD INSTALL /path/to/thinkthen_0.2.0.tgz` for a matching binary package, or use the supplied `.tar.gz` source package. Use R 4.6 series for the supplied macOS binary. Install its R dependencies first: jsonlite, rlang, cli, vctrs and tibble. Source installation also needs R development headers, a C compiler, make and the pinned Rust toolchain. A binary package includes the native engine.

Set `THINKTHEN_API_KEY` in the process environment before creating a normal client. Keep it out of scripts and reports. `tt_client()` selects the default backend. The stub needs no key or network and returns synthetic answers for installation checks.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```r
library(thinkthen)
lines <- readLines("records.jsonl", warn = FALSE, encoding = "UTF-8")
records <- lapply(lines[nzchar(trimws(lines))], function(line) jsonlite::fromJSON(
  line, simplifyVector = FALSE, bigint_as_char = TRUE
))
client <- tt_client(backend = "stub")
tryCatch({
  array <- tt_decide(list(list("first", "second")), "Does the customer request a refund?", client = client)
  cat("array", as.character(array$status), "\n")
  rows <- tt_decide(records, "Does the customer request a refund?", evidence = "/text", client = client)
  cat(sprintf("%s %s %s %s\n", rows$position, rows$status, rows$answer, rows$error_message))
}, finally = tt_close(client))
```

## Records and the ten functions

All calls take data first and return a tibble, even for one record. Data may be a character vector, list of JSON-compatible records or data frame. Use named arguments after the required domain. `evidence = "/text"` selects a JSON field. A pointer escapes a slash with `~1` and a tilde with `~0`.

```r
client <- tt_client()
records <- data.frame(id = c("a1", "a2"), text = c("Morgan works for Example Group.\nPlease refund my order.", "Where is my parcel?"))
questions <- tt_annotation_questions(list(
  refund = tt_decide_question("Does the customer request a refund?"),
  topic = tt_choose_question("Which topic?", c("billing", "shipping", "other"))
))
rules <- vctrs::list_of(tt_rule("person", "organization", "works_for"))
# Each line below is a separate call.
decision <- tt_decide(records, "Does the customer request a refund?", evidence = "/text", client = client)
choice <- tt_choose(records, "Which topic?", c("billing", "shipping", "other"), evidence = "/text", client = client)
tags <- tt_tag(records, "Which topics appear?", c("billing", "shipping"), evidence = "/text", client = client)
score <- tt_score(records, "How urgent is the request?", c("routine", "urgent"), evidence = "/text", client = client)
filtered <- tt_filter(records, "Does the customer request a refund?", evidence = "/text", client = client)
ranked <- tt_rank(records, "How urgent is the request?", c("routine", "urgent"), top = 5L, evidence = "/text", client = client)
found <- tt_find(records, "Which line states the requested action?", evidence = "/text", client = client)
names <- tt_recognize(records, c("person", "organization"), evidence = "/text", client = client)
relations <- tt_relate(records, rules, evidence = "/text", client = client)
annotated <- tt_annotate(records, questions, evidence = "/text", client = client)
tt_close(client)
```

| Function | Answer column |
| --- | --- |
| decide, filter | Logical |
| choose | Character choice |
| tag | List of character labels |
| score | Double from 0 to 1 |
| rank | Rank starting at 1 |
| find | `tt_span` or missing span |
| recognize | List of `tt_name` records |
| relate | List of `tt_relation` records |
| annotate | Named `tt_outcome` values |

A rank row's answer is its rank; `position` is its input order. Rank puts highest scores first, then failed rows; `top` limits successful rows and retains failures. Levels go from lowest to highest. Filter returns every outcome with original `item` and logical `kept`; it does not drop false, unsure or failed rows. `tt_answer(filtered)` projects logical answers from an existing filter result. Use `records[filtered$position[filtered$kept], , drop = FALSE]` to select kept input rows from a data frame. `tt_keep(...)` starts a new filter call and returns logical answers or NA.

Rows have `position`, `id`, `status`, `answer`, `probability`, `source`, `detail` and error columns. Status is `answered`, `unsure` or `failed`. Scalar missing answers use typed NA; list missing answers use NULL. An answered find can have no match. Source identifies the question, backend and served model. Detail holds typed label or level probabilities, recognition pieces or relation pairs. Span offsets count Unicode code points from 1 and include the end: use `substr(text, span$start, span$end)`.

String IDs retain their type. Integer IDs return `tt_id_integer` values; `format()` gives exact decimal text. Use `tt_id_integer("9007199254740993")` when a double cannot represent the ID exactly. Preserve IDs when saving results. `tt_bind(records, rows, prefix = "tt_")` joins by input position and requires a complete result for every input, so a truncated rank cannot bind.

## Questions, tables and batching

A typed constructor such as `tt_choose_question()` stores its domain and options. Pass it in place of wording and omit the separate domain. `tt_toolbox(list(route = question))` stores named questions; `tt_question(toolbox, "route")` retrieves one. `tt_annotation_questions()` excludes rank and annotate. Inspect every nested outcome's status; an answered annotation can contain failed children.

Pass the whole vector or table in one call so the engine can batch. `tt_client(batch = 16L)` controls packing; byte limits and backend limits also apply. One record may require multiple questions. Calls collect their results in memory. There is no separate pull-stream method.

Read JSON Lines one nonblank line at a time. `fromJSON()` on the whole file expects one JSON value, so several records cause a trailing-garbage error. `lapply()` preserves a JSON `null` as a NULL list slot; do not remove those slots, because each is an input record:

```r
lines <- readLines("messages.jsonl", warn = FALSE, encoding = "UTF-8")
lines <- lines[nzchar(trimws(lines))]
records <- lapply(lines, function(line) jsonlite::fromJSON(
  line, simplifyVector = FALSE, bigint_as_char = TRUE
))
client <- tt_client()
rows <- tt_decide(records, "Does the customer request a refund?",
                  evidence = "/text", client = client)
tt_close(client)
print(rows[c("position", "id", "status", "answer", "error_code")])
```

Keep the returned IDs and every outcome when saving results. String IDs remain strings; `bigint_as_char` keeps large integer digits when parsing JSON. NULL as the entire collection means zero records; `list(NULL)` means one missing record. A missing evidence field produces `record.missing`; an explicit null evidence field goes to the backend as JSON null. An omitted ID is allowed; an explicit null ID is invalid:

```r
client <- tt_client(backend = "stub")
no_records <- tt_decide(NULL, "Ready?", client = client)
missing_record <- tt_decide(list(NULL), "Ready?", client = client)
without_id <- tt_decide(list(list(text = "evidence")), "Ready?", client = client)
null_id <- tt_decide(list(list(id = NULL, text = "evidence")), "Ready?", client = client)
tt_close(client)
stopifnot(nrow(no_records) == 0L, nrow(missing_record) == 1L,
          as.character(missing_record$error_code) == "record.missing",
          without_id$status != "failed", null_id$status == "failed")
```

When a task requires no backend retries, the existing optional control is `tt_client(retry = tt_retry(max_retries = 0L))`. It does not change how records are batched or permit a second attempt at the task.

Use `read.csv()` for CSV input or your spreadsheet package for XLSX. `tt_bind()` attaches outcomes to input, including typed and list columns. Do not pass that whole table to base `write.csv()`: nested answers, source and detail columns need an explicit projection. Keep the full typed rows with `saveRDS()` first. For `annotated` from the example above, this projection writes only the known scalar `topic` choice and its status/errors, together with every row's ID and outer status/errors:

```r
rows <- annotated
saveRDS(rows, "outcomes.rds")
topic <- lapply(rows$answer, function(entries) entries[["topic"]])
flat <- data.frame(
  id = vapply(rows$id, function(id) if (is.null(id)) NA_character_ else if (inherits(id, "tt_id_integer")) format(id) else id, character(1)),
  position = rows$position, status = as.character(rows$status),
  error_code = as.character(rows$error_code), error_message = rows$error_message,
  topic = vapply(topic, function(outcome) if (isTRUE(outcome$status == "answered")) outcome$answer else NA_character_, character(1)),
  topic_status = vapply(topic, function(outcome) if (is.null(outcome)) NA_character_ else as.character(outcome$status), character(1)),
  topic_error_code = vapply(topic, function(outcome) if (is.null(outcome$error)) NA_character_ else as.character(outcome$error$code), character(1)),
  topic_error_message = vapply(topic, function(outcome) if (is.null(outcome$error)) NA_character_ else outcome$error$message, character(1))
)
write.csv(flat, "outcomes.csv", row.names = FALSE, na = "")
```

Choose explicit columns for other answer types; the RDS file retains spans, label lists, details and exact ID types. The SDK has no shell toolbox-run API. Its toolbox stores reusable questions. Do not assume `jsonlite::toJSON()` on typed result classes is a contract-row exporter.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```r
box <- tt_toolbox_from_json(paste(readLines("toolbox.json", warn = FALSE), collapse = "\n"))
writeLines(tt_toolbox_to_json(box), "toolbox-copy.json")
topic <- tt_question(box, "topic")
client <- tt_client(backend = "stub")
tryCatch({
  rows <- tt_choose(records, topic, client = client)
  print(rows[c("id", "status", "answer", "error_code")])
}, finally = tt_close(client))
```

`tt_question(box, "ready")` retrieves the filter definition for `tt_filter(records, question, client = client)`. These calls still take data first. The saved question owns its evidence and choices, so omit replacement domain and evidence arguments.

Call `tt_relate` alone when you need names and relations from the same evidence. Each row's `rows$detail[[i]]$names` contains all admitted `tt_name` values, including names with no returned relation; `rows$detail[[i]]$pairs` holds scored relation pairs. Read accepted relations from `rows$answer[[i]]`. A prior recognize call is unnecessary.

## Failures and cancellation

`list(NULL)` and atomic NA records return `record.missing` failed rows without a request. Invalid records and IDs also retain failed positions. Test `rows$status == "failed"` before consuming or saving answers; stop subsequent work with `stop()` if your task requires stopping after a failure. Inspect nested annotation failures too. `tt_answer()`, `_vec` helpers and `tt_keep()` warn once when rows or children fail.

Call-level errors are `thinkthen_error` conditions with a shared code and message. Usage, setup, internal and cancelled conditions have their own subclasses. Escape or Ctrl-C cancels a call and raises `thinkthen_cancelled_error`. Close owned clients with `tt_close()`; inside a function use `on.exit(tt_close(client))`.

Decide, choose and filter accept numeric cuts or `c(low, high)` bands. A decide/filter band yields false below low, unsure inside and true at or above high. Choose needs its winning choice to reach the upper cut. Tag, find, recognize and relate take numeric cuts only.

Word questions so one answer is unambiguous. Define what counts as evidence, distinguish absence from an explicit negative, and make choices distinct. Find selects a whole nonblank line, so keep line breaks. The stub checks installation, not question quality.

Each annotation child has its own status. For decide and filter, probability means the probability of yes, including when the answer is false. For choose, it means the chosen label's probability. Score and tag have no single probability; their distributions remain in detail.

`tt_bind(records, annotated)` adds each annotation child's `name`, `name_status`, `name_probability` and `name_detail` beside the input columns. It joins on input position and retains the input class. Annotation uses child names and needs no prefix. Existing columns must not overlap the added names. The original `annotated` rows retain every nested outcome and error.

```r
enriched <- tt_bind(records, annotated)
enriched[c("topic", "topic_status", "topic_probability")]
```
