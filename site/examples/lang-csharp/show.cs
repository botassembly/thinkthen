using ThinkThen;

await using var client = new ThinkThenClient();
using var stop = new CancellationTokenSource();
var refund = await client.DecideAsync(
    "Does the customer request a refund?",
    "Please refund my order.",
    cancellationToken: stop.Token);

if (refund is Answered<bool, NoDetail> answered)
    Console.WriteLine(answered.Answer);
else if (refund is Failed<bool, NoDetail> failed)
    Console.Error.WriteLine($"{failed.Error.Code}: {failed.Error.Message}");
else
    Console.WriteLine("Not sure");
