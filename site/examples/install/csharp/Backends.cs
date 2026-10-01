using System.Diagnostics;
using ThinkThen;

using var tt = Engine.Open();
const string question =
    "Does the customer ask for a refund?";
var brokenIsRefund = tt.Decide(
    question,
    "Please refund my order. It arrived broken.");
var thanksIsRefund = tt.Decide(
    question,
    "Thanks for the quick help yesterday!");
var broken = brokenIsRefund.Value.OutcomeKind;
var thanks = thanksIsRefund.Value.OutcomeKind;
Trace.Assert(broken == Outcome.Yes);
Trace.Assert(thanks == Outcome.No);
