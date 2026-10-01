using System.Diagnostics;
using System.Text.Json;
using ThinkThen;

using var tt = Engine.Open();
const string question = "Is this urgent?";
string[] inbox =
[
    "Newsletter: our autumn catalog is here. "
        + "No reply needed.",
    "Our checkout page is down and customers cannot pay",
    "Reminder: your invoice is due in 30 days",
    "Please send the signed quote by 5 pm today",
];
var request = JsonSerializer.Serialize(new
{
    rank = question,
    records = inbox,
});
var byUrgency = tt.CallTyped(request).Value;
var order = byUrgency.EnumerateArray()
    .Select(one => one.GetProperty("index").GetInt32());
Trace.Assert(order.SequenceEqual([1, 3, 2, 0]));
