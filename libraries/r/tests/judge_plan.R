# The installed functional door binds one question once, plans without a key,
# and keeps vector/grouped calls packed while dbplyr passes SQL through.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
suppressPackageStartupMessages(library(dplyr))
suppressPackageStartupMessages(library(dbplyr))
suppressPackageStartupMessages(library(purrr))
tt_engine(base_url = arm("arm/full/capture/v1"), cache = FALSE)

first <- paste0('{"state":"Each question quotes the text it asks about.",',
  '"model":"jev-1.13.0","questions":{',
  '"q1":{"type":"noul","instructions":"The text is \\"alpha\\". Q?"},',
  '"q2":{"type":"noul","instructions":"The text is \\"beta\\". Q?"}}}')
second <- paste0('{"state":"Each question quotes the text it asks about.",',
  '"model":"jev-1.13.0","questions":{',
  '"q1":{"type":"noul","instructions":"The text is \\"gamma\\". Q?"},',
  '"q2":{"type":"noul","instructions":"The text is \\"delta\\". Q?"}}}')

# Planning uses the literal expected body and independent byte/token oracles.
judge <- tt_decide("Q?", threshold = "0.2:0.95")
preview_sends <- sent_by({
  planned <- tt_plan(judge, c("alpha", NA_character_, "beta"))
  empty <- tt_plan(judge, c(NA_character_, NA_character_))
  split <- tt_plan(tt_decide("Q?", batch = 1L), c("alpha", "beta"))
  bad_context <- kind_of(tt_plan(judge, "x", context = "Valid but unbound"))
  bad_bytes <- "caf\xe9"
  Encoding(bad_bytes) <- "bytes"
  bad <- kind_of(tt_plan(judge, bad_bytes))
})
check("plan uses literal packed body, independent bytes and measured token band",
      identical(planned$first_body, first) && identical(planned$records, 2) &&
      identical(planned$requests, 2) && identical(planned$estimated_bytes, 218) &&
      identical(planned$estimated_input_tokens, list(lower = 112, upper = 198)) &&
      identical(planned$upper_bound, TRUE))
check("empty plan and malformed input are zero-send paths",
      preview_sends == 0L && identical(empty$records, 0) &&
      identical(empty$requests, 0) && is.null(empty$first_body) &&
      identical(bad, "usage") && identical(bad_context, "usage") &&
      identical(split$records, 2) && identical(split$requests, 2))

no_key_sends <- sent_by(no_key <- child(c(
  sprintf('tt_engine(base_url = "%s", cache = FALSE)', arm("arm/full/v1")),
  'preview <- tt_plan(tt_decide("Q?"), c("alpha", "beta"))',
  'cat(identical(Sys.getenv("THINKTHEN_API_KEY"), ""), identical(preview$records, 2), identical(preview$requests, 2), "\\n")'
), env = "THINKTHEN_API_KEY="))
check("a plan completes with no key and no listener request",
      no_key$status == 0L && identical(trimws(no_key$text), "TRUE TRUE TRUE") && no_key_sends == 0L)

# Grouped mutate invokes the vector door once per group, each a packed send.
rows <- data.frame(group = c("a", "a", "b", "b"),
                   body = c("alpha", "beta", "gamma", "delta"))
group_sends <- sent_by(grouped <- rows |>
  dplyr::group_by(group) |>
  dplyr::mutate(answer = tt_decide("Q?", body)$value))
check("grouped mutate answers the original four rows in two packed sends",
      group_sends == 2L && identical(grouped$answer, rep(TRUE, 4L)))
check("grouped bodies are the two independently written packed requests",
      identical(capture(), list(first, second)))

# The same prepared question works via direct, judge, and purrr partial calls.
vector <- c("alpha", NA_character_, "beta")
judge_sends <- sent_by(judged <- judge(vector))
direct_sends <- sent_by(direct <- tt_decide("Q?", vector, threshold = "0.2:0.95"))
partial_sends <- sent_by(partial <- purrr::partial(tt_decide, "Q?", threshold = "0.2:0.95")(vector))
check("band direct, judge and partial preserve values, probabilities and positions",
      all(c(judge_sends, direct_sends, partial_sends) == 1L) &&
      identical(judged$value, c(NA, NA, NA)) &&
      identical(judged$probability, c(0.9, NA_real_, 0.9)) &&
      identical(judged$value, direct$value) && identical(judged$value, partial$value) &&
      identical(judged$probability, direct$probability) &&
      identical(judged$probability, partial$probability) &&
      identical(vapply(judged$details, `[[`, 0, "index"), c(0, 2)))
number <- tt_decide("Q?", threshold = c(0.2, 0.95))
cut <- tt_choose("Which?", options = c("first", "second"), threshold = "0.95")
score <- tt_score("How much?", levels = c("low", "high"))
tag <- tt_tag("Which?", labels = c("first", "second"), threshold = "0.95")
check("all four asking verbs return functions only when input is omitted",
      all(vapply(list(judge, number, cut, score, tag), is.function, logical(1))) &&
      inherits(tt_decide("Q?", NULL), "thinkthen_call"))
number_sends <- sent_by(numeric_answer <- number("alpha"))
check("numeric and string bands keep the same judgment",
      number_sends == 1L && identical(numeric_answer$value, NA) &&
      identical(numeric_answer$probability, 0.9))
other_sends <- sent_by({
  selected <- cut("alpha")
  graded <- score("alpha")
  marked <- tag("alpha")
})
check("choose, score and tag judges apply with their ordinary answer shapes",
      other_sends == 3L && identical(selected$value, NA_character_) &&
      identical(selected$probability, NA_real_) &&
      is.numeric(graded$value) && is.null(graded$probability) &&
      is.list(marked$value) && is.null(marked$probability))
check("explicit NULL and NA remain eager no-work inputs",
      identical(tt_decide("Q?", NULL)$value, logical()) &&
      identical(tt_decide("Q?", NA_character_)$value, NA))

invalid_sends <- sent_by({
  old <- message_of(judge("x", deadline = 1))
  malformed <- kind_of(tt_decide("Q?", threshold = "0.95:0.2"))
  duplicate <- tempfile(fileext = ".json")
  writeLines('{"decide":"Q?","decide":"again"}', duplicate)
  repeated <- kind_of(tt_decide(tt_question(file = duplicate)))
  no_input <- kind_of(judge())
  no_plan_input <- kind_of(tt_plan(judge))
})
check("judge migration, malformed band, raw duplicates and omitted application fail before a send",
      invalid_sends == 0L &&
      identical(old, "deadline was renamed deadline_ms, in milliseconds") &&
      identical(c(malformed, repeated, no_input, no_plan_input),
                c("usage", "local", "usage", "usage")))

# A class/attribute lookalike cannot make plan run arbitrary R code. Public
# attribute mutation cannot give planning a second settings source, and the
# closure binding itself refuses ordinary assignment.
identity_sends <- sent_by({
  fake <- function(input) stop("forged callback ran")
  class(fake) <- c("thinkthen_judge", "function")
  attr(fake, "thinkthen_bound") <- list(kind = "decide", question = list(json = '{}'))
  forged <- kind_of(tt_plan(fake, "x"))
  changed <- judge
  body(changed) <- quote(stop("mutated callback ran"))
  mutated <- kind_of(tt_plan(changed, "x"))
  callback_ran <- FALSE
  hostile <- new.env(parent = parent.env(environment(judge)))
  makeActiveBinding("bound", function() { callback_ran <<- TRUE; list() }, hostile)
  lockEnvironment(hostile, bindings = TRUE)
  active <- judge
  environment(active) <- hostile
  active_kind <- kind_of(tt_plan(active, "x"))
  attr(judge, "thinkthen_bound") <- list(kind = "decide", question = list(json = '{}'), batch = 1L)
  same <- tt_plan(judge, c("alpha", "beta"))
  locked <- tryCatch({ environment(judge)$bound <- list(); FALSE }, error = function(e) TRUE)
  override_batch <- kind_of(tt_plan(tt_decide("Q?", batch = 1L), c("alpha", "beta"), batch = "max"))
  override_context <- kind_of(tt_plan(judge, "x", context = "other"))
})
check("forged and mutated functions, plan overrides, and a locked capture cannot split identity",
      identity_sends == 0L && locked && !callback_ran &&
      identical(c(forged, mutated, active_kind, override_batch, override_context),
                rep("usage", 5L)) &&
      identical(same$requests, 2) && identical(same$upper_bound, TRUE) && identical(same$first_body, first))

# A forged nested scalar can carry an S3 is.na callback. Validation must
# reject its class before any method runs, including when the JSON is classed.
trap_ran <- FALSE
is.na.tt_plan_trap <- function(x) {
  trap_ran <<- TRUE
  tt_decide("Q?", "unexpected judgment")
  FALSE
}
base_bound <- get("bound", environment(judge), inherits = FALSE)
forge_nested <- function(field) {
  bound <- base_bound
  trapped <- structure(if (field == "kind") "decide" else if (field == "json")
    bound$question$json else if (field == "batch") "max" else "Shared reference",
    class = "tt_plan_trap")
  if (field == "json") {
    question <- unclass(bound$question)
    question$json <- trapped
    class(question) <- "thinkthen_question"
    bound$question <- question
  } else {
    bound[[field]] <- trapped
  }
  held <- list2env(list(bound = bound), parent = parent.env(environment(judge)))
  lockEnvironment(held, bindings = TRUE)
  fake <- judge
  environment(fake) <- held
  fake
}
nested_sends <- sent_by(nested_kinds <- vapply(c("kind", "json", "batch", "context"),
  function(field) kind_of(tt_plan(forge_nested(field), "beta")), ""))
check("classed nested values refuse before is.na dispatch or any listener arrival",
      nested_sends == 0L && !trap_ran &&
      identical(unname(nested_kinds), rep("usage", 4L)))

same_sends <- sent_by(same_answer <- judge(c("alpha", "beta")))
check("an unrelated attribute cannot change the judge's planned body or actual send",
      same_sends == 1L && identical(same$requests, 2) && identical(same$upper_bound, TRUE) &&
      identical(same_answer$facts$requests_sent, 1L) &&
      identical(same_answer$probability, c(0.9, 0.9)) &&
      identical(same_answer$details[[1L]]$requests[[1L]],
                question_keys(arm("arm/full/capture/v1/systemone"), same$first_body, "jev-1.13.0")[[1L]]))

lazy <- dbplyr::lazy_frame(body = "x", con = dbplyr::simulate_dbi()) |>
  dplyr::mutate(accepted = thinkthen_decide("Q?", body, '{"threshold":0.7}'))
query <- paste(capture.output(show_query(lazy)), collapse = "\n")
check("dbplyr show_query preserves the extension spelling without a database",
      identical(query, paste("<SQL>",
        'SELECT `df`.*, thinkthen_decide(\'Q?\', `body`, \'{"threshold":0.7}\') AS `accepted`',
        "FROM `df`", sep = "\n")))

finish("judge plan", 10L)
