library(thinkthen)

question <- "Does the customer ask for a refund?"
texts <- c(
  "Please refund my order. It arrived broken.",
  "Thanks for the quick help yesterday!"
)
answers <- tt_decide(question, texts)
stopifnot(identical(answers, c(TRUE, FALSE)))
