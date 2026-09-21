# The null suite: every verb, the six kinds, and the NA contract, against
# the null backend with no network at all.
.libPaths(c("rlib", .libPaths()))
library(thinkthen)
stopifnot(Sys.getenv("ENGINE_NULL") == "1")

passed <- 0
fail <- function(what) stop(paste("failed:", what), call. = FALSE)
check <- function(what, held) {
  if (!isTRUE(held)) fail(what)
  passed <<- passed + 1
}

# decide: the three-valued answer and the NA contract.
answers <- tt_decide("Is this a complaint?",
  c("I want a refund for order 9", "thanks, all good", "maybe there is a problem"))
check("decide cut TRUE FALSE TRUE", identical(answers, c(TRUE, FALSE, TRUE)))
banded <- tt_decide("Is this a complaint?",
  c("I want a refund for order 9", "thanks, all good", "maybe there is a problem"),
  threshold = c(0.2, 0.8))
check("decide band TRUE FALSE NA", identical(banded, c(TRUE, FALSE, NA)))
check("na evidence is na out", is.na(tt_decide("Q?", c(NA_character_, "refund me"))[1]))
check("na evidence row still answers", isTRUE(tt_decide("Q?", c(NA_character_, "refund me"))[2]))

# a built question stands as built, and equals the text form with its cut.
built <- tt_question(decide = "Is this a complaint?", threshold = c(0.2, 0.8))
check("built question band", identical(tt_decide(built, "maybe there is a problem"), NA))
check("text equals built with the same cut", identical(
  tt_decide("Is this a complaint?", "maybe there is a problem", threshold = c(0.2, 0.8)),
  tt_decide(built, "maybe there is a problem")
))

# choose: the winner when the options differ, NA on an exact tie (the
# engine's own rule: a tie is no evidence), NA when the winner falls under
# a cut.
check("choose winner", identical(
  tt_choose("Which team owns this?", "I want a refund", c("refund team", "shipping", "account")),
  "refund team"
))
check("choose tie is na", is.na(
  tt_choose("Which team owns this?", "I want a refund", c("billing", "shipping", "account"))
))
check("choose under the cut is na", is.na(
  tt_choose("Which team owns this?", "I want a refund", c("refund team", "refund team two", "account"), threshold = 0.99)
))

# score: the specification's number and the nearest level in details.
scored <- tt_score("How urgent is this?", "I want a refund now", c("Routine.", "Soon.", "Immediate."))
check("score weighted position", isTRUE(all.equal(scored, 1.7)))
# Finding, recorded in NOTES.md: the stand-in's details path assumes a noul
# answer, so details on a score question fails with "the answer carries no
# probability". Nearest-level rides in annotate's score field instead.
check("details on a score question is a stand-in gap", inherits(
  tryCatch(tt_details(tt_question(score = "How urgent is this?", levels = c("Routine.", "Soon.", "Immediate.")), "I want a refund now"), thinkthen_error = function(e) e),
  "thinkthen_backend"
))
check("score column", isTRUE(all.equal(
  tt_score("How urgent?", c("I want a refund now", "thanks"), c("Routine.", "Soon.", "Immediate.")),
  c(1.7, 0.99)
)))

# tag: the labels that held, in the question's order.
tagged <- tt_tag("What is in this?", "I want a refund", c("refund", "billing"))[[1]]
check("tag holds the refund label", identical(tagged, "refund"))
check("tag column is a list", is.list(tt_tag("What is in this?", c("a", "b"), c("refund", "billing"))))

# filter: keeps the rows that held, refuses a band.
check("filter keeps", identical(
  tt_filter("Is this a complaint?", c("refund please", "thanks", "maybe later")),
  c("refund please", "maybe later")
))
band_refused <- tryCatch(
  tt_filter("Is this a complaint?", c("x"), threshold = c(0.2, 0.8)),
  thinkthen_error = function(e) e
)
check("filter refuses a band", inherits(band_refused, "thinkthen_usage"))
check("usage is not retryable", isFALSE(band_refused$retryable))

# rank: most likely yes first, ties keep input order.
check("rank order", identical(
  tt_rank("Is this urgent?", c("urgent refund", "no rush", "maybe soon")),
  c("urgent refund", "maybe soon", "no rush")
))
check("rank top", identical(tt_rank("Is this urgent?", c("a refund", "b", "c"), top = 1), "a refund"))

# find: the best unit of its peers.
check("find winner", identical(
  tt_find("Which line asks for money?", c("nothing here", "I want a refund", "still nothing")),
  "I want a refund"
))
too_few <- tryCatch(tt_find("Q?", c("one")), thinkthen_error = function(e) e$kind)
check("find needs two units", identical(too_few, "usage"))

# annotate: a set file over a frame, one column a question.
set_file <- tempfile(fileext = ".json")
writeLines(c(
  "{\"version\":1,\"questions\":{",
  "\"refund\":{\"decide\":\"Is a refund asked?\",\"threshold\":0.5},",
  "\"urgency\":{\"score\":\"How urgent?\",\"levels\":[\"low\",\"soon\",\"high\"]}}}"
), set_file)
frame <- data.frame(body = c("I want a refund now", "thanks all good"), stringsAsFactors = FALSE)
annotated <- tt_annotate(set_file, frame, on = "body")
check("annotate adds its columns", identical(names(annotated), c("body", "refund", "urgency")))
check("annotate decide column", identical(annotated$refund, c(TRUE, FALSE)))
check("annotate score column", isTRUE(all.equal(annotated$urgency[[1]], 1.7)))
check("annotate keeps input order", identical(annotated$body, frame$body))
missing <- tryCatch(tt_annotate("/no/such/file.json", frame, on = "body"),
  thinkthen_error = function(e) e$kind)
check("a named file that cannot be read is local", identical(missing, "local"))

# the six kinds as R conditions with the retry signal.
malformed <- tryCatch(tt_decide("Is this a complaint?", "this is malformed evidence"),
  thinkthen_error = function(e) e)
check("malformed evidence is backend", inherits(malformed, "thinkthen_backend"))
check("malformed is not retryable", isFALSE(malformed$retryable))
check("blank question is usage", identical(
  tryCatch(tt_decide("", "x"), thinkthen_error = function(e) e$kind), "usage"
))
check("the kind classes stack", inherits(malformed, "thinkthen_error") && inherits(malformed, "error"))
check("details carries the sends", identical(tt_details("Q?", "refund me")$sends, 1))

# the counters count sends.
tt_reset_usage()
tt_decide("Q?", c("refund me", "thanks"))
tt_decide("Q?", "refund me")
check("three judgments are three sends", identical(tt_usage()$requests, 3))

cat("null suite:", passed, "checks passed\n")
