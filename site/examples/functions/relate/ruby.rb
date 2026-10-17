require "thinkthen"

ThinkThen::Client.open do |client|
  names = [
    ["Apple", "company"],
    ["Amazon", "company"],
    ["iPhone", "product"],
    ["Kindle", "product"]
  ]
  who_makes = client.relate(
    {version: 1, relate: {relations: [
      {name: "makes", source: "company", target: "product",
       either: false}
    ]}},
    names.map { |name, kind| {name: name, kind: kind} }
  ).value
  makes = who_makes.map do |edge|
    [edge.source.name, edge.target.name]
  end
  raise unless makes == [
    ["Apple", "iPhone"],
    ["Amazon", "Kindle"]
  ]
end
