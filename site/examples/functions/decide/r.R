library(thinkthen)

question <- "Does the customer ask for a refund?"
texts <- c(
  "Please refund my order. It arrived broken.",
  "Thanks for the quick help yesterday!"
)
is_refund <- tt_decide(question, texts)$value
stopifnot(identical(is_refund, c(TRUE, FALSE)))
