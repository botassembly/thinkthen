using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

var urgency = await client.ScoreAsync(
    "How urgent is the request?", message,
    ["routine", "urgent"]);
Console.WriteLine(urgency);
