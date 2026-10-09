using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;
using System.Text.Json;
static class SessionChecks
{
    static InputRequestQuestionText Question => new() { Text = "Is it?" };
    static InputRequest Request(InputRequestInput input) => new() { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallDecide { Question = Question, Input = input, Options = new InputRequestOptions { Batch = new InputRequestBatchAlternative0 { Value = 1 } } } };
    static InputRequestSessionDescriptor Item(string text) => new() { Item = new InputRequestItem { Original = new InputRequestOriginalText { Text = text } } };
    static async Task Arrived(string name)
    {
        using var timeout = new CancellationTokenSource(TimeSpan.FromSeconds(20));
        while (!File.Exists("barrier/arrived-" + name)) await Task.Delay(10, timeout.Token);
    }
    static async IAsyncEnumerable<InputRequestSessionDescriptor> FailedFeed(Exception failure)
    {
        await Task.Yield();
        yield return failure is null ? Item("unreachable") : throw failure;
    }
    public static async Task Run()
    {
        using var engine = Engine.Open(new InputEngineSettings {
            Cache = new InputCacheDocumentAlternative1 { Value = new InputDisabledCache() },
            Batch = new InputRequestBatchAlternative0 { Value = 1 }, MaxRetries = 0,
            MaxRequests = new InputEngineSettingsMaxRequestsAlternative1(),
            MaxRequestsTotal = new InputEngineSettingsMaxRequestsTotalAlternative1(),
            MaxEstimatedInputTokensTotal = new InputEngineSettingsMaxEstimatedInputTokensTotalAlternative1()
        });
        var authored = engine.ParseQuestion(AuthoredQuestionKind.Atomic,"{\"choose\":\"Which?\",\"options\":{\"first\":\"First.\",\"second\":\"Second.\"}}");
        var choices = ((InputAuthoredOptionsAlternative1)((InputRequestDefinitionAlternative1)authored.Value).Value.Options.Value).Value;
        if (!choices.Keys.SequenceEqual(new[] { "first", "second" })) throw new Exception("native authored order");
        try { ((IDictionary<string,InputAuthoredDescription>)choices).Clear(); throw new Exception("parsed authored map changed"); }
        catch (NotSupportedException) { }
        var authoredPlan = engine.Plan(new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallChoose { Question = authored, Input = new InputRequestInputText { Text = "owned" } } });
        if (authoredPlan.Requests != 1 || authoredPlan.FirstBodyUtf8.State != PresenceState.Value || !authoredPlan.FirstBodyUtf8.Value.Contains("First.")) throw new Exception("owned authored definition");
        var preview = engine.Plan(Request(new InputRequestInputText { Text = "café\0preview" }));
        if (preview.Records != 1 || preview.Requests != 1 || preview.FirstBodyUtf8.State != PresenceState.Value ||
            !preview.FirstBodyUtf8.Value.Contains("café") || preview.EstimatedInputTokens.Lower > preview.EstimatedInputTokens.Upper)
            throw new Exception("typed canonical plan");
        using (var document = JsonDocument.Parse("{\"first_body_utf8\":null,\"future\":false}"))
        {
            var retained = new Plan(document.RootElement);
            if (retained.FirstBodyUtf8.State != PresenceState.Null || retained.ToJson().GetProperty("future").GetBoolean())
                throw new Exception("owned nullable plan and unknown field");
        }
        using (var document = JsonDocument.Parse("{}"))
        {
            try { _ = new Plan(document.RootElement).FirstBodyUtf8; throw new Exception("missing required plan member accepted"); }
            catch (KeyNotFoundException) { }
        }
        foreach (var invalid in new[] { new InputEngineSettings { Throttle = 0 }, new InputEngineSettings { Backend = "unknown-backend" }, new InputEngineSettings { Model = (string)null! } })
        {
            try { using var refused = Engine.Open(invalid); throw new Exception("invalid typed settings admitted"); }
            catch (ThinkThen.Failure error) when (error.Kind == ThinkThen.FailureKind.Usage && error.FactsJson is null) { }
        }
        foreach (InputRequestReaderFailure failure in new InputRequestReaderFailure[] {
            new InputRequestReaderFailureIo(),
            new InputRequestReaderFailureUtf8 { Location = new InputSessionSourceLocation { File = "private-file", FirstLine = 1, LastLine = 2 } },
            new InputRequestReaderFailureInvalidInput { Location = new InputSessionSourceLocation { File = "private-file" } }
        })
        {
            using var reader = engine.StartSession(Request(new InputRequestInputFeed { Name = "reader" }));
            reader.Finish(failure);
            SessionPacketTerminal? terminal = null;
            while (await reader.ReadAsync() is { } packet) if (packet is SessionPacketTerminal done) terminal = done;
            if (terminal is null || terminal.Failure.State != PresenceState.Value || terminal.Failure.Value.Error.Kind.Value != (failure is InputRequestReaderFailureIo ? "local" : "usage")) throw new Exception("typed reader failure kind");
        }
        using (var reader = engine.StartSession(Request(new InputRequestInputFeed { Name = "invalid-location" })))
        {
            try { reader.Finish(new InputRequestReaderFailureIo { Location = (InputSessionSourceLocation)null! }); throw new Exception("explicit null location admitted"); }
            catch (ThinkThen.Failure error) when (error.Kind == ThinkThen.FailureKind.Usage && error.FactsJson is null) { }
            try { reader.Finish(new InputRequestReaderFailureUtf8 { Location = new InputSessionSourceLocation { File = "private-file", FirstLine = 1 } }); throw new Exception("half location admitted"); }
            catch (ThinkThen.Failure error) when (error.Kind == ThinkThen.FailureKind.Usage && error.FactsJson is null) { }
            reader.Finish();
        }
        foreach (Exception problem in new Exception[] { new IOException("host-secret"), new System.Text.DecoderFallbackException("host-secret"), new JsonException("host-secret"), new InvalidDataException("host-secret") })
        {
            try { await engine.ExecuteAsync(Request(new InputRequestInputFeed { Name = "failed-reader" }), FailedFeed(problem)); throw new Exception("reader failure returned success"); }
            catch (SessionFailure error)
            {
                if (error.Call.Terminal.Failure.State != PresenceState.Value || error.Call.Terminal.ToJsonString().Contains("host-secret") || error.Failure.Error.Kind.Value != (problem is IOException ? "local" : "usage")) throw new Exception("reader diagnostics leaked");
            }
        }
        using (var limits = Engine.Open(new InputEngineSettings { MaxRequests = new InputEngineSettingsMaxRequestsAlternative0 { Value = ulong.MaxValue }, MaxRequestsTotal = new InputEngineSettingsMaxRequestsTotalAlternative0 { Value = ulong.MaxValue }, MaxEstimatedInputTokensTotal = new InputEngineSettingsMaxEstimatedInputTokensTotalAlternative0 { Value = ulong.MaxValue } })) { }
        OwnedCall call = await engine.DecideAsync(Question, new InputRequestInputText { Text = "session-owned" });
        if (!call.Packets.Any(p => p is SessionPacketDecideRow) || call.Terminal.Facts.State != PresenceState.Value || call.Terminal.Failure.State != PresenceState.Missing) throw new Exception("owned row or terminal");
        string owned = call.Packets.First(p => p is SessionPacketDecideRow).ToJsonString();
        using (var spent = new CancellationTokenSource())
        {
            spent.Cancel();
            var input = new InputRequestInputText { Text = "never-send" };
            foreach (Func<Task<OwnedCall>> named in new Func<Task<OwnedCall>>[] {
                () => engine.DecideAsync(Question, input, cancellation: spent.Token),
                () => engine.ChooseAsync(Question, input, cancellation: spent.Token),
                () => engine.TagAsync(Question, input, cancellation: spent.Token),
                () => engine.ScoreAsync(Question, input, cancellation: spent.Token),
                () => engine.FilterAsync(Question, input, cancellation: spent.Token),
                () => engine.RankAsync(Question, input, cancellation: spent.Token),
                () => engine.FindAsync(Question, input, cancellation: spent.Token),
                () => engine.AnnotateAsync(Question, input, cancellation: spent.Token),
                () => engine.RecognizeAsync(Question, input, cancellation: spent.Token),
                () => engine.RelateAsync(Question, input, cancellation: spent.Token)
            })
            {
                try { await named(); throw new Exception("spent token admitted"); }
                catch (OperationCanceledException) { }
            }
        }
        using (var cancel = new CancellationTokenSource())
        {
            Task<OwnedCall> pending = engine.DecideAsync(Question, new InputRequestInputText { Text = "hold-session-task" }, cancellation: cancel.Token);
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
        using (var session = engine.StartSession(Request(new InputRequestInputFeed { Name = "session", Framing = new InputRequestFramingAlternative1() })))
        {
            if (!await session.PushAsync(Item("hold-session-drain"))) throw new Exception("first rejected");
            await Arrived("hold-session-drain");
            Task<bool>? full = null;
            var admitted = new List<string>();
            for (int i = 0; i < 32; i++)
            {
                string text = "queued-after-hold-" + i;
                Task<bool> push = session.PushAsync(Item(text));
                if (!push.IsCompleted) { full = push; break; }
                if (!await push) throw new Exception("intake closed before Full");
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
        if (call.Packets.First(p => p is SessionPacketDecideRow).ToJsonString() != owned) throw new Exception("result ownership");
        using (var document = JsonDocument.Parse("{\"kind\":\"terminal\",\"facts\":null,\"future\":{\"nested\":false}}"))
        {
            var terminal = (SessionPacketTerminal)SessionPacket.Read(document.RootElement);
            if (terminal.Facts.State != PresenceState.Null || terminal.Failure.State != PresenceState.Missing || terminal.ToPlain()["future"]!["nested"]!.GetValue<bool>()) throw new Exception("presence and nested unknown");
        }
        using (var raceEngine = Engine.Open())
        {
            var race = raceEngine.StartSession(Request(new InputRequestInputFeed { Name = "race" }));
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
