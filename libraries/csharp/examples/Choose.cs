using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new InputRequestQuestionDefinition { Value = new InputRequestDefinitionAuthoredChoose {
    Choose = new InputAuthoredQuestionTextString { Value = "Which?" },
    Options = new InputAuthoredOptionsList { Value = new InputAuthoredName[] { new() { Value = "first" }, new() { Value = "second" } } }
} };
Console.WriteLine(await engine.ChooseValueAsync(question, new InputRequestInputText { Text = "example" }));
