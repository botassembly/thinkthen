require "thinkthen"

ThinkThen::Client.open do |client|
  question = "Which team owns this?"
  teams = {
    "billing" => "Invoices, fees, and refunds.",
    "shipping" => "Parcels and delivery.",
    "account" => "Logins and passwords."
  }
  parcel = "My parcel went to the wrong address."
  team = client.choose(
    {choose: question, options: teams}, parcel
  ).value
  raise unless team == "shipping"
end
