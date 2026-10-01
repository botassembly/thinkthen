using System.Diagnostics;
using System.Text.Json;
using ThinkThen;

using var tt = Engine.Open();
const string sings = """
    {"version": 1, "relate": {"relations": [
      {"name": "sings",
       "source": "singer", "target": "song"}]}}
    """;
(string Name, string Kind)[] names =
[
    ("Paul McCartney", "singer"),
    ("Ringo Starr", "singer"),
    ("Yesterday", "song"),
    ("Octopus's Garden", "song"),
];
var records = names
    .Select(one => JsonSerializer.Serialize(
        new { name = one.Name, kind = one.Kind }))
    .ToArray();
var whoSings = tt.Relate(sings, records).Value;
var pairs = whoSings.GetProperty("edges")
    .EnumerateArray()
    .Select(edge => (
        edge.GetProperty("source")
            .GetProperty("name").GetString(),
        edge.GetProperty("target")
            .GetProperty("name").GetString()));
Trace.Assert(pairs.SequenceEqual(
[
    ("Paul McCartney", "Yesterday"),
    ("Ringo Starr", "Octopus's Garden"),
]));
