using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new RequestQuestionDefinition { Value = new RequestDefinitionAuthoredScore {
    Score = new AuthoredQuestionTextString { Value = "How much?" },
    Levels = new AuthoredLevelsList { Value = new AuthoredName[] { new() { Value = "low" }, new() { Value = "high" } } }
} };
Console.WriteLine(await engine.ScoreValueAsync(question, new RequestInputText { Text = "example" }));
