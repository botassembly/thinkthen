using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var scale = new RequestQuestionFile
{
    Path = "levels.json",
};
var outage = "Our checkout page is down "
    + "and customers cannot pay.\n";
var call = await engine.ScoreAsync(
    scale,
    new RequestInputText { Text = outage });
var urgency = call.Score();
Trace.Assert(urgency == 2.0);
