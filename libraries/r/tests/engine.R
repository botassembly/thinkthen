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
sent <- sent_by(out <- run(c(sprintf('tt_engine(base_url = "%s")', generic), 'cat(tt_decide("Q?", "over the variable")$value, "\\n")'),
                           env = "THINKTHEN_BASE_URL=http://127.0.0.1:1/generic/v1"))
check("a set base_url wins over THINKTHEN_BASE_URL", sent == 1L && grepl("TRUE", out, fixed = TRUE))

# One refusal an argument, each usage and each sending nothing.
refusals <- c('base_url = 5', 'model = ""', 'throttle = 0L', 'throttle = 33L', 'max_requests = -1',
              'max_request_bytes = 0',
              'cache = TRUE', 'timeout = 2.5', 'replay = NA', 'throttle = 2.5', 'base_url = "ftp://example.test"')
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

# Eager rank and decide admit the complete collection before sending.
refused <- 'function(e) conditionMessage(e)'
sent <- sent_by(out <- run(c('tt_engine(max_requests = 1L)',
  sprintf('cat(tryCatch(tt_rank("Q?", c("m1", "m2"))$value, thinkthen_usage = %s), "\\n")', refused),
  sprintf('cat(tryCatch(tt_decide("Q?", c("m1", "m2"))$value, thinkthen_usage = %s), "\\n")', refused))))
check("max_requests = 1L refuses rank and decide over two texts", sent == 0L &&
      lengths(regmatches(out, gregexpr("this engine answers at most 1 records in one call", out, fixed = TRUE))) == 2L)

# After a verb on the default engine, which selects no throttle, a first
# explicit throttle is accepted under 0077's rule.
out <- run(c('invisible(tt_decide("Q?", "first on the default"))',
             'cat("accepted", is.null(tt_engine(throttle = 8L)), "\\n")'))
check("tt_engine(throttle = 8L) after a default call is accepted", identical(out, "accepted TRUE "))

# Secrecy: no output or condition message carries the key or URL credentials.
out <- run(sprintf('print(tryCatch(tt_engine(base_url = "%s"), error = function(e) e))', sub("://", "://user:hunter2@", generic)))
check("a base_url with credentials is refused without echoing them",
      grepl("a base address carries no user information", out, fixed = TRUE))
out <- run(c(
  'r <- tryCatch(tt_decide("Q?", "secret check", deadline_ms = 0), error = function(e) e); print(r); print(conditionMessage(r))',
  'print(tryCatch(tt_decide("", "x"), error = function(e) e)); print(tt_decide("Q?", "details check")$results[[1L]]$meta$url)'
))
check("no output names the key or the URL's credentials",
      !any(grepl("tt-test-not-a-key", said, fixed = TRUE)) && !any(grepl("hunter2", said, fixed = TRUE)))


# Live usage methods operate on the selected native engine and keep frozen facts.
sent <- sent_by(out <- run(c(
  sprintf('tt_engine(base_url = "%s", cache = FALSE)', generic),
  'done <- tt_decide("Is it urgent?", "urgent")',
  'before <- serialize(done, NULL)',
  'status <- tt_finish_usage_status()',
  'stopifnot(inherits(status,"thinkthen_UsageStatus"), identical(status$state,"written"), is.null(status$advice), identical(serialize(done,NULL), before), tt_usage()$requests_sent == 1)',
  'cat("written", status$state, "\\n")'
)))
check("live usage finalization preserves the answer and sends nothing", sent == 1L && grepl("written written", out, fixed=TRUE))

sent <- sent_by(out <- run(c(
  'state <- tempfile("usage-state-"); dir.create(state); Sys.setenv(XDG_STATE_HOME=state)',
  sprintf('tt_engine(base_url = "%s", cache = FALSE)', generic),
  'first <- tt_decide("Is it urgent?", "first")',
  'stopifnot(identical(tt_finish_usage_status()$state,"written"))',
  'folder <- file.path(state,"thinkthen"); Sys.chmod(folder,"0555")',
  'done <- tt_decide("Is it urgent?", "second")',
  'before <- serialize(done, NULL)',
  'failed <- tt_finish_usage_status(); Sys.chmod(folder,"0700")',
  'stopifnot(identical(failed$state,"failed"), identical(failed$advice,"check the usage folder permissions and free space"), identical(tt_usage_persistence(),failed), identical(tt_finish_usage_status(),failed), identical(serialize(done,NULL),before), tt_usage()$requests_sent == 2)',
  'cat("failed", failed$state, "\\n")'
)))
check("failed durability stays latched and does not lose answers", sent == 2L && grepl("failed failed",out,fixed=TRUE))

finish("engine", 7L)
