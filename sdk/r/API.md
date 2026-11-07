# R API

ThinkThen accepts a character vector, list of records or data frame. The plain verbs return typed answer vectors in input order; annotate returns one typed column per question, and filter returns the kept original records. Each `_rows` twin returns the full outcome tibble. Arguments after `...` must be named. Unused arguments raise a `thinkthen_usage_error`.

```r
client <- tt_client(backend = "stub")
teams <- tt_choose(c("Refund please", "Where is my delivery?"),
                  "Which team?", c("billing", "shipping"), client = client)
tt_close(client)
```

## Calls

The public functions take data first. Choose takes required `choices`, tag takes required `labels`, score and rank take required `levels`, recognize takes required `kinds`, and relate takes required `rules`. These domains follow the question. Recognize and relate use their domain in place of the question.

Every call accepts named `client`, `.progress`, `field` and `name`. `field` is a JSON Pointer. `name` is an optional execution name. Toolbox selection never supplies it. `tt_rank()` also accepts `top`; unranked input positions receive NA. `tt_rank_rows()` limits successful rows and retains failed rows. Decide and filter accept `yes_means` and `no_means`; choose, tag, score and rank accept named character `descriptions`.

Decide and filter accept a numeric `threshold` or `c(low, high)`. Choose, tag, find, recognize and relate accept a numeric cut. Score and rank have no threshold argument. The engine admits bounds, list contents and pointers.

## Questions

The question constructors are `tt_decide_question()`, `tt_choose_question()`, `tt_tag_question()`, `tt_score_question()`, `tt_filter_question()`, `tt_rank_question()`, `tt_find_question()`, `tt_recognize_question()` and `tt_relate_question()`. They take the same question, required domain and question options as their call. They contain no client settings.

A matching admitted question can replace inline question text. Its domain travels with it, so the call omits that domain. A second domain or question option raises a usage condition.

```r
route <- tt_choose_question("Which team?", c("billing", "shipping"))
tools <- tt_toolbox(list(route = route))
selected <- tt_question(tools, "route")
questions <- tt_annotation_questions(list(route = selected))
rules <- list(tt_rule("person", "company", "works_for"))
```

`tt_toolbox()` admits a named list of questions. `tt_toolbox_read(path)` and `tt_toolbox_write(toolbox, path)` read and write canonical saved toolboxes. `tt_question()` selects one entry. The selected question survives collection of the toolbox. `tt_annotation_questions()` admits the narrower named collection accepted by `tt_annotate()`. It excludes rank and annotate. Relation rules use `tt_rule(source, target, relation, ..., reads = NULL, either = FALSE)` alone or inside an ordinary `list()`.

## Clients

`tt_client()` accepts `backend`, `api_key`, `url`, `model`, `batch` and `retry`. Backend names derive from the engine registry. An omitted backend reads raw `THINKTHEN_BACKEND`; an explicit backend wins. Other omitted settings use engine defaults. `tt_retry()` accepts named `max_retries`, `initial_backoff_ms` and `attempt_timeout_ms`. Counts and durations must be whole integers.

`local_tt_client(client)` installs a default for its calling scope. `with_tt_client(client, code)` installs one while evaluating `code`. Both restore the previous default on exit. Neither closes the borrowed client. An omitted client uses a lazily created package default.

`tt_close(client)` releases its engine and is idempotent. A call through a closed client raises `usage.closed`. Printing a client hides credentials. `options(thinkthen.verbose = TRUE)` prints function starts and failed positions. `.progress` controls cli progress in interactive sessions.

## Rows

Every `_rows` result inherits `tt_<function>_rows`, `tt_rows` and tibble classes. Columns contain `position`, `id`, `status`, `answer`, `probability`, `source`, `error_code`, `error_message`, `error_http_status` and `detail`. Status and error codes are factors drawn from the shared registry. Probability retains full double precision. Source entries have class `tt_source` and identify the question, backend, model and optional execution name.

Decide and filter answers are logical, choose answers are character, score and rank answers are double, and tag answers are `list_of<character>`. Find answers are `tt_span` records. Recognize answers are list columns of `tt_name` records. Relate answers hold `tt_relation` records. Annotate answers contain named function and status classes under `tt_outcome`.

Span offsets count Unicode code points from one and include the end. `$text`, `$start` and `$end` expose the fields. `substr(text, span$start, span$end)` selects the original text. Names also expose kind and probability. Relations hold typed endpoints, relation, probability and either. Slicing and binding preserve record prototypes.

Detail entries are typed per function. Label detail holds named probabilities. Scale detail holds probabilities in level order. Recognition detail holds piece and name record vectors. Relation detail holds relation pairs. No raw backend reply appears in rows.

`tt_filter_rows()` returns every row, plus its original `item` and logical `kept`. `tt_filter()` returns only kept original records. `tt_rank_rows()` returns successful rows in rank order, then failures, with original input positions and original `item`; `tt_rank()` and `tt_answer()` align ranks to input order.

## Missing values and errors

`NULL` as the collection means zero records. `list(NULL)` and atomic NA elements are missing records. They make no backend request and return failed rows with `record.missing`. Unsupported record conversion and invalid IDs retain their positions as failed rows. Failed scalar answers use typed NA. Failed list answers use NULL without losing rows. Answered empty collections retain their prototypes.

The `_rows` forms retain failed rows, including single-record calls. Plain answer forms raise `thinkthen_failed_rows`, carrying every outcome in `rows`, every failed row in `failed_rows`, failed annotation children in `entries`, the first code in `code`, and the distinct codes in `codes`, in order of first appearance. Filter conditions also carry `kept`. The message names each code with the number of records that failed with it: `ThinkThen returned failed records: record.missing (1), record.empty (1).` Blank text is `record.empty`, with no backend request. Call-level failures use rlang conditions under `thinkthen_error`. The class `thinkthen_<family>_error` names the engine family. R argument mistakes use `thinkthen_usage_error` with a shared `usage.*` code. Cancellation also carries `thinkthen_cancelled_error`. Engine conditions carry the shared code and message. Escape and Ctrl-C cancel the engine call and raise `thinkthen_cancelled_error`. The package keeps native code loaded until process exit.

`errors = "na"` returns the same plain value with `NA` for not sure and for a failure, and keeps the two apart. When a record fails, the call signals one rlang warning of class `thinkthen_failed_rows_warning`, with the condition's fields and a message ending `tt_failures() lists them.` `suppressWarnings()` silences it. The class differs from `thinkthen_failed_rows`, so a `tryCatch()` for the error does not catch the warning and lose the answers. The result also gains an attribute `.thinkthen_failures`, which printing shows. This covers every plain function: `tt_answer()`, `tt_keep()`, `tt_annotate()` on its tibble and `tt_filter()` on its kept result. A result with no failure has no warning and no attribute, so it stays `identical()` to the plain answers.

`tt_failures(x)` takes an `errors = "na"` result or any `_rows` result. It returns a tibble with one row per failure, sorted by position, with the columns `position`, `id`, `question`, `error_code`, `error_message` and `error_http_status`. `question` names the annotation question and is `NA` for a whole record and for other functions. Positions count the call's input, so they survive a later reorder. A clean result or any other value gives zero rows. Subsetting with `[` drops the attribute, as base R does.

## Table helpers

`tt_answer(rows, errors = "error")` projects existing answers without another call. Not-sure answers are missing. Failed rows or annotation children raise by default; `errors = "na"` returns missing values and keeps the failures for `tt_failures()`. `tt_keep()` starts a filter call and returns its logical mask. The `_vec` helpers have been removed.

`tt_bind(data, rows, prefix)` attaches outcome columns to a copy. It supports base data frames, tibbles, grouped tibbles and data.table. It joins by input position and requires one result per original row. Complete rank results can bind in input order. Truncated rank results and name collisions raise a usage condition. Input IDs never act as joins. Use it with the `_rows` forms to keep status in table columns, such as `<prefix>_status` and `<prefix>_error_code`.

## Exact integer IDs

Character IDs retain their string type. Every integer output ID is a size-one `tt_id_integer` record. `tt_id_integer()` accepts decimal character spellings and delegates bounds to the engine. `format()` returns their exact decimal text. Slicing and binding preserve the record. Implicit numeric and character casts fail.

Ordinary integer IDs and safe whole doubles are accepted. Doubles beyond `2^53 - 1` in absolute value require a decimal-backed ID. Mixed IDs use list columns. A missing ID member differs from a present NULL or missing ID value. Invalid IDs return `record.invalid` without a backend call.

## Package

The package imports jsonlite, rlang, cli, vctrs, tibble, withr and stats. dplyr and data.table support their optional table classes. The source archive includes the engine and vendored Rust dependencies. An offline source installation needs these R dependencies, R headers, a C compiler, GNU make and Rust >= 1.95.0. Source installation supports Unix platforms. Windows uses the supplied binary package.
