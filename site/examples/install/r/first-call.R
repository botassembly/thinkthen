library(thinkthen)

question <- "Which team owns this?"
teams <- c("billing", "shipping", "account")
body <- c(
  "I was billed twice for one order.",
  "My parcel is a week late.",
  "I am locked out of my account."
)
team <- tt_choose(question, body, teams)
stopifnot(identical(
  team,
  c("billing", "shipping", "account")
))

money <- tt_rank("Is this about money?", body, top = 1)
stopifnot(identical(money$record, body[1]))
stopifnot(identical(money$probability, 0.98))
