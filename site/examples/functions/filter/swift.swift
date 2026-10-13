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
let call = try await client.filter(
    .text(question),
    input: .records(records)
)
var complaints: [String] = []
for case .filterRow(let row) in call.packets {
    let kept = row.value
    guard kept.value else { continue }
    if case .value(let index) = kept.index {
        complaints.append(reviews[Int(index)])
    }
}
precondition(complaints == [reviews[1], reviews[3]])
client.close()
