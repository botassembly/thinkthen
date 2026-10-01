import pandas as pd
import thinkthen as tt

tickets = pd.DataFrame({
    "body": [
        "Maria Chen joined Northwind Freight, "
        "a company in Chicago.",
    ],
})
kinds = ["person", "organization", "place"]
names = tt.recognize(tickets, kinds=kinds, on="body").value
found = [(n["text"], n["kind"]) for n in names["names"][0]]
assert found == [
    ("Maria Chen", "person"),
    ("Northwind Freight", "organization"),
    ("Chicago", "place"),
]
