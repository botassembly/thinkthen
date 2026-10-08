source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
question <- list(role = "atomic", body = list(decide = "Q?"))
record <- list(content = list(kind = "text", value = "kept"))
reader <- list(reading = list(unit = "file"), media = "text")
missing <- tempfile("unread-")

for (extra in list(list(paths = list()), list(paths = NULL), list(options = NULL),
                   list(jsonl = FALSE), list(jsonl = NULL))) {
  input <- c(list(kind = "records", records = list(record)), extra)
  before <- backend_count()
  error <- tryCatch(tt_decide_complete(question, input), error = identity)
  check("record input refuses a file field", inherits(error, "thinkthen_error") &&
        error$kind == "usage" && grepl("records cannot include file source fields", conditionMessage(error), fixed = TRUE))
  check("record conflict sends nothing", backend_count() == before)
}

for (records in list(list(), NULL)) {
  input <- list(kind = "files", paths = list(missing), options = reader)
  input["records"] <- list(records)
  before <- backend_count()
  error <- tryCatch(tt_decide_complete(question, input), error = identity)
  check("file input refuses an explicit record field before reading", inherits(error, "thinkthen_error") &&
        error$kind == "usage" && grepl("files cannot include records", conditionMessage(error), fixed = TRUE))
  check("file conflict leaves the missing path unread and sends nothing", !file.exists(missing) && backend_count() == before)
}

records <- tt_decide_complete(question, list(kind = "records", records = list(record)))
check("valid records retain the original", identical(records$inputs[[1L]]$original, "kept"))
file <- tempfile("input-")
writeLines("from file", file)
files <- tt_decide_complete(question, list(kind = "files", paths = list(file), options = reader, jsonl = FALSE))
check("valid files retain their source", identical(files$inputs[[1L]]$original, "from file\n"))
batch <- tt_decide_batch(question, list(kind = "records", records = list(list(content = list(kind = "text", value = "batch record")))))
row <- batch$next_row()
check("valid batches retain their original", identical(row$input$original, "batch record"))
check("batch completes", is.null(batch$next_row()))
finish("complete_input_admission", 3L)
