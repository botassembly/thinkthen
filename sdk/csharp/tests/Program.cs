using System.Runtime.CompilerServices;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Text.Json.Serialization;
using ThinkThen;

internal static partial class Program
{
    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
    };

    private static void Check(bool condition, string message)
    {
        if (!condition) throw new Exception(message);
    }

    private static async Task Main(string[] args)
    {
        if (args is ["--registry", var contract])
        {
            try { await Registry(contract); }
            catch (Exception error) { Console.Error.WriteLine(error.Message); Environment.ExitCode = 1; }
            return;
        }
        if (args is ["--sdk-fixes", var registry]) { await SdkFixes(registry); return; }
        if (args is ["--stress-adapter"]) { await StressAdapter(); return; }
        if (args is ["--abi-refusal"])
        {
            await Code(ErrorCode.UsageAbi, async () =>
            {
                await using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
                await client.DecideAsync("Ready?", "alpha");
            });
            return;
        }
        if (args is [var construction, "--stub-only"])
        {
            await Construction(construction);
            return;
        }
        Check(args.Length == 3, "supply construction file, canonical conformance directory and loopback executable");
        await Construction(args[0]);
        await AdmissionCases(args[1]);
        await Conformance(Path.Combine(args[1], "typed"), args[2]);
        await OptionCases();
        await SavedToolbox(Path.Combine(args[1], "toolbox"));
        await HostBehavior(args[2], args[1]);
        await EncoderCauses();
        await LifetimeCases(args[2]);
        await StreamThreads();
        await StressCases(args[1]);
        await Registry(Path.Combine(Path.GetDirectoryName(args[1])!, "schemas", "contract.json"));
        Console.WriteLine("Installed C# public API consumer completed.");
    }

    private static JsonNode Row<A, D>(Row<A, D> row) =>
        JsonSerializer.SerializeToNode(row, JsonOptions)
        ?? throw new Exception("row serialized as null");

    private static async IAsyncEnumerable<T> Records<T>(IEnumerable<T> records,
        [EnumeratorCancellation] CancellationToken cancellationToken = default)
    {
        foreach (T record in records)
        {
            cancellationToken.ThrowIfCancellationRequested();
            yield return record;
            await Task.Yield();
        }
    }

    private static async Task<JsonArray> Rows<A, D>(IAsyncEnumerable<Row<A, D>> rows, JsonArray? output = null)
    {
        JsonArray result = output ?? new();
        await foreach (var row in rows) result.Add(Row(row));
        return result;
    }

    private static async Task Code(ErrorCode code, Func<Task> action)
    {
        try { await action(); }
        catch (ThinkThenException error)
        {
            Check(error.Code == code, $"expected {code}, got {error.Code}");
            return;
        }
        throw new Exception($"expected {code}");
    }

    private static T Case<T>(object row) where T : class =>
        row as T ?? throw new Exception($"expected {typeof(T).Name}, got {row.GetType().Name}");

    private static async Task Cancelled(Task task, CancellationToken token = default)
    {
        try { await task; }
        catch (OperationCanceledException error)
        {
            Check(!token.CanBeCanceled || error.CancellationToken == token,
                "cancellation must preserve the supplied effective token");
            return;
        }
        throw new Exception("operation ignored cancellation");
    }

    private static async Task Construction(string path)
    {
        await using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        JsonArray fixtures = JsonNode.Parse(await File.ReadAllTextAsync(path))!.AsArray();
        HashSet<string> functions = new(StringComparer.Ordinal);
        foreach (JsonNode? sample in fixtures)
        {
            JsonNode request = sample!["request"]!;
            functions.Add(request["question"]!["function"]!.GetValue<string>());
            JsonNode actual = await ContractRows(client, request, default, new());
            Check(JsonNode.DeepEquals(actual, sample["expected"]),
                $"construction {request["question"]!["function"]}: {actual} != {sample["expected"]}");
        }
        Check(functions.SetEquals(Enum.GetNames<Function>().Select(name => name.ToLowerInvariant())),
            "canonical construction must exercise every function through the public API");
    }
}
