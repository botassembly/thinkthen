import ThinkThen

let client = try Client()
let question = "Is this a complaint?"
let reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
]
let records = InputRequestInputRecords(
    items: reviews.map { .text($0) }
)
let judgment = try await client.filterAnswer(
    .text(question),
    input: .records(records)
)
let complaints = judgment.compactMap { value -> String? in
    if case .string(let text) = value { return text }
    return nil
}
precondition(complaints == [reviews[1], reviews[3]])
client.close()
