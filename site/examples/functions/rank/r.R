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
order <- unlist(
  tt_rank(question, inbox)$value,
  use.names = FALSE
)
by_urgency <- inbox[order + 1]
stopifnot(identical(by_urgency, inbox[c(2, 4, 3, 1)]))
