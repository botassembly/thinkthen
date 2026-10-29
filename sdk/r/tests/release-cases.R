# Run the shared stub rows through an installed R package without any compiler.
args <- commandArgs(TRUE)
folder <- dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE)))
cases <- jsonlite::fromJSON(args[[1]], simplifyVector = FALSE)
canonical <- function(value) {
  if (!is.list(value)) return(value)
  if (!is.null(names(value))) value <- value[order(names(value))]
  lapply(value, canonical)
}
expected_r <- function(value) {
  if (!is.list(value)) return(value)
  if (all(c("start", "end") %in% names(value))) value$start <- value$start + 1
  if (is.numeric(value$id)) value$id <- list(integer = sprintf("%.0f", value$id))
  lapply(value, expected_r)
}
for (case in cases) {
  path <- tempfile(fileext = ".json")
  jsonlite::write_json(case, path, auto_unbox = TRUE, null = "null", digits = NA)
  output <- system2(file.path(R.home("bin"), "Rscript"), c("--vanilla", shQuote(file.path(folder, "run.R")), shQuote(path)), stdout = TRUE)
  unlink(path)
  result <- jsonlite::fromJSON(paste(output, collapse = "\n"), simplifyVector = FALSE)
  stopifnot(length(result$rows) == length(case$expected))
  comparison <- all.equal(canonical(result$rows), canonical(expected_r(case$expected)),
                          check.attributes = FALSE, tolerance = 0)
  if (!isTRUE(comparison)) stop(paste(case$request$question[["function"]], paste(comparison, collapse = "; ")))
  if (case$request$question[["function"]] == "filter") {
    stopifnot(length(result$rows) == length(case$request$records),
              any(vapply(result$rows, function(row) row$status == "failed", logical(1))))
  }
}
cat("R installed release consumer preserves every shared stub row\n")

# Feature: every named function accepts a data frame through the installed API.
source(file.path(folder, "support.R"))
client <- tt_client(backend = "stub")
quiet_projection <- function(value) withCallingHandlers(value, warning = function(warning) {
  stopifnot(inherits(warning, "thinkthen_failed_rows"))
  invokeRestart("muffleWarning")
})
for (case in cases) {
  rows <- invoke(case$request$records, case$request$question, client, case$request$name)
  vector <- quiet_projection(invoke(case$request$records, case$request$question,
                                   client, case$request$name, vector = TRUE))
  stopifnot(identical(vector, quiet_projection(tt_answer(rows))))
}
cat("R installed vector functions preserve every shared stub answer projection\n")
frame <- data.frame(body = c("Ada works at Acme", "Bea works at Acme"), stringsAsFactors = FALSE)
records <- lapply(frame$body, function(body) list(body = body))
functions <- character()
for (case in cases) {
  q <- case$request$question
  q$evidence <- "/body"
  verb <- q[["function"]]
  actual <- invoke(frame, q, client)
  expected <- invoke(records, q, client)
  stopifnot(identical(actual[names(actual) != "item"], expected[names(expected) != "item"]))
  check_rows(actual, verb, nrow(expected))
  if (verb == "filter") {
    stopifnot(identical(actual$kept, expected$kept))
    for (i in seq_len(nrow(actual))) stopifnot(identical(actual$item[[i]], frame[actual$position[[i]], , drop = FALSE]))
  }
  functions <- c(functions, verb)
}
stopifnot(length(unique(functions)) == 10L)
tt_close(client)
cat("R installed data frames preserve all ten function projections\n")
