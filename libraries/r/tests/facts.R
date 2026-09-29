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
  '"hints":["one",{"a":null,"b":1}]},"criteria":{"true":null}}}}')
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
      identical(none$value, c(NA, NA)) && identical(none$facts$records, 0) &&
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
late <- tryCatch(tt_decide("Q?", "x", deadline = 0), thinkthen_error = function(e) e)
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
  '    identical(vapply(tt_completion_read(receipt)$details, `[[`, 0, "index"), c(0, 2)), "\\n")'
)))
check("dynamic-label columns share the original-position receipt boundary",
      choice$status == 0L && identical(trimws(choice$text), "TRUE TRUE") && choice_sends == 1L)

finish("facts", 8L)
