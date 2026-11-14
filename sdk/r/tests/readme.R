# Run the installed README quickstart. The literal runner owns every guide
# branch and refusal; the publication check compares the full mirrored source.
lines <- readLines(commandArgs(TRUE)[[1]], warn = FALSE)
start <- which(lines == "```r")[[1L]]
end <- start + which(lines[(start + 1L):length(lines)] == "```")[[1L]]
source <- paste(lines[(start + 1L):(end - 1L)], collapse = "\n")
eval(parse(text = source), envir = new.env(parent = globalenv()))
cat("Installed R README quickstart passed\n")
