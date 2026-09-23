# The interrupt-and-hook proof, child half. A user's options(error = ...)
# hook is set. HOOK_BODY=thinkthen runs a two-million-record null-backend
# column, and HOOK_BODY=plain runs a plain R sleep loop, the reference a
# genuine Ctrl-C meets. HOOK_CATCH=1 wraps the body in
# tryCatch(interrupt = ...). The parent sends SIGINT after "ready".
.libPaths(c("rlib", .libPaths()))
library(thinkthen)

options(error = function() cat("HOOK RAN\n"))
body <- Sys.getenv("HOOK_BODY")
records <- rep("I want a refund for order 9", 2000000)
run <- function() {
  cat("ready\n")
  flush(stdout())
  if (body == "plain") for (i in 1:100) Sys.sleep(0.1) else tt_decide("Is this a complaint?", records)
  cat("COMPLETED\n")
}
if (Sys.getenv("HOOK_CATCH") == "1") {
  tryCatch(run(), interrupt = function(e) cat("CAUGHT\n"))
} else {
  run()
}
cat("AFTER\n")
