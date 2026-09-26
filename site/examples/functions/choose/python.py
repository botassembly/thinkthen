import thinkthen as tt

question = "Which team owns this?"
teams = {
    "billing": "Invoices, fees, and refunds.",
    "shipping": "Parcels and delivery.",
    "account": "Logins and passwords.",
}
team_question = tt.question(
    choose=question,
    options=teams,
    threshold=0.9,
)

texts = [
    "Please refund the extra fee on my invoice.",
    "My parcel went to the wrong address.",
    "I cannot reset my password.",
    (
        "My parcel never came, and now "
        "I cannot log in to track it."
    ),
]
owners = [
    tt.choose(team_question, text)
    for text in texts
]
assert owners == ["billing", "shipping", "account", None]
