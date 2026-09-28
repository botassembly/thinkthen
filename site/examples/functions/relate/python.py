import thinkthen as tt

names = [
    ("Paul McCartney", "singer"),
    ("Ringo Starr", "singer"),
    ("Yesterday", "song"),
    ("Octopus's Garden", "song"),
]
who_sings = tt.relate(
    names,
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
