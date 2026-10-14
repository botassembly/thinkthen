import thinkthen as tt

kinds = ["person", "product", "place"]
text = (
    "Sam Patel says his Kindle arrived broken "
    "in Denver."
)
facts = tt.recognize(text, kinds=kinds).value
names = [(one.text, one.kind) for one in facts.entities]
assert names == [
    ("Sam Patel", "person"),
    ("Kindle", "product"),
    ("Denver", "place"),
]
