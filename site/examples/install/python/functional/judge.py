import json

import thinkthen as tt

question = "Is this a complaint?"
reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
]
is_complaint = tt.decide(question, batch=4)

whole_list = tt.plan(is_complaint, reviews)
one_by_one = [tt.plan(is_complaint, [r]) for r in reviews]
assert whole_list["requests"] == 4
assert whole_list["upper_bound"]
body = whole_list["first_body"]
assert len(json.loads(body)["questions"]) == 4
assert whole_list["estimated_bytes"] == len(body)
bodies = [json.loads(p["first_body"]) for p in one_by_one]
assert all(len(b["questions"]) == 1 for b in bodies)
assert sum(p["requests"] for p in one_by_one) == 4

complaints = is_complaint(reviews).value
assert complaints == [False, True, False, True]
