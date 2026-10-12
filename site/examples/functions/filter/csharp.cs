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
var records = new InputRequestInputRecords
{
    Items = reviews
        .Select(review => new InputRequestItem
        {
            Original = new InputRequestOriginalText
            {
                Text = review,
            },
        })
        .ToList(),
};
var call = await engine.FilterAsync(
    new InputRequestQuestionText { Text = question },
    records);
var complaints = call.Packets
    .OfType<SessionPacketFilterRow>()
    .Select(row => row.Value.Input.Value.GetString())
    .ToList();
Trace.Assert(complaints.SequenceEqual(
    [reviews[1], reviews[3]]));
