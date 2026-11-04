require "thinkthen"

ThinkThen::Client.open do |client|
  is_refund = client.decide?("Does the customer request a refund?",
                             "Please refund my order.")
  puts is_refund
end
