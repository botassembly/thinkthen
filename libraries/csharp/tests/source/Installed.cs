using System;
using System.Text.Json;
using ThinkThen;
foreach (string forbidden in new[] { "/usr/bin/cargo", "/usr/bin/rustc", "/home/ian/workspace/repos/thinkthen", "/home/ian/workspace/experiments/290-thinkthen-csharp-c-interface/stage2/inputs/source" })
    if (System.IO.File.Exists(forbidden) || System.IO.Directory.Exists(forbidden)) throw new Exception("source/compiler visible");
using var engine = Engine.Open();
if (engine.Decide("Is it?", "consumer-csharp").Outcome != 1) throw new Exception("scalar");
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
