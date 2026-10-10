require "thinkthen"

ThinkThen::Client.open do |client|
  kinds = {person: nil, organization: nil, place: nil}
  text = "Maria Chen joined Northwind Freight, " \
    "a company in Chicago."
  facts = client.recognize(
    {version: 1, recognize: {kinds: kinds}}, text
  ).results[0].value
  names = facts.entities.map { |one| [one.text, one.kind] }
  raise unless names == [
    ["Maria Chen", "person"],
    ["Northwind Freight", "organization"],
    ["Chicago", "place"]
  ]
end
