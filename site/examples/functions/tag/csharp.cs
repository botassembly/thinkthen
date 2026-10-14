using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var labelQuestion = new RequestQuestionFile
{
    Path = "labels.json",
};
var message = "Love the new dashboard, "
    + "but export crashes the app,\n"
    + "and I was charged twice.\n";
var call = await engine.TagAsync(
    labelQuestion,
    new RequestInputText { Text = message });
var fittingLabels = call.Value;
Trace.Assert(fittingLabels.SequenceEqual(
    ["praise", "bug", "billing"]));
