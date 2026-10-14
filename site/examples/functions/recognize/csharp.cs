using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var text = "Sam Patel says his Kindle arrived broken "
    + "in Denver.";
var ask = new RequestQuestionFile
{
    Path = "kinds.json",
};
var call = await engine.RecognizeAsync(
    ask,
    new RequestInputText { Text = text });
var facts = (RecognizeEntities)call.Recognize()[0].Value;
var names = facts.Entities
    .Select(one => (one.Text, one.Kind));
Trace.Assert(names.SequenceEqual(
[
    ("Sam Patel", "person"),
    ("Kindle", "product"),
    ("Denver", "place"),
]));
