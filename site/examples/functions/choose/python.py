import thinkthen as tt

question = "Which team owns this?"
teams = {
    "billing": "Invoices, fees, and refunds.",
    "shipping": "Parcels and delivery.",
    "account": "Logins and passwords.",
}
parcel = "My parcel went to the wrong address."
team = tt.choose(question, parcel, options=teams).value
assert team == "shipping"
