using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = QuestionBuilders.Relate(new RelateQuestion {
    Relations = new AuthoredRelation[] {
        new() { Name = new() { Value = "linked" }, Source = new AuthoredName { Value = "person" }, Target = new AuthoredName { Value = "person" } }
    }
});
var input = new RequestInputEntities { Items = new RequestItem[] {
    new() { Original = new RequestOriginalJson { Value = JsonSerializer.SerializeToElement(new { name = "A", kind = "person" }) } },
    new() { Original = new RequestOriginalJson { Value = JsonSerializer.SerializeToElement(new { name = "B", kind = "person" }) } }
} };
var answer = (await engine.RelateAsync(question, input)).Value;
Console.WriteLine($"{answer[0].SourceName} -> {answer[0].TargetName}");
