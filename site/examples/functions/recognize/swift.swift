import ThinkThen

let client = try Client()
let text = "Sam Patel says his Kindle arrived broken "
    + "in Denver."
let judgment = try await client.recognizeAnswer(
    .file("kinds.json"),
    input: .text(text)
)
var names: [String] = []
for row in judgment {
    if case .entities(let facts) = row.value {
        names += facts.entities.map {
            "\($0.text) (\($0.kind))"
        }
    }
}
precondition(names == [
    "Sam Patel (person)",
    "Kindle (product)",
    "Denver (place)",
])
client.close()
