import ThinkThen

let client = try Client()
let question = "Does the customer ask for a refund?"
let broken = "Please refund my order. It arrived broken."
let judgment = try await client.decide(
  .text(question),
  input: InputText(text: broken)
)
let isRefund = judgment.value == .bool(true)
let refundProbability: Double? = judgment.probability
precondition(isRefund && refundProbability != nil)
client.close()
