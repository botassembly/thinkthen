library(thinkthen)

refund <- tt_question(file = "refund.json")
money_back <- paste0(
  "I would like to return this and get my money back.",
  "\n"
)
is_refund <- tt_decide(refund, money_back)$value
stopifnot(identical(is_refund, TRUE))
