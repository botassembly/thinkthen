require "thinkthen"

question = "Which line gives the refund deadline?"
policy = [
  "Returns need the original receipt.",
  "Refunds are issued within 30 days of purchase.",
  "Shipping is free on orders over $50."
]
refund_deadline = ThinkThen.find(question, policy)
raise unless refund_deadline.unit == policy[1]

question = "Which labels fit this message?"
labels = ["praise", "bug", "billing"]
message =
  "Love it, but export crashes and I was billed twice."
fitting_labels = ThinkThen.tag(question, message, labels:)
raise unless fitting_labels == ["praise", "bug", "billing"]
