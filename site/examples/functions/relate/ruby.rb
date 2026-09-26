require "thinkthen"

rules = [
  "Book economy class for every flight under six hours.",
  "Submit receipts within 30 days of the trip.",
  "Hotel stays are capped at 200 dollars a night.",
  "Employees may book business class on any flight.",
  "Rental cars need a manager's approval.",
  "Receipts may be submitted at any time, " \
    "with no deadline.",
  "Meals are reimbursed up to 60 dollars a day.",
  "Use the company travel portal for all bookings."
]
entities = rules.map { |rule| [rule, "rule"] }
contradictions = ThinkThen.relate(
  entities,
  relations: ["contradicts"],
  either: ["contradicts"],
  threshold: 0.5
)
pairs = contradictions.map do |edge|
  [edge.source.name, edge.target.name, edge.probability]
end
raise unless pairs == [
  [rules[0], rules[3], 0.84],
  [rules[1], rules[5], 0.99]
]
