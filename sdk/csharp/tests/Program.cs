using System.Collections;
using System.Runtime.CompilerServices;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Text.Json.Serialization;
using ThinkThen;
using Record = ThinkThen.Record;

internal static partial class Program
{
    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
    };
    private static void Check(bool condition, string message)
    { if (!condition) throw new Exception(message); }
    private static async Task Main(string[] args)
    {
        if (args.Length == 1 && args[0] == "live") { await Live(); return; }
        if (args.Length == 2 && args[1] == "--stub-only")
        {
            await Stub(args[0], "", "");
            Console.WriteLine("Packaged C#: ten shared stub cases passed without a service");
            return;
        }
        await Stub(args[0], args[1], args[2]);
        await Conformance(args[1], args[2]);
        await OptionCases(Path.Combine(Path.GetDirectoryName(args[0])!, "options.json"));
        await HostBehavior(args[2]);
        Console.WriteLine("Installed C# construction, ownership, errors and cancellation passed.");
    }
    private static JsonNode Row<T>(ResultRow<T> row)
    {
        var value = new JsonObject { ["id"] = JsonSerializer.SerializeToNode(row.Id), ["status"] = row.Status.ToString().ToLowerInvariant() };
        if (row.Status != RowStatus.Failed) value["answer"] = JsonSerializer.SerializeToNode(row.Answer, JsonOptions);
        if (row.P is double p) value["p"] = p;
        if (row.Error is not null) value["error"] = JsonSerializer.SerializeToNode(row.Error, JsonOptions);
        if (row.Detail is JsonElement detail) value["detail"] = JsonSerializer.SerializeToNode(detail);
        return value;
    }
    private static async IAsyncEnumerable<Record> Records(IReadOnlyList<Record> records)
    { foreach (var record in records) { yield return record; await Task.Yield(); } }
    private static async Task<JsonArray> Rows<T>(IAsyncEnumerable<ResultRow<T>> rows, JsonArray? output = null)
    {
        var result = output ?? new JsonArray();
        await foreach (var row in rows) result.Add(Row(row));
        return result;
    }
    private static async Task Stub(string cases, string directory, string loopback)
    {
        await using var monitor = loopback.Length == 0 ? null : await Fixture.Start(loopback, Path.Combine(directory, "decide-yes.json"));
        string? url = monitor?.Url;
        using var client = new ThinkThenClient(new() { Adapter = "stub", Url = url, Model = "ignored", Batch = 1 });
        var fixtures = JsonNode.Parse(await File.ReadAllTextAsync(cases))!.AsArray();
        Check(fixtures.Count == 10, "construction fixture must cover the ten functions");
        foreach (var fixture in fixtures)
        {
            var request = fixture!["request"]!;
            string function = request["function"]!.GetValue<string>();
            var value = request["question"]!["value"]!;
            var records = request["records"]!.AsArray().Select(node => node is JsonValue text && text.TryGetValue<string>(out var s)
                ? Record.Text(s) : Record.Json(JsonSerializer.SerializeToElement(node))).ToArray();
            Question question = function switch
            {
                "recognize" => Question.Kinds(value.Deserialize<string[]>()!),
                "relate" => Question.Relations(value.Deserialize<RelationRule[]>(JsonOptions)!),
                "annotate" => "unused",
                _ => value.GetValue<string>()
            };
            var source = Records(records);
            string[] Choices(string name) => request["options"]![name]!.Deserialize<string[]>()!;
            JsonNode actual;
            if (function == "filter")
            {
                var selected = new List<Record>();
                await foreach (var record in client.FilterAsync(question, source)) selected.Add(record);
                var expectedRecords = fixture["expected"]!.AsArray().Select((row, index) => (row, index))
                    .Where(pair => pair.row!["answer"]!.GetValue<bool>()).Select(pair => records[pair.index]).ToArray();
                Check(selected.Count == expectedRecords.Length && selected.Zip(expectedRecords).All(pair => ReferenceEquals(pair.First, pair.Second)), "filter preserves selected object identity");
                continue;
            }
            actual = function switch
            {
                "decide" => await Rows(client.DecideStreamAsync(question, source)),
                "choose" => await Rows(client.ChooseStreamAsync(question, source, Choices("options"))),
                "tag" => await Rows(client.TagStreamAsync(question, source, Choices("labels"))),
                "score" => await Rows(client.ScoreStreamAsync(question, source, Choices("levels"))),
                "rank" => await Rows(client.RankAsync(question, source)),
                "find" => await Rows(client.FindStreamAsync(question, source)),
                "annotate" => await Rows(client.AnnotateStreamAsync(JsonSerializer.SerializeToElement(value), source)),
                "recognize" => await Rows(client.RecognizeStreamAsync(question, source)),
                "relate" => await Rows(client.RelateStreamAsync(question, source)),
                _ => throw new Exception(function)
            };
            Check(JsonNode.DeepEquals(actual, fixture["expected"]), $"{function}: {actual} != {fixture["expected"]}");
            if (function == "rank") continue;
            JsonNode single = function switch
            {
                "decide" => Row(await client.DecideAsync(question, records[0])),
                "choose" => Row(await client.ChooseAsync(question, records[0], Choices("options"))),
                "tag" => Row(await client.TagAsync(question, records[0], Choices("labels"))),
                "score" => Row(await client.ScoreAsync(question, records[0], Choices("levels"))),
                "find" => Row(await client.FindAsync(question, records[0])),
                "annotate" => Row(await client.AnnotateAsync(JsonSerializer.SerializeToElement(value), records[0])),
                "recognize" => Row(await client.RecognizeAsync(question, records[0])),
                "relate" => Row(await client.RelateAsync(question, records[0])),
                _ => throw new Exception(function)
            };
            Check(JsonNode.DeepEquals(single, fixture["expected"]![0]), $"{function} single mapping");
        }
        if (monitor is not null) Check(await monitor.Command("count") == "0", "stub made an HTTP request");
    }
    private static async Task Code(string code, Func<Task> action)
    {
        try { await action(); throw new Exception($"expected {code}"); }
        catch (ThinkThenException error) { Check(error.Code == code, $"expected {code}, got {error.Code}"); }
    }
    private static async Task HostBehavior(string loopback)
    {
        using var client = new ThinkThenClient(new() { Adapter = "stub" });
        var numeric = await client.DecideAsync("Ready?", "alpha", new() { Threshold = 0.2 });
        Check(numeric.Answer == false && numeric.P == 0.03, "numeric threshold maps to its contract value");
        var ranked = await Rows(client.RankAsync("Ready?", Records(["alpha", "beta"]), new() { Top = 1 }));
        Check(ranked.Count == 1 && ranked[0]!["answer"]!.GetValue<int>() == 1, "rank top maps without retaining a second row");
        await Code("usage.threshold", async () => await client.DecideAsync("Ready?", "alpha", new() { Threshold = "bad" }));
        await Code("usage.reference", async () => await client.DecideAsync(Question.Reference("ready"), "alpha"));
        await using (var reserved = await Fixture.Start(loopback, "--unavailable"))
        {
            using var offline = new ThinkThenClient(new() { Adapter = "systemone", KeySource = "THINKTHEN_API_KEY", Url = reserved.Url });
            await Code("backend.transport", async () => await offline.DecideAsync("Ready?", "alpha"));
        }
        await using (var monitor = await Fixture.Start(loopback, "--unavailable"))
        {
            using var remote = new ThinkThenClient(new() { Adapter = "systemone", KeySource = "THINKTHEN_API_KEY", Url = monitor.Url });
            foreach (string invalid in new[] { "\ud800", "\udc00" })
            {
                await Code("usage.record", async () => await remote.DecideAsync("Ready?", invalid));
                await Code("usage.record", async () => await Rows(remote.DecideStreamAsync("Ready?", Records([invalid]))));
            }
            Check(await monitor.Command("count") == "0", "invalid C# UTF-16 never sends");
        }
        const string text = "😀 alpha\n😀 beta\n😀 gamma";
        var span = (await client.FindAsync("Which line contains the answer?", text)).Answer!;
        Check(span is not null && text[span.Start..span.End] == span.Text, "C# supplementary text and UTF-16 offsets");
        Check(span is not null && span.Start == 9 && span.End == 16 && span.Text == "😀 beta", "C# exact supplementary span");
        await DeepOwnership(client);
        await ProducerCancellation(client);
        await HeldCancellation(loopback);
        await EarlyBreak(client);
        await SourceFailures(client);
    }
    private static async Task DeepOwnership(ThinkThenClient client)
    {
        Record record;
        Question question;
        using (var document = JsonDocument.Parse("{\"id\":{\"nested\":[1,2]},\"body\":\"alpha\"}")) record = Record.Json(document.RootElement);
        using (var document = JsonDocument.Parse("{\"questions\":{\"ready\":{\"decide\":\"Ready?\"}}}")) question = Question.Toolbox(document.RootElement, "ready");
        var row = await client.DecideAsync(question, record, new() { On = "/body", Threshold = "0.4:0.6" });
        Check(record.JsonValue!.Value.GetProperty("id").GetProperty("nested").GetArrayLength() == 2, "record owns JSON");
        Check(row.Id.GetProperty("nested")[1].GetInt32() == 2, "identifier owns JSON");
        Parallel.For(0, 8, _ => Check(row.Detail!.Value.GetProperty("source").GetProperty("backend").GetString() == "stub", "detail survives concurrent reads"));
        var kinds = new[] { "thing" };
        var names = Question.Kinds(kinds);
        kinds[0] = "changed";
        var answer = await client.RecognizeAsync(names, "Ada Acme Bob Beta Carol Gamma David Delta Echo Foxtrot");
        Check(answer.Answer!.Count > 0 && answer.Answer.All(name => name.Kind == "thing"), "question owns kind array");
        Check(answer.Answer is not RecognizedName[], "answer is not a mutable array");
        try { ((IList)answer.Answer).Clear(); throw new Exception("answer list was mutable"); }
        catch (NotSupportedException) { }
        using var toolbox = JsonDocument.Parse("{\"questions\":{\"ready\":{\"decide\":\"Ready?\"}}}");
        var annotated = await client.AnnotateAsync(toolbox.RootElement, "alpha");
        Check(annotated.Answer!.Value.GetProperty("ready").GetProperty("status").GetString() == "answered", "annotate owns a typed nullable JSON answer");
    }
    private static async Task ProducerCancellation(ThinkThenClient client)
    {
        var source = new HeldSource();
        using var stop = new CancellationTokenSource();
        var enumerator = client.DecideStreamAsync("Ready?", source).GetAsyncEnumerator(stop.Token);
        Task<bool> move = enumerator.MoveNextAsync().AsTask();
        await source.Entered.Task;
        stop.Cancel();
        await Cancelled(move, stop.Token);
        await enumerator.DisposeAsync();
        Check(!source.Disposed.Task.IsCompleted, "source disposal raced MoveNextAsync");
        source.Release.SetResult();
        await source.Disposed.Task;
        Check(source.DisposalCount == 1, "source disposed exactly once after advance");
        using var already = new CancellationTokenSource(); already.Cancel();
        var untouched = new HeldSource();
        await using var cancelled = client.DecideStreamAsync("Ready?", untouched, cancellationToken: already.Token).GetAsyncEnumerator();
        await Cancelled(cancelled.MoveNextAsync().AsTask(), already.Token);
        Check(!untouched.Entered.Task.IsCompleted, "already cancelled call touched input");
    }
    private static async Task Cancelled(Task task, CancellationToken token)
    {
        try { await task; throw new Exception("operation ignored cancellation"); }
        catch (OperationCanceledException error) { Check(error.CancellationToken == token || error.CancellationToken.IsCancellationRequested, "cancellation carries the effective token"); }
    }
    private static async Task HeldCancellation(string loopback)
    {
        await using var held = await Fixture.Start(loopback, "--held", "headers");
        using var client = new ThinkThenClient(new() { Adapter = "systemone", KeySource = "THINKTHEN_API_KEY", Url = held.Url });
        using var stop = new CancellationTokenSource();
        var pending = client.DecideAsync("Ready?", "alpha", cancellationToken: stop.Token);
        Check(await held.Command("arrived") == "arrived", "held request did not arrive");
        client.Dispose(); // Calls retain the engine after client disposal.
        stop.Cancel();
        await Cancelled(pending, stop.Token);
        Check(await held.Command("closed") == "closed", "cancel did not close the held request");
    }
    private static async Task EarlyBreak(ThinkThenClient client)
    {
        var disposed = new TaskCompletionSource(TaskCreationOptions.RunContinuationsAsynchronously);
        async IAsyncEnumerable<Record> Input()
        {
            try { yield return "alpha"; await Task.Yield(); yield return "beta"; }
            finally { disposed.SetResult(); }
        }
        await foreach (var row in client.DecideStreamAsync("Ready?", Input())) { Check(row.Status == RowStatus.Answered, "first stream row"); break; }
        await disposed.Task;
        using var closed = new ThinkThenClient(new() { Adapter = "stub" }); closed.Dispose();
        try { await closed.DecideAsync("Ready?", "alpha"); throw new Exception("disposed client accepted a call"); }
        catch (ObjectDisposedException) { }
    }
    private static async Task SourceFailures(ThinkThenClient client)
    {
        var primary = new InvalidOperationException("advance failed");
        var cleanup = new InvalidOperationException("dispose failed");
        foreach (bool advancing in new[] { true, false })
        {
            try { await Rows(client.DecideStreamAsync("Ready?", new ErrorSource(advancing ? primary : null, cleanup))); throw new Exception("source error was lost"); }
            catch (InvalidOperationException error) { Check(ReferenceEquals(error, advancing ? primary : cleanup), "cleanup changed the first failure"); }
        }
        var failure = new InvalidOperationException("input failed");
        async IAsyncEnumerable<Record> Input()
        { await Task.Yield(); if (failure is not null) throw failure; yield break; }
        try { await Rows(client.DecideStreamAsync("Ready?", Input())); throw new Exception("source failure lost"); }
        catch (InvalidOperationException error) { Check(ReferenceEquals(error, failure), "source exception changed"); }
    }
    private static async Task Live()
    {
        using var client = new ThinkThenClient();
        Console.WriteLine(Row(await client.DecideAsync("Does the customer request a refund?", "Please refund my order.")));
        Console.WriteLine(Row(await client.ChooseAsync("What is the topic?", "Please refund my order.", ["billing", "shipping"])));
        Console.WriteLine(Row(await client.TagAsync("Which topics apply?", "Please refund my order.", ["refund", "delivery"])));
        Console.WriteLine(Row(await client.ScoreAsync("How urgent is it?", "Routine update.", ["routine", "urgent"])));
        await foreach (var record in client.FilterAsync("Does the customer request a refund?", Records(["Please refund my order."]))) Console.WriteLine(record.TextValue);
        Console.WriteLine(await Rows(client.RankAsync("How urgent is it?", Records(["Routine update.", "Urgent help needed."]))));
        Console.WriteLine(Row(await client.FindAsync("Who works for Acme?", "Ada works for Acme.\nBob works for Beta.")));
        using var toolbox = JsonDocument.Parse("{\"questions\":{\"refund\":{\"decide\":\"Does the customer request a refund?\"}}}");
        Console.WriteLine(Row(await client.AnnotateAsync(toolbox.RootElement, "Please refund my order.")));
        Console.WriteLine(Row(await client.RecognizeAsync(Question.Kinds("person", "company"), "Ada works for Acme.")));
        Console.WriteLine(Row(await client.RelateAsync(Question.Relations([new("works_for", "person", "company")]), "Ada works for Acme.")));
    }
}

internal sealed class HeldSource : IAsyncEnumerable<Record>, IAsyncEnumerator<Record>
{
    internal TaskCompletionSource Entered { get; } = new(TaskCreationOptions.RunContinuationsAsynchronously);
    internal TaskCompletionSource Release { get; } = new(TaskCreationOptions.RunContinuationsAsynchronously);
    internal TaskCompletionSource Disposed { get; } = new(TaskCreationOptions.RunContinuationsAsynchronously);
    internal int DisposalCount { get; private set; }
    public Record Current => "alpha";
    public IAsyncEnumerator<Record> GetAsyncEnumerator(CancellationToken cancellationToken = default) => this;
    public async ValueTask<bool> MoveNextAsync() { Entered.TrySetResult(); await Release.Task; return true; }
    public ValueTask DisposeAsync() { DisposalCount++; Disposed.SetResult(); return ValueTask.CompletedTask; }
}

internal sealed class ErrorSource(Exception? advance, Exception cleanup) : IAsyncEnumerable<Record>, IAsyncEnumerator<Record>
{
    public Record Current => "unused";
    public IAsyncEnumerator<Record> GetAsyncEnumerator(CancellationToken cancellationToken = default) => this;
    public ValueTask<bool> MoveNextAsync() => advance is null ? ValueTask.FromResult(false) : throw advance;
    public ValueTask DisposeAsync() => throw cleanup;
}
