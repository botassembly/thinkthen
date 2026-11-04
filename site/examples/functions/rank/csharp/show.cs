using ThinkThen;

await using var client = new ThinkThenClient();

// Every message, most urgent first. Each answer is a rank, starting at 1.
await foreach (var ranked in client.RankAsync(
    "How urgent is the request?", File.ReadLinesAsync("messages.txt"),
    ["routine", "urgent"]))
{
    long rank = ranked.Row;
    Console.WriteLine($"{rank} {ranked.Item}");
}
