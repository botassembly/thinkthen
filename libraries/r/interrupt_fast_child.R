# The fast-backend interrupt proof, child half. Runs one column of two
# million records against the null backend — no stub, so the engine's wait
# never idles; a SIGINT from the parent lands in the call's guarded
# interrupt check, which cancels the engine's token and reports the
# interrupt condition the tryCatch below catches. The R call polls its worker
# every 100 ms on the main thread, so the interrupt must land within about
# a tick; the batch runs about 9 s deaf, which separates the two
# behaviors. A pending interrupt is caught where it stands: the call's
# token is cancelled, so no new request starts, and the R half raises the
# interrupt condition.
.libPaths(c("rlib", .libPaths()))
library(thinkthen)

records <- rep("I want a refund for order 9", 2000000)
started <- proc.time()["elapsed"]
caught <- tryCatch(
  {
    # The parent signals only after this line: a signal during the
    # library load would halt the script before the tryCatch stands.
    cat("ready\n")
    flush(stdout())
    tt_decide("Is this a complaint?", records)
    "completed"
  },
  interrupt = function(e) "interrupt"
)
elapsed <- proc.time()["elapsed"] - started

# The session must survive the interrupt: the call after it answers.
after <- tryCatch(
  tt_decide("Is this a complaint?", "I want a refund"),
  interrupt = function(e) "interrupt again"
)

cat(paste0("outcome: ", caught, "\n"))
cat(paste0("elapsed: ", sprintf("%.3f", elapsed), "\n"))
cat(paste0("after: ", after, "\n"))
invisible(NULL)