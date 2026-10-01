using System.Diagnostics;
using System.Text.Json;
using ThinkThen;

using var tt = Engine.Open();
var kinds = new Dictionary<string, string>
{
    ["PER"] = "Part of a person's name.",
    ["ORG"] = "Part of the name of an organization: "
        + "a company, band, team, agency, "
        + "government body, or media outlet.",
    ["LOC"] = "Part of the name of a place: "
        + "a country, region, city, "
        + "or geographic feature.",
    ["MISC"] = "Part of another named entity: "
        + "a nationality, an event, a product, "
        + "or the name of a creative work.",
};
var spec = JsonSerializer.Serialize(new
{
    version = 1,
    recognize = new { kinds },
});
const string text = "Maria Chen joined Northwind Freight "
    + "in Chicago last spring.";
var facts = tt.Recognize(spec, text).Value;
var names = facts.GetProperty("entities")
    .EnumerateArray()
    .Select(one => (
        one.GetProperty("text").GetString(),
        one.GetProperty("kind").GetString()));
Trace.Assert(names.SequenceEqual(
[
    ("Maria Chen", "PER"),
    ("Northwind Freight", "ORG"),
    ("Chicago", "LOC"),
]));
