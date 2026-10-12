# The R half of the surface. The Rust half runs one engine call and converts
# the answer; this half carries the ruled shape: the tt_ prefix, NA as "not
# sure", a column in and a column out, and the six error kinds as R
# conditions with the retry signal.

# Every call runs through here (ADR 0042). The guarded check runs before the
# crossing is forced and after it returns, and the Rust half checks before
# its worker starts and at each 100 ms tick. A user's options(error = ...)
# hook is held aside while the checks run and restored before anything is
# raised, because R consults the hook at signal time (R7-13).
.tt_call <- function(expr) {
  had_hook <- getOption("error")
  if (!is.null(had_hook)) {
    options(error = NULL)
    on.exit(options(error = had_hook), add = TRUE)
  }
  held <- if (tt_interrupt_pending()) {
    list(interrupt = TRUE)
  } else {
    tryCatch(list(value = .tt_native(expr)), error = function(e) list(cond = e))
  }
  if (is.null(held$cond) && is.null(held$interrupt) && tt_interrupt_pending()) {
    held <- list(interrupt = TRUE)
  }
  if (!is.null(had_hook)) options(error = had_hook)
  if (!is.null(held$interrupt)) .tt_interrupt()
  if (inherits(held$cond, "thinkthen_interrupt")) .tt_interrupt()
  if (!is.null(held$cond)) stop(held$cond)
  held$value
}

.tt_native <- function(expr) {
  value <- expr
  if (inherits(value, "extendr_error")) stop(value$value)
  if (inherits(value, "thinkthen_error")) stop(value)
  value
}

# Claim before evaluating any asking argument or the first guarded interrupt
# check. An early exit settles only a still-claimed receipt; a running worker
# owns final settlement after its already-sent work ends.
.tt_asking <- function(completion, expr) {
  if (!is.null(completion)) {
    .tt_native(tt_completion_claim(completion))
  }
  early <- "interrupt"
  on.exit(if (!is.null(completion))
    try(.tt_native(tt_completion_settle_early(completion, early)), silent = TRUE), add = TRUE)
  tryCatch({
    value <- expr
    early <- "success"
    value
  }, error = function(e) {
    early <<- if (inherits(e, "thinkthen_error")) e$kind else "local"
    stop(e)
  })
}

# R catches names its formal arguments do not use. Every caught name is a
# usage error, including the old deadline spelling and core-only settings.
.tt_unknown <- function(...) {
  names <- names(list(...))
  if ("deadline" %in% names) .tt_usage("deadline was renamed deadline_ms, in milliseconds")
  .tt_usage(paste0("unknown keyword: ", if (length(names)) names[[1L]] else "unnamed argument"))
}

tt_completion <- function() tt_completion_new()
tt_completion_read <- function(handle) jsonlite::parse_json(.tt_call(tt_completion_read_native(handle)))

.tt_usage <- function(message) .tt_call(tt_error_native("usage", message))
.tt_local <- function(message) .tt_call(tt_error_native("local", message))

# R's own interrupt, delivered for real: the guarded check consumed the
# pending signal, so the process sends itself SIGINT and sleeps, and the
# jump lands in R's own machinery with no Rust frame under it. If the
# signal does not land, the synthetic condition below still stops the call.
.tt_interrupt <- function() {
  delivered <- FALSE
  if (.Platform$OS.type == "unix") {
    try(delivered <- tools::pskill(Sys.getpid(), 2L), silent = TRUE)
    if (isTRUE(delivered)) Sys.sleep(0.1)
  }
  had_hook <- getOption("error")
  if (!is.null(had_hook)) {
    options(error = NULL)
    on.exit(options(error = had_hook), add = TRUE)
  }
  stop(structure(class = c("interrupt", "condition"), list(message = "", call = NULL)))
}

# The one JSON writer: every string reaches jsonlite as UTF-8 under any
# locale. Native bytes that are valid UTF-8 are UTF-8, latin1 converts, and
# native bytes that are not valid UTF-8 are refused by name (R5-10).
.tt_json <- function(value) {
  utf8 <- function(text) {
    if (!is.character(text) || !length(text)) return(text)
    present <- !is.na(text)
    text[present] <- .tt_call(tt_text_utf8(text[present]))
    text
  }
  held <- rapply(list(value), utf8, classes = "ANY", how = "replace")[[1L]]
  tryCatch(as.character(jsonlite::toJSON(held, auto_unbox = TRUE, digits = NA,
                                       na = "null", null = "null")),
           error = function(e) .tt_usage("the question cannot be written as JSON"))
}


tt_usage <- function() jsonlite::parse_json(.tt_call(tt_usage_counters()))

# Live durability for the selected native engine; no facts-snapshot timestamp.
tt_usage_persistence <- function() .tt_call(tt_usage_persistence_native())
tt_finish_usage_status <- function() .tt_call(tt_finish_usage_status_native())


tt_engine <- function(base_url = NULL, model = NULL, throttle = NULL, max_requests = NULL,
                      max_requests_total = NULL, max_request_bytes = NULL,
                      cache = NULL, timeout = NULL, max_retries = NULL,
                      record = NULL, replay = NULL, profile = NULL, batch = NULL,
                      ..., backend = NULL, refresh_cache = FALSE) {
  if (length(list(...))) .tt_unknown(...)
  .tt_call(tt_engine_set(base_url, model, throttle, max_requests, max_requests_total,
                        max_request_bytes, cache, timeout, max_retries, record,
                        replay, profile, batch, backend, refresh_cache))
  invisible(NULL)
}
