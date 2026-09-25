# Shared by every R test file that with-backend.sh runs: the installed
# package, one check counter, and the backend's count and round lines.
library(thinkthen)

passed <- 0L
check <- function(what, held) {
  if (!isTRUE(held)) stop(paste("failed:", what), call. = FALSE)
  passed <<- passed + 1L
}

# The kind a call raised, or "none".
kind_of <- function(expr) {
  tryCatch({ force(expr); "none" }, thinkthen_error = function(e) e$kind)
}

# The message a call raised, or "none".
message_of <- function(expr) {
  tryCatch({ force(expr); "none" }, error = function(e) conditionMessage(e))
}

# One line to the backend, and the count line it answers.
backend_say <- function(line) {
  fifo <- file(Sys.getenv("TT_BACKEND_IN"), open = "a", raw = TRUE)
  on.exit(close(fifo))
  writeLines(line, fifo)
}
backend_count <- function() {
  out <- Sys.getenv("TT_BACKEND_OUT")
  before <- length(grep("^wait ", readLines(out), invert = TRUE))
  backend_say("count")
  repeat {
    lines <- grep("^wait ", readLines(out), invert = TRUE, value = TRUE)
    if (length(lines) > before) return(as.integer(lines[[length(lines)]]))
    Sys.sleep(0.02)
  }
}

# A base address on this file's backend, for an arm other than generic.
arm <- function(path) paste0(Sys.getenv("TT_BACKEND_ORIGIN"), "/", path)

# The count a call adds.
sent_by <- function(expr) {
  before <- backend_count()
  force(expr)
  backend_count() - before
}

# A fresh Rscript over `code` with this file's environment and its own cache
# folder, since a cache folder belongs to one backend address. Its output
# and status.
child <- function(code, env = character()) {
  cache <- tempfile("cache-")
  dir.create(cache)
  env <- c(paste0("THINKTHEN_CACHE=", cache), env)
  file <- tempfile(fileext = ".R")
  writeLines(c("library(thinkthen)", code), file)
  out <- suppressWarnings(system2(file.path(R.home("bin"), "Rscript"), file,
                                  stdout = TRUE, stderr = TRUE, env = env))
  status <- attr(out, "status")
  list(text = paste(out, collapse = "\n"), status = if (is.null(status)) 0L else status)
}

# The file's own total of requests, which with-backend.sh compares with the
# backend's final count.
finish <- function(name, expected) {
  cat(name, ": ", passed, " checks passed\n", sep = "")
  cat("expect count ", expected, "\n", sep = "")
}
