require "thinkthen"

ThinkThen::Client.open do |client|
  names = [
    ["Paul McCartney", "singer"],
    ["Ringo Starr", "singer"],
    ["Yesterday", "song"],
    ["Octopus's Garden", "song"]
  ]
  who_sings = client.relate(
    {version: 1, relate: {relations: [
      {name: "sings", source: "singer", target: "song",
       either: false}
    ]}},
    names.map { |name, kind| {name: name, kind: kind} }
  ).value
  sings = who_sings.map do |edge|
    [edge.source.name, edge.target.name]
  end
  raise unless sings == [
    ["Paul McCartney", "Yesterday"],
    ["Ringo Starr", "Octopus's Garden"]
  ]
end
