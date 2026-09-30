# The installed public column call must honor the shared Max content cuts.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
fixtures <- file.path(Sys.getenv("TT_TESTS"), "..", "..", "..", "specification", "fixtures", "batching")
corpus <- jsonlite::fromJSON(file.path(fixtures, "portable-records.json"), simplifyVector = FALSE)
body <- function(index) readLines(file.path(fixtures, sprintf("portable-%d.request.json", index)), warn = FALSE)
expected <- vapply(1:3, body, "")
url <- arm("arm/full/capture/v1/systemone")
tt_engine(base_url = arm("arm/full/capture/v1"), cache = FALSE, throttle = 1L)
answer <- tt_decide(corpus$question, unlist(corpus$texts, use.names = FALSE))
actual <- unlist(capture(), use.names = FALSE)
check("Max cuts transmit the three literal shared request bodies", identical(actual, unname(expected)))
check("public R column returns five ordered accepted rows", identical(answer$value, rep(TRUE, 5L)) &&
      identical(vapply(answer$details, `[[`, 0, "index"), as.numeric(0:4)))
check("final facts count five records and three sends", identical(answer$facts$records, 5L) &&
      identical(answer$facts$requests_sent, 3L) && identical(backend_count(), 3L))
hashes <- vapply(expected, function(one) digest(url, one), "")
check("each row names its literal body's served-address exchange digest",
      identical(lapply(answer$details, function(one) unlist(one$requests)),
                unname(list(hashes[[1L]], hashes[[1L]], hashes[[2L]], hashes[[2L]], hashes[[3L]]))))
finish("portable batch identity", 3L)
