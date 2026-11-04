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
refund_requests = messages["message"].thinkthen.filter(
    "Does the customer request a refund?"
)
print(refund_requests)
