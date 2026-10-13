using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new InputRequestQuestionDefinition { Value = new InputRequestDefinitionAuthoredChoose {
    Choose = new InputAuthoredQuestionTextAlternative0 { Value = "Which?" },
    Options = new InputAuthoredOptionsAlternative0 { Value = new InputAuthoredName[] { new() { Value = "first" }, new() { Value = "second" } } }
} };
Console.WriteLine(await engine.ChooseValueAsync(question, new InputRequestInputText { Text = "example" }));
