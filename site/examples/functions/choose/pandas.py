import pandas as pd
import thinkthen as tt

question = "Which team owns this?"
teams = {
    "billing": "Invoices, fees, and refunds.",
    "shipping": "Parcels and delivery.",
    "account": "Logins and passwords.",
}
messages = pd.Series([
    "My parcel went to the wrong address.",
])
team = tt.choose(question, messages, options=teams).value
assert team.tolist() == ["shipping"]
