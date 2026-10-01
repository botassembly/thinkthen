library(thinkthen)

question <- "Does the customer ask for a refund?"

broken <- "Please refund my order. It arrived broken."
is_refund <- tt_decide(question, broken)$value
stopifnot(identical(is_refund, TRUE))

send_back <- "I want to send this back."
is_refund <- tt_decide(
  question,
  send_back,
  threshold = "0.2:0.8"
)$value
stopifnot(identical(is_refund, NA))
