import ThinkThen

let client = try Client()
let question = "How urgent is this?"
let levels: [JSONValue] = [
    .string("Routine."),
    .string("Soon."),
    .string("Immediate."),
]
let howUrgent = try client.parseQuestion(
    .atomic,
    authored: .orderedObject([
        JSONMember("score", .string(question)),
        JSONMember("levels", .array(levels)),
    ])
)
let outage =
    "Our checkout page is down and customers cannot pay.\n"
let call = try await client.score(
    howUrgent,
    input: .text(outage)
)
var urgency = -1.0
for case .scoreRow(let row) in call.packets {
    urgency = row.value.value
}
precondition(urgency == 2.0)
client.close()
