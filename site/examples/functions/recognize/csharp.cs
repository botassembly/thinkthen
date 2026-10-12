using System.Diagnostics;
using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
string[] kinds = ["person", "organization", "place"];
var text = "Maria Chen joined Northwind Freight, "
    + "a company in Chicago.";
var ask = engine.ParseQuestion(
    AuthoredQuestionKind.Recognize,
    JsonSerializer.Serialize(new
    {
        version = 1,
        recognize = new
        {
            kinds = kinds.ToDictionary(
                kind => kind,
                kind => (string?)null),
        },
    }));
var call = await engine.RecognizeAsync(
    ask,
    new InputRequestInputText { Text = text });
var facts = (RecognizeFieldsEntities)call.Packets
    .OfType<SessionPacketRecognizeAggregate>()
    .First()
    .Value.Single()
    .Value;
var names = facts.Entities
    .Select(one => (one.Text, one.Kind));
Trace.Assert(names.SequenceEqual(
[
    ("Maria Chen", "person"),
    ("Northwind Freight", "organization"),
    ("Chicago", "place"),
]));
