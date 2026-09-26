require "thinkthen"

question = "Does the customer ask for a refund?"
refund = ThinkThen.question(
  decide: question,
  threshold: 0.2..0.8
)
send_back = "I want to send this back."
raise unless ThinkThen.decide(refund, send_back) == nil
