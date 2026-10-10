import thinkthen as tt

question = "Is this a complaint?"
reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
]
with tt.Engine() as engine:
    lines = (review for review in reviews)
    with engine.iterate(
        "filter", question, lines,
    ) as complaints:
        first_complaint = next(complaints)
assert first_complaint.input == reviews[1]
