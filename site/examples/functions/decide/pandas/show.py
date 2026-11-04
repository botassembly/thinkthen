import pandas as pd
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame({"message": ["Please refund my order."]})
messages["is_refund"] = messages["message"].thinkthen.decide(
    "Does the customer request a refund?"
)
print(messages)
