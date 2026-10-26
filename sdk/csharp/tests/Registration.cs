using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using System.Text.Json;
using ThinkThen;

internal static partial class Program
{
    private static async Task RegistrationAndSecrets()
    {
        const string secret = "synthetic-secret-must-never-appear";
        var options = new ThinkThenOptions { Backend = Backend.Stub, ApiKey = ApiKey.From(secret) };
        foreach (string display in new[] { options.ToString()!, options.ApiKey!.ToString()!,
            JsonSerializer.Serialize(options, JsonOptions) })
            Check(!display.Contains(secret, StringComparison.Ordinal), "settings or secret diagnostics exposed an API key");
        foreach (bool logging in new[] { false, true })
        {
            var captured = new CapturedLogger();
            var services = new ServiceCollection();
            if (logging) services.AddSingleton<ILogger<ThinkThenClient>>(captured);
            services.AddThinkThen(o => { o.Backend = Backend.Stub; o.ApiKey = ApiKey.From(secret); });
            await using var provider = services.BuildServiceProvider();
            var client = provider.GetRequiredService<ThinkThenClient>();
            Check(ReferenceEquals(client, provider.GetRequiredService<ThinkThenClient>()),
                "DI registration must share one client snapshot");
            Row<bool, NoDetail> row = await client.DecideAsync("Ready?", "alpha");
            Check(row.Source.Backend == Backend.Stub, "DI options did not reach the client");
            Check(!Row(row).ToJsonString().Contains(secret, StringComparison.Ordinal), "row source leaked a key");
            await Code(ErrorCode.UsageQuestion, async () => { await client.DecideAsync("", "alpha"); });
            Check(captured.Messages.All(message => !message.Contains(secret, StringComparison.Ordinal)),
                "host logger exposed a key");
            if (logging) Check(captured.Messages.Count > 0, "registered host logger received no lifecycle or error event");
            await provider.DisposeAsync();
            // The host owns disposal of the singleton, including active-call admission.
            await Rejected(async () => { await client.DecideAsync("Ready?", "alpha"); });
        }
    }
}

internal sealed class CapturedLogger : ILogger<ThinkThenClient>
{
    internal List<string> Messages { get; } = new();
    public IDisposable? BeginScope<TState>(TState state) where TState : notnull => null;
    public bool IsEnabled(LogLevel logLevel) => true;
    public void Log<TState>(LogLevel logLevel, EventId eventId, TState state, Exception? exception,
        Func<TState, Exception?, string> formatter)
    {
        lock (Messages) Messages.Add(formatter(state, exception) + exception?.ToString());
    }
}
