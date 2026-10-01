import polars as pl
import thinkthen as tt

kinds = ["person", "organization", "place"]
tickets = pl.DataFrame({
    "body": [
        "Maria Chen joined Northwind Freight, "
        "a company in Chicago.",
    ],
})
names = tt.recognize(tickets, kinds=kinds, on="body").value
assert names["text"].to_list() == [
    "Maria Chen",
    "Northwind Freight",
    "Chicago",
]
assert names["kind"].to_list() == kinds
