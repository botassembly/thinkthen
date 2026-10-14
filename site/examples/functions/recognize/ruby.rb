require "thinkthen"

ThinkThen::Client.open do |client|
  kinds = {person: nil, product: nil, place: nil}
  text = "Sam Patel says his Kindle arrived broken " \
    "in Denver."
  facts = client.recognize(
    {version: 1, recognize: {kinds: kinds}}, text
  ).value
  names = facts.entities.map { |one| [one.text, one.kind] }
  raise unless names == [
    ["Sam Patel", "person"],
    ["Kindle", "product"],
    ["Denver", "place"]
  ]
end
