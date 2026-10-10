import thinkthen as tt

question = "Is this a complaint?"
reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
]
with tt.Engine(batch=4) as engine:
    whole_list = engine.plan("decide", question, reviews)
    one_by_one = [
        engine.plan("decide", question, [r])
        for r in reviews
    ]
    assert whole_list["requests"] == 4
    assert whole_list["upper_bound"]
    assert sum(p["requests"] for p in one_by_one) == 4

    complaints = engine.decide(question, reviews).value
assert complaints == [False, True, False, True]
