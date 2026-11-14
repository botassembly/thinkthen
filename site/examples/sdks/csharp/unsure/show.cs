using ThinkThen;

await using var client = new ThinkThenClient();
var bars = new DecideOptions
{
    Threshold = Threshold.Band(0.2, 0.8),
};
try
{
    var isRefund = await client.DecideAsync(
        "Does the customer request a refund?",
        "I was charged twice for one invoice.", bars);
    Console.WriteLine(isRefund.Answer);
}
catch (UnsureException unsure)
{
    Console.WriteLine(
        $"Not sure ({unsure.Probability}): ask a person.");
}
catch (FailedException failure)
{
    Console.WriteLine($"{failure.Code}: {failure.Message}");
}
