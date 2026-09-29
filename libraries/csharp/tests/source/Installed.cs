using System;
using System.Text.Json;
using ThinkThen;
foreach (string forbidden in new[] { "/usr/bin/cargo", "/usr/bin/rustc", "/home", "/Users" })
    if (System.IO.File.Exists(forbidden) || System.IO.Directory.Exists(forbidden)) throw new Exception("source/compiler visible");
if (Environment.GetEnvironmentVariable("TT_PORTABLE_BATCH") == "1") {
    using var bulk = Engine.Open("{\"batch\":\"max\",\"cache\":false,\"throttle\":1,\"max_retries\":0}");
    string[] texts = ["alpha", "café-5544", "omega", "line 2907", "tail"];
    var bulkResult = bulk.DecideMany("Is it relevant?", texts);
    Answer[] rows = bulkResult.Value;
    if (bulkResult.Facts.Records != 5 || bulkResult.Facts.RequestsSent != 3) throw new Exception("portable bulk facts");
    if (rows.Length != 5 || rows.Any(row => row.Outcome != 1 || row.Probability != .9))
        throw new Exception("portable bulk answers");
    Console.WriteLine("PORTABLE_BATCH_CSHARP_PASS");
    return;
}
using var engine = Engine.Open();
var typed = engine.Decide("Is it?", "consumer-csharp");
if (typed.Value.Outcome != 1 || typed.Facts.Records != 1 || typed.Facts.RequestsSent != 1 ||
    typed.Facts.InputTokens != 1 || typed.Facts.Model != "jev-1.13.0") throw new Exception("typed scalar facts");
using var json = JsonDocument.Parse(engine.Call("{\"decide\":\"Is it?\",\"evidence\":\"consumer-json\"}"));
JsonElement result = json.RootElement;
if (result.ValueKind != JsonValueKind.Object || result.EnumerateObject().Select(p => p.Name).OrderBy(n => n).SequenceEqual(new[] {"facts", "value"}) == false) throw new Exception("JSON result field set");
if (result.GetProperty("value").ValueKind != JsonValueKind.True) throw new Exception("JSON result decision true");
JsonElement facts = result.GetProperty("facts");
if (facts.ValueKind != JsonValueKind.Object || !facts.EnumerateObject().Select(p => p.Name).OrderBy(n => n).SequenceEqual(new[] {"cache_answers", "input_tokens", "model", "output_tokens", "records", "requests_sent", "seconds"})) throw new Exception("JSON result facts field set");
if (facts.GetProperty("records").GetInt32() != 1 || facts.GetProperty("requests_sent").GetInt32() != 1 || facts.GetProperty("cache_answers").GetInt32() != 0 || facts.GetProperty("input_tokens").GetInt32() != 1 || facts.GetProperty("output_tokens").GetInt32() != 1 || facts.GetProperty("model").GetString() != "jev-1.13.0" || facts.GetProperty("seconds").GetDouble() < 0) throw new Exception("JSON result exact facts");
Console.WriteLine("INSTALLED_JSON_ENVELOPE_PASS");
using var spent = new System.Threading.CancellationTokenSource(); spent.Cancel();
try { engine.Decide("Is it?", "no-arrival-spent", cancellation:spent.Token); throw new Exception("spent accepted"); }
catch (Failure e) when (e.Code == 5) { }
Console.WriteLine("INSTALLED_CSHARP_CONSUMER_PASS");
