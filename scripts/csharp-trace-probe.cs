using System.Reflection;
using System.Text.Json;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Diagnostics.Tracing.Parsers;

var assembly = typeof(EventPipeEventSource).Assembly;
var requirements = new Dictionary<string, string[]>
{
    ["Microsoft.Diagnostics.Tracing.EventPipeEventSource"] = ["EventsLost"],
    ["Microsoft.Diagnostics.Tracing.Parsers.Clr.GCHeapStatsTraceData"] = ["GenerationSize0", "GenerationSize1", "GenerationSize2"],
    ["Microsoft.Diagnostics.Tracing.Parsers.Clr.GCCreateSegmentTraceData"] = ["Address", "Size"],
    ["Microsoft.Diagnostics.Tracing.Parsers.GCDynamic.CommittedUsageTraceEvent"] = ["TotalCommittedInUse", "TotalCommittedInFree", "TotalBookkeepingCommitted"]
};
var schema = new Dictionary<string, string[]>();
foreach (var requirement in requirements)
{
    Type type = assembly.GetType(requirement.Key, throwOnError: true)!;
    string[] properties = type.GetProperties().Select(value => value.Name).ToArray();
    foreach (string name in requirement.Value)
        if (!properties.Contains(name)) throw new Exception($"Missing schema field: {requirement.Key}.{name}");
    schema.Add(requirement.Key, properties);
}
string[] events = typeof(GCDynamicTraceEventParser).GetEvents().Select(value => value.Name).ToArray();
if (!events.Any(value => value.Contains("CommittedUsage"))) throw new Exception("Missing committed usage subscription");
Console.WriteLine(JsonSerializer.Serialize(new { assembly = assembly.FullName, schema, dynamic_events = events }));
