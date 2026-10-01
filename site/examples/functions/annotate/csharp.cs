using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Nodes;
using ThinkThen;

using var tt = Engine.Open();
var form = JsonNode.Parse(File.ReadAllText("form.json"));
const string body = "Steps: click Log in. Nobody gets in.";
var request = JsonSerializer.Serialize(new
{
    annotate = form,
    records = new[] { body },
});
var triage = tt.CallTyped(request).Value;
var ticket = triage.EnumerateArray().Single();
var fields = ticket.EnumerateObject()
    .Select(one => one.Name).Order();
var filled = (
    ticket.GetProperty("steps").GetBoolean(),
    ticket.GetProperty("area").GetString(),
    ticket.GetProperty("impact").GetDecimal());
Trace.Assert(fields.SequenceEqual(
    ["area", "impact", "steps"]));
Trace.Assert(filled == (true, "login", 1.98m));
