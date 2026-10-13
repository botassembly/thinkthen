using System;
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var result = await engine.DecideAsync(
    new InputRequestQuestionText { Text = "Is it?" },
    new InputRequestInputText { Text = "example" });
Console.WriteLine(result.Decision());
