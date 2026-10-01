# Shared by every R test file that with-backend.sh runs: the installed
# package, one check counter, and the backend's count and round lines.
library(thinkthen)
source(file.path(Sys.getenv("TT_TESTS"), "..", "..", "..", "conformance", "children", "children.R"))

# What a child keeps from this process (ticket 0127): the package library, the
# scratch home and XDG folders, and the tests folder a child may source.
inherited <- c("R_LIBS", "HOME", "XDG_CACHE_HOME", "XDG_CONFIG_HOME", "TT_TESTS", "TT_BACKEND_ORIGIN")

# The values a child is given: this process's address and fake key, read by
# name, its own cache, then the caller's `NAME=value` pairs, which win.
child_values <- function(cache, env = character()) {
  c(THINKTHEN_BASE_URL = Sys.getenv("THINKTHEN_BASE_URL"), THINKTHEN_API_KEY = Sys.getenv("THINKTHEN_API_KEY"),
    THINKTHEN_CACHE = cache, setNames(sub("^[^=]*=", "", env), sub("=.*", "", env)))
}

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

# The backend's count once it reads at least n, or at 30 s, from its `wait` line.
backend_wait <- function(n) {
  out <- Sys.getenv("TT_BACKEND_OUT")
  before <- length(grep("^wait ", readLines(out)))
  backend_say(paste("wait", n))
  repeat {
    lines <- grep("^wait ", readLines(out), value = TRUE)
    if (length(lines) > before) return(as.integer(sub("^wait ", "", lines[[length(lines)]])))
    Sys.sleep(0.02)
  }
}

# Millisecond promises run only under the stress profile (ticket 0356).
stress <- identical(Sys.getenv("THINKTHEN_TEST_PROFILE"), "stress")

capture <- function() {
  output <- Sys.getenv("TT_BACKEND_OUT")
  before <- length(grep("^\\{", readLines(output)))
  backend_say("capture")
  repeat {
    lines <- grep("^\\{", readLines(output), value = TRUE)
    if (length(lines) > before) {
      return(jsonlite::fromJSON(lines[[length(lines)]], simplifyVector = FALSE)$bodies)
    }
    Sys.sleep(0.02)
  }
}

# A base address on this file's backend, for an arm other than generic.
arm <- function(path) paste0(Sys.getenv("TT_BACKEND_ORIGIN"), "/", path)

# The exchange identity over the actual loopback address and a literal body.
digest <- function(url, body) {
  file <- tempfile()
  on.exit(unlink(file))
  writeBin(charToRaw(paste0("systemone\n", url, "\n", body)), file)
  if (nzchar(Sys.which("sha256sum"))) {
    answer <- system2("env", c(clean_env(), shQuote(Sys.which("sha256sum")), shQuote(file)), stdout = TRUE)
  } else if (nzchar(Sys.which("shasum"))) {
    answer <- system2("env", c(clean_env(), shQuote(Sys.which("shasum")), "-a", "256", shQuote(file)), stdout = TRUE)
  } else stop("the installed SHA-256 host tool is absent", call. = FALSE)
  strsplit(answer[[1L]], " ", fixed = TRUE)[[1L]][[1L]]
}

# The raw members of one compact JSON object, as the body spells them.
raw_members <- function(text) {
  chars <- strsplit(text, "", fixed = TRUE)[[1L]]
  at <- 2L
  members <- list()
  string_end <- function(i) {
    i <- i + 1L
    while (chars[[i]] != "\"") i <- i + if (chars[[i]] == "\\") 2L else 1L
    i
  }
  value_end <- function(i) {
    if (chars[[i]] == "\"") return(string_end(i))
    if (!(chars[[i]] %in% c("{", "["))) {
      while (!(chars[[i + 1L]] %in% c(",", "}", "]"))) i <- i + 1L
      return(i)
    }
    depth <- 0L
    repeat {
      if (chars[[i]] == "\"") i <- string_end(i)
      else if (chars[[i]] %in% c("{", "[")) depth <- depth + 1L
      else if (chars[[i]] %in% c("}", "]")) { depth <- depth - 1L; if (depth == 0L) return(i) }
      i <- i + 1L
    }
  }
  while (chars[[at]] != "}") {
    close <- string_end(at)
    name <- paste(chars[(at + 1L):(close - 1L)], collapse = "")
    end <- value_end(close + 2L)
    members[[name]] <- paste(chars[(close + 2L):end], collapse = "")
    at <- end + 1L
    if (chars[[at]] == ",") at <- at + 1L
  }
  members
}

# Every question key of one request body, in wire order, by ADR 0111
# section 2: the adapter, the URL, the model, the state and one question as
# the body carries them, joined by line feeds.
question_keys <- function(url, body) {
  parts <- raw_members(body)
  questions <- raw_members(parts$questions)
  questions <- questions[order(as.integer(sub("^q", "", names(questions))))]
  vapply(questions, function(one) digest(url, paste(parts$model, parts$state, one, sep = "\n")), "",
         USE.NAMES = FALSE)
}

# The count a call adds.
sent_by <- function(expr) {
  before <- backend_count()
  force(expr)
  backend_count() - before
}

# A fresh Rscript over `code` with only the kept names, the given values, and
# its own cache folder, since a cache folder belongs to one backend address.
# Its output and status.
child <- function(code, env = character()) {
  cache <- tempfile("cache-")
  dir.create(cache)
  file <- tempfile(fileext = ".R")
  writeLines(c("library(thinkthen)", code), file)
  out <- suppressWarnings(system2("env", c(clean_env(inherited, child_values(cache, env)),
                                           shQuote(file.path(R.home("bin"), "Rscript")), shQuote(file)),
                                  stdout = TRUE, stderr = TRUE))
  status <- attr(out, "status")
  list(text = paste(out, collapse = "\n"), status = if (is.null(status)) 0L else status)
}

# The file's own total of requests, which with-backend.sh compares with the
# backend's final count.
finish <- function(name, expected) {
  cat(name, ": ", passed, " checks passed\n", sep = "")
  cat("expect count ", expected, "\n", sep = "")
}
