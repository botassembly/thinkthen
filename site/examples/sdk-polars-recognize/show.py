import polars as pl
import thinkthen.polars  # adds the .thinkthen namespace

messages = pl.DataFrame(
    {"message": ["Morgan works for Example Group."]}
)
names = messages["message"].thinkthen.recognize(
    kinds=["person", "organization"]
)
print(names.explode().struct.unnest())
