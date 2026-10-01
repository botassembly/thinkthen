using System.Diagnostics;
using System.Text.Json;
using ThinkThen;

using var tt = Engine.Open();
const string question = "Which labels fit this message?";
string[] labels = ["praise", "bug", "billing"];
const string message =
    "Love the new dashboard, but export crashes the app,\n"
    + "and I was charged twice.\n";
var request = JsonSerializer.Serialize(new
{
    tag = question,
    labels,
    evidence = message,
});
var fittingLabels = tt.CallTyped(request).Value
    .EnumerateArray()
    .Select(one => one.GetString());
Trace.Assert(fittingLabels.SequenceEqual(labels));
