require "thinkthen"

ThinkThen::Client.open do |client|
  question = "Does the customer ask for a refund?"
  broken = "Please refund my order. It arrived broken."
  is_refund = client.decide(question, broken).value
  raise unless is_refund == true
end
