using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var answer = (await engine.RankAsync(
    new InputRequestQuestionText { Text = "Is it?" },
    new InputRequestInputRecords { Items = new InputRequestItem[] {
        new() { Original = new InputRequestOriginalText { Text = "first" } },
        new() { Original = new InputRequestOriginalText { Text = "second" } }
    } })).Rank();
Console.WriteLine($"{answer[0].Value},{answer[1].Value}");
