require "thinkthen"

names = [
  ["Paul McCartney", "singer"],
  ["Ringo Starr", "singer"],
  ["Yesterday", "song"],
  ["Octopus's Garden", "song"]
]
who_sings = ThinkThen.relate(
  names,
  relations: { sings: ["singer", "song"] }
).value
sings = who_sings.map do |edge|
  [edge.source.name, edge.target.name]
end
raise unless sings == [
  ["Paul McCartney", "Yesterday"],
  ["Ringo Starr", "Octopus's Garden"]
]
