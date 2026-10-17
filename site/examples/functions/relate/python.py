import thinkthen as tt

names = [
    ("Apple", "company"),
    ("Amazon", "company"),
    ("iPhone", "product"),
    ("Kindle", "product"),
]
who_makes = tt.relate(
    names,
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
