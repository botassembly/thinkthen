import thinkthen as tt

text = (
    "Maria Chen joined Northwind Freight "
    "in Chicago last spring."
)
kinds = {
    "PER": "Part of a person's name.",
    "ORG": (
        "Part of the name of an organization: a company, "
        "band, team, agency, government body, "
        "or media outlet."
    ),
    "LOC": "Part of the name of a place: a country, "
           "region, city, or geographic feature.",
    "MISC": (
        "Part of another named entity: a nationality, "
        "an event, a product, or the name of a "
        "creative work."
    ),
}
facts = tt.recognize(
    text,
    kinds=kinds,
).value
names = [(one.text, one.kind) for one in facts.entities]
assert names == [
    ("Maria Chen", "PER"),
    ("Northwind Freight", "ORG"),
    ("Chicago", "LOC"),
]
