library(thinkthen)

question <- "Does the customer ask for a refund?"
broken <- "Please refund my order. It arrived broken."
is_refund <- tt_decide(question, broken)$value
stopifnot(identical(is_refund, TRUE))
