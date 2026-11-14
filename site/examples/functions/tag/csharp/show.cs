using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

var topics = await client.TagAsync(
    "Which topics appear?", message,
    ["billing", "shipping"]);
Console.WriteLine($"[{string.Join(", ", topics.Answer)}]");
