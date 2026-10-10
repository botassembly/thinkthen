import pandas as pd
import thinkthen as tt

refund = tt.QuestionSource(path="refund.json")
messages = pd.Series([
    (
        "I would like to return this and get my money back."
        "\n"
    ),
])
is_refund = tt.decide(refund, messages).value
assert is_refund.tolist() == [
    "The customer asks for money back.",
]
