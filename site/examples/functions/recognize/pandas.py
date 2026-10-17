import pandas as pd
import thinkthen as tt

kinds = ["person", "product", "place"]
tickets = pd.DataFrame({
    "body": [
        "Sam Patel says his Kindle arrived broken "
        "in Denver.",
    ],
})
names = tt.recognize(tickets, kinds=kinds, on="body").value
found = [(n["text"], n["kind"]) for n in names["names"][0]]
assert found == [
    ("Sam Patel", "person"),
    ("Kindle", "product"),
    ("Denver", "place"),
]
