using System.Diagnostics;
using ThinkThen;

using var tt = Engine.Open();
const string question =
    "Does the customer ask for a refund?";
var isRefund = tt.Decide(
    question,
    "Please refund my order. It arrived broken.");
Trace.Assert(isRefund.Value.OutcomeKind == Outcome.Yes);

const string refund = """
    {"decide": "Does the customer ask for a refund?",
     "threshold": "0.2:0.8"}
    """;
isRefund = tt.Decide(refund, "I want to send this back.");
Trace.Assert(isRefund.Value.OutcomeKind == Outcome.NotSure);
