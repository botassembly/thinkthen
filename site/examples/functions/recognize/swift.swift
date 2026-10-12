import ThinkThen

let client = try Client()
let text = "Maria Chen joined Northwind Freight, "
    + "a company in Chicago."
let call = try await client.recognize(
    .file("names.json"),
    input: .text(text)
)
var names: [String] = []
for case .recognizeAggregate(let found) in call.packets {
    for row in found.value {
        if case .alternative0(let facts) = row.value {
            names += facts.entities.map {
                "\($0.text) (\($0.kind))"
            }
        }
    }
}
precondition(names == [
    "Maria Chen (person)",
    "Northwind Freight (organization)",
    "Chicago (place)",
])
client.close()
