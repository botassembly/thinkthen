import pandas as pd
import thinkthen as tt

question = "Which labels fit this message?"
labels = ["praise", "bug", "billing"]
messages = pd.DataFrame({
    "body": [
        (
            "Love the new dashboard, but export "
            "crashes the app,\n"
            "and I was charged twice.\n"
        ),
    ],
})
fitting_labels = tt.tag(
    question, messages["body"], labels=labels
).value
messages["labels"] = fitting_labels
print(messages["labels"])
