# The installed public column call keeps the shared portable question bytes.
# The content cut is gone by ADR 0111, so all five records ride one request,
# and each row names the key of its fixture question.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
fixtures <- file.path(Sys.getenv("TT_TESTS"), "..", "..", "..", "specification", "fixtures", "batching")
corpus <- jsonlite::fromJSON(file.path(fixtures, "portable-records.json"), simplifyVector = FALSE)
body <- function(index) readLines(file.path(fixtures, sprintf("portable-%d.request.json", index)), warn = FALSE, encoding = "UTF-8")
expected <- vapply(1:3, body, "")
url <- arm("arm/full/capture/v1/systemone")
tt_engine(base_url = arm("arm/full/capture/v1"), cache = FALSE, throttle = 1L)
answer <- tt_decide(corpus$question, unlist(corpus$texts, use.names = FALSE))
actual <- unlist(capture(), use.names = FALSE)
questions <- unlist(lapply(expected, function(one) raw_members(raw_members(one)$questions)), use.names = FALSE)
sent <- if (length(actual) == 1L) raw_members(raw_members(actual)$questions) else list()
check("Max sends one request whose five questions keep the fixture bytes",
      length(actual) == 1L && identical(unlist(sent, use.names = FALSE), questions) &&
      identical(names(sent), sprintf("q%d", 1:5)))
check("public R column returns five ordered accepted rows", identical(answer$value, rep(TRUE, 5L)) &&
      identical(vapply(answer$details, `[[`, 0, "index"), as.numeric(0:4)))
check("final facts count five records and one send", identical(answer$facts$records, 5L) &&
      identical(answer$facts$requests_sent, 1L) && identical(backend_count(), 1L))
check("each row names its question key over the served address",
      identical(lapply(answer$details, function(one) unlist(one$requests)),
                as.list(question_keys(url, actual))))
finish("portable batch identity", 1L)
