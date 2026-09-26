import thinkthen as tt

question = "How urgent is this?"
levels = ["Routine.", "Soon.", "Immediate."]
texts = [
    "Please update my mailing address when you can.",
    "Can you send the signed contract by Friday?",
    "Nobody can log in to the site right now.",
]
scores = [
    tt.score(question, text, levels=levels)
    for text in texts
]
assert scores == [0.06, 0.99, 2.0]
