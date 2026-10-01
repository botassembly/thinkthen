using System.Diagnostics;
using System.Text.Json;
using ThinkThen;

using var tt = Engine.Open();
var kinds = new Dictionary<string, string?>
{
    ["person"] = null,
    ["organization"] = null,
    ["place"] = null,
};
var spec = JsonSerializer.Serialize(new
{
    version = 1,
    recognize = new { kinds },
});
const string text = "Maria Chen joined Northwind Freight, "
    + "a company in Chicago.";
var facts = tt.Recognize(spec, text).Value;
var names = facts.GetProperty("entities")
    .EnumerateArray()
    .Select(one => (
        one.GetProperty("text").GetString(),
        one.GetProperty("kind").GetString()));
Trace.Assert(names.SequenceEqual(
[
    ("Maria Chen", "person"),
    ("Northwind Freight", "organization"),
    ("Chicago", "place"),
]));
