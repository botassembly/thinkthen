# Run the existing checked first example and complete table examples offline.
lines <- readLines(commandArgs(TRUE)[[1]], warn = FALSE)
starts <- which(lines == "```r")
blocks <- lapply(starts, function(start) {
  end <- start + which(lines[(start + 1L):length(lines)] == "```")[[1]]
  paste(lines[(start + 1L):(end - 1L)], collapse = "\n")
})
eval(parse(text = blocks[[1]]))
stopifnot(nrow(refunds) == 2L,
          all(c("status", "answer", "probability") %in% names(refunds)))
cat("R README code example passed\n")
