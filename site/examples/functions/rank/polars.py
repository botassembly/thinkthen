import polars as pl
import thinkthen as tt

question = "Is this urgent?"
inbox = pl.Series([
    (
        "Newsletter: our autumn catalog is here. "
        "No reply needed."
    ),
    "Our checkout page is down and customers cannot pay",
    "Reminder: your invoice is due in 30 days",
    "Please send the signed quote by 5 pm today",
])
by_urgency = tt.rank(question, inbox.to_list()).value
order = [one["index"] for one in by_urgency]
assert order == [1, 3, 2, 0]
