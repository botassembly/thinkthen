using System;
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var isRelevant = await engine.DecideAsync(
    new RequestQuestionText { Text = "Is it?" },
    new RequestInputText { Text = "example" });
Console.WriteLine(isRelevant.Value.ValueKind == System.Text.Json.JsonValueKind.Null ? "not sure" : isRelevant.Value.GetBoolean().ToString());
