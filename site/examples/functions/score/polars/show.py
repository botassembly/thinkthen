import polars as pl
import thinkthen.polars  # adds the .thinkthen namespace

messages = pl.DataFrame({"message": ["Please refund my order."]})
urgency = messages["message"].thinkthen.score(
    "How urgent is the request?",
    levels=["routine", "urgent"],
)
print(messages.with_columns(urgency=urgency))
