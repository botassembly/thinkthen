using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var question = "Does the customer ask for a refund?";
var broken = "Please refund my order. It arrived broken.";
var call = await engine.DecideAsync(
    new InputRequestQuestionText { Text = question },
    new InputRequestInputText { Text = broken });
var isRefund = call.Decision();
Trace.Assert(isRefund == true);
