source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
recording <- tempfile("example-recording-")
example <- file.path(Sys.getenv("TT_TESTS"), "..", "examples", "native.R")
run <- function(mode) { result <- child(c(sprintf('tt_engine(cache = FALSE, %s = %s)', mode, dQuote(recording, FALSE)),
                            sprintf('source(%s)', dQuote(example, FALSE)))); if (result$status != 0L) stop(result$text); result$text }
recorded <- run("record")
sent <- backend_count()
check("ten installed examples read typed native answers", grepl("ten typed answers", recorded, fixed = TRUE) && sent > 0L)
replayed <- run("replay")
check("ten installed examples replay without sending", identical(recorded, replayed) && backend_count() == sent)
finish("example replay", sent)
