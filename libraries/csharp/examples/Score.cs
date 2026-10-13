using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new InputRequestQuestionDefinition { Value = new InputRequestDefinitionAuthoredScore {
    Score = new InputAuthoredQuestionTextString { Value = "How much?" },
    Levels = new InputAuthoredLevelsList { Value = new InputAuthoredName[] { new() { Value = "low" }, new() { Value = "high" } } }
} };
Console.WriteLine(await engine.ScoreValueAsync(question, new InputRequestInputText { Text = "example" }));
