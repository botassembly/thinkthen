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
refund_requests = messages["message"].thinkthen.filter(
    "Does the customer request a refund?"
)
print(refund_requests)
