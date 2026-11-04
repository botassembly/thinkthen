import polars as pl
import thinkthen.polars  # registers the .thinkthen namespace

messages = pl.read_csv("messages.csv")
is_refund = messages["text"].thinkthen.decide(
    "Does the customer request a refund?"
)
messages = messages.with_columns(is_refund=is_refund)
print(messages.select("id", "is_refund"))
not_sure = messages.filter(pl.col("is_refund").is_null())
print("for a person:", not_sure["id"].to_list())
