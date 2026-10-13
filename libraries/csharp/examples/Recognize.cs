using ThinkThen.Results;
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = QuestionBuilders.Recognize(new InputRecognizeQuestion {
    Kinds = new Dictionary<string, InputAuthoredDescription> {
        ["person"] = new() { Value = new InputAuthoredDescriptionValueString { Value = "A person." } }
    }
});
var answer = (await engine.RecognizeAsync(question, new InputRequestInputText { Text = "John Smith" })).Recognize();
Console.WriteLine(((RecognizeEntities)answer[0].Value).Entities[0].Text);
