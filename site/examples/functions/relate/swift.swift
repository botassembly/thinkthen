import ThinkThen

let client = try Client()
let names = [
    ("Apple", "company"),
    ("Amazon", "company"),
    ("iPhone", "product"),
    ("Kindle", "product"),
]
let entities = InputEntities(items: names.map {
    let entity = DataValue.orderedObject([
        ValueMember("name", .string($0.0)),
        ValueMember("kind", .string($0.1)),
    ])
    let original = OriginalJson(value: entity)
    return Item(
        original: .value(.json(original))
    )
})
let judgment = try await client.relateAnswer(
    .file("makes.json"),
    input: .entities(entities)
)
var whoMakes: [String] = []
for edge in judgment.value {
    if case .kindName(let company) = edge.source,
       case .kindName(let product) = edge.target {
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
