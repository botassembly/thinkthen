source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
missing <- tempfile("unread-")
record <- list(original = list(kind = "text", text = "kept"))
for (input in list(
  tt_input("records", items = list(record), paths = list()),
  tt_input("records", items = list(record), source = NULL),
  tt_input("source", source = list(paths = list(missing), reading = list(unit = "file"), media = "text"), items = list()),
  tt_input("records", items = NULL),
  tt_input("source", source = NULL)
)) {
  before <- backend_count()
  error <- tryCatch(tt_decide("Q?", input), error = identity)
  check("canonical source conflict refuses before reading or sending", inherits(error, "thinkthen_usage") &&
    !file.exists(missing) && backend_count() == before)
}
before <- backend_count()
error <- tryCatch(tt_decide("Q?", tt_input("records", items = list(
  list(original = list(kind = "text", text = "unreachable"), context = FALSE)
))), error = identity)
check("invalid selector context retains native diagnosis before sending", inherits(error, "thinkthen_usage") &&
  identical(conditionMessage(error), "the per-item context does not match context_schema") &&
  !isTRUE(error$retryable) && inherits(error$facts, "thinkthen_absent") && backend_count() == before)
records <- tt_decide("Q?", tt_input("records", items = list(record)))
check("valid record retains the original", identical(records$results[[1L]]$input, "kept"))
file <- tempfile("input-")
writeLines("from file", file)
files <- tt_decide("Q?", tt_files(file, unit = "file"))
check("valid source retains its text", identical(files$results[[1L]]$input, "from file\n"))
unlink(file)
batch <- tt_batch("decide", "Q?", "batch record")
row <- batch$next_result()
check("valid session retains its original", identical(row$input, "batch record"))
check("session completes", is.null(batch$next_result()))
batch$close()
finish("source admission", 3L)
