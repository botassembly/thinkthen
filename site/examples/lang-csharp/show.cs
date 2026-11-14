using ThinkThen;

await using var client = new ThinkThenClient();
var isRefund = await client.DecideAsync(
    "Does the customer request a refund?",
    "Please refund my order.");
Console.WriteLine(isRefund);
