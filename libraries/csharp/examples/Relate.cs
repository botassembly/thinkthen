using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = QuestionBuilders.Relate(new InputRelateQuestion {
    Relations = new InputAuthoredRelation[] {
        new() { Name = new() { Value = "linked" }, Source = new InputAuthoredName { Value = "person" }, Target = new InputAuthoredName { Value = "person" } }
    }
});
var input = new InputRequestInputEntities { Items = new InputRequestItem[] {
    new() { Original = new InputRequestOriginalJson { Value = JsonSerializer.SerializeToElement(new { name = "A", kind = "person" }) } },
    new() { Original = new InputRequestOriginalJson { Value = JsonSerializer.SerializeToElement(new { name = "B", kind = "person" }) } }
} };
var answer = (await engine.RelateAsync(question, input)).Relate();
Console.WriteLine($"{answer.Value[0].SourceName} -> {answer.Value[0].TargetName}");
