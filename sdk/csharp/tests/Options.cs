using System.Collections.Immutable;
using System.Text.Json;
using System.Text.Json.Nodes;
using ThinkThen;

internal static partial class Program
{
    private static async Task OptionCases(string directory)
    {
        await using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        const string toolboxJson = """
            {
              "route":{"function":"choose","question":"Which team?","choices":["billing","shipping"]},
              "ready":{"function":"decide","question":"Ready?"},
              "labels":{"function":"tag","question":"Which labels?","labels":["billing","shipping"]},
              "scale":{"function":"score","question":"How urgent?","levels":["routine","urgent"]}
            }
            """;
        var toolbox = Toolbox.Parse(toolboxJson);
        ChooseQuestion selected = toolbox.Choose("route");
        Row<string, LabelsDetail> typed = await client.ChooseAsync(selected, "alpha");
        Row<string, LabelsDetail> direct = await client.ChooseAsync("Which team?", "alpha", ["billing", "shipping"]);
        Check(JsonNode.DeepEquals(Row(typed), Row(direct)), "selected question lost its choices or identity");
        var chosen = Case<Answered<string, LabelsDetail>>(typed);
        Check(chosen.Answer is "billing" or "shipping", "choice answer is not in the required list");
        Check(chosen.Detail is LabelsDetail { Labels.Count: 2 }, "choose detail must be typed label probabilities");
        ImmutableDictionary<string, double> probabilities = chosen.Detail!.Labels;
        Check(probabilities.Keys.ToHashSet().SetEquals(["billing", "shipping"]), "typed label names changed");
        await Code(ErrorCode.UsageName, () => Task.FromResult(toolbox.Choose("ready")));
        await Code(ErrorCode.UsageName, () => Task.FromResult(toolbox.Choose("absent")));

        var named = Case<Answered<string, LabelsDetail>>(await client.ChooseAsync(selected, "alpha", name: "execution"));
        Check(named.Answer == chosen.Answer && named.Source.Name == "execution", "execution name changed the answer or was lost");
        Check(named.Source.QuestionIdentity != chosen.Source.QuestionIdentity && chosen.Source.Name is null,
            "execution name did not join identity, or toolbox selection implied a name");
        await Code(ErrorCode.UsageName, async () => { await client.ChooseAsync(selected, "alpha", name: ""); });

        Row<ImmutableArray<string>, LabelsDetail> tags = await client.TagAsync("Which labels?", "alpha", ["billing", "shipping"]);
        Check(Case<Answered<ImmutableArray<string>, LabelsDetail>>(tags).Detail!.Labels.Count == 2,
            "tag must expose its own typed labels detail");
        Row<double, LevelsDetail> score = await client.ScoreAsync("How urgent?", "alpha", ["routine", "urgent"]);
        Check(Case<Answered<double, LevelsDetail>>(score).Detail!.Levels.Length == 2,
            "score must expose typed level probabilities without an aggregate probability");
        Check(Case<Answered<double, LevelsDetail>>(score).Probability is null, "score invented an aggregate probability");

        foreach (string settingsJson in new[]
        {
            """{"backend":"stub","url":"not an absolute uri"}""",
            """{"backend":"stub","batch":2147483648}""",
            """{"backend":"stub","retry":{"max_retries":4294967295}}""",
            """{"backend":"stub","retry":{"initial_backoff_ms":18446744073709551615}}"""
        })
            await Code(ErrorCode.UsageSettings, () => Task.FromResult(JsonSerializer.Deserialize<ThinkThenOptions>(settingsJson, JsonOptions)));
        await Code(ErrorCode.UsageSettings, async () =>
        {
            await using var fractional = new ThinkThenClient(new()
            { Backend = Backend.Stub, Retry = new() { InitialBackoff = TimeSpan.FromTicks(1) } });
        });

        JsonNode schema = JsonNode.Parse(await File.ReadAllTextAsync(Path.Combine(directory,"..","schemas","client.schema.json")))!;
        int maximum = schema["$defs"]!["Retry"]!["properties"]!["max_retries"]!["maximum"]!.GetValue<int>();
        using (var valid = new ThinkThenClient(new() { Backend = Backend.Stub, Retry = new() { MaxRetries = maximum } }))
            Successful(await valid.DecideAsync("Ready?","alpha"));
        foreach (int retries in new[] {-1,maximum+1})
            await Code(ErrorCode.UsageSettings, () => { using var invalid = new ThinkThenClient(new() { Backend = Backend.Stub, Retry = new() { MaxRetries = retries } }); return Task.CompletedTask; });

        // Invalid host question containers use the same engine error as invalid JSON.
        foreach (Action admit in new Action[]
        {
            () => RelateQuestion.Create([null!]),
            () => Toolbox.Create(null!),
            () => Toolbox.Create(new Dictionary<string, ToolboxQuestion>
                { ["\ud800"] = new AnnotationQuestion.Decide(DecideQuestion.Create("Ready?")) }),
            () => Toolbox.Create(new Dictionary<string, ToolboxQuestion> { ["bad"] = null! }),
            () => AnnotationQuestions.Create(null!),
            () => AnnotationQuestions.Create(new Dictionary<string, AnnotationQuestion>
                { ["\udc00"] = new AnnotationQuestion.Decide(DecideQuestion.Create("Ready?")) }),
            () => AnnotationQuestions.Create(new Dictionary<string, AnnotationQuestion> { ["bad"] = null! }),
            () => AnnotateQuestion.Create(null!),
            () => new ToolboxQuestion.Rank(null!),
            () => new AnnotationQuestion.Decide(null!),
            () => new AnnotationQuestion.Choose(null!),
            () => new AnnotationQuestion.Tag(null!),
            () => new AnnotationQuestion.Score(null!),
            () => new AnnotationQuestion.Filter(null!),
            () => new AnnotationQuestion.Find(null!),
            () => new AnnotationQuestion.Recognize(null!),
            () => new AnnotationQuestion.Relate(null!)
        })
            await Code(ErrorCode.UsageQuestion, () => { admit(); return Task.CompletedTask; });

        var rankedToolbox = Toolbox.Create(new Dictionary<string, ToolboxQuestion>
        { ["priority"] = new ToolboxQuestion.Rank(RankQuestion.Create("Urgency?", ["routine", "urgent"])) });
        int rankedCount = 0;
        await foreach (var item in client.RankAsync(rankedToolbox.Rank("priority"), Records(new[] { "alpha", "beta" }))) rankedCount++;
        Check(rankedCount == 2, "general toolbox lost typed rank selection");
        await Code(ErrorCode.UsageQuestion, () => Task.FromResult(AnnotationQuestions.Parse(
            """{"priority":{"function":"rank","question":"Urgency?","levels":["routine","urgent"]}}""")));

        var annotated = await client.Toolbox(AnnotationQuestions.Parse(toolboxJson)).AnnotateAsync("alpha");
        var answer = Case<Answered<ImmutableDictionary<string, AnnotationEntry>, NoDetail>>(annotated).Answer;
        Check(answer.Count == 4, "annotation lost named entries");
        ChooseEntry route = Case<ChooseEntry>(answer["route"]);
        Check(route.Function == Function.Choose, "annotation entry lost its function discriminator");
        // The entry body has a distinct type; its public properties cannot contain row metadata.
        Type outcome = typeof(ChooseEntry).GetProperties().Single(property =>
            property.PropertyType == typeof(EntryOutcome<string, LabelsDetail>)).PropertyType;
        Check(outcome.GetProperty("Position") is null && outcome.GetProperty("Source") is null
            && outcome.GetProperty("Id") is null, "annotation entry carries enclosing row metadata");

        var list = new[] { "billing", "shipping" };
        Task<Row<string, LabelsDetail>> pending = client.ChooseAsync("Which team?", "alpha", list);
        list[0] = "changed-billing";
        list[1] = "changed-shipping";
        var snapshot = Case<Answered<string, LabelsDetail>>(await pending);
        Check(snapshot.Answer is "billing" or "shipping", "a caller changed an admitted choice list");
        Check(snapshot.Detail!.Labels.Keys.ToHashSet().SetEquals(["billing", "shipping"]),
            "typed detail retained the caller's mutable list");
        Check(typeof(Unsure<bool, NoDetail>).GetProperty("Answer") is null, "unsure exposes an answer");
        Check(typeof(Failed<bool, NoDetail>).GetProperty("Probability") is null, "failed exposes a probability");
    }

    private static async Task HostBehavior(string loopback, string directory)
    {
        await using var monitor = await Fixture.Start(loopback, Path.Combine(directory, "typed", "decide-precision.json"));
        await using var client = new ThinkThenClient(new() { Backend = Backend.Stub, Url = new Uri(monitor.Url) });
        await using var remote = new ThinkThenClient(new()
        { Backend = Backend.SystemOne, Url = new Uri(monitor.Url), ApiKey = ApiKey.From("synthetic-loopback-key") });
        JsonElement Json(string json) => JsonSerializer.Deserialize<JsonElement>(json);
        foreach (string id in new[] { "null", "true", "{}", "[]", "1.5", "18446744073709551616", "-9223372036854775809" })
        {
            var row = await remote.DecideAsync("Ready?", Json("{\"id\":" + id + ",\"body\":\"alpha\"}"));
            Check(Case<Failed<bool, NoDetail>>(row).Error.Code == ErrorCode.RecordInvalid,
                $"invalid id {id} was admitted or rounded");
            Check(row.Position == 1 && row.Id is null, "invalid id lost its position or exposed an invalid identifier");
        }
        foreach (string id in new[] { "\"text-id\"", "-9223372036854775808", "9223372036854775807", "18446744073709551615" })
        {
            var row = await client.DecideAsync("Ready?", Json("{\"id\":" + id + ",\"body\":\"alpha\"}"));
            Check(row is not Failed<bool, NoDetail>, $"valid string/i64/u64 id {id} was lost");
            RecordId? typed = row.Id;
            Check(typed is not null, "valid identifier is absent");
            Check(Row(row)["id"]!.ToJsonString() == id, "identifier changed JSON type or precision");
        }
        foreach (string invalidText in new[] { "\ud800", "\udc00" })
            Check(await remote.DecideAsync("Ready?", invalidText) is Failed<bool, NoDetail> { Error.Code: ErrorCode.RecordInvalid },
                "bad UTF-16 record must remain a failed row");
        var absent = await client.DecideAsync("Ready?", Json("{\"body\":\"alpha\"}"));
        Check(absent.Id is null && Row(absent)["id"] is null, "missing id acquired a synthetic value");
        foreach (object? missing in new object?[] { null, Json("null") })
        {
            var row = await remote.DecideAsync("Ready?", missing);
            Check(Case<Failed<bool, NoDetail>>(row).Error.Code == ErrorCode.RecordMissing,
                "nullable input did not map to the explicit missing-record operation");
        }
        Check(Case<Failed<bool, NoDetail>>(await remote.DecideAsync<int?>("Ready?", null)).Error.Code
            == ErrorCode.RecordMissing, "generic nullable value did not map to missing input");
        Check(await monitor.Command("count") == "0", "stub or failed record contacted a service");

        var source = new object?[] { new Message("one", "alpha"), null, new Message("three", "beta") };
        int count = 0;
        await foreach (var item in client.FilterAsync("Ready?", Records(source)))
        {
            Check(ReferenceEquals(item.Item, source[checked((int)item.Row.Position - 1)]),
                "filter replaced the caller's original managed item");
            if (item.Row.Position == 2)
                Check(item.Row is Failed<bool, NoDetail> { Error.Code: ErrorCode.RecordMissing } && !item.Kept,
                    "filter discarded a missing record instead of yielding its failed row");
            count++;
        }
        Check(count == 3, "filter dropped a failed or unselected input");
        var ranks = new List<RankedItem<object?>>();
        await foreach (var item in client.RankAsync("Urgency?", Records(source), ["routine", "urgent"], new RankOptions { Top = 1 }))
            ranks.Add(item);
        Check(ranks.Count == 2 && ranks.Count(item => item.Row is Failed<long, LevelsDetail>) == 1,
            "rank top suppressed failures or retained too many successful rows");
        foreach (var item in ranks)
            Check(ReferenceEquals(item.Item, source[checked((int)item.Row.Position - 1)]), "rank lost original item identity");
        Check(ranks.Single(item => item.Row is Answered<long, LevelsDetail>).Row is Answered<long, LevelsDetail> { Answer: 1 },
            "rank answer must remain separate from original input position");
        foreach (Backend backend in new[] { Backend.SystemOne, Backend.OpenAI })
        {
            await using var offline = await Fixture.Start(loopback, "--unavailable");
            await using var unavailable = new ThinkThenClient(new()
            {
                Backend = backend, Url = new Uri(offline.Url), ApiKey = ApiKey.From("synthetic-loopback-key"),
                Retry = new RetryOptions { MaxRetries = 0 }
            });
            var failed = Case<Failed<bool, NoDetail>>(await unavailable.DecideAsync("Ready?", "alpha"));
            Check(failed.Error.Code == ErrorCode.BackendTransport && failed.Source.Backend == backend,
                "single backend failure must return a typed failed row for the selected backend");
        }
        await Code(ErrorCode.UsageQuestion, async () => { await client.ChooseAsync("Which?", "alpha", null!); });
        try { await client.DecideAsync("\ud800", "alpha"); throw new Exception("invalid question encoding was accepted"); }
        catch (ThinkThenException error)
        { Check(error.Code == ErrorCode.UsageQuestion && error.InnerException is System.Text.EncoderFallbackException, "host conversion lost its engine code or cause"); }
        var cycle = new Cycle(); cycle.Next = cycle;
        var invalid = await remote.DecideAsync("Ready?", cycle);
        Check(invalid is Failed<bool, NoDetail> { Error.Code: ErrorCode.RecordInvalid }, "host record conversion became a call fault");
        Check(await monitor.Command("count") == "0", "invalid host conversion contacted a backend");
        var owner = new ThinkThenClient(new() { Backend = Backend.Stub });
        var retained = Case<Answered<string, LabelsDetail>>(await owner.ChooseAsync("Which?", "alpha", ["first", "second"]));
        string serialized = Row(retained).ToJsonString();
        await owner.DisposeAsync();
        Parallel.For(0, 8, _ => Check(Row(retained).ToJsonString() == serialized && retained.Detail!.Labels.Count == 2,
            "row data did not survive disposal and concurrent reads"));
        await SerializerSnapshot();
        await MetadataCalls(client, new Message("metadata", "alpha"));
        await RegistrationAndSecrets();
    }

    private static async Task SerializerSnapshot()
    {
        var json = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            TypeInfoResolver = ConsumerJson.Default };
        var settings = new ThinkThenOptions { Backend = Backend.Stub, Json = json };
        await using var client = new ThinkThenClient(settings);
        // Construction must copy before freezing; the caller's serializer remains mutable.
        json.PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower;
        settings.Backend = Backend.OpenAI;
        settings.Json = new JsonSerializerOptions();
        Message value = new("m1", "alpha");
        var typed = await client.DecideAsync("Ready?", value,
            new DecideOptions { Evidence = Pointer.From("/messageBody") });
        Check(typed is not Failed<bool, NoDetail> && typed.Source.Backend == Backend.Stub,
            "client retained mutable options or failed to snapshot its serializer");
        var metadata = await client.DecideAsync("Ready?", value, ConsumerJson.Default.Message,
            new DecideOptions { Evidence = Pointer.From("/messageBody") });
        Check(JsonNode.DeepEquals(Row(typed), Row(metadata)), "explicit record metadata changed the call");
    }
}

internal sealed record Message(string Id, string MessageBody);

internal sealed class Cycle { public Cycle? Next { get; set; } }
