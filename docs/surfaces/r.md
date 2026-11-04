# R functions that answer questions about text

ThinkThen 0.2.0 for R gives you ten functions. Each one reads text, answers one question about it, and returns a plain R vector, one answer per text, ready for `if`, `stopifnot()` and `mutate()`.

## Install and ask your first question

Install from R-universe. It needs R 4.2.0 or later and includes the native engine:

```r
install.packages("thinkthen", repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))
```

Set `THINKTHEN_API_KEY` in the environment first; the package reads it from there. [Choose a service and get a key](service.md#choose-a-service-and-get-a-key) explains the backends and how to get a key.

```r
library(thinkthen)

is_refund <- tt_decide("Please refund my order.",
                       question = "Does the customer request a refund?")
print(is_refund)
```

<!-- output: capture -->

The text comes first and the question second, as in every tidyverse function. Name `question =` so a swapped call can't answer the wrong question.

R-universe serves binary packages for many R versions and platforms. Elsewhere it installs from source, which also needs R development headers, a C compiler, GNU make and Rust 1.95.0 or later. Source installs work on Unix; Windows uses the binary package.

## Handle not sure and failures

When ThinkThen is not sure, the answer is `NA`. `if (NA)` stops with "missing value where TRUE/FALSE needed", and `sum()` and `mean()` give `NA`, so not sure never passes for no. With decide's default cut every answer is yes or no. A band, `threshold = c(low, high)`, makes the middle not sure. Test `is.na()` first and send those texts to a person:

```r
message <- "Where is my parcel?"
is_refund <- tt_decide(message, question = "Does the customer request a refund?",
                       threshold = c(0.2, 0.8))
if (is.na(is_refund)) {
  cat("Ask a person:", message, "\n")
} else if (is_refund) {
  cat("Send to refunds:", message, "\n")
} else {
  cat("Leave in the inbox:", message, "\n")
}
```

<!-- output: capture -->

`c(0.2, 0.8)` answers no below 0.2, not sure from 0.2 up to 0.8, and yes at 0.8 or above. Decide, choose and filter take a band or a single cut; tag, find, recognize and relate take a single cut.

A failed record, such as an `NA` or a blank text, sends nothing to the backend. ThinkThen judges every other record first, then raises one `thinkthen_failed_rows` error. Its `code` is the first failure's code, `rows` holds every outcome already paid for, and `failed_rows` holds the failed ones:

```r
messages <- c("Please refund my order.", "", "Where is my parcel?")
refund_checks <- tryCatch(
  tt_decide(messages, question = "Does the customer request a refund?"),
  thinkthen_failed_rows = function(failure) {
    print(failure$failed_rows[c("position", "error_code")])
    failure$rows
  }
)
```

<!-- output: capture -->

Look for position 2 with `record.empty`. `errors = "na"` turns failed records into `NA` instead of an error, so they read like not sure; use it only where both go to a person anyway. To keep every outcome with its status, call the `_rows` form, as [Judge many records](#judge-many-records) shows.

Mistakes in the call itself raise at once, such as a bad threshold, a missing key, or a `descriptions` name that is not one of the choices. Every ThinkThen error inherits `thinkthen_error` and carries a `code`; [Errors and cancellation](#errors-and-cancellation) lists the classes.

## The ten functions

Each function takes the texts first: a character vector, a list of records or a data frame. It returns one answer per text, in input order. The many-text examples use the same three support messages:

```r
messages <- c("Please refund my order.",
              "Where is my parcel?",
              "I was charged twice for one invoice.")
```

### decide: answer a yes-or-no question

```r
is_refund <- tt_decide("Please refund my order.",
                       question = "Does the customer request a refund?")
```

`is_refund` is `TRUE`, `FALSE` or `NA` for not sure.

### choose: pick the one option that fits

```r
topic <- tt_choose("Please refund my order.",
                   question = "What is the message about?",
                   choices = c("billing", "shipping", "other"))
```

`topic` is one of the choices, or `NA` for not sure.

### tag: label every option that fits

```r
topics <- tt_tag("I was charged twice and my parcel is late again.",
                 question = "Which topics appear?",
                 labels = c("billing", "shipping"))
topics[[1]]
```

`tt_tag()` returns a list with one character vector per text, holding every label that fits.

### score: place the text on a scale from 0 to 1

```r
urgency <- tt_score("Please refund my order.",
                    question = "How urgent is the request?",
                    levels = c("routine", "urgent"))
```

Levels run from lowest to highest, and `urgency` is a number from 0 to 1 along them. A score orders a queue a person reads. Use decide for a yes-or-no gate.

### filter: keep the records that answer yes

```r
refund_requests <- tt_filter(messages,
                             question = "Does the customer request a refund?")
print(refund_requests)
```

`tt_filter()` returns your own records, only the ones that answer yes, in the class you passed: a vector stays a vector and a data frame keeps its rows. Records that are not sure are left out. When records fail, the error comes after every record is judged, and its `kept` holds the records that passed. `tt_keep()` returns the same answer as a logical mask for `dplyr::filter()`.

### rank: order every record, highest first

```r
urgency_rank <- tt_rank(messages,
                        question = "How urgent is the request?",
                        levels = c("routine", "urgent"))
messages[order(urgency_rank)]
```

`urgency_rank` holds each message's rank, starting at 1, in input order, so it lines up with `messages` and with a data frame's rows in `mutate()`. `top = 2` keeps the first two ranks and gives `NA` to the rest.

### find: point to the line that answers

```r
action <- tt_find("Morgan works for Example Group.\nPlease refund my order.",
                  question = "Which line states the requested action?")
action$text
```

Find picks one whole line, so keep the line breaks. `action` is a span with `$text`, `$start` and `$end`, counted in characters from 1 with the end included, so `substr(text, action$start, action$end)` gives the line back. `is.na(action)` is `TRUE` when no line answers.

### annotate: fill in several questions for each record

```r
inbox_form <- tt_toolbox(list(
  refund = tt_decide_question("Does the customer request a refund?"),
  topic = tt_choose_question("What is the message about?",
                             choices = c("billing", "shipping", "other")),
  urgency = tt_score_question("How urgent is the request?",
                              levels = c("routine", "urgent"))
))
triage <- tt_annotate(messages, inbox_form)
print(triage)
```

`tt_annotate()` returns a tibble with one typed column per question: `triage$refund` is logical, `triage$topic` is character and `triage$urgency` is a number. A not-sure answer is `NA` in its column. Rank can't go in an annotation.

### recognize: pull out each name and its kind

```r
mentions <- tt_recognize("Morgan works for Example Group.",
                         kinds = c("person", "organization"))[[1]]
data.frame(kind = mentions$kind, text = mentions$text)
```

`tt_recognize()` returns a list with one set of names per text. Each name has `$text`, `$kind`, `$start`, `$end` and `$probability`.

### relate: link names by the relations you allow

```r
employers <- tt_relate("Morgan works for Example Group.\nPlease refund my order.",
                       rules = tt_rule("person", "organization", "works_for"))[[1]]
paste(employers$source$text, employers$relation, employers$target$text)
```

`rules` takes one `tt_rule()` or a `list()` of them. `tt_rule(source, target, relation, reads = NULL, either = FALSE)` takes `reads` to word the relation and `either = TRUE` to make it undirected. Relate finds its own names, so it needs no recognize call first. [Relation rule fields](service.md#relation-rule-fields) describes each field.

## Judge many records

Pass the whole vector, list or data frame in one call, so ThinkThen can pack the records into batches. A column works inside `mutate()`, and each answer lands on its own row:

```r
library(dplyr)

inbox <- tibble(id = c("a1", "a2", "a3"), text = messages)
inbox <- inbox |>
  mutate(is_refund = tt_decide(text, question = "Does the customer request a refund?"),
         topic = tt_choose(text, question = "What is the message about?",
                           choices = c("billing", "shipping", "other")))
```

A data frame can also go in whole. `field = "/text"` judges its `text` column; without it, ThinkThen judges each row as one JSON object. A column named `id` is never judged. It comes back as the row's `id` in the `_rows` form.

```r
refund_inbox <- tt_filter(inbox, question = "Does the customer request a refund?",
                          field = "/text")
```

To keep every outcome, including failures, use the `_rows` form. Put one JSON record per line in `records.jsonl`, such as `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}`:

```r
library(thinkthen)

lines <- readLines("records.jsonl", warn = FALSE, encoding = "UTF-8")
records <- lapply(lines[nzchar(trimws(lines))], jsonlite::fromJSON,
                  simplifyVector = FALSE, bigint_as_char = TRUE)
refund_checks <- tt_decide_rows(records, question = "Does the customer request a refund?",
                                field = "/text")
outcome <- ifelse(refund_checks$status == "failed",
                  refund_checks$error_message, refund_checks$answer)
cat(sprintf("%s %s %s\n", refund_checks$position, refund_checks$status, outcome), sep = "")
```

<!-- output: capture -->

Look for one line per record, numbered from 1, with `answered` and `TRUE` or `FALSE`, or `failed` and the reason. A line that is `null`, or a record with no `text`, fails with `record.missing` and sends nothing to the backend; the call carries on past it. `lapply()` keeps a `null` line as a `NULL` slot, which is one missing record, so don't drop those slots. A JSON array on one line is one record; to judge each value, put each on its own line.

## Say what yes, no and each option mean

Decide and filter take `yes_means` and `no_means`. Choose, tag, score and rank take `descriptions`, a named character vector from each option or level to what it means. The answers keep their short names.

```r
is_refund <- tt_decide("Please refund my order.",
                       question = "Does the customer request a refund?",
                       yes_means = "The customer asks for money back",
                       no_means = "The customer asks for anything else")
topic <- tt_choose("Please refund my order.",
                   question = "What is the message about?",
                   choices = c("billing", "shipping", "other"),
                   descriptions = c(billing = "Invoices, charges and refunds",
                                    shipping = "Delivery and parcels",
                                    other = "Anything else"))
```

## Read the probability and details

Each function has a `_rows` twin, such as `tt_decide_rows()`, that returns a tibble with one row per record: `position`, `id`, `status`, `answer`, `probability`, `source`, `error_code`, `error_message`, `error_http_status` and `detail`. `status` is `answered`, `unsure` or `failed`. `tt_answer()` turns those rows into the plain answers without another backend call, so one call gives both:

```r
refund_checks <- tt_decide_rows(messages, question = "Does the customer request a refund?")
refund_checks[c("position", "status", "answer", "probability")]
is_refund <- tt_answer(refund_checks)
```

<!-- output: capture -->

For decide and filter, `probability` is the probability of yes, even when the answer is no. For choose it is the chosen option's probability. `detail` holds each choice's or label's probability for choose and tag, each level's probability in level order for score and rank, the pieces and names for recognize, and every name and scored pair for relate. `source` names the question, backend and model that answered.

`tt_filter_rows()` and `tt_rank_rows()` add `item`, your own record, so a row never needs looking up by position. `tt_filter_rows()` also adds `kept`.

`tt_bind(data, rows, prefix)` adds the outcome columns to a copy of a data frame, by position: `tt_bind(inbox, refund_checks, "refund")` adds `refund_answer`, `refund_status`, `refund_probability` and the rest. It keeps base data frames, tibbles, grouped tibbles and data.table as they are. Annotate rows need no prefix; each question adds its own `name`, `name_status`, `name_probability` and `name_detail` columns.

Record IDs keep their type. An integer ID comes back as `tt_id_integer`, and `format()` gives its exact digits; `tt_id_integer("9007199254740993")` holds an ID too large for a double.

## Save and load a toolbox

A toolbox is a set of named questions. Write one to a file, read it back, and pass it whole to `tt_annotate()` or pick one question by name for its function:

```r
tt_toolbox_write(inbox_form, "toolbox.json")
inbox_form <- tt_toolbox_read("toolbox.json")
triage <- tt_annotate(messages, inbox_form)
topic_question <- tt_question(inbox_form, "topic")
topics <- tt_choose(messages, topic_question)
```

A saved question carries its own choices, levels and options, so the call leaves them out. The question builders, from `tt_decide_question()` to `tt_relate_question()`, take the same arguments as their function, without the texts. `tt_toolbox_to_json()` and `tt_toolbox_from_json()` do the same with a string.

## Choose a client

With no `client`, the functions share one default client made on first use. It reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment. Make your own client to set the backend, model, batch size or retries:

```r
triage_inbox <- function(messages) {
  client <- tt_client(batch = 16L, retry = tt_retry(max_retries = 2L))
  on.exit(tt_close(client), add = TRUE)
  local_tt_client(client)
  tt_decide(messages, question = "Does the customer request a refund?")
}
```

`tt_client()` takes `backend`, `api_key`, `url`, `model`, `batch` and `retry`. `local_tt_client(client)` makes it the default until the calling function returns, and `with_tt_client(client, code)` does the same for one expression. Every function also takes `client =` directly. `tt_close()` releases a client, and closing twice is safe. Printing a client never shows its key.

## Errors and cancellation

Every ThinkThen error is an rlang condition that inherits `thinkthen_error` and carries `code`, a name from the shared list such as `usage.threshold` or `backend.status`. It also has a class for its family: `thinkthen_usage_error`, `thinkthen_record_error`, `thinkthen_backend_error`, `thinkthen_local_error` or `thinkthen_internal_error`. Failed records raise `thinkthen_failed_rows`, as [Handle not sure and failures](#handle-not-sure-and-failures) shows.

Esc or Ctrl-C cancels a running call and raises `thinkthen_cancelled_error`. An interactive session shows a progress bar; `.progress = FALSE` hides it. `options(thinkthen.verbose = TRUE)` prints when each call starts and which positions fail.

## Reference

| Function | Answer |
| --- | --- |
| `tt_decide(x, question)` | logical |
| `tt_choose(x, question, choices)` | character |
| `tt_tag(x, question, labels)` | list of character vectors |
| `tt_score(x, question, levels)` | number from 0 to 1 |
| `tt_filter(x, question)` | the kept records of `x` |
| `tt_rank(x, question, levels, top = NULL)` | rank from 1, in input order |
| `tt_find(x, question)` | `tt_span` |
| `tt_annotate(x, toolbox)` | tibble, one column per question |
| `tt_recognize(x, kinds)` | list of `tt_name` records |
| `tt_relate(x, rules)` | list of `tt_relation` records |

Every function also takes `field`, `client`, `name`, `.progress` and `errors`. `?tt_decide` and the other help pages list every argument. [Read the API reference](../../sdk/r/API.md).
