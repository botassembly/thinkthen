import pandas as pd
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame(
    {
        "message": [
            "Please refund my order.",
            "Where is my parcel?",
            "I was charged twice for one invoice.",
        ]
    }
)
messages["urgency_rank"] = messages["message"].thinkthen.rank(
    "How urgent is the request?",
    levels=["routine", "urgent"],
)
print(messages.sort_values("urgency_rank"))
