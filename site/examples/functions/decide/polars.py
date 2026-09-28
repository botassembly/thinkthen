import polars as pl
import thinkthen as tt

question = "Does the customer ask for a refund?"
tickets = pl.DataFrame({
    "body": [
        "Please refund my order. It arrived broken.",
        "Thanks for the quick help yesterday!",
    ],
})
is_refund = tt.decide(question, tickets["body"]).value
tickets = tickets.with_columns(is_refund=is_refund)
assert tickets["is_refund"].to_list() == [True, False]
