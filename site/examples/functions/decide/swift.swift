import ThinkThen

let client = try Client()
let question = "Does the customer ask for a refund?"
let broken = "Please refund my order. It arrived broken."
let judgment = try await client.decideAnswer(
    .text(question),
    input: .text(broken)
)
let isRefund = judgment ?? false
precondition(isRefund)
client.close()
