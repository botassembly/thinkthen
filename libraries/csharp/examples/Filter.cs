using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var answer = (await engine.FilterAsync(
    new InputRequestQuestionText { Text = "Is it?" },
    new InputRequestInputRecords { Items = new InputRequestItem[] {
        new() { Original = new InputRequestOriginalText { Text = "first" } },
        new() { Original = new InputRequestOriginalText { Text = "second" } }
    } })).Filter();
Console.WriteLine($"{answer.Count}:{answer[0].Value}");
