using ThinkThen.Results;
using ThinkThen;
using ThinkThen.Inputs;
using var engine = Engine.Open();
var question = QuestionBuilders.Recognize(new RecognizeQuestion {
    Kinds = new KeyValuePair<string, AuthoredDescription>[] {
        new("person", new() { Value = new AuthoredDescriptionValueString { Value = "A person." } })
    }
});
var answer = (await engine.RecognizeAsync(question, new RequestInputText { Text = "John Smith" })).Value;
Console.WriteLine(((RecognizeEntities)answer).Entities[0].Text);
