import polars as pl
import thinkthen as tt

question = "Which team owns this?"
teams = {
    "billing": "Invoices, fees, and refunds.",
    "shipping": "Parcels and delivery.",
    "account": "Logins and passwords.",
}
messages = pl.Series([
    "My parcel went to the wrong address.",
])
team = tt.choose(question, messages, options=teams).value
assert team.to_list() == ["shipping"]
