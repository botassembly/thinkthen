library(thinkthen)

question <- "Which team owns this?"
teams <- list(
  billing = "Invoices, fees, and refunds.",
  shipping = "Parcels and delivery.",
  account = "Logins and passwords."
)
parcel <- "My parcel went to the wrong address."
team <- tt_choose(
  list(choose = question, options = teams),
  parcel
)$answer
stopifnot(identical(team, "shipping"))
