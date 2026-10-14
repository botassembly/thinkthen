using System;
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var result = await engine.DecideAsync(
    new RequestQuestionText { Text = "Is it?" },
    new RequestInputText { Text = "example" });
Console.WriteLine(result.Value);
