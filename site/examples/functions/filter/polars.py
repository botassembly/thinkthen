import polars as pl
import thinkthen as tt

question = "Is this a complaint?"
reviews = pl.Series([
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
])
complaints = tt.filter(question, reviews.to_list()).value
assert complaints == [reviews[1], reviews[3]]
