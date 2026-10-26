# R API

ThinkThen accepts a character vector, list of records or data frame. Each call returns a tibble. Arguments after `...` must be named. Unused arguments raise a `thinkthen_usage_error`.

```r
client <- tt_client(backend = "stub")
rows <- tt_choose(c("Refund please", "Where is my delivery?"),
                  "Which team?", c("billing", "shipping"), client = client)
tt_close(client)
```

## Calls

The public functions take data first. Choose takes required `choices`, tag takes required `labels`, score and rank take required `levels`, recognize takes required `kinds`, and relate takes required `rules`. These domains follow the question. Recognize and relate use their domain in place of the question.

Every call accepts named `client`, `.progress`, `evidence` and `name`. `evidence` is a JSON Pointer. `name` is an optional execution name. Toolbox selection never supplies it. `tt_rank()` also accepts `top`, which limits successful rows and retains failed rows.

Decide, choose and filter accept a numeric `threshold` or `c(low, high)`. Tag, find, recognize and relate accept a numeric cut. Score and rank have no threshold argument. The engine admits bounds, list contents and pointers.

## Questions

The question constructors are `tt_decide_question()`, `tt_choose_question()`, `tt_tag_question()`, `tt_score_question()`, `tt_filter_question()`, `tt_rank_question()`, `tt_find_question()`, `tt_recognize_question()` and `tt_relate_question()`. They take the same question, required domain and question options as their call. They contain no client settings.

A matching admitted question can replace inline question text. Its domain travels with it, so the call omits that domain. A second domain or question option raises a usage condition.

```r
route <- tt_choose_question("Which team?", c("billing", "shipping"))
tools <- tt_toolbox(list(route = route))
selected <- tt_question(tools, "route")
questions <- tt_annotation_questions(list(route = selected))
rules <- vctrs::list_of(tt_rule("person", "company", "works_for"))
```

`tt_toolbox()` admits a named list of questions. `tt_question()` selects one entry. The selected question survives collection of the toolbox. `tt_annotation_questions()` admits the narrower named collection accepted by `tt_annotate()`. It excludes rank and annotate. Relation rules use `tt_rule(source, target, relation, ..., reads = NULL, either = FALSE)` inside `vctrs::list_of()`.

## Clients

`tt_client()` accepts `backend`, `api_key`, `url`, `model`, `batch` and `retry`. Backend names derive from the engine registry. Omitted settings use engine defaults. `tt_retry()` accepts named `max_retries`, `initial_backoff_ms` and `attempt_timeout_ms`. Counts and durations must be whole integers.

`tt_client_from_env()` uses the same engine resolution and accepts the same overrides. `local_tt_client(client)` installs a default for its calling scope. `with_tt_client(client, code)` installs one while evaluating `code`. Both restore the previous default on exit. Neither closes the borrowed client. An omitted client uses a lazily created package default.

`tt_close(client)` releases its engine and is idempotent. A call through a closed client raises a setup condition. Printing a client hides credentials. `options(thinkthen.verbose = TRUE)` prints function starts and failed positions. `.progress` controls cli progress in interactive sessions.

## Rows

Every result inherits `tt_<function>_rows`, `tt_rows` and tibble classes. Columns contain `position`, `id`, `status`, `answer`, `probability`, `source`, `error_code`, `error_message`, `error_http_status` and `detail`. Status and error codes are factors drawn from the shared registry. Probability retains full double precision. Source entries have class `tt_source` and identify the question, backend, model and optional execution name.

Decide and filter answers are logical, choose answers are character, score and rank answers are double, and tag answers are `list_of<character>`. Find answers are `tt_span` records. Recognize answers are list columns of `tt_name` records. Relate answers hold `tt_relation` records. Annotate answers contain named function and status classes under `tt_outcome`.

Span offsets count Unicode code points from one and include the end. `$text`, `$start` and `$end` expose the fields. `substr(text, span$start, span$end)` selects the original text. Names also expose kind and probability. Relations hold typed endpoints, relation, probability and either. Slicing and binding preserve record prototypes.

Detail entries are typed per function. Label detail holds named probabilities. Scale detail holds probabilities in level order. Recognition detail holds piece and name record vectors. Relation detail holds relation pairs. No raw backend reply appears in rows.

`tt_filter()` returns every row, plus its original `item` and logical `kept`. Rank returns successful rows in rank order, then failures, with original input positions.

## Missing values and errors

`NULL` as the collection means zero records. `list(NULL)` and atomic NA elements are missing records. They make no backend request and return failed rows with `record.missing`. Unsupported record conversion and invalid IDs retain their positions as failed rows. Failed scalar answers use typed NA. Failed list answers use NULL without losing rows. Answered empty collections retain their prototypes.

Failed records return rows, including single-record calls. Call-level failures use rlang conditions under `thinkthen_error`, with classes `thinkthen_usage_error`, `thinkthen_setup_error`, `thinkthen_cancelled_error` or `thinkthen_internal_error`. Conditions carry the shared code and message. Escape and Ctrl-C cancel the engine call and raise `thinkthen_cancelled_error`. The package keeps native code loaded until process exit.

## Table helpers

`tt_answer(rows)` projects existing answers. Every call also has a `_vec` helper. `tt_keep()` projects filter answers. These helpers emit one `thinkthen_failed_rows` cli warning when rows or annotation children fail. Their missing answers remain visible.

`tt_bind(data, rows, prefix)` attaches outcome columns to a copy. It supports base data frames, tibbles, grouped tibbles and data.table. It joins by input position and requires one result per original row. Complete rank results can bind in input order. Truncated rank results and name collisions raise a usage condition. Input IDs never act as joins.

## Exact integer IDs

Character IDs retain their string type. Every integer output ID is a size-one `tt_id_integer` record. `tt_id_integer()` accepts decimal character spellings and delegates bounds to the engine. `format()` returns their exact decimal text. Slicing and binding preserve the record. Implicit numeric and character casts fail.

Ordinary integer IDs and safe whole doubles are accepted. Doubles beyond `2^53 - 1` in absolute value require a decimal-backed ID. Mixed IDs use list columns. A missing ID member differs from a present NULL or missing ID value. Invalid IDs return `record.invalid` without a backend call.

## Package

The package imports jsonlite, rlang, cli, vctrs and tibble. dplyr and data.table support their optional table classes. The source archive includes the engine and vendored Rust dependencies. An offline source installation needs these R dependencies, R headers, a C compiler, make and the pinned Rust toolchain.
