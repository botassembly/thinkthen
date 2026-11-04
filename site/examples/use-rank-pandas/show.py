import pandas as pd
import thinkthen.pandas  # registers the .thinkthen accessor

messages = pd.read_csv("messages.csv")
messages["urgency_rank"] = messages["text"].thinkthen.rank(
    "How urgent is the request?",
    levels=["routine", "urgent"],
)
ranked = messages.sort_values("urgency_rank")
print(ranked[["urgency_rank", "id", "text"]].to_string(index=False))
