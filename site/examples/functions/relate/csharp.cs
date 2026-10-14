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
var ask = new RequestQuestionFile
{
    Path = "makes.json",
};
var entities = new RequestInputRecords
{
    Items = names
        .Select(name => new RequestItem
        {
            Original = new RequestOriginalJson
            {
                Value =
                    JsonSerializer.SerializeToElement(name),
            },
        })
        .ToList(),
};
var call = await engine.RelateAsync(ask, entities);
var whoMakes = call.Relate().Value;
var pairs = whoMakes.Select(edge => (
    ((Named)edge.Source).Name,
    ((Named)edge.Target).Name));
Trace.Assert(pairs.SequenceEqual(
[
    ("Apple", "iPhone"),
    ("Amazon", "Kindle"),
]));
