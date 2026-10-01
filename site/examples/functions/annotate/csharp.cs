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
Trace.Assert(triage.GetRawText() ==
    """[{"steps":true,"area":"login","impact":1.98}]""");
