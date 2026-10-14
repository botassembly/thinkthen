using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var text = "Sam Patel says his Kindle arrived broken "
    + "in Denver.";
var ask = QuestionBuilders.Recognize(new RecognizeQuestion
{
    Kinds = new[] { "person", "product", "place" }
        .Select(kind =>
            new KeyValuePair<string, AuthoredDescription>(
                kind, new AuthoredDescription
                {
                    Value =
                        new AuthoredDescriptionValueNull(),
                }))
        .ToList(),
});
var call = await engine.RecognizeAsync(
    ask,
    new RequestInputText { Text = text });
var facts = (RecognizeEntities)call.Value;
var names = facts.Entities
    .Select(one => (one.Text, one.Kind));
Trace.Assert(names.SequenceEqual(
[
    ("Sam Patel", "person"),
    ("Kindle", "product"),
    ("Denver", "place"),
]));
