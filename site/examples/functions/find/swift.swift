import ThinkThen

let client = try Client()
let question = "Which line gives the refund deadline?"
let policy = [
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
]
let records = InputRequestInputRecords(
    items: policy.map { .text($0) }
)
let call = try await client.find(
    .text(question),
    input: .records(records)
)
var refundDeadline = ""
for case .findAggregate(let found) in call.packets {
    if case .string(let line) = found.value.value.kind {
        refundDeadline = line
    }
}
precondition(refundDeadline == policy[1])
client.close()
