require "thinkthen"

messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]
ThinkThen::Client.open do |client|
  # Your own messages, most urgent first.
  by_urgency = client.rank("How urgent is the request?", messages,
                           levels: ["routine", "urgent"])
  by_urgency.each { |ranked| puts "#{ranked.answer} #{ranked.item}" }
end
