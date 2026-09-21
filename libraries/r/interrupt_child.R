# The interrupt proof, child half. Runs one column of 1,000 records
# against the 300 ms stub at width 32; a SIGINT from the parent jumps
# through R_CheckUserInterrupt into the interrupt handler below. The
# on.exit cleanup in every call sets the cancel token, so the engine stops
# starting requests; the ones in flight finish. The parent reads the stub's
# counters after a settle to prove nothing was served past the return.
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

stats <- jsonlite::fromJSON(paste0(stub, "/stats"))
cat("outcome:", caught, "\n")
cat("query window:", round(elapsed, 3), "s\n")
cat("stub at return:", "requests", stats$requests, "\n")
invisible(NULL)
