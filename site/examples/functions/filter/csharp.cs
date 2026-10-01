using System.Diagnostics;
using System.Text.Json;
using ThinkThen;

using var tt = Engine.Open();
const string question = "Is this a complaint?";
string[] reviews =
[
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
];
var request = JsonSerializer.Serialize(new
{
    filter = question,
    records = reviews,
});
var complaints = tt.CallTyped(request).Value
    .EnumerateArray()
    .Select(one => one.GetString());
Trace.Assert(complaints.SequenceEqual(
    [reviews[1], reviews[3]]));
