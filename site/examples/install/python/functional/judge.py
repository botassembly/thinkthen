import thinkthen as tt

question = "Is this a complaint?"
reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
]
is_complaint = tt.decide(question)

whole_list = tt.plan(is_complaint, reviews)
one_by_one = [tt.plan(is_complaint, [r]) for r in reviews]
assert whole_list["requests"] == 1
assert sum(p["requests"] for p in one_by_one) == 4

complaints = is_complaint(reviews).value
assert complaints == [False, True, False, True]
