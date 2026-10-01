import pandas as pd
import thinkthen as tt

question = "Does the customer ask for a refund?"
refund = tt.question(decide=question, threshold=(0.2, 0.8))

messages = pd.Series(
    [
        "Please refund my order. It arrived broken.",
        "I want to send this back.",
    ],
    name="message",
)
is_refund = tt.decide(refund, messages).value
assert is_refund.dtype == "boolean"
assert is_refund.tolist() == [True, pd.NA]

for_a_person = messages[is_refund.isna()]
assert for_a_person.tolist() == [
    "I want to send this back.",
]
