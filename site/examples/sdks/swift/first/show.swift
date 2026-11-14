import ThinkThen

let client = try Client()
let isRefund = try await client.decide(
  "Does the customer request a refund?",
  record: "Please refund my order."
).answer
print(isRefund)
