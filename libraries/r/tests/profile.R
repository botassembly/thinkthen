# A saved calibration name enters through the public file question and keeps
# its digest, warning, and per-row fallback at the real R and Rust boundary.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))

running <- tempfile(fileext = ".json")
writeLines('{"schema":"thinkthen.backend-profile/1","name":"new","max_evidence_bytes":1000}', running)
tt_engine(cache = FALSE, profile = running)

saved <- tempfile(fileext = ".json")
writeLines('{"decide":"Is this a request?","profile":"old"}', saved)
question <- tt_question(file = saved)
details <- tt_details(question, "Please help me.")
check("the saved name has the pinned canonical digest",
      identical(details$meta$question_sha256,
        "f06400cd4a31f433828a9d1dd4b7ad28682d6d3d43b884d8e9b74dfe99a86419"))
check("the warning carries saved and running names",
      identical(details$meta$profile_warning, list(tuned_for = "old", running = "new")))
check("the actual reply names the model", identical(details$meta$model, "jev-1.13.0"))

check("mixed file and keywords refuse before send", sent_by(check("mixed file kind",
  identical(kind_of(tt_question(file = saved, decide = "Other?")), "usage"))) == 0L)
check("an unreadable question file is local", sent_by(check("unreadable kind",
  identical(kind_of(tt_question(file = paste0(saved, "-missing"))), "local"))) == 0L)
invalid <- tempfile(fileext = ".json")
writeLines('{"decide":', invalid)
check("an invalid question file is local", sent_by(check("invalid kind",
  identical(kind_of(tt_question(file = invalid)), "local"))) == 0L)
check("rank and find refuse a profiled built question", sent_by({
  check("rank sentence", identical(message_of(tt_rank(question, c("one", "two"))),
                                  "rank takes a decide question with no profile"))
  check("find sentence", identical(message_of(tt_find(question, c("one", "two"))),
                                  "find takes a decide question with no profile"))
}) == 0L)

scored <- tempfile(fileext = ".json")
writeLines('{"score":"How urgent?","levels":["low","high"],"profile":"old"}', scored)
before <- backend_count()
values <- tt_score(tt_question(file = scored), c("one", "two"))
check("profiled score keeps two values", isTRUE(all.equal(values, c(0.1, 0.1))))
check("profiled score keeps one request per row", backend_count() - before == 2L)

unlink(c(saved, scored, invalid, running))
finish("profile", 3L)
