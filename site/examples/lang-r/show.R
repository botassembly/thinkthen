library(thinkthen)

messages <- data.frame(
  id = c("a1", "a2"),
  body = c("Please refund my order.", "Where is my parcel?")
)

refunds <- tt_decide(
  messages,
  "Does the customer request a refund?",
  field = "/body",
  threshold = c(0.2, 0.8),
  details = TRUE
)
refunds[c("status", "answer", "probability")]
