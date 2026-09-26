library(thinkthen)

question <- "Which line gives the refund deadline?"
policy <- c(
  "Returns need the original receipt.",
  "Refunds are issued within 30 days of purchase.",
  "Shipping is free on orders over $50.",
  "Gift cards cannot be exchanged for cash."
)
refund_deadline <- tt_find(question, policy)
stopifnot(identical(refund_deadline$unit, policy[2]))
