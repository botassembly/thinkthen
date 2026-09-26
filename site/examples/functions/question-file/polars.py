import polars as pl
import thinkthen as tt

question = "Does the customer ask for a refund?"
refund = tt.question(decide=question, threshold=(0.2, 0.8))
tickets = pl.DataFrame({
    "body": [
        "Please refund my order. It arrived broken.",
        "Thanks for the quick help yesterday!",
        "I want to send this back.",
    ],
})
tickets = tickets.with_columns(
    refund=tt.decide(refund, tickets["body"]),
)
assert tickets["refund"].to_list() == [True, False, None]
