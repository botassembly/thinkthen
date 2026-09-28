library(thinkthen)

question <- "Which labels fit this message?"
labels <- c("praise", "bug", "billing")
message <- paste0(
  "Love the new dashboard, ",
  "but export crashes the app, ",
  "and I was charged twice."
)
fitting_labels <- tt_tag(
  question, message, labels
)$value[[1]]
stopifnot(identical(
  fitting_labels,
  c("praise", "bug", "billing")
))
