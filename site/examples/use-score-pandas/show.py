import pandas as pd
import thinkthen.pandas  # registers the .thinkthen accessor

messages = pd.read_csv("messages.csv")
messages["urgency"] = messages["text"].thinkthen.score(
    "How urgent is the request?",
    levels=["routine", "urgent"],
)
print(messages.sort_values("urgency", ascending=False)[["id", "urgency"]])
