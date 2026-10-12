import ThinkThen

let client = try Client()
let text = "Sam Patel says his Kindle arrived broken "
    + "in Denver."
let call = try await client.recognize(
    .file("kinds.json"),
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
    "Sam Patel (person)",
    "Kindle (product)",
    "Denver (place)",
])
client.close()
