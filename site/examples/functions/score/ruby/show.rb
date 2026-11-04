require "thinkthen"

ThinkThen::Client.open do |client|
  urgency = client.score("How urgent is the request?",
                         "Please refund my order.",
                         levels: ["routine", "urgent"]).answer
  puts urgency
end
