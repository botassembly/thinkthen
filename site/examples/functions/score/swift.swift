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
let judgment = try await client.scoreAnswer(
    howUrgent,
    input: .text(outage)
)
let urgency = judgment
precondition(urgency == 2.0)
client.close()
