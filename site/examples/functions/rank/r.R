library(thinkthen)

question <- "Is this urgent?"
inbox <- c(
  paste0(
    "Newsletter: our autumn catalog is here. ",
    "No reply needed."
  ),
  "Our checkout page is down and customers cannot pay",
  "Reminder: your invoice is due in 30 days",
  "Please send the signed quote by 5 pm today"
)
by_urgency <- tt_rank(question, inbox)$value
stopifnot(identical(by_urgency$place, c(2L, 4L, 3L, 1L)))
