# Producers run only on R's thread. Native sessions own admission and framing.
tt_feed <- function(next_item = NULL, name = "records", close = function() invisible(NULL)) {
  header <- list(kind = "feed", name = name)
  structure(list(header = header, next_item = next_item, close = close), class = "thinkthen_feed")
}
tt_record <- function(original, location = NULL, ...) {
  descriptor <- list(item = c(list(original = .tt_selector(original, "thinkthen_input")), list(...)))
  if (!is.null(location)) descriptor$location <- location
  structure(descriptor, class = "thinkthen_record")
}
tt_reader_failure <- function(kind, location = NULL) {
  failure <- list(kind = kind)
  if (!is.null(location)) failure$location <- location
  structure(failure, class = "thinkthen_reader_failure")
}
.tt_feed_call <- function(function_name, question, input, options, deadline_ms) {
  if (!is.null(deadline_ms)) options$deadline_ms <- deadline_ms
  stream <- tt_batch(function_name, question, input, options)
  on.exit(stream$close())
  results <- list()
  tryCatch(repeat {
    value <- stream$next_result()
    if (is.null(value)) break
    if (inherits(value, "thinkthen_complete")) results[[length(results) + 1L]] <- value
    else results <- c(results, value)
  }, thinkthen_error = function(e) {
    e$completed <- results
    stop(e)
  })
  structure(list(results = results, facts = stream$facts()), class = c("thinkthen_Call", "thinkthen_complete"))
}
print.thinkthen_feed <- function(x, ...) { cat("<feed: content withheld>\n"); invisible(x) }
print.thinkthen_record <- function(x, ...) { cat("<record: content withheld>\n"); invisible(x) }
print.thinkthen_reader_failure <- function(x, ...) { cat("<reader failure: content withheld>\n"); invisible(x) }
