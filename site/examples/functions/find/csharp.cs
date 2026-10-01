using System.Diagnostics;
using System.Text.Json;
using ThinkThen;

using var tt = Engine.Open();
const string question =
    "Which line gives the refund deadline?";
string[] policy =
[
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
];
var request = JsonSerializer.Serialize(new
{
    find = question,
    units = policy,
});
var refundDeadline = tt.CallTyped(request).Value;
var unit = refundDeadline.GetProperty("unit").GetString();
Trace.Assert(unit == policy[1]);
