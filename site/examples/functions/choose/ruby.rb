require "thinkthen"

question = "Which team owns this?"
teams = {
  "billing" => "Invoices, fees, and refunds.",
  "shipping" => "Parcels and delivery.",
  "account" => "Logins and passwords."
}
team_question = ThinkThen.question(
  choose: question,
  options: teams,
  threshold: 0.9
)

texts = [
  "Please refund the extra fee on my invoice.",
  "My parcel went to the wrong address.",
  "I cannot reset my password.",
  "My parcel never came, and now " \
    "I cannot log in to track it."
]
owners = texts.map do |text|
  ThinkThen.choose(team_question, text)
end
raise unless owners == [
  "billing",
  "shipping",
  "account",
  nil
]
