# Omission and explicit mapping in separate processes, using ten packed requests.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
spawn <- function(code) {
  script <- tempfile(fileext = ".R")
  output <- tempfile()
  cache <- tempfile("cache-")
  dir.create(cache)
  writeLines(c("library(thinkthen)", code), script)
  env <- paste0("THINKTHEN_BASE_URL=", arm("arm/held/v1"))
  pid <- system(paste("env", paste(clean_env(inherited, child_values(cache, env)), collapse = " "),
                      paste(rscript_args(script), collapse = " "),
                      ">", shQuote(output), "2>&1 </dev/null & echo $!"), intern = TRUE)
  list(pid = as.integer(pid), output = output)
}
until <- function(predicate) {
  stop_at <- Sys.time() + 60
  while (!isTRUE(predicate()) && Sys.time() < stop_at) Sys.sleep(0.02)
  isTRUE(predicate())
}
for (explicit in c(FALSE, TRUE)) {
  cap <- if (explicit) 6L else 8L
  base <- backend_count()
  setup <- if (explicit) "tt_engine(throttle = 6L)" else "tt_engine()"
  job <- spawn(c(setup,
    'a <- tt_decide("Q?", paste("record", 1:20), batch = 2L)$value',
    'stopifnot(identical(a, rep(TRUE, 20))); cat("ANSWERS", length(a), "\\n")'))
  arrived <- backend_wait(base + cap)
  check(sprintf("the exact cap arrives: %d held, expected %d", arrived - base, cap), arrived == base + cap)
  Sys.sleep(0.3)
  check("no next request starts while held", backend_count() == base + cap)
  # Release only this round: the second fresh process must still hold.
  # Two remaining requests can arrive after the first round releases.
  backend_say("round")
  check("all ten requests arrive", backend_wait(base + 10L) == base + 10L)
  backend_say("round")
  check("the child finishes", until(function() !tools::pskill(job$pid, 0L)))
  check("all twenty answers finish", identical(readLines(job$output, warn = FALSE), "ANSWERS 20 "))
  check("exactly ten requests were sent", backend_count() == base + 10L)
}
backend_say("release")
finish("throttle_0400", 20L)
