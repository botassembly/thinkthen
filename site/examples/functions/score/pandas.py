import pandas as pd
import thinkthen as tt

question = "How urgent is this?"
levels = ["Routine.", "Soon.", "Immediate."]
messages = pd.Series([
    "Our checkout page is down and customers cannot pay.\n",
])
urgency = tt.score(question, messages, levels=levels).value
assert urgency.tolist() == [2.0]
