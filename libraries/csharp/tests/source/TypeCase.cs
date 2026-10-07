using System;
using System.Text.Json;
using System.Collections.Generic;
using ThinkThen;

// With one argument, args[0] is one JSON-door request and stdout its reply.
// "fields REQUEST" reads each annotate row member through AnnotatedField.Read.
// "plan INPUT" reads one thinkthen.plan-input/1 object and prints Engine.Plan's object.
// "limits" checks ticket 0291's zero budgets and zero cap; "helper" checks
// AnnotatedField.Read's edge table. Each prints one JSON line.
if(args.Length>=1 && args[0]=="complete"){Console.WriteLine(NativeCases.Run(args.Length==2?args[1]:Console.ReadLine()!));return;}
if(args.Length==1 && args[0]=="native"){NativeChecks.Run();Console.WriteLine("{\"native\":\"pass\"}");return;}
if (args.Length == 1 && args[0] == "carriers") { CarrierChecks.Run(); Console.WriteLine("{\"carriers\":\"pass\"}"); return; }
string mode = args.Length == 2 ? args[0] : args.Length == 1 && args[0] is "limits" or "helper" ? args[0] : "";
string request = args[^1];
using var engine = Engine.Open();
try {
    if (mode == "helper") Console.WriteLine(Helper());
    else if (mode == "limits") Console.WriteLine(Limits(engine));
    else if (mode == "plan") {
        using JsonDocument input = JsonDocument.Parse(request);
        JsonElement root = input.RootElement, question = root.GetProperty("question");
        string asked = question.ValueKind == JsonValueKind.String ? question.GetString()! : question.GetRawText();
        string[] texts = Array.ConvertAll(root.GetProperty("input").EnumerateArray().ToArray(), text => text.GetString()!);
        string? settings = root.TryGetProperty("settings", out JsonElement given) ? given.GetRawText() : null;
        Console.WriteLine(engine.Plan(root.GetProperty("verb").GetString()!, asked, texts, settings).GetRawText());
    } else if (mode == "fields") {
        var names = new Dictionary<Type, string> { [typeof(AnnotatedField.Unresolved)] = "unresolved", [typeof(AnnotatedField.Answered)] = "answered", [typeof(AnnotatedField.Failed)] = "failed" };
        var rows = new List<Dictionary<string, string>>();
        foreach (JsonElement row in engine.CallTyped(request).Value.EnumerateArray()) {
            var states = new Dictionary<string, string>();
            foreach (JsonProperty member in row.EnumerateObject()) {
                AnnotatedField field = AnnotatedField.Read(member.Value);
                states[member.Name] = names[field.GetType()] + (field is AnnotatedField.Failed failed ? $" {failed.Kind} {failed.Cause}" : "");
            }
            rows.Add(states);
        }
        Console.WriteLine(JsonSerializer.Serialize(rows));
    } else {
        using JsonDocument input = JsonDocument.Parse(request);
        if (input.RootElement.TryGetProperty("usage", out _)) Console.WriteLine(engine.Call(request));
        else {
            TypedResult<JsonElement> result = engine.CallTyped(request);
            Console.WriteLine(JsonSerializer.Serialize(new Dictionary<string, JsonElement> { ["value"] = result.Value, ["facts"] = result.Facts }));
        }
    }
} catch (Failure error) {
    Console.WriteLine(JsonSerializer.Serialize(new { failed = new { kind = error.Kind.ToString().ToLowerInvariant(), code = error.Code }, retryable = error.Retryable, facts = error.FactsJson }));
}

// ADR 0112 section 4: null is unresolved, {"failed": ...} is a failure whose
// unknown extra member reads without error, and any other value is answered.
static string Helper()
{
    foreach ((string member, AnnotatedField want) in new (string, AnnotatedField)[] {
        ("null", new AnnotatedField.Unresolved()),
        ("{\"failed\":{\"kind\":\"backend\",\"cause\":\"missing_probability\",\"later\":1}}", new AnnotatedField.Failed("backend", "missing_probability")) }) {
        using JsonDocument json = JsonDocument.Parse(member);
        if (AnnotatedField.Read(json.RootElement) != want) throw new Exception("helper " + member);
    }
    foreach (string member in new[] { "true", "\"billing\"", "[\"billing\",\"urgent\"]", "1.2" }) {
        using JsonDocument json = JsonDocument.Parse(member);
        if (AnnotatedField.Read(json.RootElement) is not AnnotatedField.Answered answered || answered.Value.GetRawText() != member) throw new Exception("helper " + member);
    }
    foreach (string member in new[] { "{\"failed\":null}", "{\"team\":\"billing\"}" }) {
        using JsonDocument json = JsonDocument.Parse(member);
        try { AnnotatedField.Read(json.RootElement); throw new Exception("helper read an object as an answer: " + member); }
        catch (InvalidOperationException) { }
    }
    return "{\"helper\":\"pass\"}";
}

// Ticket 0291: a zero cap and a zero budget each refuse before sending. Relate
// gets two entities, since one entity has no pair to ask.
static string Limits(Engine engine)
{
    static Failure Refused(Action call) {
        try { call(); } catch (Failure failure) { return failure; }
        throw new Exception("a limited call succeeded");
    }
    using (Engine capped = Engine.Open("{\"max_requests_total\":0,\"cache\":false}")) {
        Failure cap = Refused(() => capped.Decide("Is it?", "capped"));
        if (cap.Kind != FailureKind.Usage || cap.Code != 1 || !cap.Message.Contains("process send budget")) throw new Exception("cap: " + cap.Message);
    }
    foreach (Action call in new Action[] {
        () => engine.Call("{\"decide\":\"Is it?\",\"evidence\":\"zero-call\"}", TimeSpan.Zero),
        () => engine.Recognize("{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}", "zero-recognize", TimeSpan.Zero),
        () => engine.RelateWithOptions("{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}",
            new[] { "{\"name\":\"A\",\"kind\":\"alert\"}", "{\"name\":\"B\",\"kind\":\"alert\"}" }, TimeSpan.Zero, default) }) {
        Failure zero = Refused(call);
        if (zero.Kind != FailureKind.Deadline || zero.Code != 3) throw new Exception("zero budget: " + zero.Message);
    }
    return "{\"limits\":\"pass\"}";
}
