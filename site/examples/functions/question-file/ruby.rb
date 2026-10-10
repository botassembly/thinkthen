require "thinkthen"

ThinkThen::Client.open do |client|
  refund = ThinkThen::Client.question_file("refund.json")
  money_back =
    "I would like to return this and get my money back." \
    "\n"
  is_refund = client.decide(refund, money_back).value
  expected = "The customer asks for money back."
  raise unless is_refund == expected
end
