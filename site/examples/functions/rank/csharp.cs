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
var records = new InputRequestInputRecords
{
    Items = inbox
        .Select(message => new InputRequestItem
        {
            Original = new InputRequestOriginalText
            {
                Text = message,
            },
        })
        .ToList(),
};
var call = await engine.RankAsync(
    new InputRequestQuestionText { Text = question },
    records);
var byUrgency = call.Packets
    .OfType<SessionPacketRankAggregate>()
    .Single()
    .Value;
var order = byUrgency.Select(one => one.Index.Value);
Trace.Assert(order.SequenceEqual([1UL, 3UL, 2UL, 0UL]));
