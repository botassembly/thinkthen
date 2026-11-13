# Run every authored R example unchanged in document order.
lines <- readLines(commandArgs(TRUE)[[1]], warn = FALSE)
starts <- which(lines == "```r")
blocks <- lapply(starts, function(start) {
  end <- start + which(lines[(start + 1L):length(lines)] == "```")[[1]]
  paste(lines[(start + 1L):(end - 1L)], collapse = "\n")
})
stopifnot(length(blocks) > 0L)
# scripts/surface-example runs the guide's collection example and checks its rows.
collection <- which(vapply(blocks, grepl, logical(1), pattern = "records.jsonl", fixed = TRUE))
if (length(collection)) blocks <- blocks[-collection[[1L]]]
scope <- new.env(parent = globalenv())
for (block in blocks) {
  expressions <- parse(text = block)
  # Installation instructions target the package already installed by this gate.
  installation <- vapply(expressions, function(expr) is.call(expr) &&
    identical(expr[[1L]], quote(install.packages)), logical(1))
  if (!all(installation)) eval(expressions, envir = scope)
}
cat("R documentation examples passed:", length(blocks), "\n")
