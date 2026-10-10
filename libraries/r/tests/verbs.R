# Every verb's R shape, the NA contract, the six kinds, the deadline rules,
# and the error-index rows that live in one call. The generic arm answers
# every request: yes at 0.9, the first option or level at 0.9, and each tag
# label at 0.9.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(batch = 1L)  # Preserve this file's per-record send assertions.
levels3 <- c("Routine.", "Soon.", "Immediate.")

# decide: logical out, NA in gives NA with no request.
check("decide answers a logical column", identical(tt_decide("Is this a complaint?", c("refund me", "thanks"))$value, c(TRUE, TRUE)))
check("NA evidence is NA and sends nothing", sent_by(check("NA stays NA",
  identical(tt_decide("Is this a complaint?", c(NA, "refund me now"))$value, c(NA, TRUE)))) == 1L)
built <- list(decide = "Is this a complaint?", threshold = "0.2:0.95")
check("a band answers unsure as NA", identical(tt_decide(built, "maybe there is a problem")$value, NA))
check("the text form equals the built form", identical(
  tt_decide("Is this a complaint?", "maybe there is a problem", options = list(threshold = "0.2:0.95"))$value, NA))

# Named choose, score, and tag calls retain ordinary column views.
check("choose picks the first option", identical(tt_choose(list(choose = "Which team?", options = c("billing", "shipping")), c("a", NA))$value, c("billing", NA)))
# Native threshold judgment reuses the cached answer for the same
# prompt and text without another send.
check("choose under a cut it cannot reach is NA", is.na(tt_choose(list(choose = "Which team?", options = c("billing", "shipping")), "a", options = list(threshold = 0.95))$value))
saved_choice <- list(choose = "Which team?", options = c("billing", "shipping"), threshold = 0.95)
check("a built choose question retains the same unsure answer", is.na((built_choice <- tt_choose(saved_choice, "a"))$value) &&
      is.null(built_choice$results[[1L]]$value))
check("score is the weighted position", isTRUE(all.equal(tt_score(list(score = "How urgent?", levels = levels3), c("a", NA))$value, c(0.15, NA))))
check("tag answers every label that held", identical(tt_tag(list(tag = "What is in this?", labels = c("refund", "billing")), c("a", NA))$value,
                                                     list(c("refund", "billing"), character())))
check("R6-10: one label stays an array", identical(tt_tag(list(tag = "Tags?", labels = I("refund")), "refund me")$value, list("refund")))

# Filter, rank and find retain typed values, originals and native indexes.
check("filter keeps the records that held", identical(vapply(tt_filter("Is this a complaint?", c("x1", "x2"))$results, `[[`, "", "input"), c("x1", "x2")))
check("filter refuses a band", identical(kind_of(tt_filter("Q?", "x", options = list(threshold = "0.2:0.8"))), "usage"))
ranked <- tt_rank("Is this urgent?", c("r1", "r2", "r3"))$results
check("rank returns typed places and original records", all(vapply(ranked, inherits, TRUE, "thinkthen_RankResult")) &&
      identical(vapply(ranked, `[[`, 0, "value"), c(1, 2, 3)) && identical(vapply(ranked, `[[`, "", "input"), c("r1", "r2", "r3")))
check("rank top", length(tt_rank("Is this urgent?", c("r1", "r2"), options = list(top = 1L))$results) == 1L)
found <- tt_find(list(find = "Which line asks for money?"), c("u1", "u2"))$results[[1L]]
check("find retains selected index, original and probability", found$index == 0 && identical(found$value, "u1") && found$answer$probabilities$u001 == 0.9)
check("find needs two units", identical(kind_of(tt_find(list(find = "Q?"), "one")), "usage"))
check("a none other than TRUE or FALSE is refused before any request", sent_by(check("the none sentence",
  identical(message_of(tt_find(list(find = "Q?"), c("u1", "u2"), options = list(none = NA))), "invalid canonical request"))) == 0)

# Duplicate texts retain distinct original positions even when their answers tie.
check("tied duplicates keep original places", identical(
  vapply(tt_rank("Duplicate order?", c("same", "other", "same"))$results, `[[`, 0, "index"), c(0, 1, 2)))
check("empty rank has no native rows and sends nothing", sent_by(check("empty rank shape",
  identical(tt_rank("Empty order?", character())$results,
    list()))) == 0L)
check("empty find is usage and sends nothing", sent_by(check("empty find kind",
  identical(kind_of(tt_find(list(find = "Empty selection?"), character())), "usage"))) == 0L)

# details: the command's --details document.
details <- tt_details(tt_question(score = "How urgent?", levels = levels3), "urgent now")$value
check("details on a score question carries the level", identical(details$answer$level, "Routine."))
check("details on a decide question carries no level", is.null(tt_details("Q?", "refund me")$value$answer$level))

# Annotation retains original rows and separately typed question values.
set_file <- tempfile(fileext = ".json")
writeLines('{"version":1,"questions":{"refund":{"decide":"Refund?","threshold":"0.2:0.95"},
  "team":{"choose":"Which team?","options":["billing","other"]},
  "urgency":{"score":"How urgent?","levels":["low","soon","high"]},
  "labels":{"tag":"Which labels?","labels":["billing","urgent"],"threshold":0.95}}}', set_file)
frame <- data.frame(body = c("n1", "n2"), stringsAsFactors = FALSE)
annotated <- tt_annotate(tt_question(file = set_file), frame, options = list(field = list("/body")))$results
check("annotate keeps originals separately from named values", identical(annotated[[1L]]$input, list(body = "n1")) && identical(names(annotated[[1L]]$value), c("labels", "refund", "team", "urgency")))
check("R1-5: banded decide annotations remain null", all(vapply(annotated, function(row) is.null(row$value$refund), TRUE)))
check("choose and score columns keep their types", identical(vapply(annotated, function(row) row$value$team, ""), c("billing", "billing")) &&
      isTRUE(all.equal(vapply(annotated, function(row) row$value$urgency, 0), c(0.15, 0.15))))
check("R1-5: empty tag annotations remain empty lists",
      all(vapply(annotated, function(row) identical(row$value$labels, list()), TRUE)))
clash <- tt_annotate(tt_question(file = set_file), data.frame(body = "x", team = "y"), options = list(field = list("/body")))$results[[1L]]
check("an input name and annotation remain separately accessible", identical(clash$input$team, "y") && identical(clash$value$team, "billing"))
check("a set file that cannot be read is local", identical(kind_of(tt_annotate(tt_question(file = "/no/such.json"), frame, options = list(field = list("/body")))), "local"))

# R3-3: main's set parser refuses a NUL in a member name first, and the
# message crosses with the NUL escaped. The Rust unit test pins the escape.
nul <- tempfile(fileext = ".json")
writeLines('{"version":1,"questions":{"a\\u0000b":{"decide":"Q?"}}}', nul)
nul_sends <- sent_by(nul_error <- tryCatch(tt_annotate(tt_question(file = nul), frame, options = list(field = list("/body"))), thinkthen_error = identity))
check("R3-3: a NUL name is refused and the process lives", nul_sends == 0L && inherits(nul_error, "thinkthen_local") && inherits(nul_error$complete, "thinkthen_CallError") && identical(conditionMessage(nul_error),
  "`questions.a\\u0000b` uses lowercase letters, digits, and underscores, and is not empty"))

# R2-5 and R5-11: a percent and the separator reach the engine's refusal
# with the kind intact. Main's messages name no option, and the Rust unit
# test pins the doubled percent.
held <- tryCatch(tt_choose(list(choose = "Which?", options = c("100% sure %s", "100% sure %s")), "t")$value, error = function(e) e)
check("R2-5: a duplicate percent option is usage", inherits(held, "thinkthen_usage") &&
      identical(conditionMessage(held), "invalid canonical request"))
check("R5-11: a separator in an option keeps the usage kind",
      inherits(tryCatch(tt_choose(list(choose = "Which?", options = c("a\x1fb", "a\x1fb")), "t")$value, error = function(e) e), "thinkthen_usage"))

# The deadline rules (R7-11, R2-10, R1-11): each refusal sends nothing.
deadline_kind <- function(value) {
  kind <- NULL
  count <- sent_by(kind <- kind_of(tt_decide("Q?", paste("deadline", format(value)), deadline_ms = value)$value))
  paste(kind, count)
}
for (value in list(5000L, I(5000), -1L, -1, NULL)) {
  check(paste("the deadline", format(value), "answers"), grepl("^none", deadline_kind(value)))
}
for (value in list(0L, I(0L), 0)) {
  check(paste("the deadline", format(value), "is spent"), identical(deadline_kind(value), "deadline 0"))
}
for (value in list(NA_integer_, factor("5"), I(factor("5")), as.difftime(5, units = "secs"), TRUE,
                   1e300, Inf, -Inf, NaN, -2, -2L, 4294967295001)) {
  check(paste("the deadline", format(value), "is usage"), identical(deadline_kind(value), "usage 0"))
}

# The six kinds as conditions with the retry signal.
blank_sent <- sent_by(refused <- tryCatch(tt_decide("", "x")$value, thinkthen_error = function(e) e))
check("a blank question is usage, and usage is not retryable",
      inherits(refused, "thinkthen_usage") && isFALSE(refused$retryable) && inherits(refused, "error") && blank_sent == 0L)

# The shared local fault is a recording read on decide, not an annotate file.
recording <- tempfile("replay-read-")
dir.create(recording)
seeded <- sent_by(seed <- child(c(
  sprintf('tt_engine(record = "%s", cache = FALSE)', recording),
  'invisible(tt_decide("Q?", "recording read proof"))'
)))
# The recording is one question store by ADR 0111; overwriting it with text
# makes the replay read fail.
entry <- file.path(recording, "thinkthen.sqlite")
check("one recorded decide exchange was seeded", seed$status == 0L && seeded == 1L && file.exists(entry))
writeLines("not a question store", entry)
replayed <- sent_by(read_result <- child(c(
  sprintf('tt_engine(replay = "%s")', recording),
  'e <- tryCatch(tt_decide("Q?", "recording read proof"), thinkthen_error = function(e) e)',
  'cat(inherits(e, "thinkthen_local"), identical(e$kind, "local"), isFALSE(e$retryable), "\\n")'
)))
check("a failed recording read is a non-retryable local error before any send",
      read_result$status == 0L && identical(trimws(read_result$text), "TRUE TRUE TRUE") && replayed == 0L)

# The failed marker: the malformed arm breaks the last question.
failed <- child(c(
  sprintf('f <- "%s"', set_file),
  'call <- tt_annotate(f, data.frame(body = "m1"), on = "body")',
  'a <- call$value',
  'cat(identical(a$labels, list(list(failed = list(kind = "backend", cause = "missing_answer")))), identical(a$team, "billing"), "\\n")',
  'cat(call$facts$records > 0, any(vapply(call$details, function(x) !is.null(x$failed), FALSE)), any(vapply(call$details, function(x) !is.null(x$answer), FALSE)), "\\n")',
  'e <- tryCatch(tt_choose("Which?", c("m2", "m3"), c("x", "y")), error = function(e) e)',
  'cat(class(e)[[1]], conditionMessage(e), "\\n")'
), env = paste0("THINKTHEN_BASE_URL=", arm("arm/malformed/missing_answer/v1")))
check("a failed annotate cell carries its marker and ordered observations beside good answers",
      grepl("TRUE TRUE", failed$text, fixed = TRUE) &&
      grepl("TRUE TRUE TRUE", failed$text, fixed = TRUE))
# A one-question request with its one answer broken is refused whole, so a
# broken choose, score, or tag column raises backend and never reads NA.
check("a broken choose column raises backend", grepl(
  "thinkthen_backend a backend question failed in a batch", failed$text, fixed = TRUE))

# A question that names a model still answers through the dynamic many path.
# Batch one preserves this older file's per-record request count.
named <- list(choose = "Which team?", options = c("billing", "shipping"), model = "other-model")
check("a named-model choose answers every row", identical(tt_choose(named, c("n1", "n2"), options = list(batch = 1L))$value, c("billing", "billing")))

# The counters count sends, as doubles.
before <- tt_usage()
invisible(tt_decide("Q?", c("u1-new", "u2-new"))$value)
check("two judgments are two sends", identical(tt_usage()$requests_sent - before$requests_sent, 2L))
check("the counters name five counts", identical(names(before), c("requests_sent", "retries", "input_tokens", "output_tokens", "cache_answers")))

finish("verbs", 30L)
