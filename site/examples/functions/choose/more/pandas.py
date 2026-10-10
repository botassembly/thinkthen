import pandas as pd
import thinkthen as tt
import thinkthen.pandas

question = "Which team owns this?"
teams = {
    "billing": "Invoices, fees, and refunds.",
    "shipping": "Parcels and delivery.",
    "account": "Logins and passwords.",
}
team_question = {
    "choose": question, "options": teams, "threshold": 0.9,
}
tickets = pd.DataFrame({
    "body": [
        "Please refund the extra fee on my invoice.",
        "My parcel went to the wrong address.",
        "I cannot reset my password.",
        (
            "My parcel never came, and now "
            "I cannot log in to track it."
        ),
    ],
})
owners = tickets["body"].tt.choose(team_question).value
tickets["owner"] = owners
print(tickets["owner"])
