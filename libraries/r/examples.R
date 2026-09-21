# Every R function's example, run as one test.
#
# The file `examples.json` is keyed by function: the call and the answer
# the null backend gives, plus any question files the call names. The
# site's function pages and surface pages draw their R tab from this file
# (site.md's one source), so an example nobody runs cannot reach the site.
#
# One fresh Rscript per example keeps every expected answer
# deterministic. Offline, no stub, no key. Run by `check.sh`.
#
# Run with: ENGINE_NULL=1 Rscript examples.R

.libPaths(c("rlib", .libPaths()))
library(thinkthen)

file <- jsonlite::fromJSON("examples.json", simplifyVector = FALSE)
examples <- file$examples
failed <- 0

run_one <- function(example) {
  workdir <- tempfile("example-")
  dir.create(workdir)
  on.exit(unlink(workdir, recursive = TRUE), add = TRUE)
  for (name in names(example$files %||% list())) {
    writeLines(example$files[[name]], file.path(workdir, name))
  }
  program <- paste0(".libPaths(c('", normalizePath("rlib"), "', .libPaths())); library(thinkthen); setwd('",
                    workdir, "'); ", example$r)
  out <- suppressWarnings(system2(
    file.path(R.home("bin"), "Rscript"),
    c("-e", shQuote(program)),
    stdout = TRUE, stderr = TRUE, env = c("ENGINE_NULL=1"), wait = TRUE
  ))
  paste(out, collapse = "\n")
}

`%||%` <- function(one, two) if (is.null(one)) two else one

for (name in names(examples)) {
  example <- examples[[name]]
  got <- run_one(example)
  if (grepl(example$expected, got, fixed = TRUE)) {
    cat(paste0("ok       ", name, "\n"))
  } else {
    cat(paste0("FAILED   ", name, ": want ", example$expected, ", got ", got, "\n"))
    failed <- failed + 1
  }
}
cat(sprintf("%d of %d examples ok\n", length(examples) - failed, length(examples)))
if (failed > 0) stop("examples failed", call. = FALSE)
