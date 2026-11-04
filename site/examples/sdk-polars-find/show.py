import polars as pl
import thinkthen.polars  # adds the .thinkthen namespace

messages = pl.DataFrame(
    {
        "message": [
            "Morgan works for Example Group.\nPlease refund my order."
        ]
    }
)
actions = messages["message"].thinkthen.find(
    "Which line states the requested action?"
)
print(actions.struct.unnest())
