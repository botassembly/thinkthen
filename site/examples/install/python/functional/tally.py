import thinkthen as tt

question = "Is this a complaint?"
reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
]
with tt.Engine() as engine:
    complaints = engine.decide(question, reviews)
    first_two = engine.decide(question, reviews[:2])
assert complaints.value == [False, True, False, True]
assert first_two.value == [False, True]
review_count = (
    complaints.facts.records + first_two.facts.records
)
assert review_count == 6
