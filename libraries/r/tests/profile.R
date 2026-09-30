# A saved calibration name enters through the public file question and keeps
# its digest, warning, and per-row fallback at the real R and Rust boundary.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
case <- jsonlite::fromJSON(file.path(Sys.getenv("TT_TESTS"), "..", "..", "..",
                                      "conformance", "calibration.json"), simplifyVector = FALSE)

running <- tempfile(fileext = ".json")
writeLines(jsonlite::toJSON(case$runtime_profile, auto_unbox = TRUE), running)
tt_engine(cache = FALSE, profile = running)

saved <- tempfile(fileext = ".json")
writeLines(jsonlite::toJSON(case$question, auto_unbox = TRUE), saved)
question <- tt_question(file = saved)
details <- tt_details(question, case$evidence)$value
check("the saved name has the pinned canonical digest",
      identical(details$meta$question_sha256, case$question_sha256))
check("the warning carries saved and running names",
      identical(details$meta$profile_warning, case$warning))
check("the actual reply names the model", identical(details$meta$model, case$model))

check("mixed file and keywords refuse before send", sent_by(check("mixed file kind",
  identical(kind_of(tt_question(file = saved, decide = "Other?")), "usage"))) == 0L)
check("an unreadable question file is local", sent_by(check("unreadable kind",
  identical(kind_of(tt_question(file = paste0(saved, "-missing"))), "local"))) == 0L)
over <- tempfile(fileext = ".json")
writeChar(paste0('{"decide":"Is it?"}', strrep(" ", 1048577L - 19L)), over, eos = NULL)
for (large in c(over, "/dev/zero")) {
  check("an overlarge question file is local", sent_by(check("overlarge sentence",
    identical(message_of(tt_question(file = large)), "the question file is too large"))) == 0L)
}
invalid <- tempfile(fileext = ".json")
writeLines('{"decide":', invalid)
check("an invalid question file is local", sent_by(check("invalid kind",
  identical(kind_of(tt_question(file = invalid)), "local"))) == 0L)
check("rank and find refuse a profiled built question", sent_by({
  check("rank sentence", identical(message_of(tt_rank(question, c("one", "two"))$value),
                                  "rank takes a decide question with no profile"))
  check("find sentence", identical(message_of(tt_find(question, c("one", "two"))$value),
                                  "find takes a decide question with no profile"))
}) == 0L)

scored <- tempfile(fileext = ".json")
writeLines('{"score":"How urgent?","levels":["low","high"],"profile":"old"}', scored)
before <- backend_count()
values <- tt_score(tt_question(file = scored), c("one", "two"), batch = 1L)$value
check("profiled score keeps two values", isTRUE(all.equal(values, c(0.1, 0.1))))
check("profiled score keeps one request per row", backend_count() - before == 2L)

unlink(c(saved, scored, invalid, running, over))
finish("profile", 3L)
