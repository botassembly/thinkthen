library(thinkthen)

question <- "Which team owns this?"
teams <- c(
  billing = "Invoices, fees, and refunds.",
  shipping = "Parcels and delivery.",
  account = "Logins and passwords."
)
parcel <- "My parcel went to the wrong address."
team <- tt_choose(question, parcel, teams)$value
stopifnot(identical(team, "shipping"))
