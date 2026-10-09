# R names and value conversion only. Native admission owns every request rule.
.tt_selector <- function(value, family) {
  if (inherits(value, family)) return(unclass(value))
  if (identical(family, "thinkthen_question")) {
    if (is.character(value) && length(value) == 1L) return(list(kind = "text", text = value))
    return(list(kind = "definition", value = value))
  }
  if (is.character(value) && length(value) == 1L) list(kind = "text", text = value)
  else list(kind = "json", value = value)
}

tt_input <- function(kind, ...) structure(c(list(kind = kind), list(...)), class = "thinkthen_input")

# These selectors distinguish authority and evidence; they perform no admission.
tt_question <- function(value = NULL, file, name, reference) {
  selected <- if (!missing(file)) list(kind = "file", path = path.expand(file))
    else if (!missing(name)) list(kind = "name", name = name)
    else if (!missing(reference)) list(kind = "reference", reference = reference)
    else .tt_selector(value, "thinkthen_question")
  structure(selected, class = "thinkthen_question")
}
tt_files <- function(paths, unit = "line", window = NULL, media = "text") {
  reading <- list(unit = unit)
  if (!is.null(window)) reading$window <- window
  structure(list(kind = "source", source = list(paths = I(paths), reading = reading, media = media)),
            class = "thinkthen_input")
}

.tt_request <- function(function_name, question, input, options) {
  question <- .tt_selector(question, "thinkthen_question")
  column <- NULL
  if (is.null(input) || (is.atomic(input) && !is.object(input) && !is.raw(input))) {
    column <- .tt_call(tt_request_column(unname(as.list(input)), function_name))
    if (length(column$values) != length(input)) input <- column$values
  }
  if (inherits(input, "thinkthen_input")) input <- unclass(input)
  else if (is.null(input) || (!is.null(column) && (column$length != 1 || !length(column$values))) || is.data.frame(input) || function_name %in% c("filter", "rank", "find", "annotate", "relate") ||
           (is.character(input) && length(input) != 1L)) {
    # Native column conversion retains original slots separately from canonical items.
    if (is.data.frame(input)) input <- lapply(seq_len(nrow(input)), function(at) lapply(input, `[[`, at))
    items <- unname(lapply(input, function(value) list(original = .tt_selector(value, "thinkthen_input"))))
    input <- list(kind = "records", items = items)
  } else input <- .tt_selector(input, "thinkthen_input")
  call <- list(`function` = function_name, question = question, input = input)
  if (!is.list(options) || length(options)) call["options"] <- list(options)
  request <- .tt_json(list(schema = "thinkthen.request/1", call = call))
  if (!is.null(column)) {
    attr(request, "positions") <- column$positions
    attr(request, "length") <- column$length
  }
  request
}

.tt_request_failure <- function(failure, completed = NULL, positions = NULL, length = NULL) {
  condition <- structure(list(message = failure$message, kind = failure$kind,
    retryable = failure$retryable, facts = failure$facts, complete = failure,
    completed = completed, positions = positions, length = length, call = NULL),
    class = c(paste0("thinkthen_", failure$kind), "thinkthen_error", "error", "condition"))
  stop(condition)
}
.tt_named_call <- function(function_name, question, input, options, deadline_ms, completion) {
  .tt_asking(completion, {
    request <- .tt_call(tt_request_admit(.tt_request(function_name, question, input, options)))
    result <- .tt_call(tt_request_native(request, deadline_ms, completion))
    if (!is.null(result$failure)) .tt_request_failure(result$failure, result$results, result$positions, result$length)
    .tt_column_view(result, function_name)
  })
}
tt_decide <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("decide", question, input, options, deadline_ms, completion)
tt_choose <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("choose", question, input, options, deadline_ms, completion)
tt_tag <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("tag", question, input, options, deadline_ms, completion)
tt_score <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("score", question, input, options, deadline_ms, completion)
tt_filter <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("filter", question, input, options, deadline_ms, completion)
tt_rank <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("rank", question, input, options, deadline_ms, completion)
tt_find <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("find", question, input, options, deadline_ms, completion)
tt_annotate <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("annotate", question, input, options, deadline_ms, completion)
tt_recognize <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("recognize", question, input, options, deadline_ms, completion)
tt_relate <- function(question, input, options = list(), deadline_ms = NULL, completion = NULL) .tt_named_call("relate", question, input, options, deadline_ms, completion)

tt_plan <- function(question, input, function_name = "decide", options = list()) {
  request <- .tt_call(tt_request_admit(.tt_request(function_name, question, input, options)))
  .tt_call(tt_request_plan(request))
}

tt_batch <- function(function_name, question, input, options = list()) {
  native <- .tt_call(tt_request_batch_start(.tt_request(function_name, question, input, options)))
  state <- new.env(parent = emptyenv())
  state$facts <- NULL
  state$ended <- FALSE
  poll <- function() {
    if (state$ended) return(NULL)
    event <- .tt_call(tt_request_batch_poll(native))
    if (is.null(event) || identical(event$kind, "observation")) return(NULL)
    if (event$kind %in% c("row", "aggregate")) return(event$value)
    state$ended <- TRUE
    state$facts <- event$facts
    if (!is.null(event$failure)) .tt_request_failure(event$failure, positions = attr(native, "positions"), length = attr(native, "length"))
    NULL
  }
  pull <- function() {
    repeat {
      value <- poll()
      if (!is.null(value) || state$ended) return(value)
      Sys.sleep(0.001)
    }
  }
  cancel <- function() { .tt_call(tt_request_batch_cancel(native)); invisible(NULL) }
  close <- function() { cancel(); state$ended <- TRUE; invisible(NULL) }
  structure(list(next_result = pull, poll = poll, cancel = cancel, close = close,
                 facts = function() state$facts, positions = attr(native, "positions"),
                 length = attr(native, "length")), class = "thinkthen_batch")
}
print.thinkthen_question <- function(x, ...) { cat("<question: content withheld>\n"); invisible(x) }
print.thinkthen_input <- function(x, ...) { cat("<input: content withheld>\n"); invisible(x) }
print.thinkthen_batch <- function(x, ...) { cat("<batch>\n"); invisible(x) }

# Ordinary views fill host slots only; complete results and identities stay native.
.tt_column_view <- function(call, function_name) {
  if (is.null(call$positions) || !function_name %in% c("decide", "choose", "tag", "score")) return(call)
  rows <- call$results
  values <- lapply(rows, function(row) if (function_name == "tag") as.character(row$value) else row$value)
  na <- switch(function_name, decide = NA, choose = NA_character_, score = NA_real_, NULL)
  scalar <- function(value) is.null(value) || (length(value) == 1L && typeof(value) == typeof(na))
  view <- if (!is.null(na) && all(vapply(values, scalar, TRUE))) rep(na, call$length) else rep(list(switch(function_name, tag = character(), NA)), call$length)
  at <- call$positions[seq_along(values)] + 1L
  if (is.list(view)) view[at] <- values
  else view[at] <- vapply(values, function(value) if (is.null(value)) na else value, na)
  call$value <- view
  if (function_name %in% c("decide", "choose")) {
    probability <- rep(NA_real_, call$length)
    probability[at] <- vapply(rows, function(row) {
      if (function_name == "decide") row$answer$probability
      else if (is.null(row$value)) NA_real_ else row$answer$probabilities[[row$value]]
    }, 0)
    call$probability <- probability
  }
  call
}
