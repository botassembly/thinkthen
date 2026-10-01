import polars as pl
import thinkthen as tt

question = "Is this a complaint?"
reviews = pl.DataFrame({
    "body": [
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
        "Does this come in blue?",
        "The strap snapped on day two.",
    ],
})
complaints = tt.filter(
    question,
    reviews["body"].to_list(),
).value
assert complaints == [
    "The zipper broke the first time I used it.",
    "The strap snapped on day two.",
]
