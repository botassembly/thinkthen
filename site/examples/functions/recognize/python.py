import thinkthen as tt

text = (
    "Maria Chen joined Northwind Freight, "
    "a company in Chicago."
)
kinds = ["person", "organization", "place"]
relations = {
    "works_for": ("person", "organization"),
    "based_in": ("organization", "place"),
}
facts = tt.recognize(
    text,
    kinds=kinds,
    relations=relations,
).value
names = [(one.name, one.kind) for one in facts.entities]
assert names == [
    ("Maria Chen", "person"),
    ("Northwind Freight", "organization"),
    ("Chicago", "place"),
]
links = [
    (one.relation, one.source.name, one.target.name)
    for one in facts.relations
]
assert links == [
    ("works_for", "Maria Chen", "Northwind Freight"),
    ("based_in", "Northwind Freight", "Chicago"),
]
