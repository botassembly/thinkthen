require "thinkthen"

question = "Is this a complaint?"
reviews = [
  "Arrived a day early. Thank you!",
  "The zipper broke the first time I used it.",
  "Does this come in blue?",
  "The strap snapped on day two."
]
kept = ThinkThen.filter(question, reviews)
raise unless kept == [reviews[1], reviews[3]]
