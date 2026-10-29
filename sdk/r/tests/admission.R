# Feature: shared serialized admission cases use the installed public boundaries.
library(thinkthen)
cases <- jsonlite::fromJSON(commandArgs(TRUE)[[1]], simplifyVector = FALSE)
for (case in cases) {
  message <- case$message
  result <- tryCatch({
    if (case$boundary == "settings") {
      do.call(tt_client, case$input)
    } else {
      value <- case$input
      if (case$boundary == "question") {
        value <- list(entries = list(admission = list(question = value)))
        message <- paste0("toolbox.entries.[0].", message)
      }
      tt_toolbox_from_json(as.character(jsonlite::toJSON(value, auto_unbox = TRUE, null = "null")))
    }
    NULL
  }, thinkthen_error = identity)
  stopifnot(inherits(result, "thinkthen_usage_error"), identical(result$code, case$code),
            identical(conditionMessage(result), message),
            !grepl("secret-placeholder", conditionMessage(result), fixed = TRUE))
}
cat("R shared settings and serialized question admission passed\n")
