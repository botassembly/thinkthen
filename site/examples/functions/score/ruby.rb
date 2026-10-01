require "thinkthen"

question = "How urgent is this?"
levels = ["Routine.", "Soon.", "Immediate."]
outage =
  "Our checkout page is down and customers cannot pay.\n"
urgency = ThinkThen.score(question, outage, levels:).value
raise unless urgency == 2.0
