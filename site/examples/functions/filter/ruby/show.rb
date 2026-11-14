require "thinkthen"

messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]
ThinkThen::Client.open do |client|
  # Only the messages that answer yes.
  # A failed message raises ThinkThen::FailedError after the kept ones.
  refund_requests =
    client.filter("Does the customer request a refund?", messages)
  refund_requests.each { |refund_request| puts refund_request }
end
