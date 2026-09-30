# One installed R call's actual packed wire, final account, original positions,
# and complete question JSON values. This file has its own loopback backend.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))

url <- arm("arm/full/capture/v1/systemone")
tt_engine(base_url = arm("arm/full/capture/v1"), cache = FALSE)
packed_receipt <- tt_completion()
packed <- tt_decide("Q?", c("alpha", NA_character_, "beta"), completion = packed_receipt)
expected <- paste0('{"state":"Each question quotes the text it asks about.",',
  '"model":"jev-1.13.0","questions":{',
  '"q1":{"type":"noul","instructions":"The text is \\"alpha\\". Q?"},',
  '"q2":{"type":"noul","instructions":"The text is \\"beta\\". Q?"}}}')
bodies <- capture()
check("default Max captures one exact packed body", length(bodies) == 1L && identical(bodies[[1L]], expected))
hash <- digest(url, expected)
check("packed digest comes from independent body and served URL",
      identical(vapply(packed$details, function(one) one$requests[[1L]], ""), rep(hash, 2L)))
check("one packed send has full call facts and original R indexes",
      inherits(packed, "thinkthen_call") && identical(packed$value, c(TRUE, NA, TRUE)) &&
      identical(packed$probability, c(0.9, NA_real_, 0.9)) &&
      identical(packed$facts$records, 2) && identical(packed$facts$requests_sent, 1) &&
      identical(packed$facts$input_tokens, 1) && identical(packed$facts$output_tokens, 1) &&
      identical(vapply(packed$details, `[[`, 0, "index"), c(0, 2)) &&
      identical(vapply(packed$details, `[[`, 0, "requests_sent"), c(1, 0)))
check("a completed receipt uses the same original R indexes as the returned call",
      identical(vapply(tt_completion_read(packed_receipt)$details, `[[`, 0, "index"), c(0, 2)) &&
      identical(tt_completion_read(packed_receipt)$facts$requests_sent, packed$facts$requests_sent))

# A structured question is deliberately sent as one record per request by
# the core planner. Its nested array/object and explicit null must survive.
structured <- tt_question(decide = list(ask = "Refund?", hints = list("one", list(a = NULL, b = 1))),
                          true = NULL)
check("explicit null stays present while omitted false stays absent",
      identical(structured$json,
        '{"decide":{"ask":"Refund?","hints":["one",{"a":null,"b":1}]},"true":null}'))
one <- tt_decide(structured, "gamma")
bodies <- capture()
structured_body <- paste0('{"state":"gamma","model":"jev-1.13.0","questions":{',
  '"q1":{"type":"noul","instructions":{"ask":"Refund?",',
  '"hints":["one",{"a":null,"b":1}]}}}}')
check("production parser and wire retain full structured meaning",
      identical(one$value, TRUE) && length(bodies) == 2L && identical(bodies[[2L]], structured_body) &&
      identical(one$details[[1L]]$requests, digest(url, structured_body)))
labelled <- tt_question(choose = list(ask = "Which?", hints = I("x")),
                        options = list(blue = NULL, red = list(meaning = I("a"))))
check("named label meanings keep null and single-element arrays",
      identical(labelled$json,
        '{"choose":{"ask":"Which?","hints":["x"]},"options":{"blue":null,"red":{"meaning":["a"]}}}'))
check("printing structured text is safe and does not dump its nested values",
      identical(capture.output(print(structured)), "<thinkthen decide: <structured text>>"))

with_sides <- tt_decide("Q?", "semantic", true = "Yes means refund", false = "No means no refund")
side_bodies <- capture()
check("true and false keywords reach the one sent question",
      identical(with_sides$value, TRUE) && identical(with_sides$probability, 0.9) &&
      grepl("Yes means refund", side_bodies[[3L]], fixed = TRUE) &&
      grepl("No means no refund", side_bodies[[3L]], fixed = TRUE))

plain <- tt_decide("Context?", c("delta", "epsilon"), batch = 1L)
shared <- tt_decide("Context?", c("delta", "epsilon"), batch = 1L, context = "Shared reference")
check("per-call batch one and literal context reach the same question through different requests",
      identical(plain$facts$requests_sent, 2) && identical(shared$facts$requests_sent, 2) &&
      identical(vapply(plain$details, `[[`, "", "question_sha256"),
                vapply(shared$details, `[[`, "", "question_sha256")) &&
      !identical(vapply(plain$details, function(one) one$requests[[1L]], ""),
                 vapply(shared$details, function(one) one$requests[[1L]], "")))

none <- tt_decide("Q?", c(NA_character_, NA_character_))
check("all missing input has a measured host no-work account",
      identical(none$value, c(NA, NA)) && identical(none$probability, c(NA_real_, NA_real_)) &&
      identical(none$facts$records, 0) &&
      identical(none$facts$requests_sent, 0) && is.null(none$facts$model) &&
      is.null(none$facts$input_tokens) && length(none$details) == 0L)

early <- tt_completion()
claimed <- NULL
bad <- tryCatch(tt_decide({ claimed <<- tt_completion_read(early)$state; "" }, "x",
                          completion = early), thinkthen_error = function(e) e)
settled <- tt_completion_read(early)
check("pre-accounting usage preserves absent facts",
      identical(claimed, "claimed") && inherits(bad, "thinkthen_usage") &&
      identical(settled$state, "terminal") &&
      identical(settled$kind, "usage") && is.null(settled$facts) && length(settled$details) == 0L)
check("claimed handle refuses reuse before sending", identical(kind_of(
  tt_decide("Q?", "x", completion = early)), "usage"))
late <- tryCatch(tt_decide("Q?", "x", deadline_ms = 0), thinkthen_error = function(e) e)
check("accounted zero-send deadline keeps final facts on its condition",
      inherits(late, "thinkthen_deadline") && identical(late$facts$records, 0) &&
      identical(late$facts$requests_sent, 0) && is.null(late$facts$model) &&
      length(late$details) == 0L)
bad_bytes <- "caf\xe9"
Encoding(bad_bytes) <- "bytes"
check("host operations and invalid controls fail as named usage before a send",
      identical(kind_of(tt_rank(character(), c("a", "b"))), "usage") &&
      identical(kind_of(tt_annotate("missing.json", data.frame(a = "x"), on = "missing")), "usage") &&
      identical(kind_of(tt_question(file = bad_bytes)), "usage") &&
      identical(kind_of(tt_decide("Q?", "x", context = bad_bytes)), "usage") &&
      identical(kind_of(tt_completion_read("not a handle")), "usage"))
bad_keywords <- sent_by({
  old <- tryCatch(tt_decide("Q?", "x", deadline = 1), thinkthen_error = function(e) e)
  check("old deadline spelling has the exact migration sentence",
        inherits(old, "thinkthen_usage") &&
        identical(conditionMessage(old), "deadline was renamed deadline_ms, in milliseconds"))
  check("unknown and engine-only keywords are usage",
        identical(kind_of(tt_decide("Q?", "x", surprise = 1)), "usage") &&
        identical(kind_of(tt_decide("Q?", evidence = "x")), "usage") &&
        identical(kind_of(tt_decide("Q?", "x", none = TRUE)), "usage") &&
        identical(kind_of(tt_engine(usd_per_million_input = "1", usd_per_million_output = "2")), "usage") &&
        identical(kind_of(tt_choose("Q?", "x", options = c("a", "b"), true = NULL)), "usage") &&
        identical(kind_of(tt_score("Q?", "x", levels = c("low", "high"), threshold = 0.5)), "usage") &&
        identical(kind_of(tt_decide("Q?", "x", deadline_ms = 4294967295001)), "usage"))
})
check("invalid keywords send no request", identical(bad_keywords, 0L))
repeated_file <- tempfile(fileext = ".json")
writeLines('{"decide":"Q?","decide":"Other?"}', repeated_file)
repeated_sends <- sent_by(repeated <- tryCatch(tt_question(file = repeated_file),
                                               thinkthen_error = function(e) e))
check("raw question file duplicate fields reach the core before R map conversion",
      inherits(repeated, "thinkthen_local") && repeated_sends == 0L)

limited_sends <- sent_by(limited <- child(c(
  sprintf('tt_engine(base_url = "%s", cache = FALSE, max_requests = 1L, batch = 1L)', arm("arm/full/v1")),
  'receipt <- tt_completion()',
  'error <- tryCatch(tt_decide("Q?", c(NA_character_, "first", "second"), completion = receipt), thinkthen_error = function(e) e)',
  'final <- tt_completion_read(receipt)',
  'cat(inherits(error, "thinkthen_usage"), identical(error$facts$requests_sent, 1),',
  '    identical(vapply(error$details, `[[`, 0, "index"), 1), identical(final$kind, "usage"),',
  '    identical(vapply(final$details, `[[`, 0, "index"), 1), "\\n")'
)))
check("accounted batch refusal keeps original indexes in error and receipt",
      limited$status == 0L && identical(trimws(limited$text), "TRUE TRUE TRUE TRUE TRUE") && limited_sends == 1L)

choice_sends <- sent_by(choice <- child(c(
  sprintf('tt_engine(base_url = "%s", cache = FALSE)', arm("generic/v1")),
  'receipt <- tt_completion()',
  'answer <- tt_choose("Which?", c("a", NA_character_, "b"), c("billing", "shipping"), completion = receipt)',
  'cat(identical(vapply(answer$details, `[[`, 0, "index"), c(0, 2)),',
  '    identical(vapply(tt_completion_read(receipt)$details, `[[`, 0, "index"), c(0, 2)),',
  '    identical(answer$value, c("billing", NA_character_, "billing")),',
  '    identical(answer$probability, c(0.9, NA_real_, 0.9)), "\\n")'
)))
check("dynamic-label columns share the original-position receipt boundary",
      choice$status == 0L && identical(trimws(choice$text), "TRUE TRUE TRUE TRUE") && choice_sends == 1L)

unsure_sends <- sent_by(unsure <- tt_choose("Which?", "c", c("billing", "shipping"), threshold = 0.95))
check("an unsure choice keeps its selected-label probability missing",
      identical(unsure$value, NA_character_) && identical(unsure$probability, NA_real_) && unsure_sends == 1L)
check("score and tag withhold probability", is.null(tt_score("How much?", character(), c("low", "high"))$probability) &&
      is.null(tt_tag("Which?", character(), c("a", "b"))$probability))

total_sends <- sent_by(total <- child(c(
  sprintf('tt_engine(base_url = "%s", cache = FALSE, max_requests_total = 1L, batch = 1L)', arm("arm/full/v1")),
  'first <- tt_decide("Q?", "first")',
  'receipt <- tt_completion()',
  'blocked <- tryCatch(tt_decide("Q?", "second", completion = receipt), thinkthen_error = function(e) e)',
  'cat(identical(first$value, TRUE), inherits(blocked, "thinkthen_usage"),',
  '    identical(blocked$facts$requests_sent, 0),',
  '    identical(tt_completion_read(receipt)$kind, "usage"), "\\n")'
)))
check("process cap counts a prior call and refuses the next before a send",
      total$status == 0L && identical(trimws(total$text), "TRUE TRUE TRUE TRUE") && total_sends == 1L)

# The token cap reaches R through from_env; a regression if from_env stops reading the variable.
capped_sends <- sent_by(capped <- child(c(
  'tt_engine(cache = FALSE)',
  'e <- tryCatch(tt_decide("Q?", "text"), thinkthen_error = function(e) e)',
  'cat(e$kind, conditionMessage(e), sep = "\\n")'
), env = "THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL=10"))
check("the token cap variable refuses a call before any send",
      capped$status == 0L && capped_sends == 0L && identical(capped$text, paste0("usage\n",
        "max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request")))

finish("facts", 11L)
