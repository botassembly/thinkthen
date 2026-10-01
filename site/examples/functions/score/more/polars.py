import polars as pl
import thinkthen as tt

question = "How urgent is this?"
levels = ["Routine.", "Soon.", "Immediate."]
tickets = pl.DataFrame({
    "body": [
        "Please update my mailing address when you can.",
        "Can you send the signed contract by Friday?",
        "Nobody can log in to the site right now.",
    ],
})
urgency = tt.score(
    question,
    tickets["body"],
    levels=levels,
).value
tickets = tickets.with_columns(urgency=urgency)
print(tickets)
