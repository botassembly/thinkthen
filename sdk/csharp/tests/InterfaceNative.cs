using System.Text.Json;
using System.Text.Json.Serialization;
using ThinkThen;

internal static partial class Program
{
    private static async Task InterfaceNativeCases()
    {
        DescriptionAdmissionCases();
        string? backend = Environment.GetEnvironmentVariable("THINKTHEN_BACKEND");
        try
        {
            Environment.SetEnvironmentVariable("THINKTHEN_BACKEND", "stub");
            await using (var defaultClient = new ThinkThenClient())
                Check((await defaultClient.DecideAsync("Ready?", "alpha")).Source.Backend == Backend.Stub,
                    "default client ignored THINKTHEN_BACKEND");
            Environment.SetEnvironmentVariable("THINKTHEN_BACKEND", "unrecognized-for-this-test");
            await using (var explicitClient = new ThinkThenClient(new() { Backend = Backend.Stub }))
                Check((await explicitClient.DecideAsync("Ready?", "alpha")).Source.Backend == Backend.Stub,
                    "environment overrode an explicit backend");
        }
        finally { Environment.SetEnvironmentVariable("THINKTHEN_BACKEND", backend); }

        await using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        foreach (object blank in new object[] { "", " \n", new Dictionary<string, object>(), new { id = "empty" } })
            await Code(ErrorCode.RecordEmpty, async () => { await client.DecideAsync("Ready?", blank); });
        foreach (object collection in new object[] { new[] { "alpha", "beta" }, new List<string> { "alpha" },
            Records(new[] { "alpha" }), JsonSerializer.Deserialize<JsonElement>("[1,2]") })
        {
            try { await client.DecideAsync("Ready?", collection); throw new Exception("single call accepted a collection"); }
            catch (ThinkThenException error)
            {
                Check(error.ErrorCode == ErrorCode.UsageRecordKind && error.Message.Contains("StreamAsync", StringComparison.Ordinal),
                    "collection refusal lost code or stream hint");
            }
        }
        var structured = await client.DecideAsync("Ready?", new Dictionary<string, string> { ["body"] = "alpha" });
        Check(structured.IsAnswered, "single call refused a dictionary record");
        foreach (string text in new[] { "{}", "[]" })
            Check((await client.DecideAsync("Ready?", text)).IsAnswered, "literal JSON-looking text became structured emptiness");
        var continuing = new List<Row<bool, NoDetail>>();
        await foreach (var row in client.DecideStreamAsync("Ready?", Records(new[] { "alpha", " ", "beta" }))) continuing.Add(row);
        Check(continuing.Count == 3 && continuing[0].IsAnswered && continuing[1] is Failed<bool, NoDetail> { Error.ErrorCode: ErrorCode.RecordEmpty }
            && continuing[2].IsAnswered && continuing[2].Position == 3, "blank stream row stopped or reordered later results");

        var records = Enumerable.Range(0, 64).Select(index => (object?)new Message(index.ToString(), "refund request " + index))
            .Concat(new object?[] { null, "", " \n" }).ToArray();
        var all = new List<FilterItem<object?>>();
        await foreach (var item in client.FilterStreamAsync("Does it ask for a refund?", Records(records))) all.Add(item);
        var expected = all.Where(item => item.Kept).Select(item => item.Item).ToList();
        var failed = all.Where(item => item.Row.IsFailed).Select(item => item.Row).ToList();
        Check(expected.Count > 0 && expected.Count < 64 && failed.Count == 3, "filter witness needs kept, rejected and failed records");
        var kept = new List<object?>();
        try
        {
            await foreach (var item in client.FilterAsync("Does it ask for a refund?", Records(records))) kept.Add(item);
            throw new Exception("kept-only filter dropped its failures");
        }
        catch (ThinkThenException error)
        {
            Check(kept.Count == expected.Count && kept.Zip(expected).All(pair => ReferenceEquals(pair.First, pair.Second)),
                "kept-only filter changed or dropped a kept input record");
            Check(error.FailedRows.Count == failed.Count && error.FailedRows.Select(row => row.Position).SequenceEqual(failed.Select(row => row.Position)),
                "filter exception lost failed rows or positions");
            Check(error.ErrorCode == ((Failed<bool, NoDetail>)failed[0]).Error.ErrorCode, "filter exception lost the first failure code");
        }
        var band = new FilterOptions { Threshold = Threshold.Band(0.2, 0.8) };
        var bandExpected = new List<object?>();
        int uncertain = 0;
        await foreach (var item in client.FilterStreamAsync("Does it ask for a refund?", Records(records.Take(64)), band))
        {
            if (item.Kept) bandExpected.Add(item.Item);
            if (item.Row.IsUnsure) uncertain++;
        }
        var bandActual = new List<object?>();
        await foreach (var item in client.FilterAsync("Does it ask for a refund?", Records(records.Take(64)), band)) bandActual.Add(item);
        Check(uncertain > 0 && bandActual.Count == bandExpected.Count && bandActual.Zip(bandExpected).All(pair => ReferenceEquals(pair.First, pair.Second)),
            "kept-only filter retained uncertain records");

        using var plain = DecideQuestion.Create("Ready?");
        using var meaningful = DecideQuestion.Create("Ready?", new() { YesMeans = "Ready for release", NoMeans = "Needs more work" });
        Check((await client.DecideAsync(plain, "alpha")).Source.QuestionIdentity !=
            (await client.DecideAsync(meaningful, "alpha")).Source.QuestionIdentity, "answer meanings were lost before engine admission");
        using var described = ChooseQuestion.Create("Which team?", ["billing", "shipping"],
            new() { Descriptions = new Dictionary<string, string> { ["billing"] = "Payment and refunds", ["shipping"] = "Delivery" } });
        Check(described.Choices.SequenceEqual(new[] { "billing", "shipping" }), "descriptions replaced choice names");
        Check((await client.ChooseAsync(described, "alpha")).IsAnswered, "described choices failed admission or projection");
        await Code(ErrorCode.UsageQuestion, () => Task.FromResult(ChooseQuestion.Create("Which team?", ["billing", "shipping"],
            new() { Descriptions = new Dictionary<string, string> { ["unknown"] = "Must not disappear" } })));
        using var describedRank = RankQuestion.Create("Which is urgent?", ["routine", "urgent"], new()
        {
            Descriptions = new Dictionary<string, string> { ["routine"] = "Can wait", ["urgent"] = "Needs attention now" }
        });
        Check(describedRank.Levels.SequenceEqual(new[] { "routine", "urgent" }), "rank descriptions replaced level names");
        int describedRanks = 0;
        string? describedRankIdentity = null;
        await foreach (var ranked in client.RankAsync(describedRank, Records(records.Take(2).Cast<Message>())))
        {
            Check(ranked.Row.IsAnswered && ranked.Item.Equals(records[(int)ranked.Row.Position - 1]) && !ReferenceEquals(ranked.Item, records[(int)ranked.Row.Position - 1]),
                "described rank lost its answer or original record");
            describedRankIdentity = ranked.Row.Source.QuestionIdentity;
            describedRanks++;
        }
        Check(describedRanks == 2, "described rank lost a record");
        using var filterMeanings = FilterQuestion.Create("Does it ask for a refund?", new()
            { YesMeans = "Keep refund requests", NoMeans = "Omit other messages" });
        string? meaningIdentity = null;
        await foreach (var item in client.FilterStreamAsync(filterMeanings, Records(new[] { "alpha" }))) meaningIdentity = item.Row.Source.QuestionIdentity;
        await foreach (var item in client.FilterStreamAsync("Does it ask for a refund?", Records(new[] { "alpha" })))
            Check(meaningIdentity is not null && item.Row.Source.QuestionIdentity != meaningIdentity, "filter answer meanings were lost");
        int ranks = 0;
        await foreach (var item in client.RankAsync("Which is urgent?", Records(records.Take(5).Cast<Message>()), ["routine", "urgent"]))
        {
            Check(item.Item.Equals(records[(int)item.Row.Position - 1]) && !ReferenceEquals(item.Item, records[(int)item.Row.Position - 1]), "rank lost the original record");
            Check(item.Row.Source.QuestionIdentity != describedRankIdentity, "rank descriptions were lost before native execution");
            ranks++;
        }
        Check(ranks == 5, "rank lost a record");
        using var shortForm = new AnnotationQuestions { { "ready", plain }, { "team", described } };
        Check(shortForm.Count == 2 && shortForm.SequenceEqual(new[] { "ready", "team" }), "short form lost question names");
        ValueMap<string, AnnotationEntry> shortAnswers = await client.AnnotateAsync(shortForm, "alpha");
        bool ready = shortAnswers["ready"];
        string team = shortAnswers["team"];
        Check(shortAnswers.Count == 2 && (team == "billing" || team == "shipping"), "short form lost typed child answers");
        try { shortForm.Add("again", plain); throw new Exception("admitted form was mutable"); }
        catch (InvalidOperationException) { }
        Check(new RowError(ErrorCode.RecordMissing, "missing").Code == "record.missing", "record error lost its canonical code name");
        try { await client.DecideAsync("Ready?", ""); }
        catch (ThinkThenException error) { Check(error.Code == "record.empty", "exception lost its canonical code name"); }
        await RankSnapshotCases();
        Console.WriteLine("C# native public interface cases passed.");
    }
    private static async Task RankSnapshotCases()
    {
        const long maximumGrowth = 32 * 1024 * 1024;
        long baseline = 0;
        async IAsyncEnumerable<Message> LargeRecords()
        {
            for (int index = 0; index < 2048; index++)
            {
                if (index == 256) baseline = GC.GetTotalMemory(forceFullCollection: true);
                if (index == 1792)
                    Check(GC.GetTotalMemory(forceFullCollection: true) - baseline < maximumGrowth,
                        "top-one rank retains unselected host records");
                yield return new Message(index.ToString(), new string('x', 32768));
            }
            await Task.CompletedTask;
        }
        await using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        int count = 0;
        await foreach (var ranked in client.RankAsync("Which message is urgent?", LargeRecords(), ["routine", "urgent"], new() { Top = 1 }))
        {
            Check(ranked.Item.Id == ((int)ranked.Row.Position - 1).ToString() && ranked.Item.MessageBody.Length == 32768,
                "rank snapshot lost typed record properties");
            count++;
        }
        Check(count == 1, "top-one rank returned the wrong number of records");
        var unusual = new List<RankedItem<string?>>();
        await foreach (var item in client.RankAsync("Urgent?", Records(new string?[] { "alpha", "\ud800", null }), ["low", "high"], new() { Top = 1 })) unusual.Add(item);
        Check(unusual.Count == 3 && unusual.Single(item => item.Row.Position == 2).Item == "\ud800" &&
            unusual.Single(item => item.Row.Position == 2).Row is Failed<long, LevelsDetail> { Error.Code: "record.invalid" } &&
            unusual.Single(item => item.Row.Position == 3).Item is null,
            "rank lost an unrepresentable or missing failed input");
        int undefined = 0;
        await foreach (var item in client.RankAsync("Urgent?", Records(new[] { default(JsonElement), JsonSerializer.Deserialize<JsonElement>("\"alpha\"") }), ["low", "high"]))
        {
            if (item.Row.Position == 1) Check(item.Item.ValueKind == JsonValueKind.Undefined && item.Row.IsFailed, "rank converted undefined to JSON null");
            undefined++;
        }
        Check(undefined == 2, "rank lost a failed undefined record");
        string loopback = Environment.GetEnvironmentVariable("THINKTHEN_LOOPBACK") ?? throw new Exception("prepare loopback for rank conversion proof");
        await using var monitor = await Fixture.Start(loopback, "--stress", "healthy");
        await using var remote = new ThinkThenClient(new()
        {
            Backend = Backend.SystemOne, Url = new Uri(monitor.Url), ApiKey = ApiKey.From("synthetic-loopback-key"),
            Retry = new() { MaxRetries = 0 }
        });
        try
        {
            await foreach (var _ in remote.RankAsync("Urgent?", Records(new[] { new WriteOnlyRecord("alpha") }), ["low", "high"])) { }
            throw new Exception("rank accepted a write-only record converter");
        }
        catch (JsonException error)
        {
            Check(error.Message.Contains(nameof(WriteOnlyRecord), StringComparison.Ordinal) && error.InnerException is NotSupportedException,
                "rank conversion error lost its target type or cause");
        }
        int empty = 0;
        await foreach (var _ in remote.RankAsync("Urgent?", Records(Array.Empty<WriteOnlyRecord>()), ["low", "high"])) empty++;
        Check(empty == 0 && await monitor.Command("count") == "0", "rank preflight or empty input sent a backend request");
        Console.WriteLine("C# bounded typed rank snapshot and zero-request conversion refusal passed.");
    }
}

[JsonConverter(typeof(WriteOnlyRecordConverter))]
internal sealed record WriteOnlyRecord(string Body);
internal sealed class WriteOnlyRecordConverter : JsonConverter<WriteOnlyRecord>
{
    public override WriteOnlyRecord Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) =>
        throw new NotSupportedException("This converter cannot read records.");
    public override void Write(Utf8JsonWriter writer, WriteOnlyRecord value, JsonSerializerOptions options) =>
        writer.WriteStringValue(value.Body);
}
