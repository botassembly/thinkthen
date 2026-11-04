using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

// Throws UnsureException when not sure,
// ThinkThenException when the message failed.
bool isRefund = await client.DecideAsync(
    "Does the customer request a refund?", message);
Console.WriteLine(isRefund);
