# R functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a plain R vector you can branch on, with one answer per record.

Use R 4.2.0 or later. Run `install.packages("thinkthen", repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))`. The package includes the engine. Set `THINKTHEN_API_KEY` in the environment first.

Every answer is in exactly one of three states:

- **answered**: there is an answer, and it is the value you get back.
- **not sure**: the model's number fell between the two bars you set, or no choice was likely enough. The value is `NA`.
- **failed**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **A missing answer is `NA` or a stopped call, never `FALSE`.** Not sure is `NA`, and R's `if` stops on `NA`. A failed record stops the call with one `thinkthen_failed`, after every record is judged. A missing answer never reads as "no" or as an empty value.
2. **Details keep the state beside the answer.** Add `details = TRUE` to any function but filter and failed judgements remain rows, without raising or warning. Its `status`, `probability` and error columns are always safe to read. Usage, setup, authentication and cancellation errors still stop the call.
3. **Printing never stops.** An answer prints as R prints any value, such as `TRUE` or `NA`. A details data frame prints as a table. A failure's message uses the words the `thinkthen` command prints, such as `failed: backend.status: the backend answered status 503`.

Every function takes the text first and the question second. Name `question =` so a swapped call can't answer the wrong question. All calls use table semantics, even for a one-element vector. Python's scalar failure rule does not add a separate R call shape.

## decide

### The simplest if

A decide answer goes straight into an `if`.

```r
library(thinkthen)

if (tt_decide("Please refund my order.", question = "Does the customer request a refund?")) {
  cat("refund queue\n")
} else {
  cat("inbox\n")
}
```

If the message could not be judged, the call stops with a `thinkthen_failed`. It never falls through to `inbox`.

### Yes, no, or not sure

Set two bars and decide can also be not sure. `is.na()` tells you whether there is a yes or no to read.

```r
library(thinkthen)

is_refund <- tt_decide(
  "Where is my parcel?",
  question = "Does the customer request a refund?",
  threshold = c(0.2, 0.8)
)

if (!is.na(is_refund)) {
  cat(if (is_refund) "refund queue" else "inbox", "\n")
} else {
  cat("a person reads it\n")
}
```

A failed record still stops this call. To send not sure and failed to the same place, catch `thinkthen_no_answer` around the call. It covers both.

```r
tryCatch(
  {
    is_refund <- tt_decide(
      "Where is my parcel?",
      question = "Does the customer request a refund?",
      threshold = c(0.2, 0.8)
    )
    cat(if (is_refund) "refund queue" else "inbox", "\n")
  },
  thinkthen_no_answer = function(no_answer) cat("a person reads it\n")
)
```

Not sure is not an error. A call that was not sure signals `thinkthen_unsure` quietly: with no handler, nothing happens and you get the `NA`.

Don't wrap an answer in `isTRUE()`. It turns `NA` into `FALSE`, which is how not sure gets read as no.

### Did the call work

```r
tryCatch(
  {
    is_refund <- tt_decide(
      "Please refund my order.",
      question = "Does the customer request a refund?"
    )
    print(is_refund)
  },
  thinkthen_failed = function(failure) {
    cat("could not judge it:", conditionMessage(failure), "\n")
  },
  thinkthen_error = function(failure) {
    cat("the call did not complete:", conditionMessage(failure), "\n")
  }
)
```

`thinkthen_failed` reports failed judgements after every input is judged. It also belongs to `thinkthen_error`, so put its specific handler first. The broader handler covers usage, setup, authentication and cancellation errors as well; those keep their own classes. Neither handler reads an answer.

Not sure counts as working: the plain call signals `thinkthen_unsure` quietly and gives `NA`. This is a condition, not an error, so the error handlers above do not catch it. To inspect all states without stopping for failed judgements, request `details = TRUE` and check its status columns.

### Both bars and the probability

```r
is_refund <- tt_decide(
  "Where is my parcel?",
  question = "Does the customer request a refund?",
  threshold = c(0.2, 0.8),
  details = TRUE
)
print(is_refund$probability)
```

- No `threshold`: one bar at 0.5. The answer is always yes or no.
- `threshold = 0.7`: one bar. Yes at 0.7 or above, otherwise no.
- `threshold = c(0.2, 0.8)`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

`probability` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. It is `NA` only when the record failed.

The number is the model's own. To choose bars for your data, run examples you have already labelled through `tt_decide()` with `details = TRUE`, keep the `probability` column, and compare with your labels.

## choose with your own choices

Give your choices as a character vector. The answer comes back as a factor, and your choices are its levels.

```r
library(thinkthen)

topic_choices <- c("billing", "shipping", "other")

team_for <- function(topic) {
  switch(as.character(topic),
    billing = "billing team",
    shipping = "warehouse",
    other = "front desk",
    stop("no team for: ", topic)
  )
}

topic <- tt_choose(
  "I was charged twice for one invoice.",
  question = "What is the message about?",
  choices = topic_choices
)

if (is.na(topic)) {
  cat("a person reads it\n")
} else {
  cat(team_for(topic), "\n")
}
```

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So check `is.na(topic)` before using the answer. `threshold = 0.3` moves the bar.

The choices (`"billing"`, `"shipping"`, `"other"`) are the words the model reads, so make them plain words or short phrases, not codes. `levels(topic)` gives them back in the order you wrote them.

R can't check a `switch` against a factor's levels. Add a choice and nothing points at `team_for`, so its last branch stops instead of guessing.

A choose answer does not belong in an `if`. R can't refuse it: `if (topic)` runs, and is quietly always yes. `topic == "billing"` is an ordinary comparison, and it is `NA` when the answer is not sure.

## Many records

Every function takes many records the same way it takes one: give it a character vector, a list or a data frame, and it gives one answer per record, in order. With `details = TRUE`, each row carries your own record as `item`.

```r
library(thinkthen)

messages <- c(
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice."
)

refund_checks <- tt_decide(
  messages,
  question = "Does the customer request a refund?",
  threshold = c(0.2, 0.8),
  details = TRUE
)

where <- ifelse(refund_checks$answer, "refund queue", "inbox")
where[refund_checks$status == "unsure"] <- "a person reads it"
where[refund_checks$status == "failed"] <- "try again later"
cat(paste0(where, ": ", refund_checks$item), sep = "\n")
```

One failed record does not stop the others. Each row keeps its own state, with `position` counted from 1 and the record's `id` when it has one. A one-element input uses these same table rules; it does not become a different scalar call.

A plain call is the raise-if-failed step: leave off `details = TRUE` and it stops with one `thinkthen_failed` after collecting all results if a record failed. Its `outcomes` holds every result. There is no separate `raise_if_failed` method. Not sure stays `NA`; `errors = "na"` also keeps failed answers as `NA`, warns once with `thinkthen_record_warning`, and lets `tt_failures()` list the failures. Details retain failures without that error or warning.

Pass the whole collection in one call so ThinkThen can send the records in batches. Don't call once per record in a loop or in `sapply()`.

To save every result, call `tt_details_write(refund_checks, "outcomes.jsonl")`. It writes each row as a line of JSON. Each line includes the state, so nothing is lost.

## Many-result not sure or failed

Here `refund_checks` is the details data frame from the many-record call above. Its `status` tells not sure and failed apart; only an answered row's value is used as yes or no.

```r
where_it_goes <- function(is_refund) {
  switch(as.character(is_refund$status),
    answered = if (is_refund$answer) "refund queue" else "inbox",
    unsure = "a person reads it",
    failed = "try again later",
    stop("no route for: ", is_refund$status)
  )
}

for (i in seq_len(nrow(refund_checks))) {
  is_refund <- refund_checks[i, ]
  cat(where_it_goes(is_refund), ": ", is_refund$item, "\n", sep = "")
}
```

Yes and no are the two answers. Not sure and failed are the two ways of having none. `status` is a factor with exactly three levels: `answered`, `unsure` and `failed`. R can't check that a `switch` names every state, so end it with a `stop()`.

## filter

filter means "give me the yeses". It hands back your own records, in order.

```r
library(thinkthen)

messages <- c(
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice."
)

for (refund_request in tt_filter(messages, question = "Does the customer request a refund?")) {
  cat("refund queue:", refund_request, "\n")
}
```

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `threshold = 0.7` moves the bar. Two bars are refused with `thinkthen_usage_error`: when you want not sure, use `tt_decide()` with `details = TRUE`, as in the section above.

If a record fails, filter stops with one `thinkthen_failed` after every record is judged. Its `kept` holds every kept record, and its `rows` holds every failed result with its `item`.

```r
tryCatch(
  for (refund_request in tt_filter(messages, question = "Does the customer request a refund?")) {
    cat("refund queue:", refund_request, "\n")
  },
  thinkthen_failed = function(failure) {
    cat(paste("refund queue:", failure$kept), sep = "\n")
    failed <- failure$rows
    cat(paste("try again later:", failed$item, failed$error_code), sep = "\n")
  }
)
```

## tag

Labels work as choices do. The answer for each record is a factor of the labels that fit, and it can be empty.

```r
library(thinkthen)

topic_labels <- c("billing", "shipping")

topics <- tt_tag(
  "I was charged twice and my parcel is late again.",
  question = "Which topics appear?",
  labels = topic_labels
)[[1]]

if ("billing" %in% topics) {
  cat("billing team\n")
}
for (topic in topics) {
  cat(topic, "\n")
}
```

`tt_tag()` gives a list with one factor per record, so `[[1]]` is the first record's labels. tag is never not sure. `threshold =` sets how likely a label must be to count.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it. The order of the supplied character vector defines the scale.

```r
library(thinkthen)

urgency_levels <- c("routine", "urgent")

urgency <- tt_score(
  "Please refund my order.",
  question = "How urgent is the request?",
  levels = urgency_levels
)

if (urgency > 0.5) {
  cat("answer today\n")
}
print(round(urgency, 2))
```

The score is a position between your levels, not one of them, so it is a plain number and there is nothing to `switch` on. To get exactly one level back, ask with `tt_choose()`. score is never not sure.

## rank

rank puts many records in order along your levels, highest first.

```r
library(thinkthen)

messages <- c(
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice."
)

ranked <- tt_rank(
  messages,
  question = "How urgent is the request?",
  levels = urgency_levels,
  details = TRUE
)
cat(paste(ranked$answer, ranked$item), sep = "\n")
```

`ranked$answer` is the rank, starting at 1, and `ranked$item` is your record. The rows come in rank order. rank reads the whole collection before it gives anything back. `top = 5` selects the five highest-ranked records after judging every input, and the records below them get no row. A failed record comes back after the ranked ones, with `status` set to `failed`.

Without `details = TRUE`, `tt_rank()` gives each record's rank in the order of your records, so it lines up with a data frame's rows, and `messages[order(ranks)]` puts the messages in rank order. With `top`, a record below the top has `NA` for its rank.

## find

find points to the one line that answers.

```r
library(thinkthen)

message <- "Morgan works for Example Group.\nPlease refund my order."
action <- tt_find(message, question = "Which line states the requested action?")

if (is.na(action)) {
  cat("a person reads it\n")
} else if (!nzchar(action$text)) {
  cat("no line answers\n")
} else {
  cat(action$text, "\n")
}
```

The answer is a `tt_span` with `$text`, `$start` and `$end`. When no line answers, the span is empty: its `$text` is `""`, and its `$start` and `$end` are `NA`. An empty span is an answer, so `is.na(action)` is `FALSE` for it. `substr(message, action$start, action$end)` is the line: the offsets count characters from 1, and the end is included. Like choose, find can be not sure with no bar set.

## recognize

```r
library(thinkthen)

names <- tt_recognize(
  "Morgan works for Example Group.",
  kinds = c("person", "organization")
)[[1]]

for (i in seq_along(names)) {
  name <- names[i]
  cat(name$kind, name$text, "\n")
}
```

`tt_recognize()` gives a list with one set of names per record. Each name also has `$start`, `$end` and `$probability`. The set can be empty. recognize is never not sure.

## relate

```r
library(thinkthen)

message <- "Morgan works for Example Group.\nPlease refund my order."
rules <- tt_rule("person", "organization", "works_for")

relations <- tt_relate(message, rules = rules)[[1]]

for (i in seq_along(relations)) {
  relation <- relations[i]
  cat(relation$source$text, relation$relation, relation$target$text, "\n")
}
```

A rule is the kind it starts from, then the kind it points to, then the relation name. `rules` takes one `tt_rule()` or a `list()` of them. Each relation has `$source`, `$target`, `$relation`, `$probability` and `$either`. With `details = TRUE`, `relations$detail[[1]]$names` holds every name found, including names in no relation.

## annotate

annotate asks several questions of each record and gives back one form per record.

```r
library(thinkthen)

messages <- c(
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice."
)

questions <- tt_annotation_questions(list(
  refund = tt_decide_question("Does the customer request a refund?", threshold = c(0.2, 0.8)),
  topic = tt_choose_question("What is the message about?", choices = topic_choices),
  urgency = tt_score_question("How urgent is the request?", levels = urgency_levels)
))

forms <- tt_annotate(messages, questions, details = TRUE)

for (i in seq_len(nrow(forms))) {
  form <- forms[i, ]
  if (form$status == "failed") {
    cat("try again later:", form$item, "\n")
    next
  }
  if (any(c(form$refund_status, form$topic_status, form$urgency_status) == "failed")) {
    cat("try again later:", form$item, "\n")
  } else if (any(c(form$refund_status, form$topic_status) == "unsure")) {
    cat("a person reads it:", form$item, "\n")
  } else if (all(c(form$refund_status, form$topic_status, form$urgency_status) == "answered") && form$refund) {
    cat(sprintf("refund queue: %s %.2f\n", form$topic, form$urgency))
  }
}
```

`forms` has one row per record and one answer column per question, under the name you gave it: `refund` is logical, `topic` is a factor and `urgency` is a number. Each question also has its own state beside its answer, in `refund_status`, `topic_status` and `urgency_status`, with its probability and error columns named the same way. One entry can be not sure or failed while the others answer. Check every entry used by a branch before reading its answer; an answered form alone does not mean its entries all answered. `form$refund` is a decide answer, so it works in an `if`. Here `topic_choices` is the three-choice vector from the choose section.

Without `details = TRUE`, `tt_annotate()` gives only the answer columns, with `NA` where an entry is not sure. By default its ordinary failure check stops with `thinkthen_failed` after collection if a form or any entry failed; an unsure entry is not a failure. An answered form with one, several or all entries failed remains visible in details, along with every entry's state. The plain call still reports those failed entries; it does not adopt Python's parent-only scalar promotion rule.

Question names must not collide with generated table columns such as `status`, `probability`, `position`, `id` or `item`, or with another question's generated suffix columns. For example, `refund` beside `refund_status` is refused. The call raises `thinkthen_usage_error` naming the clashing name before it sends any paid request. This applies to both plain and details tables, so switching to details cannot change which question names are valid.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```r
is_refund <- tt_decide(
  "I was charged twice for one invoice.",
  question = "Does the customer request a refund?",
  yes_means = "the customer wants money back, including for a duplicate charge",
  no_means = "the customer wants anything else"
)

topic <- tt_choose(
  "I was charged twice for one invoice.",
  question = "What is the message about?",
  choices = topic_choices,
  descriptions = c(
    billing = "charges, invoices and refunds",
    shipping = "parcels and delivery"
  )
)
```

decide and filter take `yes_means` and `no_means`. choose, tag, score and rank take `descriptions`, a character vector named by your choices, labels or levels.

## Records and fields

A record is one string of a character vector, one row of a data frame, or one element of a list. `field = "/text"` picks one column of a data frame or one field of a list element. Without it, ThinkThen judges the whole record.

Give a data frame an `id` column. It is never judged, and each details row carries it back as `id`.

There is no separate one-record call. One record is a vector of length one, and its answer is a vector of length one. A blank record (an empty string, or only spaces) gives a failed result with code `record.empty`, an `NA` record gives one with code `record.missing`, and nothing is sent to the service for either.

## Tables

A data frame needs nothing extra. Give any function the data frame, or one of its columns, and the answers line up with its rows.

```r
library(thinkthen)

frame <- read.csv("messages.csv")

frame$is_refund <- tt_decide(
  frame,
  question = "Does the customer request a refund?",
  field = "/text",
  threshold = c(0.2, 0.8)
)
```

You get one answer per row, in a column of the right type. Where there is no answer the column holds R's missing value, `NA`. Tables treat missing values their own way. `subset()` and `dplyr::filter()` drop the `NA` rows, for example. The same call works on a column inside `dplyr::mutate()`.

To see why an answer is missing, ask for the details:

```r
checks <- tt_decide(
  frame,
  question = "Does the customer request a refund?",
  field = "/text",
  threshold = c(0.2, 0.8),
  details = TRUE
)
needs_a_person <- checks[checks$status == "unsure", ]
```

`details = TRUE` gives one row per record with `position`, `id`, `item`, `answer`, `status`, `probability`, the error columns `error_code`, `error_message` and `error_http_status`, and `detail`. For a data frame, `item` is your own rows, so `checks$item$text` is the text column. There is one details form for every function but filter, which has none.

A plain column can't hold a failure. So if any row fails, the call stops with one `thinkthen_failed` after every row is judged, and its `outcomes` holds every result. Pass `errors = "na"` to get `NA` for a failed row instead; the call then warns once, and `tt_failures()` on the returned column lists each failure. Subsetting the column drops that list, so read it first.

`tt_filter(frame, ...)` gives back the rows that answered yes. Like every filter, it takes one bar. It takes no `details` and no `errors`: a failed row stops the call after every row is judged.

Choices come back as a factor column whose levels are your choices. tag, recognize and relate give a list column, with one element per row. rank gives each row its rank, so the column lines up with your frame. For find, a row where no line answers holds an empty span, and `NA` means there is no answer. annotate gives one column per question, named as you named it, and the details put each question's state beside it, as `topic_status`.

## Saved questions

A question object holds a question and its options, so many calls can share one definition. Pass it in place of the question text.

```r
asks_refund <- tt_decide_question(
  "Does the customer request a refund?",
  threshold = c(0.2, 0.8),
  field = "/text"
)
is_refund <- tt_decide(
  data.frame(id = "a1", text = "Please refund my order."),
  question = asks_refund
)
```

There is one builder per function, such as `tt_choose_question()` and `tt_score_question()`. A toolbox keeps named questions in a JSON file:

```r
toolbox <- tt_toolbox_read("toolbox.json")
topic <- tt_choose(
  data.frame(id = "a1", text = "Where is my parcel?"),
  question = tt_question(toolbox, "topic")
)
```

Asking a toolbox for a name it does not have stops with `thinkthen_usage_error`. So does giving a function a question saved for another function.

## Settings

The functions use a shared client that reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment. To choose settings yourself, make a client and pass it as `client =`.

```r
client <- tt_client(model = "...", batch = 16, retry = tt_retry(max_retries = 5))
is_refund <- tt_decide(
  "Please refund my order.",
  question = "Does the customer request a refund?",
  client = client
)
tt_close(client)
```

The options are `backend`, `url`, `model`, `api_key`, `batch` and `retry`. Times in `tt_retry()` are in milliseconds, and their names end in `_ms`. Keep the key in the environment, not in your code. Inside a function, write `on.exit(tt_close(client))` so the client is closed even after an error.

Base R has no async calls and runs one call at a time, so every call waits for its answer. To go faster, give one call the whole collection, and it sends the records in batches.

## Usage

```r
client <- tt_client()
tt_decide("Please refund my order.", question = "Does the customer request a refund?", client = client)
usage <- tt_usage(client)
cat(usage$attempts, usage$input_tokens$reported, usage$output_tokens$reported, "\n")
tt_close(client)
```

`tt_usage(client)` counts everything the client has sent so far: `attempts`, `responses`, retries, bytes sent, and tokens in and out as the service reported them.

## Cancel

Press Esc in RStudio, or Ctrl-C in a terminal, to stop a call. The call stops with `thinkthen_cancelled_error`.

## Errors

| What happened | When it stops | What is signalled |
| --- | --- | --- |
| A mistake in the call, such as a bad threshold or two bars on filter | At the call | `thinkthen_usage_error` |
| The client could not start, such as no API key | At the call | `thinkthen_setup_error` |
| The call was cancelled | At the call | `thinkthen_cancelled_error` |
| The service refuses authentication, or every request in the first batch fails and one has HTTP 4xx | At the call | `thinkthen_backend_error` |
| A record could not be judged, or the service failed | At the end of a plain call, after every record is judged. Retained without raising or warning with `details = TRUE`; `errors = "na"` warns instead | `thinkthen_failed` |
| Not sure | Never. The answer is `NA` | `thinkthen_unsure`, quietly, at the end of a plain call |
| You print or store a returned answer, or inspect a failed judgement in details | Never | |

HTTP 401/403 always stop the call. A backend error also stops it when every request in the first batch fails and at least one is HTTP 4xx; other 4xx responses do not automatically become call errors. These service refusals are caught by `thinkthen_error`.

Two classes let you catch by meaning:

- `tryCatch(..., thinkthen_no_answer = )` catches `thinkthen_unsure` and `thinkthen_failed`: there was no answer to read.
- `tryCatch(..., thinkthen_error = )` catches everything that went wrong, including `thinkthen_failed`. It does not catch `thinkthen_unsure`, because not sure is a result and not an error.

`thinkthen_failed` has both `thinkthen_no_answer` and `thinkthen_error` in its class vector. It carries `code`, `http_status`, `rows`, the failed results, and `outcomes`, every result. From filter it also carries `kept`. Its message has one line per failed record, such as `failed: backend.status: the backend answered status 503`. `thinkthen_unsure` carries `rows`, the not-sure results with their `probability`, and `outcomes`.

`thinkthen_unsure` is a condition but not an error, so `tryCatch(..., error = )` never catches it and an unhandled one does nothing. R's own error from `if (NA)` has no ThinkThen class. To catch not sure by name, put the `tryCatch()` around the call, not around the `if`.

## Upgrading from 0.1

The [R-universe package metadata](https://botassembly.r-universe.dev/api/packages/thinkthen) identifies version 0.1.2 and its source commit. The baseline is the matching [published 0.1.2 package/source](https://proxy.golang.org/github.com/botassembly/thinkthen/@v/v0.1.2.zip). Replace its call envelopes with the results shown above.

| 0.1.2 | Now |
| --- | --- |
| `tt_decide(...)$value` | Plain answer vector, or the details form carrying state |
| `tt_engine`, `tt_question` | Client and typed saved questions |
| Per-call `facts` | Client usage; uncertainty remains a missing value |

Handle failed judgements with `thinkthen_failed`, or catch the broader `thinkthen_error` when usage, setup, authentication and cancellation need the same route. The no-answer catch remains `thinkthen_no_answer`, which also covers quietly signalled not sure. These condition names do not change wire codes such as `record.empty`.

---

## More examples

The [complete guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/r.md) is the tested reference for this SDK. [Recorded examples](https://thinkthen.dev/sdks/r/) show real backend answers.
