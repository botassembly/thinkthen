using System.Diagnostics;
using System.Text.Json;
using ThinkThen;

using var tt = Engine.Open();
const string question = "How urgent is this?";
string[] levels = ["Routine.", "Soon.", "Immediate."];
const string text = "Our checkout page is down "
    + "and customers cannot pay.\n";
var request = JsonSerializer.Serialize(new
{
    score = question,
    levels,
    evidence = text,
});
var urgency = tt.CallTyped(request).Value.GetDouble();
Trace.Assert(urgency == 2.0);
