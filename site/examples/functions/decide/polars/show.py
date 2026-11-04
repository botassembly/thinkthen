import polars as pl
import thinkthen.polars  # adds the .thinkthen namespace

messages = pl.DataFrame({"message": ["Please refund my order."]})
is_refund = messages["message"].thinkthen.decide(
    "Does the customer request a refund?"
)
print(messages.with_columns(is_refund=is_refund))
