source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))

stub <- tt_client(backend = "stub")
case <- commandArgs(TRUE)[[1]]
if (case == "rank") {
  # Feature: rank vectors attach each answer to its original input, including top.
  input <- c("alpha", "bravo", "charlie")
  rows <- tt_rank(details = TRUE, input, "How much?", c("low", "medium", "high"), client = stub)
  cat("rank fixture positions:", rows$position, "answers:", rows$answer, "\n")
  ranks <- tt_rank(input, "How much?", c("low", "medium", "high"), client = stub)
  stopifnot(identical(ranks, c(3, 1, 2)))
  stopifnot(identical(tt_answer(rows), c(3, 1, 2)))
  limited <- tt_rank(input, "How much?", c("low", "medium", "high"), top = 1L, client = stub)
  stopifnot(identical(limited, c(NA_real_, NA_real_, 1)))
  empty <- tt_rank(character(), "How much?", c("low", "high"), client = stub)
  stopifnot(identical(empty, numeric()))
} else if (case == "client") {
  # Feature: local client scopes select the requested backend and restore on exit.
  previous <- tt_client(backend = "openai", api_key = "synthetic-local-test-key",
    url = Sys.getenv("THINKTHEN_TEST_URL"), retry = tt_retry(max_retries = 0L))
  with_tt_client(NULL, {
    (function() {
      local_tt_client(stub)
      stopifnot(identical(as.character(tt_decide(details = TRUE, "text", "Ready?")$source[[1]]$backend), "stub"))
    })()
  })
  tt_close(previous)
  with_tt_client(previous, {
    (function() {
      local_tt_client(stub)
      stopifnot(identical(as.character(tt_decide(details = TRUE, "text", "Ready?")$source[[1]]$backend), "stub"))
      tryCatch((function() {
        local_tt_client(previous)
        stop("scope exit")
      })(), error = function(e) NULL)
      stopifnot(identical(as.character(tt_decide(details = TRUE, "text", "Ready?")$source[[1]]$backend), "stub"))
    })()
    condition(tt_decide(details = TRUE, "text", "Ready?"), "thinkthen_usage_error", "usage.closed")
  })
  tt_close(previous)
} else stop("Unknown wrong-answer case")
tt_close(stub)
cat("R", case, "public regression passed\n")
