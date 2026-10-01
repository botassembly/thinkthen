using System;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using ThinkThen;

static class Program
{
    static void Check(bool condition, string label) { if (!condition) throw new Exception(label); }
    static Failure Fails(Action call, int code)
    {
        try { call(); }
        catch (Failure f) { Check(f.Code == code && f.Kind == (FailureKind)code && !f.Retryable && f.Message.Length > 0, $"error {code}: {f.Code} {f.Message}"); return f; }
        throw new Exception($"expected error {code}");
    }
    static long N(JsonElement facts, string name) => facts.GetProperty(name).GetInt64();
    static string? Model(JsonElement facts) => facts.TryGetProperty("model", out JsonElement model) ? model.GetString() : null;
    static bool Has(JsonElement facts, string name) => facts.TryGetProperty(name, out _);
    static void AnswerIs(Answer answer, int outcome, double probability) => Check(answer.Outcome == outcome && (int)answer.OutcomeKind == outcome && answer.Probability == probability, $"answer {answer.Outcome}/{answer.Probability}");
    static void Marker(string name)
    {
        string path = Path.Combine(Environment.GetEnvironmentVariable("TT_BARRIER_DIR")!, "arrived-" + name);
        Check(SpinWait.SpinUntil(() => File.Exists(path), TimeSpan.FromSeconds(5)), "arrival " + name);
    }
    static void Release(string name) => File.WriteAllText(Path.Combine(Environment.GetEnvironmentVariable("TT_BARRIER_DIR")!, "release-" + name), "");
    static void Direct()
    {
        IntPtr engine = Native.thinkthen_engine_new(); Check(engine != IntPtr.Zero, "direct engine");
        try
        {
            byte[] question = Engine.CString("Is it?"); byte[] text = Encoding.UTF8.GetBytes("direct");
            Answer answer = new() { Outcome = 123, Probability = -1 };
            int rc = Native.thinkthen_decide_opts(engine, question, text, (nuint)text.Length, -1, IntPtr.Zero, ref answer);
            Check(rc == 0, $"direct native code {rc}"); AnswerIs(answer, 1, .9);
            IntPtr spent = Native.thinkthen_cancel_token_new();
            try
            {
                Native.thinkthen_cancel(spent); Native.thinkthen_cancel(spent);
                answer = new Answer { Outcome = 123, Probability = -1 };
                byte[] refused = Encoding.UTF8.GetBytes("no-arrival-spent-token");
                rc = Native.thinkthen_decide_opts(engine, question, refused, (nuint)refused.Length, -1, spent, ref answer);
                int last = Native.thinkthen_error_code(engine);
                string message = Marshal.PtrToStringUTF8(Native.thinkthen_error_message(engine)) ?? "";
                Check(rc == 5 && last == 5 && message.Length > 0 && answer.Outcome == 123 && answer.Probability == -1, "spent token did not refuse without output");
            }
            finally { Native.thinkthen_cancel_token_free(spent); }
            Console.WriteLine("DIRECT_PINVOKE_PASS: spent token code 5, untouched output");
        }
        finally { Native.thinkthen_engine_free(engine); }
    }
    static void Held(Engine engine, string state, int expected, bool many = false)
    {
        using var cancel = new CancellationTokenSource();
        Task<Failure?> call = Task.Run(() => {
            try
            {
                if (many) engine.DecideManyWithOptions("Is it?", Enumerable.Range(1, 6).Select(i => $"hold-bulk-{i}").ToArray(), null, cancel.Token);
                else engine.Decide("Is it?", state, expected == 3 ? TimeSpan.FromMilliseconds(25) : null, cancel.Token);
                return null;
            }
            catch (Failure f) { return f; }
        });
        Marker(state);
        if (expected == 5) { cancel.Cancel(); cancel.Cancel(); }
        Thread.Sleep(100);
        if (many) for (int i = 1; i <= 6; i++) Release($"hold-bulk-{i}"); else Release(state);
        Check(call.Wait(TimeSpan.FromSeconds(20)), $"held call did not finish {state}");
        Check(call.Result is { Code: var code } && code == expected, $"held {state}: {call.Result?.Code}");
        Console.WriteLine($"HELD_{state.ToUpperInvariant().Replace('-', '_')}_PASS: code {expected}, untouched output");
    }
    static void Matrix()
    {
        using var engine = Engine.Open();
        using (var configured = Engine.Open("{}")) Check(configured != null, "settings constructor");
        Failure badSettings = Fails(() => { using var invalid = Engine.Open("{\"unknown\":1}"); }, 1);
        Check(badSettings.FactsJson is null, "pre-call settings refusal has no started facts");
        foreach ((string text, int outcome, double p) in new[] { ("yes", 1, .9), ("no", 0, .1), ("unsure", 2, .5), ("café", 1, .9), ("a\0b", 1, .9) })
            AnswerIs(engine.Decide(text == "unsure" ? "{\"decide\":\"Is it?\",\"threshold\":\"0.4:0.8\"}" : "Is it?", text).Value, outcome, p);
        var scalar = engine.Decide("Is it?", "no-usage");
        AnswerIs(scalar.Value, 1, .9);
        Check(N(scalar.Facts, "records") == 1 && N(scalar.Facts, "requests_sent") == 1 && Model(scalar.Facts) == "jev-1.13.0" &&
            !Has(scalar.Facts, "input_tokens") && !Has(scalar.Facts, "output_tokens"), "reported model without usage");
        var firstBulk = engine.DecideMany("Is it?", "first", "second", "third");
        Answer[] rows = firstBulk.Value;
        Check(N(firstBulk.Facts, "records") == 3 && N(firstBulk.Facts, "requests_sent") >= 1 && Model(firstBulk.Facts) == "jev-1.13.0", "typed bulk facts");
        for (int i = 0; i < rows.Length; i++) AnswerIs(rows[i], new[] {1,0,1}[i], new[] {.9,.1,.6}[i]);
        var cachedBulk = engine.DecideMany("Is it?", "first", "second", "third");
        Check(N(cachedBulk.Facts, "records") == 3 && N(cachedBulk.Facts, "cache_answers") == 3 && N(cachedBulk.Facts, "requests_sent") == 0 &&
            cachedBulk.Value.Select(a => a.Probability).SequenceEqual(new[] {.9,.1,.6}), "identical bulk answers each question from the cache");
        rows = engine.DecideMany("Is it?", "first", "second", "first", "second").Value;
        Check(rows.Select(a => a.Probability).SequenceEqual(new[] {.9,.1,.9,.1}), "bulk cache/order");
        var empty = engine.DecideMany("Is it?");
        Check(empty.Value.Length == 0 && N(empty.Facts, "records") == 0 && N(empty.Facts, "requests_sent") == 0 && Model(empty.Facts) is null && !Has(empty.Facts, "input_tokens"), "zero bulk facts");
        try { engine.Decide("Is it?", "\ud800"); throw new Exception("invalid UTF-8 accepted"); }
        catch (EncoderFallbackException) { }
        var requests = new[] {
          "{\"decide\":\"Is it?\",\"evidence\":\"json-decide\",\"details\":true}",
          "{\"choose\":\"Which team?\",\"options\":[\"first\",\"second\"],\"evidence\":\"choose\"}",
          "{\"tag\":\"Which labels?\",\"labels\":[\"first\",\"second\"],\"evidence\":\"tag\"}",
          "{\"score\":\"What level?\",\"levels\":[\"Low.\",\"High.\"],\"evidence\":\"score\"}",
          "{\"filter\":\"Is it?\",\"records\":[\"filter-one\",\"filter-two\"]}",
          "{\"rank\":\"Is it?\",\"records\":[\"rank-one\",\"rank-two\"]}",
          "{\"find\":\"Which line?\",\"units\":[\"find-one\",\"find-two\"]}",
          "{\"annotate\":{\"version\":1,\"questions\":{\"check\":{\"decide\":\"Is it?\"}}},\"records\":[\"annotate-one\"]}",
          "{\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}},\"version\":1,\"evidence\":\"Maria Chen\"}",
          "{\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]},\"version\":1,\"records\":[{\"name\":\"First\",\"kind\":\"alert\"},{\"name\":\"Second\",\"kind\":\"alert\"}]}"
        };
        for (int i = 0; i < requests.Length; i++)
        {
            using JsonDocument json = JsonDocument.Parse(engine.Call(requests[i]));
            JsonElement envelope = json.RootElement;
            Check(envelope.ValueKind == JsonValueKind.Object && envelope.EnumerateObject().Select(p => p.Name).OrderBy(n => n).SequenceEqual(new[] { "facts", "value" }), "JSON door envelope fields");
            JsonElement facts = envelope.GetProperty("facts");
            Check(facts.ValueKind == JsonValueKind.Object && facts.EnumerateObject().Select(p => p.Name).OrderBy(n => n).SequenceEqual(new[] { "cache_answers", "input_tokens", "model", "output_tokens", "records", "requests_sent", "seconds" }), "JSON door facts fields");
            Check(facts.GetProperty("records").GetInt32() >= 0 && facts.GetProperty("requests_sent").GetInt32() >= 0 && facts.GetProperty("cache_answers").GetInt32() >= 0 && facts.GetProperty("input_tokens").GetInt32() >= 0 && facts.GetProperty("output_tokens").GetInt32() >= 0 && facts.GetProperty("model").GetString() == "jev-1.13.0" && facts.GetProperty("seconds").GetDouble() >= 0, "JSON door facts types and values");
            JsonElement value = envelope.GetProperty("value");
            switch (i)
            {
                case 0: Check(value.GetProperty("schema").GetString() == "thinkthen.result/1" && value.GetProperty("value").GetBoolean(), "decide details"); break;
                case 1: Check(value.GetString() == "first", "choose"); break;
                case 2: Check(value.GetArrayLength() == 2 && value[0].GetString() == "first" && value[1].GetString() == "second", "tag"); break;
                case 3: Check(value.GetDouble() == .1, "score"); break;
                case 4: Check(value.GetArrayLength() == 2 && value[0].GetString() == "filter-one", "filter"); break;
                case 5: Check(value.GetArrayLength() == 2 && value[0].GetProperty("index").GetInt32() == 0 && value[0].GetProperty("record").GetString() == "rank-one" && value[0].GetProperty("probability").GetDouble() == .9 && value[1].GetProperty("record").GetString() == "rank-two", "rank"); break;
                case 6: Check(value.GetProperty("index").GetInt32() == 0 && value.GetProperty("unit").GetString() == "find-one" && value.GetProperty("probability").GetDouble() == .9, "find"); break;
                case 7: Check(value.GetArrayLength() == 1 && value[0].GetProperty("check").GetBoolean(), "annotate"); break;
                case 8: Check(value.GetProperty("entities").GetArrayLength() == 1, "recognize JSON"); break;
                case 9: Check(value.GetProperty("edges").GetArrayLength() == 2, "relate JSON"); break;
            }
        }
        var recognized = engine.Recognize("{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}", "John Smith");
        Check(N(recognized.Facts, "records") == 1 && N(recognized.Facts, "requests_sent") >= 1, "typed recognize facts");
        Check(recognized.Value.GetProperty("entities").GetArrayLength() == 1, "recognize");
        var related = engine.Relate("{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}", "{\"name\":\"Third\",\"kind\":\"alert\"}", "{\"name\":\"Fourth\",\"kind\":\"alert\"}");
        Check(N(related.Facts, "records") == 1 && N(related.Facts, "requests_sent") >= 1, "typed relate facts");
        Check(related.Value.GetProperty("edges").GetArrayLength() == 2, "relate");
        using (JsonDocument json = JsonDocument.Parse(engine.Call("{\"usage\":true}"))) Check(json.RootElement.TryGetProperty("requests_sent", out _), "usage");
        foreach (Action check in new Action[] { () => engine.Decide("Is it?\0suffix", "x"), () => engine.DecideMany("Is it?\0suffix", "x"), () => engine.Call("{}\0suffix"), () => engine.Recognize("{}\0suffix", "x"), () => engine.Relate("{}\0suffix") })
        { try { check(); throw new Exception("NUL accepted"); } catch (ArgumentException ex) when (ex.Message.Contains("NUL")) { } }
        Failure saved = Fails(() => engine.Decide("{invalid", "text"), 1);
        using (var other = Engine.Open()) Fails(() => other.Decide("Is it?", ""), 1);
        Check(saved.Message.Length > 0, "copied failure lost");
        Failure backend = Fails(() => engine.Decide("Is it?", "status-401"), 2);
        using (JsonDocument facts = JsonDocument.Parse(backend.FactsJson ?? throw new Exception("missing started failure facts")))
            Check(facts.RootElement.GetProperty("requests_sent").GetInt32() == 1, "started failure request facts");
        string copiedBackendFacts = backend.FactsJson!;
        Fails(() => engine.Decide("Is it?", "zero-deadline", TimeSpan.Zero), 3);
        using (var canceled = new CancellationTokenSource()) { canceled.Cancel(); Fails(() => engine.Decide("Is it?", "no-arrival-pre-cancelled", cancellation:canceled.Token), 5); }
        var concurrent = new[] { "failure-one", "failure-two", "success" }.Select(state => Task.Run(() => { try { return (state, result:engine.Decide("Is it?", state), failure:(Failure?)null); } catch (Failure f) { return (state, result:default(TypedResult<Answer>), failure:f); } })).ToArray();
        Task.WaitAll(concurrent);
        Check(concurrent[0].Result.failure is { Code: 2 } first && first.Message.Contains("401") && concurrent[1].Result.failure is { Code: 2 } second && second.Message.Contains("403"), "concurrent per-thread errors");
        AnswerIs(concurrent[2].Result.result!.Value, 1, .9);
        var heldOne = Task.Run(() => engine.Decide("Is it?", "hold-facts-one"));
        var heldTwo = Task.Run(() => engine.Decide("Is it?", "hold-facts-two"));
        try { Marker("hold-facts-one"); Marker("hold-facts-two"); }
        finally { Release("hold-facts-one"); Release("hold-facts-two"); }
        var ownedOne = heldOne.GetAwaiter().GetResult();
        var ownedTwo = heldTwo.GetAwaiter().GetResult();
        Check(N(ownedOne.Facts, "requests_sent") == 1 && N(ownedTwo.Facts, "requests_sent") == 1 &&
            ownedOne.Facts.GetProperty("seconds").GetDouble() > 0 && ownedTwo.Facts.GetProperty("seconds").GetDouble() > 0,
            "two overlapping owned call facts");
        Held(engine, "hold-deadline", 3);
        Held(engine, "hold-bulk-1", 5, true);
        Held(engine, "hold-scalar", 5);
        AnswerIs(engine.Decide("Is it?", "recovery-scalar").Value, 1, .9);
        Console.WriteLine("STRICT_CANCELLED_SCALAR_PASS: code 5, untouched output, fresh-token recovery");
        string copiedError = saved.Message;
        engine.Dispose();
        Check(saved.Message == copiedError, "copied error changed after engine teardown");
        Check(backend.FactsJson == copiedBackendFacts && Model(ownedOne.Facts) == "jev-1.13.0" && Model(ownedTwo.Facts) == "jev-1.13.0", "owned facts survived later call and close");
        try { engine.Call("{\"usage\":true}"); throw new Exception("closed engine accepted call"); }
        catch (ObjectDisposedException) { }
        Console.WriteLine("MATRIX_PASS");
    }
    public static int Main(string[] args)
    {
        try { if (args.Length == 1 && args[0] == "direct") Direct(); else if (args.Length == 1 && args[0] == "matrix") Matrix(); else throw new ArgumentException("mode direct|matrix"); return 0; }
        catch (Exception ex) { Console.Error.WriteLine(ex); return 1; }
    }
}
