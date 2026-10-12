import ThinkThen

let client = try Client()
let names = [
    ("Paul McCartney", "singer"),
    ("Ringo Starr", "singer"),
    ("Yesterday", "song"),
    ("Octopus's Garden", "song"),
]
let entities = InputRequestInputEntities(items: names.map {
    let entity = JSONValue.orderedObject([
        JSONMember("name", .string($0.0)),
        JSONMember("kind", .string($0.1)),
    ])
    let original = InputRequestOriginalJson(value: entity)
    return InputRequestItem(
        original: .value(.json(original))
    )
})
let call = try await client.relate(
    .file("sings.json"),
    input: .entities(entities)
)
var whoSings: [String] = []
for case .relateAggregate(let found) in call.packets {
    for edge in found.value.value {
        if case .alternative0(let singer) = edge.source,
           case .alternative0(let song) = edge.target {
            whoSings.append(
                "\(singer.name) sings \(song.name)"
            )
        }
    }
}
precondition(whoSings == [
    "Paul McCartney sings Yesterday",
    "Ringo Starr sings Octopus's Garden",
])
client.close()
