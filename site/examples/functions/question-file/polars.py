import polars as pl
import thinkthen as tt

refund = tt.question(file="refund.json")
messages = pl.Series([
    (
        "I would like to return this and get my money back."
        "\n"
    ),
])
is_refund = tt.decide(refund, messages).value
assert is_refund.to_list() == [True]
