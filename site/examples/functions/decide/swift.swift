import ThinkThen

let client = try Client()
let question = "Does the customer ask for a refund?"
let broken = "Please refund my order. It arrived broken."
let call = try await client.decide(
    .text(question),
    input: .text(broken)
)
var isRefund = false
for case .decideRow(let row) in call.packets {
    let value = row.value.value
    if case .alternative0(.alternative0(let yes)) = value {
        isRefund = yes
    }
}
precondition(isRefund)
client.close()
