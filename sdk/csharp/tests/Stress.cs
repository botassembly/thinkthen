using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Nodes;
using ThinkThen;

internal static partial class Program
{
    // Feature: installed C# streams retain every stress position under tasks and cancellation.
    private static async Task StressCases(string directory)
    {
        var start = new ProcessStartInfo("python3") { UseShellExecute = false };
        start.ArgumentList.Add(Path.Combine(directory, "..", "scripts", "stress-adapters.py"));
        start.ArgumentList.Add("--surface");
        start.ArgumentList.Add("csharp");
        start.ArgumentList.Add("--");
        start.ArgumentList.Add(Environment.ProcessPath!);
        if (Path.GetFileNameWithoutExtension(Environment.ProcessPath) == "dotnet")
            start.ArgumentList.Add(Environment.GetCommandLineArgs()[0]);
        start.ArgumentList.Add("--stress-adapter");
        using var process = Process.Start(start)!;
        await process.WaitForExitAsync();
        Check(process.ExitCode == 0, "shared C# stress adapter failed");
    }

    private static async Task<int> StressConsume(ThinkThenClient client, JsonNode sample, int call, System.Collections.Concurrent.ConcurrentDictionary<int, byte> widths)
    {
        int count = sample["records"]!.GetValue<int>(), received = 0, failed = 0;
        int basis = sample["identifier_base"]!.GetValue<int>();
        call += basis;
        var input = Enumerable.Range(0, count).Select(i => new { id = $"call-{call}-record-{basis + i}", body = $"record {i}" });
        await foreach (var row in client.DecideStreamAsync("Ready?", Records(input)))
        {
            JsonNode value = Row(row);
            widths.TryAdd(value["id"]!.GetValue<string>().Length, 0);
            Check(value["position"]!.GetValue<int>() == ++received, "stress position changed");
            Check(value["id"]!.GetValue<string>() == $"call-{call}-record-{basis + received - 1}", "stress id changed");
            if (value["status"]!.GetValue<string>() == "failed")
            {
                failed++;
                Check(JsonNode.DeepEquals(value["error"]!["code"], sample["scenario"]!["code"]), "stress error changed");
            }
            else
            {
                Check(value["status"]!.GetValue<string>() == "answered" && value["answer"]!.GetValue<bool>(), "stress answer changed");
                Check(JsonNode.DeepEquals(value["probability"], sample["probability"]), "stress probability changed");
            }
            if (sample["scenario"]!["status"]!.GetValue<string>() == "answered" || sample["scenario"]!["name"]!.GetValue<string>() == "refused")
                Check(JsonNode.DeepEquals(value["status"], sample["scenario"]!["status"]), "stress status changed");
        }
        Check(received == count && (failed > 0) == (sample["scenario"]!["status"]!.GetValue<string>() == "failed"), "stress lost rows or expected failures");
        return failed;
    }

    private static async Task StressAdapter()
    {
        JsonNode sample = JsonNode.Parse(await File.ReadAllTextAsync(Environment.GetEnvironmentVariable("THINKTHEN_STRESS_CASE")!))!;
        ThinkThenOptions options = sample["client"]!.Deserialize<ThinkThenOptions>(JsonOptions)!;
        options.ApiKey = ApiKey.From(sample["client"]!["api_key"]!.GetValue<string>());
        await using var client = new ThinkThenClient(options);
        int calls = 0, rows = 0, failed = 0;
        var widths = new System.Collections.Concurrent.ConcurrentDictionary<int, byte>();
        bool cancel = sample["cancel"]!.GetValue<bool>();
        if (cancel)
        {
            using var stop = new CancellationTokenSource();
            await using var stream = client.DecideStreamAsync("Ready?", Records(Enumerable.Repeat("body", sample["records"]!.GetValue<int>())), cancellationToken: stop.Token).GetAsyncEnumerator();
            Task<bool> pending = stream.MoveNextAsync().AsTask();
            Check(await Console.In.ReadLineAsync() == "cancel", "unknown stress control event");
            stop.Cancel();
            await Cancelled(pending, stop.Token);
            calls = 1;
        }
        else for (int round = 0; round < sample["rounds"]!.GetValue<int>(); round++)
        {
            var results = await Task.WhenAll(Enumerable.Range(0, sample["concurrency"]!.GetValue<int>()).Select(index => StressConsume(client, sample, round * sample["concurrency"]!.GetValue<int>() + index, widths)));
            foreach (int result in results) { calls++; rows += sample["records"]!.GetValue<int>(); failed += result; }
            if (Environment.GetEnvironmentVariable("THINKTHEN_STRESS_CHECKPOINT_COMMAND") is string hook)
            {
                var checkpoint = new ProcessStartInfo("/bin/sh") { UseShellExecute = false };
                checkpoint.ArgumentList.Add("-c"); checkpoint.ArgumentList.Add(hook);
                using var process = Process.Start(checkpoint)!;
                await process.WaitForExitAsync();
                Check(process.ExitCode == 0, "resource checkpoint failed");
            }
        }
        Console.WriteLine(JsonSerializer.Serialize(new { calls, rows, failed, cancelled = cancel, identifier_width_mask = widths.Keys.Aggregate(0L, (mask, width) => mask | (1L << width)) }));
    }
}
