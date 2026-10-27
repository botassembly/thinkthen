source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
stub <- tt_client(backend = "stub")

# Feature: required lists, named options and numeric threshold translation.
q <- tt_choose_question("Which team?", c("billing", "shipping"), threshold = c(.2, .8))
stopifnot(inherits(q, "tt_choose_question"), inherits(q, "tt_question"))
condition(tt_choose("text", "Which team?", client = stub), "thinkthen_usage_error")
condition(tt_choose("text", q, c("other", "team"), client = stub), "thinkthen_usage_error")
condition(tt_choose("text", q, NULL, client = stub), "thinkthen_usage_error")
condition(tt_decide("text", q, client = stub), "thinkthen_usage_error")
for (bad in list("0.2:0.8", c(.8, .2), c(.1, .2, .3))) {
  condition(tt_decide("text", "Ready?", threshold = bad, client = stub),
            "thinkthen_usage_error", "usage.threshold")
}
for (call in list(
  function() tt_tag("text", "Tags?", "tag", threshold = c(.2, .8), client = stub),
  function() tt_find("text", "Which?", threshold = c(.2, .8), client = stub),
  function() tt_recognize("text", "person", threshold = c(.2, .8), client = stub))) {
  condition(call(), "thinkthen_usage_error", "usage.threshold")
}
for (call in list(
  function() tt_choose("text", "Which?", character(), client = stub),
  function() tt_tag("text", "Which?", character(), client = stub),
  function() tt_score("text", "How?", character(), client = stub),
  function() tt_rank("text", "How?", client = stub),
  function() tt_recognize("text", client = stub),
  function() tt_relate("text", client = stub),
  function() tt_decide("text", "Ready?", on = "/body", client = stub),
  function() tt_client(adapter = "stub"),
  function() tt_client(mode = "proxy"),
  function() tt_client(key_source = "OPENAI_API_KEY"),
  function() tt_client(batch = 1.5))) {
  condition(call(), "thinkthen_usage_error")
}
condition(tt_client(backend = "unknown"), "thinkthen_usage_error")
retry <- tt_retry(max_retries = 0L, initial_backoff_ms = 1L, attempt_timeout_ms = 1000L)
stopifnot(inherits(retry, "tt_retry"))

# Feature: named unknown options and wrong types retain shared admission messages.
fixture <- jsonlite::fromJSON(file.path(dirname(dirname(dirname(dirname(
  sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE)))))),
  "conformance", "public-contract", "usage-admission.json"), simplifyVector = FALSE)
# Each shared case reaches an exported boundary. Question constructors and rules
# use the public request envelope, which adds request. to the canonical path.
for (case in fixture) {
  stopifnot(identical(case$kind, "admission"))
  route <- switch(case$boundary,
    question = "projection: exported question or rule constructor in request envelope",
    toolbox = "direct: saved toolbox JSON",
    settings = "projection: exported retry constructor validates settings",
    stop(paste("unclassified admission boundary", case$boundary)))
  input <- case$input
  expected <- if (case$boundary == "question") paste0("request.", case$message) else case$message
  invoke <- switch(case$boundary,
    question = switch(input[["function"]],
      score = function() tt_score_question(input$question, unlist(input$levels)),
      tag = function() tt_tag_question(input$question, tags = unlist(input$tags)),
      relate = function() {
        rule <- input$rules[[1]]
        tt_rule(source = rule$source, target = rule$target, relation = rule$relation)
      },
      stop(paste("unclassified admission function", input[["function"]]))),
    toolbox = function() tt_toolbox_from_json(jsonlite::toJSON(input, auto_unbox = TRUE, null = "null")),
    settings = function() do.call(tt_retry, input$retry),
    stop(paste("unclassified admission boundary", case$boundary)))
  error <- condition(invoke(), "thinkthen_usage_error", case$code)
  stopifnot(identical(conditionMessage(error), expected),
            !grepl("secret-placeholder", conditionMessage(error), fixed = TRUE))
  cat("public admission: ", case$name, " (", route, ")\n", sep = "")
}
tags_case <- Filter(function(case) identical(case$name, "R tags in a serialized question"), fixture)[[1]]
for (call in list(
  function() tt_tag_question("Topics?", tags = c("billing", "shipping")),
  function() tt_tag("text", "Topics?", tags = c("billing", "shipping"), client = stub),
  function() tt_tag_question("Topics?", labels = "billing", tags = NULL),
  function() tt_tag("text", tt_tag_question("Topics?", "billing"), tags = "shipping", client = stub))) {
  error <- condition(call(), "thinkthen_usage_error", tags_case$code)
  stopifnot(identical(conditionMessage(error), paste0("request.", tags_case$message)))
}
for (call in list(
  function() tt_tag_question("Topics?"),
  function() tt_tag("text", "Topics?", client = stub))) {
  error <- condition(call(), "thinkthen_usage_error", "usage.question")
  stopifnot(identical(conditionMessage(error),
    "request.question.labels: expected one field value; received absent field"))
}
for (call in list(
  function() tt_tag_question("Topics?", labels = c(1L, 2L)),
  function() tt_tag("text", "Topics?", labels = c(1L, 2L), client = stub))) {
  error <- condition(call(), "thinkthen_usage_error", "usage.question")
  stopifnot(identical(conditionMessage(error),
    "request.question.labels[0]: expected a string; received integer `1`"))
}
for (call in list(
  function() tt_client(backend = "stub", retri = NULL),
  function() tt_retry(max_retry = 0L),
  function() tt_rule("person", "person", "knows", read = "/body"))) {
  error <- condition(call(), "thinkthen_usage_error")
  stopifnot(grepl(": expected ", conditionMessage(error), fixed = TRUE),
            grepl("received unknown field", conditionMessage(error), fixed = TRUE),
            grepl("did you mean", conditionMessage(error), fixed = TRUE))
}
# Unnamed dots retain the host conversion error without evaluating their values.
unnamed <- condition(tt_decide_question("Ready?", TRUE), "thinkthen_r_error", "r.arguments")
unevaluated <- condition(tt_decide_question("Ready?", stop("must stay unevaluated")),
                         "thinkthen_r_error", "r.arguments")
stopifnot(identical(conditionMessage(unnamed), conditionMessage(unevaluated)))

# Feature: selected questions own their domain and outlive the toolbox.
selected <- local({
  tools <- tt_toolbox(list(route = q))
  condition(tt_question(tools, "absent"), "thinkthen_usage_error", "usage.name")
  tt_question(tools, "route")
})
invisible(gc())
rows <- tt_choose(c("Refund please", "Where is my order?"), selected, client = stub)
check_rows(rows, "choose", 2L)
stopifnot(is.character(rows$answer), identical(rows$position, c(1, 2)),
          all(rows$answer %in% c("billing", "shipping") | is.na(rows$answer)))
rebound <- vctrs::vec_rbind(rows[1, ], rows[2, ])
stopifnot(identical(vctrs::vec_ptype(rebound), vctrs::vec_ptype(rows)))

# Feature: missing records retain typed missing answers and row positions.
for (input in list(c(NA_character_, NA_character_), list(NULL, NULL))) {
  out <- tt_decide(input, "Ready?", client = stub)
  check_rows(out, "decide", 2L)
  stopifnot(identical(out$position, c(1, 2)), all(out$status == "failed"),
            all(as.character(out$error_code) == "record.missing"),
            identical(out$answer, c(NA, NA)), all(is.na(out$probability)))
  stopifnot(identical(warned(tt_answer(out)), c(NA, NA)))
}
stopifnot(identical(warned(tt_decide_vec(NA_character_, "Ready?", client = stub)), NA),
          identical(warned(tt_keep(NA_character_, "Ready?", client = stub)), NA))
for (input in list(NULL, character(), list())) {
  out <- tt_decide(input, "Ready?", client = stub)
  check_rows(out, "decide", 0L)
  stopifnot(is.logical(out$answer))
}
missing_tag <- tt_tag(list(NULL), "Tags?", "billing", client = stub)
stopifnot(length(missing_tag$answer) == 1L, is.null(missing_tag$answer[[1]]),
          inherits(missing_tag$answer, "vctrs_list_of"))

# Feature: invalid ID values are failed records rather than identity coercions.
for (id in list(TRUE, 1.25, list(1L), list(key = "value"))) {
  out <- tt_decide(list(list(id = id, body = "text")), "Ready?", client = stub)
  stopifnot(nrow(out) == 1L, out$position == 1, out$status == "failed",
            as.character(out$error_code) == "record.invalid")
}

# Feature: result copies do not share mutable answer storage.
copy <- rows
copy$answer[[1]] <- "changed"
stopifnot(!identical(copy$answer, rows$answer))

# Feature: finalization and explicit close do not invalidate independent owners.
for (i in seq_len(3L)) {
  client <- tt_client(backend = "stub")
  alias <- client
  rm(client)
  invisible(gc())
  check_rows(tt_choose("text", selected, client = alias), "choose", 1L)
  tt_close(alias)
  tt_close(alias)
  closed <- condition(tt_choose("text", selected, client = alias), "thinkthen_usage_error", "usage.abi")
  stopifnot(identical(conditionMessage(closed), "client: expected an open client; received a closed client"))
  rm(alias)
}
invisible(gc())
with_tt_client(stub, {
  temporary <- tt_client(backend = "stub")
  tryCatch(with_tt_client(temporary, stop("scope exit")), error = function(e) NULL)
  tt_close(temporary)
  check_rows(tt_decide("text", "Ready?"), "decide", 1L)
})
local({
  local_tt_client(stub)
  check_rows(tt_decide("text", "Ready?"), "decide", 1L)
})

# Feature: keys stay out of settings prints, failures and diagnostic output.
key <- "synthetic-secret-never-render"
client <- tt_client(backend = "openai", api_key = key, url = "http://127.0.0.1:1/v1", retry = retry)
rendered <- capture.output({
  print(client)
  str(client)
  e <- condition(tt_decide("text", "Ready?", threshold = "bad", client = client),
                 "thinkthen_usage_error")
  print(e)
})
stopifnot(!any(grepl(key, rendered, fixed = TRUE)))
tt_close(client)
tt_close(stub)
cat("R typed questions, missing rows, resources and redaction passed\n")

# Feature: execution names are independent of selection and survive source projection.
client <- tt_client(backend = "stub")
question <- tt_question(tt_toolbox(list(route = tt_decide_question("Ready?"))), "route")
anonymous <- tt_decide("text", question, client = client)
named <- tt_decide("text", question, client = client, name = "nightly")
stopifnot(is.null(anonymous$source[[1]]$name), identical(named$source[[1]]$name, "nightly"),
          !identical(anonymous$source[[1]]$question_identity, named$source[[1]]$question_identity),
          identical(anonymous$answer, named$answer))
condition(tt_decide(character(), question, client = client, name = ""), "thinkthen_usage_error", "usage.name")

# Feature: rank remains selectable while annotation rejects it before execution.
rank <- tt_rank_question("How urgent?", c("routine", "urgent"))
selected <- tt_question(tt_toolbox(list(priority = rank)), "priority")
check_rows(tt_rank(c("alpha", "bravo"), selected, client = client), "rank", 2L)
condition(tt_annotation_questions(list(priority = selected)), "thinkthen_usage_error")

# Feature: question snapshots do not retain mutable caller vectors.
choices <- c("billing", "shipping")
question <- tt_choose_question("Which team?", choices)
choices[[1]] <- "changed"
stopifnot(all(tt_choose(c("alpha", "bravo"), question, client = client)$answer %in% c("billing", "shipping", NA)))
tt_close(client)

# Feature: annotation projection warns once for failed children and keeps their types.
client <- tt_client(backend = "stub")
questions <- tt_annotation_questions(list(missing = tt_decide_question("Ready?", evidence = "/missing")))
answer <- warned(tt_annotate_vec(list(list(body = "text")), questions, client = client))
child <- answer[[1]]$missing
stopifnot(inherits(child, "tt_decide_failed_outcome"), is.factor(child$status),
          is.factor(child$error$code), as.character(child$error$code) == "record.missing", is.na(child$answer))
tt_close(client)
