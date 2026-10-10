require "thinkthen"

ThinkThen::Client.open do |client|
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
