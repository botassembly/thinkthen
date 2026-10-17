import polars as pl
import thinkthen as tt

kinds = ["person", "product", "place"]
tickets = pl.DataFrame({
    "body": [
        "Sam Patel says his Kindle arrived broken "
        "in Denver.",
    ],
})
names = tt.recognize(tickets, kinds=kinds, on="body").value
assert names["text"].to_list() == [
    "Sam Patel",
    "Kindle",
    "Denver",
]
assert names["kind"].to_list() == kinds
