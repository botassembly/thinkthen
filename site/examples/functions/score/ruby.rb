require "thinkthen"

question = "How urgent is this?"
levels = ["Routine.", "Soon.", "Immediate."]
texts = [
  "Please update my mailing address when you can.",
  "Can you send the signed contract by Friday?",
  "Nobody can log in to the site right now."
]
urgency = texts.map do |text|
  ThinkThen.score(question, text, levels:)
end
raise unless urgency == [0.06, 0.99, 2.0]
