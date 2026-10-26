library(thinkthen)
secret <- "synthetic-private-host-class-never-print"
opaque <- structure(function() NULL, class = secret)
url <- Sys.getenv("THINKTHEN_TEST_URL")
stopifnot(nzchar(url))

expect_usage <- function(expression, code) {
  error <- tryCatch({ force(expression); NULL }, error = identity)
  stopifnot(inherits(error, "thinkthen_usage_error"), identical(error$code, code),
            is.null(error$call), nzchar(conditionMessage(error)),
            !any(grepl(secret, capture.output(str(error)), fixed = TRUE)))
  conditionMessage(error)
}
expect_caller_error <- function(expression) {
  error <- tryCatch({ force(expression); NULL }, error = identity)
  stopifnot(inherits(error, "error"), !inherits(error, "thinkthen_error"),
            identical(conditionMessage(error), "caller iteration failed"))
}
`[[.broken_records` <- function(x, ...) stop("caller iteration failed", call. = FALSE)

for (backend in c("systemone", "openai")) {
  settings <- list(backend = backend, url = url, api_key = secret,
                   retry = list(max_retries = 0L))
  client <- do.call(tt_client, settings)
  settings_messages <- character()
  bad_models <- list(NA_character_, NA_real_, NA_integer_, NA, NaN, Inf, -Inf,
                     opaque)
  for (bad in bad_models) {
    args <- settings
    args$model <- bad
    settings_messages <- c(settings_messages,
      expect_usage(do.call(tt_client, args), "usage.settings"))
  }
  for (field in c("backend", "api_key", "url", "batch")) {
    args <- settings
    args[field] <- list(NA_character_)
    settings_messages <- c(settings_messages,
      expect_usage(do.call(tt_client, args), "usage.settings"))
  }
  for (field in c("max_retries", "initial_backoff_ms", "attempt_timeout_ms")) {
    args <- settings
    args$retry <- setNames(list(Inf), field)
    settings_messages <- c(settings_messages,
      expect_usage(do.call(tt_client, args), "usage.settings"))
  }
  stopifnot(length(unique(settings_messages)) == 1L)

  # Every question route preserves the call-level context before record admission.
  question_calls <- list(
    function() tt_decide("text", opaque, client = client),
    function() tt_decide("text", NA_character_, client = client),
    function() tt_decide("text", "Ready?", threshold = NA_real_, client = client),
    function() tt_decide("text", "Ready?", threshold = c(.2, Inf), client = client),
    function() tt_decide("text", "Ready?", evidence = NA_character_, client = client),
    function() tt_choose("text", "Which?", choices = c("a", NA_character_), client = client),
    function() tt_tag("text", "Tags?", labels = NA_character_, client = client),
    function() tt_score("text", "How?", levels = list("low", opaque), client = client),
    function() tt_rank("text", "How?", levels = c("low", "high"), top = Inf, client = client),
    function() tt_find("text", "Where?", threshold = NaN, client = client),
    function() tt_recognize("text", kinds = c("person", NA_character_), client = client),
    function() tt_relate("text", list(list(source = "person", target = "person", relation = opaque)), client = client),
    function() tt_annotate("text", list(q = list(`function` = "decide", question = opaque)), client = client),
    function() tt_question(list(q = list(`function` = "decide", question = NA_character_)), "q"),
    function() tt_question(list(q = list(`function` = "decide", question = "Ready?", threshold = Inf)), "q"),
    function() tt_question(list(q = list(`function` = "decide", question = "Ready?")), NA_character_),
    function() tt_question(list(q = list(`function` = "decide", question = "Ready?")), opaque)
  )
  question_messages <- vapply(question_calls, function(call) {
    expect_usage(call(), "usage.question")
  }, character(1))
  stopifnot(length(unique(question_messages)) == 1L)

  # jsonlite represents an unclassed function as a sequence. Native validation
  # owns this representable value and keeps its semantic error message.
  args <- settings
  args$model <- function() NULL
  expect_usage(do.call(tt_client, args), "usage.settings")

  # Finite semantic validation still belongs to the engine.
  error <- tryCatch(tt_decide("text", "Ready?", threshold = 2, client = client), error = identity)
  stopifnot(inherits(error, "thinkthen_usage_error"), identical(error$code, "usage.threshold"))
  optional <- do.call(tt_client, c(settings, list(model = NULL, batch = NULL)))
  stopifnot(nrow(tt_decide(character(), "Ready?", threshold = NULL, evidence = NULL, client = optional)) == 0L)

  # The same unrepresentable value is a failed row when it belongs to a record.
  rows <- tt_decide(list(NULL, opaque, NA_character_), "Ready?", client = client)
  stopifnot(nrow(rows) == 3L, identical(rows$position, c(1, 2, 3)),
            all(rows$status == "failed"),
            identical(as.character(rows$error_code),
                      c("record.missing", "record.invalid", "record.missing")),
            !any(grepl(secret, capture.output(str(rows$error_message)), fixed = TRUE)))
  expect_caller_error(tt_decide(structure(list("text"), class = "broken_records"), "Ready?", client = client))
  expect_caller_error(tt_decide(character(), stop("caller iteration failed", call. = FALSE), client = client))
  expect_caller_error(tt_client(backend = "stub", model = stop("caller iteration failed", call. = FALSE)))
}
cat("R settings, question and record conversion contexts passed\n")
