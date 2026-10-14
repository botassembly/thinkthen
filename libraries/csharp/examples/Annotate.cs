using ThinkThen.Results;
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = new RequestQuestionDefinition { Value = new RequestDefinitionAnnotateDefinition {
    Questions = new KeyValuePair<string, AnnotateQuestion>[] { new("relevant", new AnnotateQuestionDecide {
        Decide = new AuthoredQuestionTextString { Value = "Is it?" }
    }) }
} };
var answer = (await engine.AnnotateAsync(question, new RequestInputText { Text = "example" })).Value;
Console.WriteLine(((AnnotatedFieldBoolean)answer["relevant"]).Value);
