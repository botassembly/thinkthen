require "thinkthen"

ThinkThen::Client.open do |client|
  question = "Which line gives the refund deadline?"
  policy = [
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash."
  ]
  refund_deadline = client.find(question, policy).value
  raise unless refund_deadline == policy[1]

  question = "Which labels fit this message?"
  labels = ["praise", "bug", "billing"]
  message =
    "Love the new dashboard, " \
    "but export crashes the app,\n" \
    "and I was charged twice.\n"
  fitting_labels = client.tag(
    {tag: question, labels: labels}, message
  ).value
  raise unless fitting_labels == [
    "praise", "bug", "billing"
  ]
end
