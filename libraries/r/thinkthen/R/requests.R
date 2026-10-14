# R names and value conversion only. Native admission owns every request rule.
.tt_selector <- function(value, family) {
  if (inherits(value, family)) return(unclass(value))
  if (is.character(value) && length(value) == 1L) list(kind = "text", text = value)
  else list(kind = "json", value = value)
}

tt_item <- function(original, context = NULL, options = NULL, seed_spans = NULL, examples = NULL, images = NULL) {
  fields <- list(original = .tt_selector(original, "thinkthen_input"), context = context, options = options,
                 seed_spans = seed_spans, examples = examples, images = images)
  structure(fields[!vapply(fields, is.null, TRUE)], class = c("thinkthen_item", "thinkthen_complete"))
}

tt_input <- function(kind, ...) structure(c(list(kind = kind), list(...)), class = "thinkthen_input")

# These selectors distinguish authority and evidence; they perform no admission.
tt_question <- function(value = NULL, file, name, reference) {
  selected <- if (!missing(file)) list(kind = "file", path = path.expand(file))
    else if (!missing(name)) list(kind = "name", name = name)
    else if (!missing(reference)) list(kind = "reference", reference = reference)
    else if (inherits(value, "thinkthen_question")) unclass(value)
    else if (is.character(value) && length(value) == 1L) list(kind = "text", text = value)
    else list(kind = "definition", value = value)
  structure(selected, class = "thinkthen_question")
}
tt_files <- function(paths, unit = "line", window = NULL, media = "text") {
  reading <- list(unit = unit)
  if (!is.null(window)) reading$window <- window
  structure(list(kind = "source", source = list(paths = I(paths), reading = reading, media = media)),
            class = "thinkthen_input")
}

.tt_request <- function(function_name, question, input, options) {
  question <- structure(.tt_json(if (inherits(question, "thinkthen_question")) unclass(question) else question),
                        selector = inherits(question, "thinkthen_question"), literal = is.character(question) && length(question) == 1L)
  column <- is.null(input) || (is.atomic(input) && !is.object(input) && !is.raw(input))
  if (inherits(input, "thinkthen_feed")) input <- structure(input$header, class = "thinkthen_input")
  if (inherits(input, "thinkthen_input")) input <- .tt_json(unclass(input))
  else {
    collection <- is.data.frame(input) || (column && length(input) != 1L)
    if (is.data.frame(input)) input <- lapply(seq_len(nrow(input)), function(at) lapply(input, `[[`, at))
    scalar <- if (collection) NULL else structure(.tt_json(if (inherits(input, "thinkthen_item")) unclass(input) else input), text = is.character(input) && length(input) == 1L, item = inherits(input, "thinkthen_item"))
    cells <- if (inherits(input, "thinkthen_item")) list() else unname(lapply(input, function(value) structure(.tt_json(if (inherits(value, "thinkthen_input") || inherits(value, "thinkthen_item")) unclass(value) else value),
      item = inherits(value, "thinkthen_item"), text = is.character(value) && length(value) == 1L, selector = inherits(value, "thinkthen_input"), missing = column && is.na(value))))
    input <- structure(cells, scalar = scalar, column = column)
  }
  if (is.list(options) && !length(options)) options <- NULL else options <- .tt_json(options)
  .tt_call(tt_request_build(function_name, question, input, options))
}

.tt_request_failure <- function(failure, completed = NULL, positions = NULL, length = NULL, parent = NULL) {
  .tt_call(tt_request_error_native(failure, completed, positions, length, parent))
}
.tt_named_call <- function(function_name, question, input, options, deadline_ms, completion) {
  .tt_asking(completion, {
    if (inherits(input, "thinkthen_feed")) {
      if (!is.null(completion)) .tt_usage("completion receipts are unavailable for record feeds")
      return(.tt_feed_call(function_name, question, input, options, deadline_ms))
    }
    request <- .tt_request(function_name, question, input, options)
    result <- .tt_call(tt_request_native(request, deadline_ms, completion))
    if (!is.null(result$failure)) .tt_request_failure(result$failure, result$results, result$positions, result$length)
    result
  })
}

tt_plan <- function(question, input, function_name = "decide", options = list()) {
  request <- .tt_request(function_name, question, input, options)
  .tt_call(tt_request_plan(request))
}

tt_batch <- function(function_name, question, input, options = list()) .tt_batch(function_name, question, input, options, FALSE)

.tt_batch <- function(function_name, question, input, options, collect) {
  native <- .tt_call(tt_request_batch_start(.tt_request(function_name, question, input, options), collect))
  state <- new.env(parent = emptyenv())
  state$facts <- NULL
  state$ended <- FALSE
  state$stopped <- FALSE
  state$pending <- NULL
  state$released <- FALSE
  state$producer_error <- NULL
  producer <- if (inherits(input, "thinkthen_feed")) input else NULL
  release <- function() {
    if (state$released) return(invisible(NULL))
    state$released <- TRUE
    if (!is.null(producer)) tryCatch(producer$close(), error = function(e) { if (is.null(state$producer_error)) stop(e) })
    invisible(NULL)
  }
  reg.finalizer(state, function(e) release(), onexit = TRUE)
  finish <- function(failure = NULL) {
    .tt_call(tt_request_batch_finish(native, if (is.null(failure)) NULL else .tt_json(unclass(failure))))
    state$stopped <- TRUE
    state$pending <- NULL
    release()
  }
  push <- function(record) {
    descriptor <- if (inherits(record, "thinkthen_record")) unclass(record)
      else if (inherits(record, "thinkthen_item")) list(item = unclass(record))
      else list(item = list(original = .tt_selector(record, "thinkthen_input")))
    .tt_call(tt_request_batch_push(native, .tt_json(descriptor)))
  }
  advance <- function() {
    if (is.null(producer) || is.null(producer$next_item) || state$stopped) return(invisible(NULL))
    if (is.null(state$pending)) {
      state$pending <- tryCatch(producer$next_item(), error = function(e) { state$producer_error <- e; tt_reader_failure("producer") })
      if (is.null(state$pending)) { finish(); return(invisible(NULL)) }
      if (inherits(state$pending, "thinkthen_reader_failure")) { finish(state$pending); return(invisible(NULL)) }
    }
    status <- tryCatch(push(state$pending), thinkthen_error = function(e) {
      finish(tt_reader_failure("invalid_input")); "closed"
    })
    if (status != "full") state$pending <- NULL
    if (status == "closed") { state$stopped <- TRUE; release() }
  }
  poll <- function() {
    if (state$ended) return(NULL)
    event <- .tt_call(tt_request_batch_poll(native))
    if (!is.null(event) && event$kind %in% c("terminal", "end")) {
      state$ended <- TRUE
      state$stopped <- TRUE
      state$facts <- event$facts
      release()
      if (!is.null(event$failure)) .tt_request_failure(event$failure, positions = attr(native, "positions"), length = attr(native, "length"), parent = state$producer_error)
      return(NULL)
    }
    advance()
    if (is.null(event) || identical(event$kind, "observation")) return(NULL)
    event$value
  }
  cancel <- function() {
    .tt_call(tt_request_batch_cancel(native))
    state$stopped <- TRUE
    state$pending <- NULL
    release()
    invisible(NULL)
  }
  pull <- function() {
    tryCatch(repeat {
      value <- poll()
      if (!is.null(value) || state$ended) return(value)
      .tt_call(tt_request_batch_wait(native, !state$stopped && !is.null(producer)))
    }, interrupt = function(e) { cancel(); stop(e) })
  }
  close <- function() { cancel(); state$ended <- TRUE; invisible(NULL) }
  methods <- list(next_result = pull, poll = poll, push = push, finish = finish,
                 cancel = cancel, close = close, facts = function() state$facts,
                 positions = attr(native, "positions"), length = attr(native, "length"))
  if (collect) methods$result <- function() .tt_call(tt_request_batch_result(native))
  structure(methods, class = "thinkthen_batch")
}
print.thinkthen_question <- function(x, ...) { cat("<question: content withheld>\n"); invisible(x) }
print.thinkthen_input <- function(x, ...) { cat("<input: content withheld>\n"); invisible(x) }
print.thinkthen_batch <- function(x, ...) { cat("<batch>\n"); invisible(x) }
