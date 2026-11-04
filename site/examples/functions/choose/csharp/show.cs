using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

string topic = await client.ChooseAsync(
    "What is the message about?", message,
    ["billing", "shipping", "other"]);
Console.WriteLine(topic);
