library(thinkthen)

question <- "Which labels fit this message?"
labels <- c("praise", "bug", "billing")
message <- paste0(
  "Love the new dashboard, ",
  "but export crashes the app, ",
  "and I was charged twice."
)
tags <- tt_tag(question, message, labels)[[1]]
stopifnot(identical(tags, c("praise", "bug", "billing")))
