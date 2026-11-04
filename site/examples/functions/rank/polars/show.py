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
urgency_rank = messages["message"].thinkthen.rank(
    "How urgent is the request?",
    levels=["routine", "urgent"],
)
print(messages.with_columns(urgency_rank=urgency_rank).sort("urgency_rank"))
