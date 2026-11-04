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
messages["is_refund"] = messages["message"].thinkthen.decide(
    "Does the customer request a refund?",
    threshold=(0.2, 0.8),
)
print(messages)
not_sure = messages["is_refund"].isna()
print("Ask a person:", messages.loc[not_sure, "message"].tolist())
