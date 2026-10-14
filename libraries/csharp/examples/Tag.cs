using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new RequestQuestionDefinition { Value = new RequestDefinitionAuthoredTag {
    Tag = new AuthoredQuestionTextString { Value = "Which labels?" },
    Labels = new AuthoredLabelsList { Value = new AuthoredName[] { new() { Value = "first" }, new() { Value = "second" } } }
} };
Console.WriteLine(string.Join(",", (await engine.TagAsync(question, new RequestInputText { Text = "example" })).Value));
