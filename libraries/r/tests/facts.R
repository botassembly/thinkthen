# Native results and completion observations retain independent ownership.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(base_url = arm("arm/full/capture/v1"), cache = FALSE)
receipt <- tt_completion()
answer <- tt_decide("Q?", c("alpha", NA_character_, "beta"), completion = receipt)
check("native answers retain compact indexes and ordinary missing slots", inherits(answer, "thinkthen_Call") &&
  identical(answer$positions, c(0, 2)) && identical(answer$value, c(TRUE, NA, TRUE)) &&
  identical(vapply(answer$results, `[[`, 0, "index"), c(0, 1)))
settled <- tt_completion_read(receipt)
check("completion retains original observation positions and native final facts", settled$state == "terminal" &&
  settled$kind == "success" && identical(vapply(settled$details, `[[`, 0, "index"), c(0, 2)) &&
  settled$facts$requests_sent == answer$facts$requests_sent)
settled$details[[1]]$index <- 99
check("completion read returns independently owned ordinary values", tt_completion_read(receipt)$details[[1]]$index == 0)
early <- tt_completion()
claimed <- NULL
bad <- tryCatch(tt_decide({claimed <<- tt_completion_read(early)$state; ""}, "x", completion=early), thinkthen_error=identity)
check("admission settles an already claimed completion without invented facts", claimed == "claimed" &&
  inherits(bad,"thinkthen_usage") && tt_completion_read(early)$kind == "usage" &&
  is.null(tt_completion_read(early)$facts))
check("completion cannot be reused", kind_of(tt_decide("Q?", "x", completion=early)) == "usage")
late <- tryCatch(tt_decide("Q?", "x", deadline_ms=0), thinkthen_error=identity)
check("zero-send deadline retains its native condition and facts", inherits(late,"thinkthen_deadline") &&
  late$facts$requests_sent == 0 && inherits(late$complete,"thinkthen_CallError"))
structured <- list(decide=list(ask="Refund?",hints=list("one",list(a=NULL,b=1))),true=NULL)
one <- tt_decide(structured,"gamma")
check("structured wording retains null and nested values", is.null(one$results[[1]]$question$true) &&
  identical(one$results[[1]]$question$text, structured$decide) &&
  inherits(one$results[[1]]$question$false,"thinkthen_absent"))
with_sides <- tt_decide(list(decide="Q?",true="Yes means refund",false="No means no refund"), "semantic")
body <- tail(capture(),1)[[1]]
check("descriptions reach native authored wording", grepl("Yes means refund",body,fixed=TRUE) &&
  grepl("No means no refund",body,fixed=TRUE))
plain <- tt_decide("Context?", c("delta","epsilon"), options=list(batch=1L))
shared <- tt_decide("Context?", c("delta","epsilon"), options=list(batch=1L,context="Shared reference"))
check("context changes exchanges while retaining question identity", plain$facts$requests_sent == 2 && shared$facts$requests_sent == 2 &&
  identical(unclass(plain$results[[1]]$meta$question_sha256),unclass(shared$results[[1]]$meta$question_sha256)) &&
  !identical(plain$results[[1]]$meta$requests,shared$results[[1]]$meta$requests))
repeated <- tempfile(fileext=".json")
writeLines('{"decide":"Q?","decide":"Other?"}',repeated)
check("question files preserve duplicate-field refusal before sending", sent_by(check("native file error",
  kind_of(tt_decide(tt_question(file=repeated),"x")) == "local")) == 0L)
limited_sends <- sent_by(limited <- child(c(
  'tt_engine(cache=FALSE,max_requests=1L,batch=1L)',
  'receipt <- tt_completion()',
  'failure <- tryCatch(tt_decide("Q?",c("first","second"),completion=receipt),thinkthen_error=identity)',
  'stopifnot(inherits(failure,"thinkthen_usage"),tt_completion_read(receipt)$kind=="usage")',
  'cat("ADMISSION REFUSED")'
)))
check("whole collection admission refuses before sending", limited$status == 0L && limited_sends == 0L && limited$text == "ADMISSION REFUSED")
capped_sends <- sent_by(capped <- child(c(
  'tt_engine(cache=FALSE)',
  'failure <- tryCatch(tt_decide("Q?","text"),thinkthen_error=identity)',
  'cat(failure$kind)'
), env="THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL=10"))
check("token spending cap crosses from the environment without a send", capped$status==0L && capped_sends==0L && capped$text=="usage")
finish("facts and completion",7L)
