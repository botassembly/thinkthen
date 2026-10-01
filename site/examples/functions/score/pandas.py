import pandas as pd
import thinkthen as tt

question = "How urgent is this?"
levels = ["Routine.", "Soon.", "Immediate."]
tickets = pd.DataFrame({
    "body": [
        "Please update my mailing address when you can.",
        "Can you send the signed contract by Friday?",
        "Nobody can log in to the site right now.",
    ],
})
tickets["urgency"] = tt.score(
    question,
    tickets["body"],
    levels=levels,
).value
print(tickets["urgency"])
