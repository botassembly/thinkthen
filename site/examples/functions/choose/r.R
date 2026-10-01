library(thinkthen)

question <- "Which team owns this?"
teams <- c(
  billing = "Invoices, fees, and refunds.",
  shipping = "Parcels and delivery.",
  account = "Logins and passwords."
)
team_question <- tt_question(
  choose = question,
  options = teams,
  threshold = 0.9
)
texts <- c(
  "Please refund the extra fee on my invoice.",
  "My parcel went to the wrong address.",
  "I cannot reset my password.",
  paste0(
    "My parcel never came, and now ",
    "I cannot log in to track it."
  )
)
owners <- tt_choose(team_question, texts)$value
stopifnot(identical(
  owners,
  c("billing", "shipping", "account", NA)
))
