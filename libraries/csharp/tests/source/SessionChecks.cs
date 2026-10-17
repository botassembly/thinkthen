using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;
using System.Text.Json;
static class SessionChecks
{
    static object? PublicValue(object? value) {
        if (value is null || value is string || value.GetType().IsPrimitive || value.GetType().IsEnum) return value;
        if (value is JsonElement json) return json.Clone();
        var type = value.GetType();
        if (type.IsGenericType && type.GetGenericTypeDefinition() == typeof(Presence<>)) {
            var state = (PresenceState)type.GetProperty("State")!.GetValue(value)!;
            return new { State = state, Value = state == PresenceState.Value ? PublicValue(type.GetProperty("Value")!.GetValue(value)) : null };
        }
        if (value is System.Collections.IDictionary dictionary) return dictionary.Keys.Cast<string>().ToDictionary(key => key, key => PublicValue(dictionary[key]));
        if (value is System.Collections.IEnumerable sequence) return sequence.Cast<object?>().Select(PublicValue).ToArray();
        return type.GetProperties(System.Reflection.BindingFlags.Public | System.Reflection.BindingFlags.Instance).ToDictionary(property => property.Name, property => PublicValue(property.GetValue(value)));
    }
    static readonly JsonSerializerOptions SnapshotOptions = new() { MaxDepth = int.MaxValue };
    static string FactsSnapshot(Facts facts) => JsonSerializer.Serialize(PublicValue(facts), SnapshotOptions);
    static string RowSnapshot(SessionPacketDecideRow row) => JsonSerializer.Serialize(PublicValue(row), SnapshotOptions);
    public static async Task Usage(string mode)
    {
        var engine = Engine.Open(new EngineSettings { Cache = new CacheDocumentDisabledCache { Value = new DisabledCache() }, MaxRetries = 0 });
        var initial = engine.UsagePersistence();
        var expected = mode == "disabled" ? UsagePersistenceState.Disabled : UsagePersistenceState.Written;
        if (initial.State != expected || initial.Advice is not null) throw new Exception("initial persistence");
        OwnedCall? call = null;
        string? facts = null;
        if (mode != "disabled")
        {
            call = await engine.DecideAsync(Question, new RequestInputText { Text = "consumer-csharp" });
            facts = FactsSnapshot(call.Terminal.Facts.Value);
            if (call.Packets.OfType<SessionPacketDecideRow>().Single().Value.Answer is not AnswerYesNo { Probability: .9 }) throw new Exception("persistence lost answer");
            if (mode == "failed" && (engine.UsagePersistence().State != UsagePersistenceState.Pending || call.Terminal.Facts.Value.UsagePersistence.Value.State != ThinkThen.Results.UsagePersistence.Pending)) throw new Exception("held persistence");
        }
        var finished = engine.FinishUsageStatus();
        if (mode == "failed") expected = UsagePersistenceState.Failed;
        if (finished.State != expected || engine.UsagePersistence() != finished || engine.FinishUsageStatus() != finished) throw new Exception("latched persistence");
        if (finished.Advice != (mode == "failed" ? "check the usage folder permissions and free space" : null)) throw new Exception("safe native advice");
        engine.Dispose();
        if (finished.State != expected || (call is not null && FactsSnapshot(call.Terminal.Facts.Value) != facts)) throw new Exception("persistence ownership or facts changed");
        try { _ = engine.UsagePersistence(); throw new Exception("disposed engine observed"); }
        catch (ObjectDisposedException) { }
        Console.WriteLine("INSTALLED_CSHARP_USAGE_PASS");
    }
    public static void ImmediateDiagnostics(Engine engine)
    {
        try {
            using var refused = engine.StartSession(new Request { Schema = new RequestVersionThinkthenRequest1(), Call = new RequestCall { Function = new RequestFunctionDecide(), Question = new RequestQuestionText { Text = "" }, Input = new RequestInputText { Text = "unreachable" } } });
            throw new Exception("empty question admitted");
        }
        catch (ThinkThen.Failure error) {
            if (error.Kind != ThinkThen.FailureKind.Usage || error.Retryable || error.Facts is not null || error.Message != "a question is text, not white space") throw;
        }
        Console.WriteLine("INSTALLED_CSHARP_IMMEDIATE_DIAGNOSTICS_PASS");
    }
    static RequestQuestionText Question => new() { Text = "Is it?" };
    static Request Request(RequestInput input) => new() { Schema = new RequestVersionThinkthenRequest1(), Call = new RequestCall { Function = new RequestFunctionDecide(), Question = Question, Input = input, Options = new RequestOptions { Batch = new RequestBatchInteger { Value = 1 } } } };
    static RequestSessionDescriptor Item(string text) => new() { Item = new RequestItem { Original = new RequestOriginalText { Text = text } } };
    static async Task Arrived(string name)
    {
        using var timeout = new CancellationTokenSource(TimeSpan.FromSeconds(20));
        while (!File.Exists("barrier/arrived-" + name)) await Task.Delay(10, timeout.Token);
    }
    static async IAsyncEnumerable<RequestSessionDescriptor> FailedFeed(Exception failure)
    {
        await Task.Yield();
        yield return failure is null ? Item("unreachable") : throw failure;
    }
    static Plan AuthoredPlan(Engine engine, RequestQuestionDefinition question, string original)
    {
        return engine.Plan(new Request { Schema = new RequestVersionThinkthenRequest1(), Call = new RequestCall { Function = new RequestFunctionDecide(), Question = question, Input = new RequestInputText { Text = "owned" } } });
    }
    sealed class ManagedThreads : System.Diagnostics.Tracing.EventListener {
        private int starts;
        public int Starts => Volatile.Read(ref starts);
        protected override void OnEventSourceCreated(System.Diagnostics.Tracing.EventSource source) {
            if (source.Name == "Microsoft-Windows-DotNETRuntime") EnableEvents(source, System.Diagnostics.Tracing.EventLevel.Verbose, (System.Diagnostics.Tracing.EventKeywords)0x10000);
        }
        protected override void OnEventWritten(System.Diagnostics.Tracing.EventWrittenEventArgs observed) {
            if (observed.EventId == 85) Interlocked.Increment(ref starts);
        }
    }
    static async Task WorkerOwnership(Engine engine) {
        using var threads = new ManagedThreads();
        int before = threads.Starts;
        var session = engine.StartSession(Request(new RequestInputFeed { Name = "worker-lifetime" }));
        const int turns = 16;
        try {
            for (int turn = 0; turn < turns; turn++) {
                var pending = session.ReadAsync();
                while (pending.IsCompleted) {
                    if (await pending is null) throw new Exception("feed ended before finish");
                    pending = session.ReadAsync();
                }
                await Task.Yield();
                if (!await session.PushAsync(Item("worker-lifetime-" + turn))) throw new Exception("worker input closed");
                while (await pending is not SessionPacketDecideRow) pending = session.ReadAsync();
            }
            session.Finish();
            while (await session.ReadAsync() is not null) { }
        }
        finally { session.Dispose(); }
        // Runtime delivery is asynchronous; let its bounded event batch arrive.
        await Task.Delay(100);
        if (threads.Starts - before >= turns) throw new Exception("session operations churn managed worker threads: " + (threads.Starts - before));
        try { await session.ReadAsync(); throw new Exception("disposed session retained work"); }
        catch (ObjectDisposedException) { }
    }
    static async Task<Task<SessionPacket?>> BlockedDrain(OwnedSession session, CancellationToken token = default)
    {
        while (true)
        {
            var pending = session.ReadAsync(token);
            try { if (await pending.WaitAsync(TimeSpan.FromMilliseconds(100)) is null) throw new Exception("held provider settled before release"); }
            catch (TimeoutException) { return pending; }
        }
    }
    static async Task HeldDrainDisposal(Engine engine)
    {
        using var session = engine.StartSession(Request(new RequestInputText { Text = "hold-session-dispose" }));
        await Arrived("hold-session-dispose");
        session.Cancel();
        try
        {
            // Each fresh reader can drain after the preceding reader was cancelled.
            for (int attempt = 0; attempt < 3; attempt++)
            {
                using var cancellation = new CancellationTokenSource();
                var pending = await BlockedDrain(session, cancellation.Token);
                cancellation.Cancel();
                try { await pending.WaitAsync(TimeSpan.FromSeconds(2)); throw new Exception("cancelled drain returned"); }
                catch (OperationCanceledException) { }
            }
            var disposed = await BlockedDrain(session);
            await Task.Run(session.Dispose).WaitAsync(TimeSpan.FromSeconds(2));
            try { await disposed.WaitAsync(TimeSpan.FromSeconds(2)); throw new Exception("disposed drain returned"); }
            catch (OperationCanceledException) { }
            if (File.Exists("barrier/release-hold-session-dispose")) throw new Exception("drain disposal waited for provider release");
        }
        finally { File.WriteAllText("barrier/release-hold-session-dispose", ""); }
    }
    public static async Task Run()
    {
        using var engine = Engine.Open(new EngineSettings {
            Cache = new CacheDocumentDisabledCache { Value = new DisabledCache() },
            Batch = new RequestBatchInteger { Value = 1 }, MaxRetries = 0,
            MaxRequests = new EngineSettingsMaxRequestsNull(),
            MaxRequestsTotal = new EngineSettingsMaxRequestsTotalNull(),
            MaxEstimatedInputTokensTotal = new EngineSettingsMaxEstimatedInputTokensTotalNull()
        });
        try { await engine.DecideAsync(Question, (IAsyncEnumerable<RequestSessionDescriptor>)null!); throw new Exception("null caller feed admitted"); }
        catch (ArgumentNullException error) when (error.ParamName == "feed") { }
        var authored = FixtureQuestions.Parse(AuthoredQuestionKind.Atomic,"{\"choose\":\"Which?\",\"options\":{\"first\":\"First.\",\"second\":\"Second.\"}}");
        var choices = ((AuthoredOptionsObject)((RequestDefinitionAuthoredChoose)authored.Value).Options.Value).Value;
        if (!choices.Select(entry => entry.Key).SequenceEqual(new[] { "first", "second" })) throw new Exception("native authored order");
        try { ((IList<KeyValuePair<string,AuthoredDescription>>)choices).Clear(); throw new Exception("parsed authored map changed"); }
        catch (NotSupportedException) { }
        var authoredPlan = engine.Plan(new Request { Schema = new RequestVersionThinkthenRequest1(), Call = new RequestCall { Function = new RequestFunctionChoose(), Question = authored, Input = new RequestInputText { Text = "owned" } } });
        if (authoredPlan.Requests != 1 || authoredPlan.FirstBodyUtf8.State != PresenceState.Value || !authoredPlan.FirstBodyUtf8.Value.Contains("First.")) throw new Exception("owned authored definition");
        foreach (var (batch, schema) in new[] { ("max", "string"), (@"\u006dax", "string"), ("max", @"\u0073tring"), (@"\u006dax", @"\u0073tring") })
        {
            string original = "{\"item_schema\":{\"type\":\"" + schema + "\"},\"decide\":\"Is it?\",\"batch\":\"" + batch + "\",\"true\":{\"future\":{\"second\":2,\"first\":1}}}";
            var parsed = FixtureQuestions.Parse(AuthoredQuestionKind.Atomic, original);
            var definition = ((RequestDefinitionAuthoredDecide)parsed.Value);
            if (definition.Batch.Value is not AuthoredQuestionBatchMax || definition.ItemSchema.Value is not AuthoredInputDeclarationString) throw new Exception("decoded authored constant types");
            var plan = AuthoredPlan(engine, parsed, original);
            if (plan.Requests != 1 || plan.FirstBodyUtf8.State != PresenceState.Value) throw new Exception("decoded authored constant plan");
        }
        RequestQuestionDefinition? deepQuestion = null;
        string deepMeaning = "";
        foreach (int depth in new[] { 70, 100 })
        {
            deepMeaning = new string('[', depth) + "false" + new string(']', depth);
            string original = "{\"decide\":\"Is it?\",\"true\":" + deepMeaning + "}";
            deepQuestion = FixtureQuestions.Parse(AuthoredQuestionKind.Atomic, original);
            var plan = AuthoredPlan(engine, deepQuestion, original);
            if (plan.Requests != 1) throw new Exception("deep authored plan");
        }
        _ = FixtureQuestions.Parse(AuthoredQuestionKind.Set, "{\"version\":1,\"questions\":{\"first\":{\"decide\":\"Is it?\"}}}");
        var preview = engine.Plan(Request(new RequestInputText { Text = "café\0preview" }));
        if (preview.Records != 1 || preview.Requests != 1 || preview.FirstBodyUtf8.State != PresenceState.Value ||
            !preview.FirstBodyUtf8.Value.Contains("café") || preview.EstimatedInputTokens.Lower > preview.EstimatedInputTokens.Upper)
            throw new Exception("typed canonical plan");
        foreach (var invalid in new[] { new EngineSettings { Throttle = 0 }, new EngineSettings { Backend = "unknown-backend" }, new EngineSettings { Model = (string)null! } })
        {
            try { using var refused = Engine.Open(invalid); throw new Exception("invalid typed settings admitted"); }
            catch (ThinkThen.Failure error) when (error.Kind == ThinkThen.FailureKind.Usage && error.Facts is null) { }
        }
        await WorkerOwnership(engine);
        await HeldDrainDisposal(engine);
        ImmediateDiagnostics(engine);
        foreach (RequestReaderFailure failure in new RequestReaderFailure[] {
            new RequestReaderFailureIo(),
            new RequestReaderFailureUtf8 { Location = new SessionSourceLocation { File = "private-file", FirstLine = 1, LastLine = 2 } },
            new RequestReaderFailureInvalidInput { Location = new SessionSourceLocation { File = "private-file" } }
        })
        {
            using var reader = engine.StartSession(Request(new RequestInputFeed { Name = "reader" }));
            reader.Finish(failure);
            SessionPacketTerminal? terminal = null;
            while (await reader.ReadAsync() is { } packet) if (packet is SessionPacketTerminal done) terminal = done;
            if (terminal is null || terminal.Failure.State != PresenceState.Value || terminal.Failure.Value.Error.Kind.Value != (failure is RequestReaderFailureIo ? "local" : "usage")) throw new Exception("typed reader failure kind");
        }
        using (var reader = engine.StartSession(Request(new RequestInputFeed { Name = "invalid-location" })))
        {
            try { reader.Finish(new RequestReaderFailureIo { Location = (SessionSourceLocation)null! }); throw new Exception("explicit null location admitted"); }
            catch (ThinkThen.Failure error) when (error.Kind == ThinkThen.FailureKind.Usage && error.Facts is null) { }
            try { reader.Finish(new RequestReaderFailureUtf8 { Location = new SessionSourceLocation { File = "private-file", FirstLine = 1 } }); throw new Exception("half location admitted"); }
            catch (ThinkThen.Failure error) when (error.Kind == ThinkThen.FailureKind.Usage && error.Facts is null) { }
            reader.Finish();
        }
        foreach (Exception problem in new Exception[] { new IOException("host-secret"), new System.Text.DecoderFallbackException("host-secret"), new JsonException("host-secret"), new InvalidDataException("host-secret") })
        {
            try { await engine.ExecuteAsync(Request(new RequestInputFeed { Name = "failed-reader" }), FailedFeed(problem)); throw new Exception("reader failure returned success"); }
            catch (SessionFailure error)
            {
                if (error.Call.Terminal.Failure.State != PresenceState.Value || error.Failure.Error.Message.Contains("host-secret") || error.Failure.Error.Kind.Value != (problem is IOException ? "local" : "usage")) throw new Exception("reader diagnostics leaked");
            }
        }
        using (var limits = Engine.Open(new EngineSettings { MaxRequests = new EngineSettingsMaxRequestsInteger { Value = ulong.MaxValue }, MaxRequestsTotal = new EngineSettingsMaxRequestsTotalInteger { Value = ulong.MaxValue }, MaxEstimatedInputTokensTotal = new EngineSettingsMaxEstimatedInputTokensTotalInteger { Value = ulong.MaxValue } })) { }
        string deepOriginal = "{\"text\":\"session-owned\",\"nested\":" + deepMeaning + "}";
        using var deepDocument = JsonDocument.Parse(deepOriginal, new JsonDocumentOptions { MaxDepth = int.MaxValue });
        var call = await engine.DecideAsync(deepQuestion!, new RequestInputJson { Value = deepDocument.RootElement }, new RequestOptions { Field = new[] { "/text" } });
        if (call.Value.GetRawText() != deepMeaning) throw new Exception("authored decision meaning changed");
        var deepRow = call.Packets.OfType<SessionPacketDecideRow>().Single();
        if (deepRow.Value.Value.GetRawText() != deepMeaning || deepRow.Value.Input.Value.GetRawText() != deepOriginal) throw new Exception("deep meaning or original result changed");
        if (!call.Packets.Any(p => p is SessionPacketDecideRow) || call.Terminal.Facts.State != PresenceState.Value || call.Terminal.Failure.State != PresenceState.Missing) throw new Exception("owned row or terminal");
        string facts = FactsSnapshot(call.Terminal.Facts.Value);
        string owned = RowSnapshot(call.Packets.OfType<SessionPacketDecideRow>().First());
        using (var spent = new CancellationTokenSource())
        {
            spent.Cancel();
            var input = new RequestInputText { Text = "never-send" };
            foreach (Func<Task> named in new Func<Task>[] {
                async () => { await engine.DecideAsync(Question, input, cancellation: spent.Token); },
                async () => { await engine.ChooseAsync(Question, input, cancellation: spent.Token); },
                async () => { await engine.TagAsync(Question, input, cancellation: spent.Token); },
                async () => { await engine.ScoreAsync(Question, input, cancellation: spent.Token); },
                async () => { await engine.FilterAsync(Question, input, cancellation: spent.Token); },
                async () => { await engine.RankAsync(Question, input, cancellation: spent.Token); },
                async () => { await engine.FindAsync(Question, input, cancellation: spent.Token); },
                async () => { await engine.AnnotateAsync(Question, input, cancellation: spent.Token); },
                async () => { await engine.RecognizeAsync(Question, input, cancellation: spent.Token); },
                async () => { await engine.RelateAsync(Question, input, cancellation: spent.Token); }
            })
            {
                try { await named(); throw new Exception("spent token admitted"); }
                catch (OperationCanceledException) { }
            }
        }
        using (var cancel = new CancellationTokenSource())
        {
            var pending = engine.DecideAsync(Question, new RequestInputText { Text = "hold-session-task" }, cancellation: cancel.Token);
            await Arrived("hold-session-task");
            int progress = 0;
            Task unrelated = Task.Run(async () => { await Task.Yield(); Interlocked.Increment(ref progress); });
            await unrelated.WaitAsync(TimeSpan.FromSeconds(2));
            cancel.Cancel();
            try { await pending.WaitAsync(TimeSpan.FromSeconds(2)); throw new Exception("cancelled call returned"); }
            catch (OperationCanceledException) { }
            if (progress != 1 || File.Exists("barrier/release-hold-session-task")) throw new Exception("held cancellation progress");
            File.WriteAllText("barrier/release-hold-session-task", "");
        }
        using (var session = engine.StartSession(Request(new RequestInputFeed { Name = "session", Framing = new RequestFramingLines() })))
        {
            if (!await session.PushAsync(Item("hold-session-drain"))) throw new Exception("first rejected");
            await Arrived("hold-session-drain");
            Task<bool>? full = null;
            var admitted = new List<string>();
            for (int i = 0; i < 32; i++)
            {
                string text = "queued-after-hold-" + i;
                Task<bool> push = session.PushAsync(Item(text));
                try
                {
                    if (!await push.WaitAsync(TimeSpan.FromSeconds(1))) throw new Exception("intake closed before Full");
                }
                catch (TimeoutException) { full = push; break; }
                admitted.Add(text);
            }
            if (full is null) throw new Exception("queue did not reach Full");
            File.WriteAllLines("barrier/admitted-feed", admitted);
            session.Finish();
            if (await full.WaitAsync(TimeSpan.FromSeconds(2))) throw new Exception("Closed accepted");
            session.Cancel();
            Task<SessionPacket?> drain = session.ReadAsync();
            // It may have queued observations; drain concurrently with provider settlement.
            File.WriteAllText("barrier/release-hold-session-drain", "");
            SessionPacketTerminal? terminal = null;
            for (SessionPacket? packet = await drain.WaitAsync(TimeSpan.FromSeconds(20)); packet is not null; packet = await session.ReadAsync().WaitAsync(TimeSpan.FromSeconds(20)))
                if (packet is SessionPacketTerminal settled) terminal = settled;
            if (terminal is null || terminal.Failure.State != PresenceState.Value) throw new Exception("missing cancellation terminal");
        }
        // Keep actual received rows after all native owners have been released.
        engine.Dispose();
        if (FactsSnapshot(call.Terminal.Facts.Value) != facts || RowSnapshot(call.Packets.OfType<SessionPacketDecideRow>().First()) != owned) throw new Exception("result ownership");
        using (var raceEngine = Engine.Open())
        {
            var race = raceEngine.StartSession(Request(new RequestInputFeed { Name = "race" }));
            using var cancel = new CancellationTokenSource();
            Task read = race.ReadAsync(cancel.Token);
            await Task.WhenAll(Task.Run(cancel.Cancel), Task.Run(race.Dispose));
            try { await read.WaitAsync(TimeSpan.FromSeconds(2)); }
            catch (OperationCanceledException) { }
            catch (ObjectDisposedException) { }
        }
        Console.WriteLine("INSTALLED_CSHARP_SESSION_PASS");
    }
}
