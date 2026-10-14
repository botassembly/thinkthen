import ThinkThen

let client = try Client()
let question = "Is this urgent?"
let inbox = [
    "Newsletter: our autumn catalog is here. "
        + "No reply needed.",
    "Our checkout page is down and customers cannot pay",
    "Reminder: your invoice is due in 30 days",
    "Please send the signed quote by 5 pm today",
]
let records = InputRequestInputRecords(
    items: inbox.map { .text($0) }
)
let judgment = try await client.rankAnswer(
    .text(question),
    input: .records(records)
)
let byUrgency = judgment
precondition(byUrgency == [1, 3, 2, 0])
client.close()
