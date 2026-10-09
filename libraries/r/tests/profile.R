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
details <- tt_decide(question, case$evidence)$results[[1L]]
check("the saved name has the pinned canonical digest",
      identical(unclass(details$meta$question_sha256), case$question_sha256))
check("the warning carries saved and running names",
      isTRUE(all.equal(thinkthen:::.tt_complete_plain(details$meta$profile_warning)[names(case$warning)], case$warning)))
check("the actual reply names the model", identical(details$meta$model, case$model))

check("mixed file and keywords refuse before send", sent_by(check("mixed file kind",
  identical(kind_of(tt_decide(tt_question(file = saved), "x", options = list(decide = "Other?"))), "usage"))) == 0L)
check("an unreadable question file is local", sent_by(check("unreadable kind",
  identical(kind_of(tt_decide(tt_question(file = paste0(saved, "-missing")), "x")), "local"))) == 0L)
# File-size boundaries belong to the release profile.
if (identical(Sys.getenv("THINKTHEN_TEST_PROFILE"), "full")) {
  over <- tempfile(fileext = ".json")
  writeChar(paste0('{"decide":"Is it?"}', strrep(" ", 1048577L - 19L)), over, eos = NULL)
  for (large in c(over, "/dev/zero")) {
    check("an overlarge question file is local", sent_by(check("overlarge kind",
      identical(kind_of(tt_decide(tt_question(file = large), "x")), "local"))) == 0L)
  }
  unlink(over)
}
invalid <- tempfile(fileext = ".json")
writeLines('{"decide":', invalid)
check("an invalid question file is local", sent_by(check("invalid kind",
  identical(kind_of(tt_decide(tt_question(file = invalid), "x")), "local"))) == 0L)
rank_sends <- sent_by(ranked <- tt_rank(question, c("one", "two")))
check("rank retains the saved calibration warning and native account",
      rank_sends == 1L && ranked$facts$records == 2 && ranked$facts$requests_sent == 1 &&
      all(vapply(ranked$results, function(row)
        identical(thinkthen:::.tt_complete_plain(row$meta$profile_warning)[names(case$warning)], case$warning), TRUE)))
check("find refuses the saved decide function before sending", sent_by(check("find kind",
  identical(kind_of(tt_find(question, c("one", "two"))), "local"))) == 0L)

scored <- tempfile(fileext = ".json")
writeLines('{"score":"How urgent?","levels":["low","high"],"profile":"old"}', scored)
before <- backend_count()
values <- tt_score(tt_question(file = scored), c("one", "two"), options = list(batch = 1L))$value
check("profiled score keeps two values", isTRUE(all.equal(values, c(0.1, 0.1))))
check("profiled score keeps one request per row", backend_count() - before == 2L)

unlink(c(saved, scored, invalid, running))
finish("profile", 4L)
