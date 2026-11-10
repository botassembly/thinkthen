using ThinkThen;

internal static partial class Program
{
    private static async Task UsageCases()
    {
        var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        Usage before = client.Usage;
        await client.DecideAsync("Ready?", "alpha");
        await client.DisposeAsync();
        Usage after = client.Usage;
        await client.DisposeAsync();
        Check(before.Attempts == 0 && after.Attempts == 0 && after.Responses == 0,
            "stub usage reported remote requests");
        Check(after.CacheHits == 0 && after.ReplayHits == 0, "stub reported local reuse");
        await Code(ErrorCode.UsageSettings, () =>
        {
            using var invalid = new ThinkThenClient(new() { Backend = Backend.OpenAI,
                ResponseStore = new("unused-responses", ResponseStoreMode.Cache, "sdk-test", "generation") { MaxEntries = 0 } });
            return Task.CompletedTask;
        });
        Check(after.ActiveAttempts == 0 && after.ActiveCalls == 0,
            "disposed client retained unresolved work");
        Check(after.InputTokens == new TokenUsage(0, 0) && after.OutputTokens == new TokenUsage(0, 0),
            "stub token usage must be known zero");
    }
}
