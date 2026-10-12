using System.Diagnostics;
using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var question = "Which labels fit this message?";
string[] labels = ["praise", "bug", "billing"];
var message = "Love the new dashboard, "
    + "but export crashes the app,\n"
    + "and I was charged twice.\n";
var labelQuestion = engine.ParseQuestion(
    AuthoredQuestionKind.Atomic,
    JsonSerializer.Serialize(
        new { tag = question, labels }));
var call = await engine.TagAsync(
    labelQuestion,
    new InputRequestInputText { Text = message });
var fittingLabels = call.Packets
    .OfType<SessionPacketTagRow>()
    .Single()
    .Value.Value;
Trace.Assert(fittingLabels.SequenceEqual(
    ["praise", "bug", "billing"]));
