import polars as pl
import thinkthen as tt

names = pl.DataFrame({
    "name": [
        "Paul McCartney",
        "Ringo Starr",
        "Yesterday",
        "Octopus's Garden",
    ],
    "kind": ["singer", "singer", "song", "song"],
})
who_sings = tt.relate(
    names.to_dicts(),
    relations={"sings": ("singer", "song")},
).value
sings = [
    (edge.source.name, edge.target.name)
    for edge in who_sings
]
assert sings == [
    ("Paul McCartney", "Yesterday"),
    ("Ringo Starr", "Octopus's Garden"),
]
