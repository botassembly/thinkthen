require "thinkthen"

question = "Which line gives the refund deadline?"
policy = [
  "Returns need the original receipt.",
  "Refunds are issued within 30 days of purchase.",
  "Shipping is free on orders over $50.",
  "Gift cards cannot be exchanged for cash."
]
refund_deadline = ThinkThen.find(question, policy).value
raise unless refund_deadline.unit == policy[1]
