import ThinkThen

let client = try Client()
let text =
  "Sam Patel says his Kindle arrived broken "
  + "in Denver."
let judgment = try await client.recognize(
  .file("kinds.json"),
  input: InputText(text: text)
)
var names: [String] = []
if case .entities(let facts) = judgment.value {
  names += facts.entities.map {
    "\($0.text) (\($0.kind))"
  }
}
precondition(
  names == [
    "Sam Patel (person)",
    "Kindle (product)",
    "Denver (place)",
  ])
client.close()
