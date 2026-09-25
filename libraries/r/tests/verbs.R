# Every verb's R shape, the NA contract, the six kinds, the deadline rules,
# and the error-index rows that live in one call. The generic arm answers
# every request: yes at 0.9, the first option or level at 0.9, and each tag
# label at 0.9.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
levels3 <- c("Routine.", "Soon.", "Immediate.")

# decide: logical out, NA in gives NA with no request.
check("decide answers a logical column", identical(tt_decide("Is this a complaint?", c("refund me", "thanks")), c(TRUE, TRUE)))
check("NA evidence is NA and sends nothing", sent_by(check("NA stays NA",
  identical(tt_decide("Is this a complaint?", c(NA, "refund me now")), c(NA, TRUE)))) == 1L)
built <- tt_question(decide = "Is this a complaint?", threshold = c(0.2, 0.95))
check("a band answers unsure as NA", identical(tt_decide(built, "maybe there is a problem"), NA))
check("the text form equals the built form", identical(
  tt_decide("Is this a complaint?", "maybe there is a problem", threshold = c(0.2, 0.95)), NA))

# choose, score, and tag: one annotate crossing a column (R2-23 counts it).
check("choose picks the first option", identical(tt_choose("Which team?", c("a", NA), c("billing", "shipping")), c("billing", NA)))
check("choose under a cut it cannot reach is NA", is.na(tt_choose("Which team?", "a", c("billing", "shipping"), threshold = 0.95)))
check("score is the weighted position", isTRUE(all.equal(tt_score("How urgent?", c("a", NA), levels3), c(0.15, NA))))
check("tag answers every label that held", identical(tt_tag("What is in this?", c("a", NA), c("refund", "billing")),
                                                     list(c("refund", "billing"), character())))
check("R6-10: one label stays an array", identical(tt_tag("Tags?", "refund me", labels = "refund"), list("refund")))

# filter, rank, and find keep the ruled shapes.
check("filter keeps the records that held", identical(tt_filter("Is this a complaint?", c("x1", "x2")), c("x1", "x2")))
check("filter refuses a band", identical(kind_of(tt_filter("Q?", "x", threshold = c(0.2, 0.8))), "usage"))
ranked <- tt_rank("Is this urgent?", c("r1", "r2", "r3"))
check("rank is place, record, probability", identical(names(ranked), c("place", "record", "probability")) &&
      identical(ranked$place, 1:3) && identical(ranked$record, c("r1", "r2", "r3")))
check("rank top", identical(nrow(tt_rank("Is this urgent?", c("r1", "r2"), top = 1)), 1L))
found <- tt_find("Which line asks for money?", c("u1", "u2"))
check("find is place, unit, probability", identical(found, list(place = 1L, unit = "u1", probability = 0.9)))
check("find needs two units", identical(kind_of(tt_find("Q?", "one")), "usage"))

# details: the command's --details document.
details <- tt_details(tt_question(score = "How urgent?", levels = levels3), "urgent now")
check("details on a score question carries the level", identical(details$answer$level, "Routine."))
check("details on a decide question carries no level", is.null(tt_details("Q?", "refund me")$answer$level))

# annotate: a column a question, typed by its kind (R1-5).
set_file <- tempfile(fileext = ".json")
writeLines('{"version":1,"questions":{"refund":{"decide":"Refund?","threshold":"0.2:0.95"},
  "team":{"choose":"Which team?","options":["billing","other"]},
  "urgency":{"score":"How urgent?","levels":["low","soon","high"]},
  "labels":{"tag":"Which labels?","labels":["billing","urgent"],"threshold":0.95}}}', set_file)
frame <- data.frame(body = c("n1", "n2"), stringsAsFactors = FALSE)
annotated <- tt_annotate(set_file, frame, on = "body")
check("annotate adds its columns in set order", identical(names(annotated), c("body", "refund", "team", "urgency", "labels")))
check("R1-5: a banded decide column stays logical", identical(annotated$refund, c(NA, NA)))
check("choose and score columns keep their types", identical(annotated$team, c("billing", "billing")) &&
      isTRUE(all.equal(annotated$urgency, c(0.15, 0.15))))
check("R1-5: an empty tag column stays a list of character vectors",
      identical(annotated$labels, list(character(0), character(0))))
check("R3-15: a question named like an input column is refused before any request", sent_by(check("the clash is usage",
  identical(message_of(tt_annotate(set_file, data.frame(body = "x", team = "y"), on = "body")),
            "annotate cannot add a question named 'team': the input already has a column by that name; rename one"))) == 0L)
check("a set file that cannot be read is local", identical(kind_of(tt_annotate("/no/such.json", frame, on = "body")), "local"))

# R3-3: main's set parser refuses a NUL in a member name first, and the
# message crosses with the NUL escaped. The Rust unit test pins the escape.
nul <- tempfile(fileext = ".json")
writeLines('{"version":1,"questions":{"a\\u0000b":{"decide":"Q?"}}}', nul)
check("R3-3: a NUL name is refused and the process lives", identical(message_of(tt_annotate(nul, frame, on = "body")),
  "`questions.a\\u0000b` uses lowercase letters, digits, and underscores, and is not empty"))

# R2-5 and R5-11: a percent and the separator reach the engine's refusal
# with the kind intact. Main's messages name no option, and the Rust unit
# test pins the doubled percent.
held <- tryCatch(tt_choose("Which?", "t", options = c("100% sure %s", "100% sure %s")), error = function(e) e)
check("R2-5: a duplicate percent option is usage", inherits(held, "thinkthen_usage") &&
      identical(conditionMessage(held), "the question file's `options`: a list holds each option once"))
check("R5-11: a separator in an option keeps the usage kind",
      inherits(tryCatch(tt_choose("Which?", "t", options = c("a\x1fb", "a\x1fb")), error = function(e) e), "thinkthen_usage"))

# The deadline rules (R7-11, R2-10, R1-11): each refusal sends nothing.
deadline_kind <- function(value) {
  kind <- NULL
  count <- sent_by(kind <- kind_of(tt_decide("Q?", paste("deadline", format(value)), deadline = value)))
  paste(kind, count)
}
for (value in list(5L, I(5), -1L, -1, NULL)) {
  check(paste("the deadline", format(value), "answers"), grepl("^none", deadline_kind(value)))
}
for (value in list(0L, I(0L), 0)) {
  check(paste("the deadline", format(value), "is spent"), identical(deadline_kind(value), "deadline 0"))
}
for (value in list(NA_integer_, factor("5"), I(factor("5")), as.difftime(5, units = "secs"), TRUE,
                   1e300, Inf, -Inf, NaN, -2, -2L, 4294967296)) {
  check(paste("the deadline", format(value), "is usage"), identical(deadline_kind(value), "usage 0"))
}

# The six kinds as conditions with the retry signal.
refused <- tryCatch(tt_decide("", "x"), thinkthen_error = function(e) e)
check("a blank question is usage, and usage is not retryable",
      inherits(refused, "thinkthen_usage") && isFALSE(refused$retryable) && inherits(refused, "error"))

# The failed marker: the malformed arm breaks the last question.
failed <- child(c(
  sprintf('f <- "%s"', set_file),
  'a <- tt_annotate(f, data.frame(body = "m1"), on = "body")',
  'cat(identical(a$labels, list(list(failed = list(kind = "backend", cause = "missing_answer")))), identical(a$team, "billing"), "\\n")',
  'e <- tryCatch(tt_choose("Which?", c("m2", "m3"), c("x", "y")), error = function(e) e)',
  'cat(class(e)[[1]], conditionMessage(e), "\\n")'
), env = paste0("THINKTHEN_BASE_URL=", arm("arm/malformed/missing_answer/v1")))
check("a failed annotate cell carries the ruled marker beside good answers", grepl("TRUE TRUE", failed$text, fixed = TRUE))
# A one-question request with its one answer broken is refused whole, so a
# broken choose, score, or tag column raises backend and never reads NA.
check("a broken choose column raises backend", grepl(
  "thinkthen_backend the reply was refused: the response carries no answer for question `q1`", failed$text, fixed = TRUE))

# A question that names a model cannot join a set, so the bulk verbs ask it
# one row at a time and still answer.
named <- tt_question(choose = "Which team?", options = c("billing", "shipping"), model = "other-model")
check("a named-model choose answers every row", identical(tt_choose(named, c("n1", "n2")), c("billing", "billing")))

# The counters count sends, as doubles.
before <- tt_usage()
invisible(tt_decide("Q?", c("u1-new", "u2-new")))
check("two judgments are two sends", identical(tt_usage()$requests_sent - before$requests_sent, 2))
check("the counters are the four doubles", identical(names(before), c("requests_sent", "cache_answers", "input_tokens", "output_tokens")))

finish("verbs", 28L)
