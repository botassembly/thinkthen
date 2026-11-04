import pandas as pd
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame(
    {"message": ["Morgan works for Example Group.\nPlease refund my order."]}
)
actions = messages["message"].thinkthen.find(
    "Which line states the requested action?"
)
# NA when there is no answer: no line found, not sure, or failed.
for action in actions:
    if action is pd.NA:
        print("no answer")
    else:
        print(action.text)
