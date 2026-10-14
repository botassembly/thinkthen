using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new RequestQuestionDefinition { Value = new RequestDefinitionAuthoredChoose {
    Choose = new AuthoredQuestionTextString { Value = "Which?" },
    Options = new AuthoredOptionsList { Value = new AuthoredName[] { new() { Value = "first" }, new() { Value = "second" } } }
} };
Console.WriteLine(await engine.ChooseValueAsync(question, new RequestInputText { Text = "example" }));
