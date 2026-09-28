require "thinkthen"

text = "Maria Chen joined Northwind Freight, " \
  "a company in Chicago."
kinds = ["person", "organization", "place"]
relations = {
  works_for: ["person", "organization"],
  based_in: ["organization", "place"]
}
facts = ThinkThen.recognize(text, kinds:, relations:).value
names = facts.entities.map { |one| [one.text, one.kind] }
raise unless names == [
  ["Maria Chen", "person"],
  ["Northwind Freight", "organization"],
  ["Chicago", "place"]
]
links = facts.relations.map do |one|
  [one.relation, one.source.name, one.target.name]
end
raise unless links == [
  ["works_for", "Maria Chen", "Northwind Freight"],
  ["based_in", "Northwind Freight", "Chicago"]
]
