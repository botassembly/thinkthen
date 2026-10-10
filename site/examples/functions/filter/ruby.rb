require "thinkthen"

ThinkThen::Client.open do |client|
  question = "Is this a complaint?"
  reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two."
  ]
  complaints = client.filter(question, reviews).value
  raise unless complaints == [reviews[1], reviews[3]]
end
