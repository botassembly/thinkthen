import ThinkThen

let client = try Client()
let howUrgent = Question.file("levels.json")
let outage =
    "Our checkout page is down and customers cannot pay.\n"
let judgment = try await client.scoreAnswer(
    howUrgent,
    input: .text(outage)
)
let urgency = judgment
precondition(urgency == 2.0)
client.close()
