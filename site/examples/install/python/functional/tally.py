import thinkthen as tt

question = "Is this a complaint?"
reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
]
tally = tt.Tally()
is_complaint = tt.decide(question, tally=tally)

complaints = is_complaint(reviews).value
first_two = is_complaint(reviews[:2]).value
assert complaints == [False, True, False, True]
assert first_two == [False, True]
assert tally.facts["records"] == 6
