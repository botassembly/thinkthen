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
ended <- function(one) until(function() !tools::pskill(one$pid, 0L), 60)

# Check point before a call: the deadline argument signals as the Rust call
# forces it, after .tt_call's first check. The Rust check before the spawn
# stops it, and nothing is sent.
one <- child(c(sprintf('tryCatch(tt_decide("Q?", "before", deadline_ms = { tools::pskill(Sys.getpid(), 2L); 5 }), interrupt = %s)', caught)), held)
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

# The tick, single call: CAUGHT before any release, and the held request is
# the only one. Under the stress profile CAUGHT also comes within 0.5 s.
single <- spawn(c(sprintf('tryCatch(tt_decide("Q?", "single"), interrupt = %s)', caught), 'Sys.sleep(5)'))
check("the single call reaches the wire", backend_wait(1L) == 1L)
signalled <- now()
tools::pskill(single$pid, 2L)
check("a single call answers the interrupt before release", until(function() !is.na(caught_at(single)), 30))
took <- caught_at(single) - signalled
cat(sprintf("signal to CAUGHT, single: %.3f s\n", took))
if (stress) check("a single call answers the interrupt within 0.5 s", isTRUE(took < 0.5))
backend_say("round")
Sys.sleep(1)
check("the single count stays 1 after release", backend_count() == 1L)
invisible(ended(single))

# The tick, batch at throttle 8: the cap is exactly 8, CAUGHT comes before
# release (within 0.5 s under stress), and the cancelled batch sends nothing
# more after release.
receipt_caught <- 'function(e) { cat("CAUGHT", format(as.numeric(Sys.time()), digits = 15), "\\n"); cat("ON_INTERRUPT", tt_completion_read(h)$state, "\\n"); flush(stdout()) }'
batch <- spawn(c('tt_engine(throttle = 8L)',
  'h <- tt_completion(); cat("BEFORE", tt_completion_read(h)$state, "\\n")',
  sprintf('tryCatch(tt_decide("Q?", paste("batch", 1:200), batch = 2L, completion = h), interrupt = %s)', receipt_caught),
  'for (i in 1:600) { a <- tt_completion_read(h); if (identical(a$state, "terminal")) break; Sys.sleep(0.05) }',
  'b <- tt_completion_read(h)',
  'reuse <- tryCatch(tt_decide("Q?", "reuse", completion = h), thinkthen_error = function(e) e$kind)',
  'cat("FINAL", a$kind, a$facts$requests_sent, a$facts$records, identical(a, b), reuse, "\\n")',
  'a$details[[1]]$index <- 99; cat("OWNED", tt_completion_read(h)$details[[1]]$index, "\\n")'))
invisible(backend_wait(1L + 8L))
Sys.sleep(0.3)
at_signal <- backend_count()
check("throttle 8 holds exactly 8 on the wire", at_signal == 1L + 8L)
signalled <- now()
tools::pskill(batch$pid, 2L)
check("a batch answers the interrupt before release", until(function() !is.na(caught_at(batch)), 30))
took <- caught_at(batch) - signalled
cat(sprintf("signal to CAUGHT, batch: %.3f s\n", took))
if (stress) check("a batch answers the interrupt within 0.5 s", isTRUE(took < 0.5))
backend_say("round")
Sys.sleep(1)
later <- backend_count()
Sys.sleep(2)
check("the cancelled batch sends nothing after release", later == at_signal && backend_count() == at_signal)
invisible(ended(batch))
batch_lines <- paste(lines_of(batch), collapse = "\n")
check("completion moves unused through running to final accounted cancellation",
      grepl("BEFORE unused", batch_lines, fixed = TRUE) &&
      grepl("ON_INTERRUPT running", batch_lines, fixed = TRUE) &&
      grepl("FINAL cancelled 8 16 TRUE usage", batch_lines, fixed = TRUE))
check("completion reads are owned snapshots", grepl("OWNED 0", batch_lines, fixed = TRUE))

# The caller drops its only R handle while a sent scalar is still held. The
# worker owns its Rust reference until cancellation settles and joins.
base <- backend_count()
collected <- spawn(c('h <- tt_completion()',
  'tryCatch(tt_decide("Q?", "collected", completion = h), interrupt = function(e) { rm(h); invisible(gc()); cat("COLLECTED\\n"); flush(stdout()) })'))
check("a handle can be collected during a held send", backend_wait(base + 1L) == base + 1L)
tools::pskill(collected$pid, 2L)
check("R returns promptly after collecting the handle",
      until(function() grepl("COLLECTED", paste(lines_of(collected), collapse = "\n"), fixed = TRUE), 30))
backend_say("round")
invisible(ended(collected))
check("the collected handle starts no later send", backend_count() == base + 1L)

# R2-23: choose, score, and tag over 50 texts cross as one parallel
# annotate, so more than one request is on the wire before any release.
bulk <- c(choose = 'tt_choose("Which?", paste("c", 1:50), c("a", "b"), batch = 2L)',
          score = 'tt_score("How much?", paste("s", 1:50), c("low", "high"), batch = 2L)',
          tag = 'tt_tag("Which labels?", paste("t", 1:50), c("x", "y"), batch = 2L)')
for (verb in names(bulk)) {
  base <- backend_count()
  job <- spawn(c('tt_engine(throttle = 8L)', bulk[[verb]]))
  # Throttle 8 holds eight sends; the file's total counts all eight before the kill.
  check(paste("R2-23:", verb, "puts more than one request on the wire"), backend_wait(base + 8L) - base > 1L)
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
  if (grepl("tt_", body)) backend_wait(base + 1L) else Sys.sleep(0.5)
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
finish("interrupt", 1L + 8L + 1L + 24L + 2L)
