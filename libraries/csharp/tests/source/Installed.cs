using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;
if (Environment.GetEnvironmentVariable("TT_USAGE") is string usageMode) { await SessionChecks.Usage(usageMode); return; }
if (Environment.GetEnvironmentVariable("TT_SESSIONS") == "1") { await SessionChecks.Run(); return; }
foreach (string forbidden in new[] { "/usr/bin/cargo", "/usr/bin/rustc", "/home", "/Users" })
    if (File.Exists(forbidden) || Directory.Exists(forbidden)) throw new Exception("source/compiler visible");
static RequestInputRecords Records(params string[] texts) => new() {
    Items = texts.Select(text => new RequestItem { Original = new RequestOriginalText { Text = text } }).ToArray()
};
static void CheckRows(OwnedCall call, int count, ulong sends) {
    var rows = call.Packets.OfType<SessionPacketDecideRow>().ToArray();
    if (rows.Length != count || rows.Any(row => !row.Value.Value.GetBoolean() || row.Value.Answer is not AnswerYesNo { Probability: .9 })) throw new Exception("typed decisions");
    if (call.Terminal.Facts.State != PresenceState.Value || call.Terminal.Facts.Value.Records != (ulong)count ||
        call.Terminal.Facts.Value.RequestsSent != sends) throw new Exception("typed native facts");
}
if (Environment.GetEnvironmentVariable("TT_PORTABLE_BATCH") == "1") {
    using var bulk = Engine.Open(new EngineSettings {
        Batch = new RequestBatchString { Value = "max" }, Cache = new CacheDocumentDisabledCache { Value = new DisabledCache() }, Throttle = 1, MaxRetries = 0
    });
    CheckRows(await bulk.DecideAsync(new RequestQuestionText { Text = "Is it relevant?" }, Records("alpha", "café-5544", "omega", "line 2907", "tail")),5,1);
    Console.WriteLine("PORTABLE_BATCH_CSHARP_PASS"); return;
}
foreach (var type in typeof(Engine).Assembly.GetExportedTypes()) {
    if (type.Namespace == "ThinkThen.Inputs" && type.Name.StartsWith("Input", StringComparison.Ordinal)) throw new Exception("schema prefix is public");
    foreach (var method in type.GetMethods(System.Reflection.BindingFlags.Public | System.Reflection.BindingFlags.Instance | System.Reflection.BindingFlags.Static | System.Reflection.BindingFlags.DeclaredOnly))
        if (new[] { "ToJson", "ToJsonString", "ToPlain", "ParseQuestion", "Write", "Read" }.Contains(method.Name)) throw new Exception("transport method is public: " + type.Name + "." + method.Name);
}
using var engine = Engine.Open(new EngineSettings());
foreach (string text in new[] { "consumer-csharp", "consumer-json" }) {
    var call = await engine.DecideAsync(new RequestQuestionText { Text = "Is it?" },Records(text));
    CheckRows(call,1,1);
    var facts = call.Terminal.Facts.Value;
    if (facts.CacheAnswers != 0 || facts.InputTokens.State != PresenceState.Value || facts.InputTokens.Value != 1 ||
        facts.OutputTokens.State != PresenceState.Value || facts.OutputTokens.Value != 1 || facts.Model.State != PresenceState.Value || facts.Model.Value != "jev-1.13.0" || facts.Seconds < 0)
        throw new Exception("typed result exact facts");
}
Console.WriteLine("INSTALLED_TYPED_RESULT_PASS");
using var spent = new CancellationTokenSource(); spent.Cancel();
try { await engine.DecideAsync(new RequestQuestionText { Text = "Is it?" },Records("no-arrival-spent"),cancellation:spent.Token); throw new Exception("spent accepted"); }
catch (OperationCanceledException) { }
Console.WriteLine("INSTALLED_CSHARP_CONSUMER_PASS");
