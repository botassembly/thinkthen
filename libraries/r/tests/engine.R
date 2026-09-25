# tt_engine: the settings over the environment's own (decision 4). The
# engine slot is process-wide, so each case runs in its own child.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
generic <- arm("generic/v1")
said <- character()
run <- function(code, env = character()) {
  held <- child(code, env)
  said <<- c(said, held$text)
  held$text
}

# The environment seeds the engine: THINKTHEN_CACHE names folder A, and a
# repeated question is answered from it (decision 18).
folder <- tempfile("seed-")
dir.create(folder)
sent <- sent_by(out <- run(c(
  sprintf('tt_engine(throttle = 8L, base_url = "%s")', generic),
  'invisible(tt_decide("Q?", "seed text")); invisible(tt_decide("Q?", "seed text"))',
  'cat("sent", tt_usage()$requests_sent, "\\n")'
), env = paste0("THINKTHEN_CACHE=", folder)))
check("the seed run counts one send", sent == 1L && grepl("sent 1", out, fixed = TRUE))
check("the answer lands in THINKTHEN_CACHE", length(list.files(folder, recursive = TRUE)) > 0L)

# A set base_url wins over the variable.
sent <- sent_by(out <- run(c(sprintf('tt_engine(base_url = "%s")', generic), 'cat(tt_decide("Q?", "over the variable"), "\\n")'),
                           env = "THINKTHEN_BASE_URL=http://127.0.0.1:1/generic/v1"))
check("a set base_url wins over THINKTHEN_BASE_URL", sent == 1L && grepl("TRUE", out, fixed = TRUE))

# One refusal an argument, each usage and each sending nothing.
refusals <- c('base_url = 5', 'model = ""', 'throttle = 0L', 'throttle = 33L', 'max_requests = -1',
              'cache = TRUE', 'cache_bytes = -1', 'throttle = 2.5', 'base_url = "ftp://example.test"')
sent <- sent_by(out <- run(c(
  sprintf('cat(tryCatch(tt_engine(%s), thinkthen_usage = function(e) "refused"), "\\n")', refusals),
  'tt_engine(throttle = 8L)',
  'cat(tryCatch(tt_engine(throttle = 4L), thinkthen_usage = function(e) conditionMessage(e)), "\\n")',
  'cat(is.null(tt_engine(throttle = 8L)), "\\n")'
)))
check("each refused argument is usage", lengths(regmatches(out, gregexpr("refused", out))) == length(refusals))
check("a second tt_engine with other settings is usage naming the settings in force",
      grepl("the engine is already set with throttle = 8; start a new R session to change it", out, fixed = TRUE))
check("equal settings a second time do nothing", grepl("TRUE", out, fixed = TRUE))
check("no refusal sends", sent == 0L)

# cache = FALSE sends a repeated question twice.
sent <- sent_by(run(c('tt_engine(cache = FALSE)', 'invisible(tt_decide("Q?", "again"))',
                      'invisible(tt_decide("Q?", "again"))')))
check("cache = FALSE sends every repeat", sent == 2L)

# max_requests: rank holds its input and refuses before any request, and a
# streaming decide sends the first record and refuses at the second.
refused <- 'function(e) conditionMessage(e)'
sent <- sent_by(out <- run(c('tt_engine(max_requests = 1L)',
  sprintf('cat(tryCatch(tt_rank("Q?", c("m1", "m2")), thinkthen_usage = %s), "\\n")', refused),
  sprintf('cat(tryCatch(tt_decide("Q?", c("m1", "m2")), thinkthen_usage = %s), "\\n")', refused))))
check("max_requests = 1L refuses rank and decide over two texts", sent == 1L &&
      lengths(regmatches(out, gregexpr("this engine answers at most 1 records in one call", out, fixed = TRUE))) == 2L)

# After a verb on the default engine, which selects no throttle, a first
# explicit throttle is accepted under 0077's rule.
out <- run(c('invisible(tt_decide("Q?", "first on the default"))',
             'cat("accepted", is.null(tt_engine(throttle = 8L)), "\\n")'))
check("tt_engine(throttle = 8L) after a default call is accepted", grepl("accepted TRUE", out, fixed = TRUE))

# Secrecy: no output or condition message carries the key or URL credentials.
out <- run(sprintf('print(tryCatch(tt_engine(base_url = "%s"), error = function(e) e))', sub("://", "://user:hunter2@", generic)))
check("a base_url with credentials is refused without echoing them",
      grepl("a base address carries no user information", out, fixed = TRUE))
out <- run(c(
  'r <- tryCatch(tt_decide("Q?", "secret check", deadline = 0), error = function(e) e); print(r); print(conditionMessage(r))',
  'print(tryCatch(tt_decide("", "x"), error = function(e) e)); print(tt_details("Q?", "details check")$meta$url)'
))
check("no output names the key or the URL's credentials",
      !any(grepl("tt-test-not-a-key", said, fixed = TRUE)) && !any(grepl("hunter2", said, fixed = TRUE)))

finish("engine", 7L)
