import ThinkThen

let tt = try Engine()
let question = "Does the customer ask for a refund?"
var isRefund = try tt.decide(
    question,
    "Please refund my order. It arrived broken."
)
precondition(isRefund.value.outcome == .yes)

let refund = """
    {"decide": "\(question)", "threshold": "0.2:0.8"}
    """
isRefund = try tt.decide(
    refund,
    "I want to send this back."
)
precondition(isRefund.value.outcome == .unsure)
tt.close()
