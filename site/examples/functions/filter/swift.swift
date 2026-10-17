import ThinkThen

let client = try Client()
let question = "Is this a complaint?"
let reviews = [
  "Arrived a day early. Thank you!",
  "The zipper broke the first time I used it.",
  "Does this come in blue?",
  "The strap snapped on day two.",
]
let records = InputRecords(
  items: reviews.map { .text($0) }
)
let judgment = try await client.filter(
  .text(question),
  input: .records(records)
)
let complaints = judgment.value
precondition(
  complaints == [.string(reviews[1]), .string(reviews[3])]
)
client.close()
