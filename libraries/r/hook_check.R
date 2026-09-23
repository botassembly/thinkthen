# The error-hook ordering, as a suite file (fourth review item 10): R
# consults options(error) at signal time, not unwind time, so the old
# shape - hook held aside for the crossing, restored by on.exit - let a
# thinkthen error skip the hook, continue the script, and exit zero.
# The fixed shape restores the hook before raising, so a thinkthen
# error meets exactly what a plain stop meets: the hook runs, the script
# continues per the hook's own choice, and without a hook the script
# halts nonzero.
.libPaths(c("rlib", .libPaths()))
library(thinkthen)
stopifnot(Sys.getenv("ENGINE_NULL") == "1")

run <- function(code) {
  file <- tempfile(fileext = ".R")
  writeLines(c(
    '.libPaths(c("rlib", .libPaths()))',
    'library(thinkthen)',
    code
  ), file)
  out <- suppressWarnings(system2(
    file.path(R.home("bin"), "Rscript"),
    c(file),
    stdout = TRUE, stderr = TRUE
  ))
  status <- attr(out, "status")
  list(text = paste(out, collapse = "\n"), status = if (is.null(status)) 0L else status)
}

failed <- FALSE
check <- function(what, held) {
  if (!isTRUE(held)) {
    failed <<- TRUE
    cat("FAIL ", what, "\n", sep = "")
  } else {
    cat("ok    ", what, "\n", sep = "")
  }
}

# A set hook runs for a thinkthen error and the script continues per the
# hook's choice - identical to a plain stop under a hook.
held <- run(paste(
  'options(error = function() cat("HOOK RAN\n"))',
  'tt_decide("q", "x", deadline = NA)',
  'cat("AFTER\n")',
  sep = "\n"
))
check("the hook runs for an engine error", grepl("HOOK RAN", held$text, fixed = TRUE))
check("the script continues under a hook, as for a plain stop", grepl("AFTER", held$text, fixed = TRUE))

# Without a hook the script halts nonzero, as for any uncaught error.
bare <- run(paste(
  'tt_decide("q", "x", deadline = NA)',
  'cat("NEVER\n")',
  sep = "\n"
))
check("without a hook the error halts the script", !grepl("NEVER", bare$text, fixed = TRUE))
check("without a hook the exit status is nonzero", bare$status != 0L)

# A plain stop under the same hook: the reference behavior itself.
plain <- run(paste(
  'options(error = function() cat("HOOK RAN\n"))',
  'stop("plain")',
  'cat("AFTER\n")',
  sep = "\n"
))
check(
  "the thinkthen error and the plain stop agree under a hook",
  grepl("HOOK RAN", plain$text, fixed = TRUE) && grepl("AFTER", plain$text, fixed = TRUE)
)

if (failed) quit(status = 1)
cat("the error hook ordering holds\n")
