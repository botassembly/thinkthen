import polars as pl
import thinkthen as tt
import thinkthen.polars  # adds the .thinkthen namespace

messages = pl.DataFrame(
    {"message": ["Morgan works for Example Group.\nPlease refund my order."]}
)
rules = [
    tt.RelationRule(
        source="person", target="organization", relation="works_for"
    )
]
relations = messages["message"].thinkthen.relate(rules=rules)
print(
    relations.explode().struct.unnest().select(
        pl.col("source").struct.field("text").alias("source"),
        "relation",
        pl.col("target").struct.field("text").alias("target"),
    )
)
