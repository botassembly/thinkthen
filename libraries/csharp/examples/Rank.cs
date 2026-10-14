using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var answer = (await engine.RankAsync(
    new RequestQuestionText { Text = "Is it?" },
    new RequestInputRecords { Items = new RequestItem[] {
        new() { Original = new RequestOriginalText { Text = "first" } },
        new() { Original = new RequestOriginalText { Text = "second" } }
    } })).Rank();
Console.WriteLine($"{answer[0]},{answer[1]}");
