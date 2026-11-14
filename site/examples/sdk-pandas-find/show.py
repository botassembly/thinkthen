import pandas as pd
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame(
    {
        "message": [
            "Morgan works for Example Group.\nPlease refund my order."
        ]
    }
)
actions = messages["message"].thinkthen.find(
    "Which line states the requested action?"
)
# Use details=True to distinguish no span from a missing answer.
for action in actions:
    if action is None or action is pd.NA:
        print("no answer")
    else:
        print(action.text)
