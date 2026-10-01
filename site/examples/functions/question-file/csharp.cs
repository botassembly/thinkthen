using System.Diagnostics;
using ThinkThen;

using var tt = Engine.Open();
var refund = File.ReadAllText("refund.json");
const string text = "I would like to return this "
    + "and get my money back.\n";
var isRefund = tt.Decide(refund, text).Value;
Trace.Assert(isRefund.OutcomeKind == Outcome.Yes);
