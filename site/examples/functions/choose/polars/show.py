import polars as pl
import thinkthen.polars  # adds the .thinkthen namespace

messages = pl.DataFrame({"message": ["Please refund my order."]})
topic = messages["message"].thinkthen.choose(
    "What is the message about?",
    choices=["billing", "shipping", "other"],
)
print(messages.with_columns(topic=topic))
