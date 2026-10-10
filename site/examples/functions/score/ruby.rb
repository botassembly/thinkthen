require "thinkthen"

ThinkThen::Client.open do |client|
  question = "How urgent is this?"
  levels = ["Routine.", "Soon.", "Immediate."]
  outage =
    "Our checkout page is down and customers cannot pay.\n"
  urgency = client.score(
    {score: question, levels: levels}, outage
  ).value
  raise unless urgency == 2.0
end
