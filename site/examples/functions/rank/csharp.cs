using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var question = "Is this urgent?";
string[] inbox =
[
    "Newsletter: our autumn catalog is here. "
        + "No reply needed.",
    "Our checkout page is down and customers cannot pay",
    "Reminder: your invoice is due in 30 days",
    "Please send the signed quote by 5 pm today",
];
var records = new RequestInputRecords
{
    Items = inbox
        .Select(message => new RequestItem
        {
            Original = new RequestOriginalText
            {
                Text = message,
            },
        })
        .ToList(),
};
var call = await engine.RankAsync(
    new RequestQuestionText { Text = question },
    records);
var order = call.Rank();
Trace.Assert(order.SequenceEqual([1UL, 3UL, 2UL, 0UL]));
