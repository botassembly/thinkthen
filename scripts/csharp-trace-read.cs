using System.Text.Json;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Diagnostics.Tracing.Parsers;

using var source = new EventPipeEventSource(args[0]);
using var output = new StreamWriter(args[1]);
int collection = -1, events = 0, commits = 0, segments = 0, stats = 0;
void Write(object value) => output.WriteLine(JsonSerializer.Serialize(value));
void Event(TraceEvent value)
{
    var fields = value.PayloadNames.Select((name, index) => KeyValuePair.Create(name, value.PayloadValue(index))).ToDictionary();
    Write(new { name = value.EventName, time = value.TimeStamp, relative_ms = value.TimeStampRelativeMSec, collection, fields });
    events++;
}
source.Clr.GCStart += value => { collection = value.Count; Event(value); };
source.Clr.GCStop += Event;
source.Clr.GCHeapStats += value => { stats++; Event(value); };
source.Clr.GCCreateSegment += value => { segments++; Event(value); };
source.Clr.GCFreeSegment += Event;
source.Clr.GCGenerationRange += Event;
source.Clr.GCGlobalHeapHistory += Event;
source.Clr.GCDynamicEvent.GCCommittedUsage += value =>
{
    Write(new { name = "GC/CommittedUsage", collection, value.Version, value.TotalCommittedInUse,
        value.TotalCommittedInGlobalDecommit, value.TotalCommittedInFree, value.TotalCommittedInGlobalFree, value.TotalBookkeepingCommitted });
    commits++;
};
source.Process();
output.Flush();
var summary = new { events_lost = source.EventsLost, events, commits, segments, stats,
    session_start = source.SessionStartTime, session_end = source.SessionEndTime };
File.WriteAllText(args[2], JsonSerializer.Serialize(summary));
Console.WriteLine(JsonSerializer.Serialize(summary));
if (source.EventsLost != 0 || commits == 0 || segments == 0 || stats == 0)
    throw new Exception("Trace lacks complete GC/segment/commit attribution");
