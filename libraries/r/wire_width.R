# The width bench: one column, one crossing, the engine's width on the
# wire. Reads the stub's counters to prove both.
.libPaths(c("rlib", .libPaths()))
library(thinkthen)

port <- Sys.getenv("STUB_PORT")
stub <- paste0("http://127.0.0.1:", port, "/v1")

records <- rep(c("I want a refund for order 9", "thanks, all good",
                 "maybe there is a problem"), length.out = 1000)

started <- proc.time()["elapsed"]
answers <- tt_decide("Is this a complaint?", records)
wall <- proc.time()["elapsed"] - started

stats <- jsonlite::fromJSON(paste0(stub, "/stats"))
cat("width:", length(records), "records, wall", round(wall, 3), "s\n")
cat("stub:", "requests", stats$requests, "max_in_flight", stats$max_in_flight,
    "connections", stats$connections, "\n")
stopifnot(length(answers) == 1000)
invisible(NULL)
