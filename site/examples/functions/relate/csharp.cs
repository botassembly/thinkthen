using System.Diagnostics;
using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;
using Named = ThinkThen.Results
    .RelatedEntityEdgePropertiesSourceFieldsKindName;

using var engine = Engine.Open();
var names = new[]
{
    new { name = "Paul McCartney", kind = "singer" },
    new { name = "Ringo Starr", kind = "singer" },
    new { name = "Yesterday", kind = "song" },
    new { name = "Octopus's Garden", kind = "song" },
};
var sings = new
{
    name = "sings",
    source = "singer",
    target = "song",
};
var ask = engine.ParseQuestion(
    AuthoredQuestionKind.Relate,
    JsonSerializer.Serialize(new
    {
        version = 1,
        relate = new { relations = new[] { sings } },
    }));
var entities = new InputRequestInputRecords
{
    Items = names
        .Select(name => new InputRequestItem
        {
            Original = new InputRequestOriginalJson
            {
                Value =
                    JsonSerializer.SerializeToElement(name),
            },
        })
        .ToList(),
};
var call = await engine.RelateAsync(ask, entities);
var whoSings = call.Packets
    .OfType<SessionPacketRelateAggregate>()
    .Single()
    .Value.Value;
var pairs = whoSings.Select(edge => (
    ((Named)edge.Source).Name,
    ((Named)edge.Target).Name));
Trace.Assert(pairs.SequenceEqual(
[
    ("Paul McCartney", "Yesterday"),
    ("Ringo Starr", "Octopus's Garden"),
]));
