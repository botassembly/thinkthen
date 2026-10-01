import pandas as pd
import thinkthen as tt

names = pd.DataFrame({
    "name": [
        "Paul McCartney",
        "Ringo Starr",
        "Yesterday",
        "Octopus's Garden",
    ],
    "kind": ["singer", "singer", "song", "song"],
})
who_sings = tt.relate(
    list(zip(names["name"], names["kind"])),
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
