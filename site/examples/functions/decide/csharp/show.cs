using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

// A failed message throws FailedException at await. An unsure message returns
// a result whose Answer would throw UnsureException; printing remains safe.
var isRefund = await client.DecideAsync(
    "Does the customer request a refund?", message);
Console.WriteLine(isRefund);
