require "thinkthen"

messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]
ThinkThen::Client.open do |client|
  # Every message, most urgent first. rank starts at 1.
  by_urgency = client.rank("How urgent is the request?", messages,
                           levels: ["routine", "urgent"])
  by_urgency.each { |ranked| puts "#{ranked.rank} #{ranked.item}" }
end
