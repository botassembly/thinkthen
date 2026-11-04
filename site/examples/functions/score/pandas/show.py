import pandas as pd
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame({"message": ["Please refund my order."]})
messages["urgency"] = messages["message"].thinkthen.score(
    "How urgent is the request?",
    levels=["routine", "urgent"],
)
print(messages)
