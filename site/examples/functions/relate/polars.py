import polars as pl
import thinkthen as tt

names = pl.DataFrame({
    "name": [
        "Apple",
        "Amazon",
        "iPhone",
        "Kindle",
    ],
    "kind": ["company", "company", "product", "product"],
})
who_makes = tt.relate(
    names.to_dicts(),
    relations={"makes": ("company", "product")},
).value
makes = [
    (edge.source.name, edge.target.name)
    for edge in who_makes
]
assert makes == [
    ("Apple", "iPhone"),
    ("Amazon", "Kindle"),
]
