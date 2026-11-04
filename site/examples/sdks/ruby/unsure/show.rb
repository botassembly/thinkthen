require "thinkthen"

message = "I was charged twice for one invoice."
ThinkThen::Client.open do |client|
  is_refund = client.decide?("Does the customer request a refund?",
                             message, threshold: 0.2..0.8)
  puts is_refund
rescue ThinkThen::UnsureError => unsure
  puts "Not sure (#{unsure.probability}): ask a person."
rescue ThinkThen::Error => failure
  puts "#{failure.code}: #{failure.message}"
end
