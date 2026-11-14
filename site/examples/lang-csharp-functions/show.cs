using ThinkThen;

const string refund = "Please refund my order.";
var messages = new List<Dictionary<string, string>>
{
    new() { ["id"] = "a1", ["text"] = "Please refund my order." },
    new() { ["id"] = "a2", ["text"] = "Where is my parcel?" },
    new() { ["id"] = "a3", ["text"] = "I was charged twice for one invoice." },
};
const string text =
    "Morgan works for Example Group.\nPlease refund my order.\n";
const string asksRefund = "Does the customer request a refund?";
const string howUrgent = "How urgent is the request?";
string[] urgency = ["routine", "urgent"];
string[] topics = ["billing", "shipping", "other"];
var field = Pointer.From("/text");

await using var client = new ThinkThenClient();

var decision = await client.DecideAsync(asksRefund, refund);
if (HasAnswer("decide", decision))
    Console.WriteLine($"decide: {decision.Answer}");

var filterOptions = new FilterOptions { Field = field };
try
{
    // One bar: each message is yes or no; failures are reported after kept messages.
    await foreach (var message in
        client.FilterAsync(asksRefund, messages, filterOptions))
        Console.WriteLine($"filter: {message["id"]}");
}
catch (FailedException failure)
{
    // The kept messages come first, then every failed message.
    foreach (var failedMessage in failure.Rows)
        Console.Error.WriteLine(
            $"filter {failedMessage.Position}: {failedMessage}");
    Environment.ExitCode = 1;
}

var choice = await client.ChooseAsync(
    "What is the message about?", refund, topics);
if (HasAnswer("choose", choice))
    Console.WriteLine($"choose: {choice.Answer}");

var labels = await client.TagAsync(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    ["billing", "shipping"]);
if (HasAnswer("tag", labels))
    Console.WriteLine($"tag: [{string.Join(", ", labels.Answer)}]");

var score = await client.ScoreAsync(howUrgent, refund, urgency);
if (HasAnswer("score", score))
    Console.WriteLine($"score: {score.Answer}");

var rankOptions = new RankOptions { Field = field };
await foreach (var ranked in
    client.RankAsync(howUrgent, messages, urgency, rankOptions))
    if (HasAnswer($"rank {ranked.Position}", ranked))
        Console.WriteLine($"rank: {ranked.Answer} {ranked.Item["id"]}");

var action = await client.FindAsync(
    "Which line states the requested action?", text);
if (HasAnswer("find", action) && action.Answer is { } span)
    Console.WriteLine($"find: \"{span.Text}\"");

var names = await client.RecognizeAsync(
    "Morgan works for Example Group.", ["person", "organization"]);
if (HasAnswer("recognize", names))
    foreach (var name in names.Answer)
        Console.WriteLine(
            $"recognize: {name.Kind} {name.Text}");

RelationRule[] rules =
[
    new()
    {
        Source = "person", Target = "organization", Relation = "works_for",
    },
];
var relations = await client.RelateAsync(text, rules);
if (HasAnswer("relate", relations))
    foreach (var relation in relations.Answer)
        Console.WriteLine(
            $"relate: {relation.Source.Text} {relation.Relation} "
            + $"{relation.Target.Text}");

var questions = new AnnotationQuestions
{
    { "refund", DecideQuestion.Create(asksRefund) },
    { "topic", ChooseQuestion.Create("What is the message about?", topics) },
};
var form = await client.AnnotateAsync(questions, refund);
if (HasAnswer("annotate", form))
{
    var refundCheck = form.Answer.Decide("refund");
    var topicChoice = form.Answer.Choose("topic");
    if (HasAnswer("annotate refund", refundCheck))
        Console.WriteLine($"annotate: refund {refundCheck.Answer}");
    if (HasAnswer("annotate topic", topicChoice))
        Console.WriteLine($"annotate: topic {topicChoice.Answer}");
}

// True for an answered row. A not sure or failed row is written to
// standard error, such as "decide: unsure", and its answer is never read.
// Scalar calls throw FailedException before this helper; failed rows remain
// available from bulk calls and form entries.
static bool HasAnswer<A, D>(string label, Row<A, D> row)
{
    if (row.IsAnswered) return true;
    Console.Error.WriteLine($"{label}: {row}");
    if (row.IsFailed) Environment.ExitCode = 1;
    return false;
}
