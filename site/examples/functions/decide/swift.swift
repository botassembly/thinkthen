import ThinkThen

let tt = try Engine()
let question = "Does the customer ask for a refund?"
let refund = """
    {"decide": "\(question)", "threshold": "0.2:0.8"}
    """
let texts = [
    "Please refund my order. It arrived broken.",
    "Thanks for the quick help yesterday!",
    "I want to send this back.",
]
let isRefund = try texts.map {
    try tt.decide(refund, $0).value.outcome
}
precondition(isRefund == [.yes, .no, .unsure])
tt.close()
