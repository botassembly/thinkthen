import polars as pl
import thinkthen as tt

question = "Which line gives the refund deadline?"
policy = pl.Series([
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
])
refund_deadline = tt.find(question, policy.to_list()).value
assert refund_deadline["unit"] == policy[1]
