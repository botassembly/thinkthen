import ThinkThen

let client = try Client()
let names = [
    ("Apple", "company"),
    ("Amazon", "company"),
    ("iPhone", "product"),
    ("Kindle", "product"),
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
let judgment = try await client.relateAnswer(
    .file("makes.json"),
    input: .entities(entities)
)
var whoMakes: [String] = []
for edge in judgment.value {
    if case .alternative0(let company) = edge.source,
       case .alternative0(let product) = edge.target {
        whoMakes.append(
            "\(company.name) makes \(product.name)"
        )
    }
}
precondition(whoMakes == [
    "Apple makes iPhone",
    "Amazon makes Kindle",
])
client.close()
