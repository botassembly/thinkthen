# The fast-backend interrupt proof, child half. Runs one column of two
# million records against the null backend — no stub, so the engine's wait
# never idles — and lets a SIGINT from the parent jump through
# R_CheckUserInterrupt into the tryCatch below. The R call polls its worker
# every 100 ms on the main thread, so the interrupt must land within about
# a tick; the batch runs about 9 s deaf, which separates the two
# behaviors. On the interrupt jump the call's on.exit cleanup cancels the
# engine's token, so no new request starts.
.libPaths(c("rlib", .libPaths()))
library(thinkthen)

records <- rep("I want a refund for order 9", 2000000)
started <- proc.time()["elapsed"]
caught <- tryCatch(
  {
    tt_decide("Is this a complaint?", records)
    "completed"
  },
  interrupt = function(e) "interrupt"
)
elapsed <- proc.time()["elapsed"] - started

cat(paste0("outcome: ", caught, "\n"))
cat(paste0("elapsed: ", sprintf("%.3f", elapsed), "\n"))
invisible(NULL)