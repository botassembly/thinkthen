import thinkthen as tt

question = "How urgent is this?"
levels = ["Routine.", "Soon.", "Immediate."]
outage = (
    "Our checkout page is down and customers cannot pay.\n"
)
urgency = tt.score(question, outage, levels=levels).value
assert urgency == 2.0
