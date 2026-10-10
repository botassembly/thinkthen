import pandas as pd
import thinkthen as tt
import thinkthen.pandas

question = "Does the customer ask for a refund?"
refund = {"decide": question, "threshold": "0.2:0.8"}

messages = pd.Series(
    [
        "Please refund my order. It arrived broken.",
        "I want to send this back.",
    ],
    name="message",
)
is_refund = messages.tt.decide(refund).value
print(is_refund)

for_a_person = messages[is_refund.isna()]
print(for_a_person)
