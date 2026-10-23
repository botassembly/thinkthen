# Install only built candidate packages in an empty R library.
args <- commandArgs(TRUE)
root <- args[[1]]
expected <- trimws(readLines(file.path(root, "r/r-version")))
stopifnot(identical(as.character(getRversion()), expected))
lib <- Sys.getenv("R_LIBS_USER")
dir.create(lib, recursive = TRUE, showWarnings = FALSE)
.libPaths(c(lib, .Library))
archives <- list.files(file.path(root, "r"), pattern = "\\.(tgz|tar.gz)$", full.names = TRUE)
stopifnot(length(archives) == 2L)
install_binary <- function(path) {
  # A built-only archive cannot silently fall back to compiling source.
  files <- utils::untar(path, list = TRUE)
  description <- files[grepl("^[^/]+/DESCRIPTION$", files)]
  stopifnot(length(description) == 1L)
  metadata <- tempfile()
  dir.create(metadata)
  utils::untar(path, files = description, exdir = metadata)
  fields <- read.dcf(file.path(metadata, description))
  stopifnot("Built" %in% colnames(fields))
  unlink(metadata, recursive = TRUE)
  status <- system2(file.path(R.home("bin"), "R"), c("CMD", "INSTALL", paste0("--library=", shQuote(lib)), shQuote(path)))
  stopifnot(status == 0L)
}
for (path in archives[order(!grepl("jsonlite", archives))]) install_binary(path)
stopifnot(identical(normalizePath(find.package("jsonlite")), normalizePath(file.path(lib, "jsonlite"))))
stopifnot(identical(normalizePath(find.package("thinkthen")), normalizePath(file.path(lib, "thinkthen"))))
cases <- jsonlite::fromJSON(file.path(root, "checks/stub.json"), simplifyVector = FALSE)
canonical <- function(value) {
  if (!is.list(value)) return(value)
  if (!is.null(names(value))) value <- value[order(names(value))]
  lapply(value, canonical)
}
for (case in cases) {
  path <- tempfile(fileext = ".json")
  jsonlite::write_json(case, path, auto_unbox = TRUE, null = "null", digits = NA)
  output <- system2(file.path(R.home("bin"), "Rscript"), c("--vanilla", shQuote(file.path(root, "checks/run.R")), shQuote(path)), stdout = TRUE)
  result <- jsonlite::fromJSON(paste(output, collapse = "\n"), simplifyVector = FALSE)
  if (case$request[["function"]] == "filter") {
    expected <- Filter(Negate(is.null), Map(function(record, row) if (isTRUE(row$answer)) record else NULL, case$request$records, case$expected))
    stopifnot(identical(result$selected, expected))
  } else {
    comparison <- all.equal(canonical(result$rows), canonical(case$expected), check.attributes = FALSE, tolerance = 0)
    if (!isTRUE(comparison)) stop(paste(case$request[["function"]], paste(comparison, collapse = "; ")))
  }
  unlink(path)
}
cat("candidate R: ten shared stub cases pass from binary packages\n")
source(file.path(root, "checks/r-frames.R"))
