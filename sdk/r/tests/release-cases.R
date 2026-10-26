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
