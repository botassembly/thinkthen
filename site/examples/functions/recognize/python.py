import thinkthen as tt

kinds = ["person", "organization", "place"]
text = (
    "Maria Chen joined Northwind Freight, "
    "a company in Chicago."
)
facts = tt.recognize(text, kinds=kinds).value
names = [(one.text, one.kind) for one in facts.entities]
assert names == [
    ("Maria Chen", "person"),
    ("Northwind Freight", "organization"),
    ("Chicago", "place"),
]
