using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var question = "Is this a complaint?";
string[] reviews =
[
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
];
var records = new RequestInputRecords
{
    Items = reviews
        .Select(review => new RequestItem
        {
            Original = new RequestOriginalText
            {
                Text = review,
            },
        })
        .ToList(),
};
var call = await engine.FilterAsync(
    new RequestQuestionText { Text = question },
    records);
var complaints = call.Value
    .Select(one => one.GetString());
Trace.Assert(complaints.SequenceEqual(
    [reviews[1], reviews[3]]));
