# Feature: large frames cross the serializer once, independently of row count.
local({
  .GlobalEnv$.thinkthen_frame_calls <- 0L
  trace("toJSON", where = asNamespace("jsonlite"), print = FALSE,
    tracer = quote(if (is.data.frame(x)) {
      .GlobalEnv$.thinkthen_frame_calls <- .GlobalEnv$.thinkthen_frame_calls + 1L
    }))
  on.exit({
    untrace("toJSON", where = asNamespace("jsonlite"))
    rm(".thinkthen_frame_calls", envir = .GlobalEnv)
  })
  source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
  stub <- tt_client(backend = "stub")
  large <- data.frame(body = rep("βeta", 100000L))
  result <- tt_filter(large, "Ready?", evidence = "/body", client = stub)
  message("Frame serialization calls: ", .GlobalEnv$.thinkthen_frame_calls)
  stopifnot(.GlobalEnv$.thinkthen_frame_calls == 1L, nrow(result) == nrow(large),
    identical(result$position, as.double(seq_len(nrow(large)))),
    all(result$status == "answered"), identical(result$kept, result$answer))
})
cat("R single-call large frame encoding passed\n")
