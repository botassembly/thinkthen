using System.Diagnostics;
using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;
using Named = ThinkThen.Results
    .RelationEndpointRelatedEntity;

using var engine = Engine.Open();
var names = new[]
{
    new { name = "Apple", kind = "company" },
    new { name = "Amazon", kind = "company" },
    new { name = "iPhone", kind = "product" },
    new { name = "Kindle", kind = "product" },
};
var makes = new
{
    name = "makes",
    source = "company",
    target = "product",
};
var ask = engine.ParseQuestion(
    AuthoredQuestionKind.Relate,
    JsonSerializer.Serialize(new
    {
        version = 1,
        relate = new { relations = new[] { makes } },
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
var whoMakes = call.Packets
    .OfType<SessionPacketRelateAggregate>()
    .Single()
    .Value.Value;
var pairs = whoMakes.Select(edge => (
    ((Named)edge.Source).Name,
    ((Named)edge.Target).Name));
Trace.Assert(pairs.SequenceEqual(
[
    ("Apple", "iPhone"),
    ("Amazon", "Kindle"),
]));
