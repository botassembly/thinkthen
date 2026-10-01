require "thinkthen"

kinds = ["person", "organization", "place"]
text = "Maria Chen joined Northwind Freight, " \
  "a company in Chicago."
facts = ThinkThen.recognize(text, kinds:).value
names = facts.entities.map { |one| [one.text, one.kind] }
raise unless names == [
  ["Maria Chen", "person"],
  ["Northwind Freight", "organization"],
  ["Chicago", "place"]
]
