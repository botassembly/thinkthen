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
let call = try await client.rank(
    .text(question),
    input: .records(records)
)
var byUrgency = [Int](repeating: -1, count: inbox.count)
for case .rankAggregate(let ranked) in call.packets {
    for one in ranked.value {
        if case .value(let index) =
            one.index
        {
            byUrgency[Int(one.value) - 1] = Int(index)
        }
    }
}
precondition(byUrgency == [1, 3, 2, 0])
client.close()
