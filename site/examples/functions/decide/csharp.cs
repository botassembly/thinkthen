using System.Diagnostics;
using ThinkThen;

using var tt = Engine.Open();
const string question =
    "Does the customer ask for a refund?";
const string text =
    "Please refund my order. It arrived broken.";
var isRefund = tt.Decide(question, text).Value;
Trace.Assert(isRefund.OutcomeKind == Outcome.Yes);
