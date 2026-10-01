require "thinkthen"

refund = ThinkThen.question(file: "refund.json")
money_back =
  "I would like to return this and get my money back." \
  "\n"
is_refund = ThinkThen.decide(refund, money_back).value
raise unless is_refund == true
