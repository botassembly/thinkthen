# The interrupt proof, child half. Runs one column of 1,000 records
# against the 300 ms stub at width 32; a SIGINT from the parent jumps
# through R_CheckUserInterrupt into the interrupt handler below. The
# on.exit cleanup in every call fires the cancel token, and the token
# rides every engine call, so the engine stops starting requests while
# the ones in flight finish. The child then SLEEPS for four seconds and
# reads the stub's counter itself — still alive, still inside the window
# the full batch would need — so the stop is proven before process exit,
# not by it.
.libPaths(c("rlib", .libPaths()))
library(thinkthen)

port <- Sys.getenv("STUB_PORT")
stub <- paste0("http://127.0.0.1:", port, "/v1")

records <- rep("I want a refund for order 9", 1000)
started <- proc.time()["elapsed"]
caught <- tryCatch(
  {
    tt_decide("Is this a complaint?", records)
    "completed"
  },
  interrupt = function(e) "interrupt"
)
elapsed <- proc.time()["elapsed"] - started

# The full batch would take about 9.4 s (1000 x 300 ms over width 32).
# Sleep past that window while alive; the counters must not move.
Sys.sleep(4)
stats <- jsonlite::fromJSON(paste0(stub, "/stats"))
cat("outcome:", caught, "\n")
cat("query window:", round(elapsed, 3), "s\n")
cat("child read at +4s while alive:", "requests", stats$requests, "\n")
invisible(NULL)
