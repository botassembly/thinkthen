using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new InputRequestQuestionDefinition { Value = new InputRequestDefinitionAuthoredTag {
    Tag = new InputAuthoredQuestionTextString { Value = "Which labels?" },
    Labels = new InputAuthoredLabelsList { Value = new InputAuthoredName[] { new() { Value = "first" }, new() { Value = "second" } } }
} };
Console.WriteLine(string.Join(",", await engine.TagValueAsync(question, new InputRequestInputText { Text = "example" })));
