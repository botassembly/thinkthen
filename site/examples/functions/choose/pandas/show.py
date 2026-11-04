import pandas as pd
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame({"message": ["Please refund my order."]})
messages["topic"] = messages["message"].thinkthen.choose(
    "What is the message about?",
    choices=["billing", "shipping", "other"],
)
print(messages)
