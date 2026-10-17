using System.Diagnostics;
using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var names = new[]
{
    new { name = "Apple", kind = "company" },
    new { name = "Amazon", kind = "company" },
    new { name = "iPhone", kind = "product" },
    new { name = "Kindle", kind = "product" },
};
var ask = QuestionBuilders.Relate(new RelateQuestion
{
    Relations =
    [
        new AuthoredRelation
        {
            Name = new() { Value = "makes" },
            Source = new AuthoredName { Value = "company" },
            Target = new AuthoredName { Value = "product" },
        },
    ],
});
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
var whoMakes = call.Value;
var pairs = whoMakes.Select(edge => (
    edge.SourceName,
    edge.TargetName));
Trace.Assert(pairs.SequenceEqual(
[
    ("Apple", "iPhone"),
    ("Amazon", "Kindle"),
]));
