import ThinkThen

let tt = try Engine()
let question = "Does the customer ask for a refund?"
let isRefund = try tt.decide(
    question,
    "Please refund my order. It arrived broken."
)
precondition(isRefund.value.outcome == .yes)
tt.close()
