using ThinkThen.Results;
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new InputRequestQuestionDefinition { Value = new InputRequestDefinitionAnnotateDefinition {
    Questions = new Dictionary<string, InputAnnotateQuestion> { ["relevant"] = new InputAnnotateQuestionDecide {
        Decide = new InputAuthoredQuestionTextAlternative0 { Value = "Is it?" }
    } }
} };
var answer = (await engine.AnnotateAsync(question, new InputRequestInputText { Text = "example" })).Annotate();
Console.WriteLine(((AnnotatedFieldBoolean)answer[0].Value["relevant"]).Value);
