import thinkthen as tt

question = "Which line gives the refund deadline?"
policy = [
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
]
found = tt.find(question, policy)
assert found["unit"] == policy[1]
