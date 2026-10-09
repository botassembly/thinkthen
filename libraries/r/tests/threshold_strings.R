# String bands and cuts enter ordinary calls and native question definitions.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(base_url = arm("arm/full/capture/v1"), cache = FALSE)
file <- tempfile(fileext = ".json")
writeLines('{"decide":"Q?","threshold":"0.2:0.95"}', file)
entrypoints <- list(
  direct = function() tt_decide("Q?", "x", options = list(threshold = "0.2:0.95")),
  built = function() tt_decide(list(decide = "Q?", threshold = "0.2:0.95"), "x"),
  saved = function() tt_decide(tt_question(file = file), "x")
)
for (name in names(entrypoints)) {
  arrivals <- sent_by(answer <- entrypoints[[name]]()$results[[1]])
  check(paste(name, "keeps unsure and yes probability in one send"),
        arrivals == 1L && is.null(answer$value) && identical(answer$answer$probability, 0.9))
}
expected <- paste0('{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{',
                   '"q1":{"type":"noul","instructions":"The text is \\"x\\". Q?"}}}')
check("all three entrypoints send the same literal request body", identical(capture(), rep(list(expected), 3L)))
choose_question <- list(choose = "Which?", options = c("first", "second"), threshold = "0.95")
tag_question <- list(tag = "Which?", labels = c("first", "second"), threshold = "0.95")
choice <- sent_by(chosen <- tt_choose(choose_question, "x")$results[[1]])
tags <- sent_by(tagged <- tt_tag(tag_question, "x")$results[[1]])
check("string cuts keep choose and tag selection rules",
      choice == 1L && tags == 1L && is.null(chosen$value) && identical(tagged$value, list()))
check("native planning admits valid string cuts without sending", sent_by({
  check("choose plan", tt_plan(choose_question, "x", "choose")$records == 1)
  check("tag plan", tt_plan(tag_question, "x", "tag")$records == 1)
}) == 0L)
invalid <- list(
  reversed_direct = function() tt_decide("Q?", "x", options = list(threshold = "0.95:0.2")),
  reversed_built = function() tt_plan(list(decide = "Q?", threshold = "0.95:0.2"), "x"),
  blank_direct = function() tt_decide("Q?", "x", options = list(threshold = "0.2:")),
  blank_built = function() tt_plan(list(decide = "Q?", threshold = "0.2:"), "x"),
  choose_band = function() tt_choose(choose_question, "x", options = list(threshold = "0.2:0.95")),
  tag_band = function() tt_tag(tag_question, "x", options = list(threshold = "0.2:0.95"))
)
for (name in names(invalid)) {
  arrivals <- sent_by(kind <- kind_of(invalid[[name]]()))
  check(paste(name, "is usage before any request"), identical(kind, "usage") && arrivals == 0L)
}
finish("threshold strings", 5L)
