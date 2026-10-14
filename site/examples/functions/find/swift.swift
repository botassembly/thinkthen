import ThinkThen

let client = try Client()
let question = "Which line gives the refund deadline?"
let policy = [
  "Returns need the original receipt.",
  "Refunds are issued within 30 days of purchase.",
  "Shipping is free on orders over $50.",
  "Gift cards cannot be exchanged for cash.",
]
let records = InputRecords(
  items: policy.map { .text($0) }
)
let judgment = try await client.find(
  .text(question),
  input: .records(records)
)
var refundDeadline = ""
if case .string(let line)? = judgment.value?.kind {
  refundDeadline = line
}
precondition(refundDeadline == policy[1])
client.close()
