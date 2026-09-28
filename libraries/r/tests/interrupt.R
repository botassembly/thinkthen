# Ctrl-C under ADR 0042: the check points before and around a call, the
# 100 ms tick for a single call and a batch, the throttle cap, R2-23's
# parallel bulk verbs, and the hook and blank-line twins of R7-13. Each
# child runs on the held arm in the background with its own cache. The
# parent signals only once the backend's count shows the request on the
# wire (R6-12), and every held reply is let go by a `round` line.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
held <- paste0("THINKTHEN_BASE_URL=", arm("arm/held/v1"))
now <- function() as.numeric(Sys.time())
caught <- 'function(e) { cat("CAUGHT", format(as.numeric(Sys.time()), digits = 15), "\\n"); flush(stdout()) }'

# A background child: its pid and its output file.
spawn <- function(code, env = held) {
  file <- tempfile(fileext = ".R")
  out <- tempfile()
  cache <- tempfile("cache-")
  dir.create(cache)
  writeLines(c("library(thinkthen)", code), file)
  pid <- system(paste("env", paste(clean_env(inherited, child_values(cache, env)), collapse = " "),
                      shQuote(file.path(R.home("bin"), "Rscript")),
                      shQuote(file), ">", shQuote(out), "2>&1 </dev/null & echo $!"), intern = TRUE)
  list(pid = as.integer(pid), out = out)
}
lines_of <- function(one) if (file.exists(one$out)) readLines(one$out, warn = FALSE) else character()
until <- function(held, seconds) {
  stop_at <- now() + seconds
  while (!isTRUE(held()) && now() < stop_at) Sys.sleep(0.02)
  isTRUE(held())
}
caught_at <- function(one) {
  line <- grep("^CAUGHT ", lines_of(one), value = TRUE)
  if (length(line)) as.numeric(strsplit(line[[1]], " ")[[1]][[2]]) else NA
}
ended <- function(one) until(function() !tools::pskill(one$pid, 0L), 20)
settled <- function(least) {
  count <- -1L
  until(function() {
    first <- backend_count()
    Sys.sleep(0.1)
    count <<- backend_count()
    first == count && count >= least
  }, 60)
  count
}

# Check point before a call: the deadline argument signals as the Rust call
# forces it, after .tt_call's first check. The Rust check before the spawn
# stops it, and nothing is sent.
one <- child(c(sprintf('tryCatch(tt_decide("Q?", "before", deadline = { tools::pskill(Sys.getpid(), 2L); 5 }), interrupt = %s)', caught)), held)
check("an interrupt before the spawn arrives and sends nothing", grepl("CAUGHT", one$text, fixed = TRUE) && backend_count() == 0L)

# Check points around .tt_call: before it forces its expression, and after
# the expression returns.
around <- child(c(
  'tools::pskill(Sys.getpid(), 2L)',
  sprintf('tryCatch(thinkthen:::.tt_call(Sys.setenv(TT_SET = "1")), interrupt = %s)', caught),
  'cat("set", Sys.getenv("TT_SET", "no"), "\\n")',
  sprintf('r <- tryCatch(thinkthen:::.tt_call({ tools::pskill(Sys.getpid(), 2L); TRUE }), interrupt = function(e) "interrupted")'),
  'cat("after", r, "\\n")'), held)$text
check("the first check stops the expression", grepl("set no", around, fixed = TRUE))
check("the second check replaces the value", grepl("after interrupted", around, fixed = TRUE))

# The tick, single call: CAUGHT within 0.5 s of the signal, before any
# release, and the held request is the only one.
single <- spawn(c(sprintf('tryCatch(tt_decide("Q?", "single"), interrupt = %s)', caught), 'Sys.sleep(5)'))
check("the single call reaches the wire", settled(1L) == 1L)
signalled <- now()
tools::pskill(single$pid, 2L)
invisible(until(function() !is.na(caught_at(single)), 5))
took <- caught_at(single) - signalled
cat(sprintf("signal to CAUGHT, single: %.3f s\n", took))
check("a single call answers the interrupt within 0.5 s, before release", isTRUE(took < 0.5))
backend_say("round")
Sys.sleep(1)
check("the single count stays 1 after release", backend_count() == 1L)
invisible(ended(single))

# The tick, batch at throttle 8: the cap is exactly 8, CAUGHT within 0.5 s,
# and the cancelled batch sends nothing more after release.
batch <- spawn(c('tt_engine(throttle = 8L)', sprintf('tryCatch(tt_decide("Q?", paste("batch", 1:200)), interrupt = %s)', caught),
                 'Sys.sleep(5)'))
at_signal <- settled(2L)
check("throttle 8 holds exactly 8 on the wire", at_signal == 1L + 8L)
signalled <- now()
tools::pskill(batch$pid, 2L)
invisible(until(function() !is.na(caught_at(batch)), 5))
took <- caught_at(batch) - signalled
cat(sprintf("signal to CAUGHT, batch: %.3f s\n", took))
check("a batch answers the interrupt within 0.5 s", isTRUE(took < 0.5))
backend_say("round")
Sys.sleep(1)
later <- backend_count()
Sys.sleep(2)
check("the cancelled batch sends nothing after release", later == at_signal && backend_count() == at_signal)
invisible(ended(batch))

# R2-23: choose, score, and tag over 50 texts cross as one parallel
# annotate, so more than one request is on the wire before any release.
bulk <- c(choose = 'tt_choose("Which?", paste("c", 1:50), c("a", "b"))',
          score = 'tt_score("How much?", paste("s", 1:50), c("low", "high"))',
          tag = 'tt_tag("Which labels?", paste("t", 1:50), c("x", "y"))')
for (verb in names(bulk)) {
  base <- backend_count()
  job <- spawn(c('tt_engine(throttle = 8L)', bulk[[verb]]))
  check(paste("R2-23:", verb, "puts more than one request on the wire"), settled(base + 2L) - base > 1L)
  tools::pskill(job$pid, 9L)
  ended(job)
  backend_say("round")
}

# R7-13 and the blank line: a Ctrl-C during a held call meets a hook as a
# Ctrl-C during plain R code does. The held twin prints one more blank line
# than the plain twin (ADR 0042's third window), and the other lines match.
twin <- function(body, catch) {
  run <- sprintf('run <- function() { cat("ready\\n"); flush(stdout()); %s; cat("COMPLETED\\n") }', body)
  call <- if (catch) 'tryCatch(run(), interrupt = function(e) cat("CAUGHT\\n"))' else 'run()'
  base <- backend_count()
  job <- spawn(c('options(error = function() cat("HOOK RAN\\n"))', run, call, 'cat("AFTER\\n")'))
  until(function() "ready" %in% lines_of(job), 60)
  if (grepl("tt_", body)) settled(base + 1L) else Sys.sleep(0.5)
  tools::pskill(job$pid, 2L)
  ended(job)
  backend_say("round")
  lines_of(job)
}
for (catch in c(FALSE, TRUE)) {
  plain <- twin("for (i in 1:100) Sys.sleep(0.1)", catch)
  call <- twin(sprintf('tt_decide("Q?", "hook %s")', catch), catch)
  words <- function(lines) lines[nzchar(lines)]
  check(paste("the hook meets a held call as plain R, catch =", catch), identical(words(call), words(plain)) &&
        identical(sum(!nzchar(call)), sum(!nzchar(plain)) + 1L) &&
        identical(words(plain), if (catch) c("ready", "CAUGHT", "AFTER") else c("ready", "HOOK RAN", "AFTER")))
}

backend_say("release")
finish("interrupt", 1L + 8L + 24L + 2L)
