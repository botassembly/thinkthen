import polars as pl
import thinkthen as tt

tickets = pl.DataFrame({
    "body": [
        "Maria Chen joined Northwind Freight, "
        "a company in Chicago.",
    ],
})
kinds = ["person", "organization", "place"]
names = tt.recognize(tickets, kinds=kinds, on="body").value
print(names.select("row", "text", "kind"))
