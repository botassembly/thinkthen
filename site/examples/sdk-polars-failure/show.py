import polars as pl
import thinkthen as tt
import thinkthen.polars  # adds the .thinkthen namespace

messages = pl.DataFrame({"message": ["Please refund my order.", None]})
try:
    is_refund = messages["message"].thinkthen.decide(
        "Does the customer request a refund?"
    )
except tt.Failed as failure:
    outcomes = failure.outcomes
    print(
        outcomes.select(
            "status",
            "answer",
            "error_code",
        )
    )
