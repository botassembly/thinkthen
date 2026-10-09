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
    public static async Task Run()
    {
        using var engine = Engine.Open();
        OwnedCall call = await engine.DecideAsync(Question, new InputRequestInputText { Text = "session-owned" });
        if (!call.Packets.Any(p => p is SessionPacketDecideRow) || call.Terminal.Facts.State != PresenceState.Value || call.Terminal.Failure.State != PresenceState.Missing) throw new Exception("owned row or terminal");
        string owned = call.Packets.First(p => p is SessionPacketDecideRow).ToJsonString();
        using (var spent = new CancellationTokenSource())
        {
            spent.Cancel();
            try { await engine.DecideAsync(Question, new InputRequestInputText { Text = "never-send" }, cancellation: spent.Token); throw new Exception("spent token admitted"); }
            catch (OperationCanceledException) { }
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
