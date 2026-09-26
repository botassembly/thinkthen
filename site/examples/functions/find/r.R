library(thinkthen)

question <- "Which line gives the refund deadline?"
policy <- c(
  "Returns need the original receipt.",
  "Refunds are issued within 30 days of purchase.",
  "Shipping is free on orders over $50.",
  "Gift cards cannot be exchanged for cash."
)
found <- tt_find(question, policy)
stopifnot(identical(found$unit, policy[2]))
