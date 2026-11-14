require "thinkthen"

ThinkThen::Client.open do |client|
  # Reading .answer raises ThinkThen::UnsureError when not sure,
  # ThinkThen::RecordError on failure.
  is_refund = client.decide("Does the customer request a refund?",
                             "Please refund my order.").answer
  puts is_refund
end
