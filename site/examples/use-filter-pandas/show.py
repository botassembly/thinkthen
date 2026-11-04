import pandas as pd
import thinkthen.pandas  # registers the .thinkthen accessor

messages = pd.read_csv("messages.csv")
refund_requests = messages.thinkthen.filter(
    "Does the customer request a refund?", field="/text"
)
print(refund_requests)
