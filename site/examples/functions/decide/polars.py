import polars as pl
import thinkthen as tt

question = "Does the customer ask for a refund?"
messages = pl.Series([
    "Please refund my order. It arrived broken.",
])
is_refund = tt.decide(question, messages).value
assert is_refund.to_list() == [True]
