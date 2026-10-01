import polars as pl
import thinkthen as tt

question = "How urgent is this?"
levels = ["Routine.", "Soon.", "Immediate."]
messages = pl.Series([
    "Our checkout page is down and customers cannot pay.\n",
])
urgency = tt.score(question, messages, levels=levels).value
assert urgency.to_list() == [2.0]
