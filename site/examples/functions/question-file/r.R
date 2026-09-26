library(thinkthen)

question <- "Does the customer ask for a refund?"
refund <- tt_question(
  decide = question,
  threshold = c(0.2, 0.8)
)
texts <- c(
  "Please refund my order. It arrived broken.",
  "Thanks for the quick help yesterday!",
  "I want to send this back."
)
answers <- tt_decide(refund, texts)
stopifnot(identical(answers, c(TRUE, FALSE, NA)))
