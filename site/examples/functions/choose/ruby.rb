require "thinkthen"

question = "Which team owns this?"
teams = {
  "billing" => "Invoices, fees, and refunds.",
  "shipping" => "Parcels and delivery.",
  "account" => "Logins and passwords."
}
parcel = "My parcel went to the wrong address."
team = ThinkThen.choose(
  question, parcel, options: teams
).value
raise unless team == "shipping"
