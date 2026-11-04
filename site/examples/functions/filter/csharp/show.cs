using ThinkThen;

await using var client = new ThinkThenClient();

// Only the messages that answer yes.
// A failed message throws after the kept ones.
await foreach (string refund in client.FilterAsync(
    "Does the customer request a refund?", File.ReadLinesAsync("messages.txt")))
    Console.WriteLine(refund);
