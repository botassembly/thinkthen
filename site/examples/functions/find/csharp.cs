using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var question = "Which line gives the refund deadline?";
string[] policy =
[
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
];
var records = new RequestInputRecords
{
    Items = policy
        .Select(line => new RequestItem
        {
            Original = new RequestOriginalText
            {
                Text = line,
            },
        })
        .ToList(),
};
var call = await engine.FindAsync(
    new RequestQuestionText { Text = question },
    records);
var refundDeadline = call.Value.GetString();
Trace.Assert(refundDeadline == policy[1]);
