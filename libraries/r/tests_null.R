# The null suite: every verb, the six kinds, and the NA contract, against
# the null backend with no network at all.
.libPaths(c("rlib", .libPaths()))
library(thinkthen)
stopifnot(Sys.getenv("ENGINE_NULL") == "1")

# The stand-in's one synthesized partial failure (0054) is a compile-time
# build door since review 1: this suite needs a package installed with
# THINKTHEN_R_SYNTHETIC_PARTIAL=1 (check.sh installs one and restores the
# production shape afterwards). No environment variable arms it.

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
# Settled 2026-09-21: the nearest level's name rides in tt_details on a
# score question, and it is absent on every other verb.
check("details on a score question carries the nearest level", identical(
  tt_details(tt_question(score = "How urgent is this?", levels = c("Routine.", "Soon.", "Immediate.")), "I want a refund now")$nearest,
  "Immediate."
))
check("details on a decide question carries no nearest", is.null(
  tt_details("Is this a complaint?", "I want a refund")$nearest
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

# rank: the ruled pair (settled 2026-09-21) as a data frame — most likely
# yes first, ties keep input order.
ranked <- tt_rank("Is this urgent?", c("urgent refund", "no rush", "maybe soon"))
check("rank order", identical(ranked$record, c("urgent refund", "maybe soon", "no rush")))
check("rank carries the pair", identical(names(ranked), c("place", "record", "probability")))
check("rank probabilities descend", all(diff(ranked$probability) <= 0))
check("rank top", identical(tt_rank("Is this urgent?", c("a refund", "b", "c"), top = 1)$record, "a refund"))

# find: the ruled pair (settled 2026-09-21) — place, unit, probability.
found <- tt_find("Which line asks for money?", c("nothing here", "I want a refund", "still nothing"))
check("find winner", identical(found$unit, "I want a refund"))
check("find carries the pair", identical(names(found), c("place", "unit", "probability")))
check("find place", identical(found$place, 2L))
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

# 0053 and 0054 on the details list: the ordered requests digests (one
# 64-figure digest a logical request) and failed_questions, always
# present, zero for one good question.
trail <- tt_details("Q?", "refund me")
check("details carries the requests list",
      is.character(trail$requests) && length(trail$requests) == 1L &&
        grepl("^[0-9a-f]{64}$", trail$requests[[1]]))
check("details carries failed_questions as zero",
      identical(as.integer(trail$failed_questions), 0L))

# The stand-in's one synthesized partial failure (0054): the reply answers
# one question and omits the last in name order, so its column widens to a
# list whose failed cell is the ruled marker `list(failed = ...)`, never
# NA, while the good answer rides beside it.
partial_file <- tempfile(fileext = ".json")
writeLines(paste0(
  "{\"version\":1,\"questions\":{",
  "\"refund\":{\"decide\":\"Is this a refund request?\",\"threshold\":0.5},",
  "\"topic\":{\"decide\":\"Is this a billing problem?\",\"threshold\":0.5}}}"
), partial_file)
partial <- tt_annotate(
  partial_file,
  data.frame(body = "order 4471: charged twice, please refund", stringsAsFactors = FALSE),
  on = "body"
)
check("the failed field carries the ruled marker",
      identical(partial$topic[[1]], list(failed = list(kind = "backend", cause = "missing_answer"))))
check("the good answer rides beside the failure", identical(partial$refund[[1]], TRUE))
clean <- tt_annotate(
  partial_file,
  data.frame(body = "I want a refund for order 4471", stringsAsFactors = FALSE),
  on = "body"
)
check("a clean row keeps its plain column", identical(clean$topic[[1]], TRUE))

# The review's finding 5: an answer column's type comes from its question's
# kind, never from the shape of the first answer. The set below holds one
# question of every kind, and the first record is the misleading one: its
# decision answer is unsure (R's NULL) and its empty tag answer is R's
# zero-length character. Every later answer must still come back in its
# kind's own shape. The null backend answers a tag question the same for
# every row (its rule reads the labels, not the evidence), so the tag
# columns prove the cell shape rather than a per-row change of labels.
kinded_file <- tempfile(fileext = ".json")
writeLines(c(
  "{\"version\":1,\"questions\":{",
  "\"refund\":{\"decide\":\"Is a refund asked?\",\"threshold\":\"0.2:0.8\"},",
  "\"topic\":{\"choose\":\"Which topic?\",\"options\":[\"refund\",\"billing\",\"other\"]},",
  "\"labels\":{\"tag\":\"What is in this?\",\"labels\":[\"refund\",\"maybe\",\"billing\"]},",
  "\"empty\":{\"tag\":\"What is in this?\",\"labels\":[\"billing\",\"other\"]},",
  "\"urgency\":{\"score\":\"How urgent?\",\"levels\":[\"low\",\"soon\",\"high\"]}}}"
), kinded_file)
kinded <- tt_annotate(
  kinded_file,
  data.frame(body = c(
    "maybe there is a problem",
    "I want a refund and maybe more",
    "thanks, all good"
  ), stringsAsFactors = FALSE),
  on = "body"
)
check("an unsure first answer leaves the decision column logical",
      identical(kinded$refund, c(NA, TRUE, FALSE)))
check("a tag column holds every label of a row",
      is.list(kinded$labels) && identical(kinded$labels[[2]], c("refund", "maybe")))
check("a tag column whose first answer holds no label stays a list of labels",
      is.list(kinded$empty) && identical(kinded$empty[[2]], character(0)))
check("a score column stays double with its decimal answers",
      is.double(kinded$urgency) && isTRUE(all.equal(kinded$urgency, c(1.05, 1.7, 0.99))))
check("a choose column stays character", is.character(kinded$topic))

# The offline backend cannot vary a choose answer from row to row (its
# null reply is one fixed distribution per question), so the
# first-answer-unsure case is proven on the converter itself: R's NULL is
# the engine's unsure, and the later cells must still come back as their
# choices.
chosen <- thinkthen:::.tt_answer_column(list(NULL, "refund", "billing"), "choose")
check("a choose column with an unsure first answer keeps later choices",
      identical(chosen, c(NA_character_, "refund", "billing")))

# The contract owns the one checked deadline conversion (phase 1): zero
# stays the spent deadline, the sentinel and NULL mean no deadline, and a
# NaN, a negative other than the sentinel, or an oversized budget is a
# usage error before any thread exists (the review's crash group).
check("zero stays the spent deadline", identical(
  tryCatch(tt_decide("Q?", "refund me", deadline = 0), thinkthen_error = function(e) e$kind),
  "deadline"))
check("the sentinel means no deadline", isTRUE(tt_decide("Q?", "refund me", deadline = -1)))
check("a NaN deadline is usage", identical(
  tryCatch(tt_decide("Q?", "refund me", deadline = NaN), thinkthen_error = function(e) e$kind),
  "usage"))
check("a negative deadline is usage", identical(
  tryCatch(tt_decide("Q?", "refund me", deadline = -2), thinkthen_error = function(e) e$kind),
  "usage"))
check("an oversized deadline is usage", identical(
  tryCatch(tt_decide("Q?", "refund me", deadline = Inf), thinkthen_error = function(e) e$kind),
  "usage"))

# the counters count sends. No reset exists (ruling 4): the difference
# across the three sends carries the same proof.
before <- tt_usage()$requests
tt_decide("Q?", c("refund me", "thanks"))
tt_decide("Q?", "refund me")
check("three judgments are three sends", identical(tt_usage()$requests - before, 3))

# the defect kind raises its own R condition; the Rust half's unit test
# constructs the contract Error with kind defect and packs it (check.sh).
defect_cond <- tryCatch(
  stop(thinkthen:::.tt_condition(paste0("defect", "\u001f", "false", "\u001f",
                                        "the engine broke its own contract"))),
  thinkthen_defect = function(e) e
)
check("the defect kind raises thinkthen_defect", inherits(defect_cond, "thinkthen_defect"))
check("its retryable rides the condition", identical(defect_cond$retryable, FALSE))
check("its message rides the condition",
      identical(defect_cond$message, "the engine broke its own contract"))

# One-item member lists stay JSON arrays (surfaces-review-5: auto_unbox
# wrote labels = "only", and tt_tag with one label was refused while
# Python and Ruby answered). Each array field the writer emits, in its
# one-item form, read back from the JSON the engine receives.
is_array <- function(json, field) {
  parsed <- jsonlite::fromJSON(json, simplifyVector = FALSE)
  is.list(parsed[[field]]) && is.null(names(parsed[[field]])) && length(parsed[[field]]) == 1L
}
for (kind in c("choose", "score", "tag")) {
  field <- switch(kind, choose = "options", score = "levels", tag = "labels")
  check(paste("a built", kind, "keeps one", field, "as an array"),
        is_array(thinkthen:::.tt_body(kind, "Q?", "only", NULL, NULL), field))
}
check("a plain tag call keeps one label as an array", {
  body <- list(tag = "Tags?", labels = I("only"))
  is_array(thinkthen:::.tt_json(body), "labels")
})
check("recognize keeps one kind as an array",
      is_array(thinkthen:::.tt_recognize_spec("person", NULL, NULL, NULL), "kinds"))
check("recognize keeps one relation rule as an array",
      is_array(thinkthen:::.tt_recognize_spec("person", "met", NULL, NULL), "relations"))
check("relate keeps one relation rule as an array",
      is_array(thinkthen:::.tt_relate_spec("met", NULL, NULL, NULL), "relations"))
check("relate keeps one either rule as an array",
      is_array(thinkthen:::.tt_relate_spec(NULL, "met", NULL, NULL), "relations"))
check("tt_tag answers with one label",
      is.character(tt_tag("Tags?", "refund me", labels = "only")[[1L]]))
check("a built tag question answers with one label",
      is.character(tt_tag(tt_question(tag = "Tags?", labels = "only"), "refund me")[[1L]]))
one_option <- tryCatch(tt_choose("Which?", "t", options = "only"), error = function(e) e)
check("one choose option meets the contract's count rule, not a shape error",
      inherits(one_option, "thinkthen_usage") &&
        identical(conditionMessage(one_option), "the question file's `options`: `choose` takes 2 to 255 options"))

# The R half's own refusals carry the usage kind (surfaces-review-5: they
# were plain errors no thinkthen_usage handler caught).
usage_of <- function(expr) inherits(tryCatch(expr, error = function(e) e), "thinkthen_usage")
check("a bad threshold at the call is a usage error", usage_of(tt_decide("Q?", "t", threshold = 2)))
check("a bad threshold in a built question is a usage error",
      usage_of(tt_question(decide = "Q?", threshold = 2)))
check("relate with no rules is a usage error",
      usage_of(tt_relate(c("a", "b"), relations = character(0))))
check("a question with two kinds is a usage error",
      usage_of(tt_question(decide = "Q?", tag = "T?", labels = "a")))

cat("null suite:", passed, "checks passed\n")
