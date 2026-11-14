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
            Check(await CallFailed(client.DecideAsync("Ready?",blank)) is IFailed { Error.Code:"record.empty" },"blank scalar failures retain their own row");
        foreach (object collection in new object[] { new[] { "alpha", "beta" }, new List<string> { "alpha" },
            Records(new[] { "alpha" }), JsonSerializer.Deserialize<JsonElement>("[1,2]") })
        {
            try { await client.DecideAsync("Ready?", collection); throw new Exception("single call accepted a collection"); }
            catch(ArgumentException error)
            { Check(error.Message.Contains("DecideStreamAsync",StringComparison.Ordinal),"collection refusal names the ordinary stream form"); }
        }
        var structured = await client.DecideAsync("Ready?", new Dictionary<string, string> { ["body"] = "alpha" });
        Check(structured.IsAnswered, "single call refused a dictionary record");
        foreach (string text in new[] { "{}", "[]" })
            Check((await client.DecideAsync("Ready?", text)).IsAnswered, "literal JSON-looking text became structured emptiness");
        var continuing = new List<Row<bool, string>>();
        await foreach (var row in client.DecideStreamAsync("Ready?", Records(new[] { "alpha", " ", "beta" }))) continuing.Add(row);
        Check(continuing.Count == 3 && continuing[0].IsAnswered && continuing[1] is IFailed { Error.ErrorCode: ErrorCode.RecordEmpty }
            && continuing[2].IsAnswered && continuing[2].Position == 3, "blank stream row stopped or reordered later results");

        var records = Enumerable.Range(0, 64).Select(index => (object?)new Message(index.ToString(), "refund request " + index))
            .Concat(new object?[] { null, "", " \n" }).ToArray();
        var expected=new List<object?>();
        await foreach(var item in client.FilterAsync("Does it ask for a refund?",Records(records.Take(64)))) expected.Add(item);
        Check(expected.Count > 0 && expected.Count < 64, "filter witness needs kept and rejected records");
        Check(expected.All(item=>records.Take(64).Any(record=>ReferenceEquals(record,item))),"every kept value is an original managed record");
        var kept = new List<object?>();
        try
        {
            await foreach (var item in client.FilterAsync("Does it ask for a refund?", Records(records))) kept.Add(item);
            throw new Exception("kept-only filter dropped its failures");
        }
        catch(FailedException error)
        {
            Check(kept.Count==expected.Count&&kept.Zip(expected).All(pair=>ReferenceEquals(pair.First,pair.Second)),"filter preserves every kept original");
            Check(error.Rows.Count==3&&error.Rows.Select(row=>row.Position).SequenceEqual(new ulong[]{65,66,67}),"filter preserves failed rows and positions");
            Check(error.Code=="record.missing","filter preserves the first failed code");
        }
        await Code(ErrorCode.UsageThreshold,()=>Task.FromResult(Toolbox.Parse("""{"filter":{"function":"filter","question":"Ready?","threshold":{"low":0.2,"high":0.8}}}""")));

        var plain = DecideQuestion.Create("Ready?");
        var meaningful = DecideQuestion.Create("Ready?", new() { YesMeans = "Ready for release", NoMeans = "Needs more work" });
        Check((await client.DecideAsync(plain, "alpha")).Source.QuestionIdentity !=
            (await client.DecideAsync(meaningful, "alpha")).Source.QuestionIdentity, "answer meanings were lost before engine admission");
        var described = ChooseQuestion.Create("Which team?", ["billing", "shipping"],
            new() { Descriptions = new Dictionary<string, string> { ["billing"] = "Payment and refunds", ["shipping"] = "Delivery" } });
        Check(described.Choices.SequenceEqual(new[] { "billing", "shipping" }), "descriptions replaced choice names");
        Check((await client.ChooseAsync(described, "alpha")).IsAnswered, "described choices failed admission or projection");
        await Code(ErrorCode.UsageQuestion, () => Task.FromResult(ChooseQuestion.Create("Which team?", ["billing", "shipping"],
            new() { Descriptions = new Dictionary<string, string> { ["unknown"] = "Must not disappear" } })));
        var describedRank = RankQuestion.Create("Which is urgent?", ["routine", "urgent"], new()
        {
            Descriptions = new Dictionary<string, string> { ["routine"] = "Can wait", ["urgent"] = "Needs attention now" }
        });
        Check(describedRank.Levels.SequenceEqual(new[] { "routine", "urgent" }), "rank descriptions replaced level names");
        int describedRanks = 0;
        string? describedRankIdentity = null;
        await foreach (var ranked in client.RankAsync(describedRank, Records(records.Take(2).Cast<Message>())))
        {
            Check(ranked.IsAnswered && ReferenceEquals(ranked.Item,records[(int)ranked.Position-1]),
                "described rank lost its answer or original record");
            describedRankIdentity = ranked.Source.QuestionIdentity;
            describedRanks++;
        }
        Check(describedRanks == 2, "described rank lost a record");
        var filterMeanings = FilterQuestion.Create("Does it ask for a refund?", new()
            { YesMeans = "Keep refund requests", NoMeans = "Omit other messages" });
        var savedFilter=Toolbox.Create(new Dictionary<string,ToolboxQuestion>{{"refund",new ToolboxQuestion.Filter(filterMeanings)}});
        using(var saved=JsonDocument.Parse(savedFilter.ToJson())) Check(saved.RootElement.GetProperty("entries").GetProperty("refund").GetProperty("question").GetProperty("yes_means").GetString()=="Keep refund requests","filter answer meanings survive native admission and saving");
        int ranks = 0;
        await foreach (var item in client.RankAsync("Which is urgent?", Records(records.Take(5).Cast<Message>()), ["routine", "urgent"]))
        {
            Check(ReferenceEquals(item.Item,records[(int)item.Position-1]), "rank lost the original record");
            Check(item.Source.QuestionIdentity != describedRankIdentity, "rank descriptions were lost before native execution");
            ranks++;
        }
        Check(ranks == 5, "rank lost a record");
        var shortForm = new AnnotationQuestions { { "ready", plain }, { "team", described } };
        Check(shortForm.Count == 2 && shortForm.SequenceEqual(new[] { "ready", "team" }), "short form lost question names");
        var shortAnswers=(await client.AnnotateAsync(shortForm,"alpha")).Answer;
        bool ready=shortAnswers.Decide("ready").Answer;
        string team=shortAnswers.Choose("team").Answer;
        Check(shortAnswers.Entries.Count==2&&(team=="billing"||team=="shipping"),"short forms retain typed ordinary child answers");
        try { shortForm.Add("again", plain); throw new Exception("admitted form was mutable"); }
        catch (InvalidOperationException) { }
        Check(new RowError(ErrorCode.RecordMissing, "missing").Code == "record.missing", "record error lost its canonical code name");
        Check(await CallFailed(client.DecideAsync("Ready?","")) is IFailed { Error.Code:"record.empty" },"blank scalar failures retain their canonical failure code");
        await RankSnapshotCases();
        Console.WriteLine("C# native public interface cases passed.");
    }
    private static async Task RankSnapshotCases()
    {
        var originals=new List<Message>();
        async IAsyncEnumerable<Message> RecordsWithIdentity()
        {
            for(int index=0;index<64;index++) { var record=new Message(index.ToString(),new string('x',1024)); originals.Add(record); yield return record; }
            await Task.CompletedTask;
        }
        await using var client=new ThinkThenClient(new(){Backend=Backend.Stub});
        int count=0;
        await foreach(var row in client.RankAsync("Urgent?",RecordsWithIdentity(),["routine","urgent"],new(){Top=1}))
        { Check(originals.Count==64&&ReferenceEquals(row.Item,originals[checked((int)row.Position-1)]),"top rank consumes all records and retains the winning original"); count++; }
        Check(count==1,"top-one rank returns one ranked result");
        var unusual=new List<Row<long,string?>>();
        await foreach(var row in client.RankAsync("Urgent?",Records(new string?[]{"alpha","\ud800",null}),["low","high"],new(){Top=1})) unusual.Add(row);
        Check(unusual.Count==3&&unusual.Single(row=>row.Position==2) is IFailed {Error.Code:"record.invalid"}&&unusual.Single(row=>row.Position==2).Item=="\ud800"&&unusual.Single(row=>row.Position==3).Item is null,"rank retains unrepresentable and missing failed originals");
        int undefined=0;
        await foreach(var row in client.RankAsync("Urgent?",Records(new[]{default(JsonElement),JsonSerializer.Deserialize<JsonElement>("\"alpha\"")}),["low","high"]))
        { if(row.Position==1) Check(row.Item.ValueKind==JsonValueKind.Undefined&&row.IsFailed,"rank preserves undefined original values"); undefined++; }
        Check(undefined==2,"rank retains failed undefined records");
        var writeOnly=new WriteOnlyRecord("alpha");
        int written=0;
        await foreach(var row in client.RankAsync("Urgent?",new[]{writeOnly},["low","high"]))
        { Check(ReferenceEquals(row.Item,writeOnly)&&row.IsAnswered,"rank needs no inverse serializer for its original record"); written++; }
        Check(written==1,"write-only record produced one rank");
        Console.WriteLine("C# rank retains original managed objects and failed records.");
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
