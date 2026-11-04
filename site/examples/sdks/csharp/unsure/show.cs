using ThinkThen;

await using var client = new ThinkThenClient();
var bars = new DecideOptions
{
    Threshold = Threshold.Band(0.2, 0.8),
};
try
{
    bool isRefund = await client.DecideAsync(
        "Does the customer request a refund?",
        "I was charged twice for one invoice.", bars);
    Console.WriteLine(isRefund);
}
catch (UnsureException unsure)
{
    Console.WriteLine(
        $"Not sure ({unsure.Probability}): ask a person.");
}
catch (ThinkThenException failure)
{
    Console.WriteLine($"{failure.Code}: {failure.Message}");
}
