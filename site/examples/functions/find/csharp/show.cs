using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

// Null when no line answers the question.
var action = await client.FindAsync(
    "Which line states the requested action?", message);
Console.WriteLine(action.Answer?.Text ?? "no line answers");
