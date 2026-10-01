import pandas as pd
import thinkthen as tt

question = "Which line gives the refund deadline?"
policy = pd.Series([
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
])
refund_deadline = tt.find(question, policy.tolist()).value
assert refund_deadline["unit"] == policy[1]
