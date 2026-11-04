import polars as pl
import thinkthen.polars  # adds the .thinkthen namespace

messages = pl.DataFrame(
    {"message": ["I was charged twice and my parcel is late again."]}
)
topics = messages["message"].thinkthen.tag(
    "Which topics appear?",
    labels=["billing", "shipping"],
)
print(messages.with_columns(topics=topics).select("topics"))
