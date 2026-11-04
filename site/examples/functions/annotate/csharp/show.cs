using ThinkThen;

await using var client = new ThinkThenClient();
using var refundQuestion = DecideQuestion.Create(
    "Does the customer request a refund?");
using var topicQuestion = ChooseQuestion.Create("What is the message about?",
    ["billing", "shipping", "other"]);
using var urgencyQuestion = ScoreQuestion.Create("How urgent is the request?",
    ["routine", "urgent"]);
using var questions = AnnotationQuestions.Create(
    new Dictionary<string, AnnotationQuestion>
    {
        ["refund"] = new AnnotationQuestion.Decide(refundQuestion),
        ["topic"] = new AnnotationQuestion.Choose(topicQuestion),
        ["urgency"] = new AnnotationQuestion.Score(urgencyQuestion),
    });

await foreach (var form in client.AnnotateStreamAsync(
    questions, File.ReadLinesAsync("messages.txt")))
{
    // Each answer throws UnsureException when not sure, as a row does.
    ValueMap<string, AnnotationEntry> answers = form;
    bool isRefund = answers["refund"];
    string topic = answers["topic"];
    double urgency = answers["urgency"];
    Console.WriteLine($"{isRefund} {topic} {urgency}");
}
