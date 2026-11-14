using System.Text.Json;
using ThinkThen;

await using var client = new ThinkThenClient();
var questions = new AnnotationQuestions
{
    { "refund", DecideQuestion.Create("Does the customer request a refund?") },
    { "topic", ChooseQuestion.Create("What is the message about?",
        new[] { "billing", "shipping", "other" }) },
    { "urgency", ScoreQuestion.Create("How urgent is the request?",
        new[] { "routine", "urgent" }) },
};

await foreach (var form in client.AnnotateStreamAsync(
    questions, File.ReadLinesAsync("messages.txt")))
{
    // Serialization keeps the state of the form and every entry.
    Console.WriteLine(JsonSerializer.Serialize(form));
}
