import polars as pl
import thinkthen as tt

question = "Does the customer ask for a refund?"
tickets = pl.DataFrame({
    "body": [
        "Please refund my order. It arrived broken.",
        "Thanks for the quick help yesterday!",
    ],
})
tickets = tickets.with_columns(
    refund=tt.decide(question, tickets["body"]),
)
assert tickets["refund"].to_list() == [True, False]
