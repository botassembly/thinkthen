import polars as pl
import thinkthen as tt

question = "Which line gives the refund deadline?"
policy = pl.DataFrame({
    "line": [
        "Returns need the original receipt.",
        "Refunds are issued within 30 days of purchase.",
        "Shipping is free on orders over $50.",
        "Gift cards cannot be exchanged for cash.",
    ],
})
lines = policy["line"].to_list()
refund_deadline = tt.find(question, lines).value
assert refund_deadline["unit"] == lines[1]
