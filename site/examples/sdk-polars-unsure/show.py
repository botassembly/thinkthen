import polars as pl
import thinkthen.polars  # adds the .thinkthen namespace

messages = pl.DataFrame(
    {
        "message": [
            "Please refund my order.",
            "Where is my parcel?",
            "I was charged twice for one invoice.",
        ]
    }
)
is_refund = messages["message"].thinkthen.decide(
    "Does the customer request a refund?",
    threshold=(0.2, 0.8),
)
messages = messages.with_columns(is_refund=is_refund)
print(messages)
not_sure = messages.filter(pl.col("is_refund").is_null())
print("Ask a person:", not_sure["message"].to_list())
