import pandas as pd
import thinkthen as tt

refund = tt.question(file="refund.json")
tickets = pd.DataFrame({
    "body": [
        (
            "I would like to return this and get "
            "my money back."
        ),
        "I want to send this back.",
    ],
})
lines = tickets["body"] + "\n"
tickets["is_refund"] = tt.decide(refund, lines).value
print(tickets["is_refund"])
