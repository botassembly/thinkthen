using System.Diagnostics;
using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var question = "How urgent is this?";
string[] levels = ["Routine.", "Soon.", "Immediate."];
var outage = "Our checkout page is down "
    + "and customers cannot pay.\n";
var scale = engine.ParseQuestion(
    AuthoredQuestionKind.Atomic,
    JsonSerializer.Serialize(
        new { score = question, levels }));
var call = await engine.ScoreAsync(
    scale,
    new InputRequestInputText { Text = outage });
var urgency = call.Packets
    .OfType<SessionPacketScoreRow>()
    .Single()
    .Value.Value;
Trace.Assert(urgency == 2.0);
