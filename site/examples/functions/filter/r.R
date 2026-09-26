library(thinkthen)

question <- "Is this a complaint?"
reviews <- c(
  "Arrived a day early. Thank you!",
  "The zipper broke the first time I used it.",
  "Does this come in blue?",
  "The strap snapped on day two."
)
complaints <- tt_filter(question, reviews)
stopifnot(identical(complaints, reviews[c(2, 4)]))
