using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new InputRequestQuestionDefinition { Value = new InputRequestDefinitionAuthoredScore {
    Score = new InputAuthoredQuestionTextAlternative0 { Value = "How much?" },
    Levels = new InputAuthoredLevelsAlternative0 { Value = new InputAuthoredName[] { new() { Value = "low" }, new() { Value = "high" } } }
} };
Console.WriteLine(await engine.ScoreValueAsync(question, new InputRequestInputText { Text = "example" }));
