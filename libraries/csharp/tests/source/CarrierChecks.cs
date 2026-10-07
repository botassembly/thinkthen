using ThinkThen;
using System.Text.Json;

// Pure fixtures only: no native call or parity result is produced here.
static class CarrierChecks
{
    static void Check(bool value) { if (!value) throw new Exception("owned carrier contract"); }
    static void Refuses(Action action)
    {
        try { action(); }
        catch (Exception error) when (error is JsonException or ArgumentException or KeyNotFoundException or InvalidOperationException or FormatException) { return; }
        throw new Exception("invalid serialized field accepted");
    }
    public static void Run()
    {
        string id = new('a', 64);
        CallFacts facts = CompleteReaders.Facts("{\"call_id\":\""+id+"\",\"cache_answers\":0,\"records\":0,\"requests_sent\":0,\"seconds\":0,\"input_tokens\":0,\"estimated_cost_usd\":\"0.000000\"}");
        Check(facts.CallId.Value == id && facts.InputTokens.Present && facts.InputTokens.Value == 0 && !facts.OutputTokens.Present);
        Check(facts.EstimatedCostUsd.Value == "0.000000");
        Refuses(() => CompleteReaders.Facts("{\"cache_answers\":0,\"records\":0,\"requests_sent\":0,\"seconds\":0}"));
        Refuses(() => new AnswerId(new string('A', 64)));
        Refuses(() => CompleteReaders.Atomic("{\"kind\":\"other\"}"));
        Refuses(() => CompleteReaders.Atomic("{\"kind\":\"yes_no\",\"probability\":null}"));
        Refuses(() => CompleteReaders.Atomic("{\"kind\":\"tag\",\"probabilities\":{\"a\":null}}"));
        Refuses(() => CompleteReaders.Probabilities("{\"a\":1,\"a\":0}"));
        AtomicAnswer choice = CompleteReaders.Atomic("{\"kind\":\"choice\",\"pick\":\"β\",\"probabilities\":{\"β\":0.7,\"a\":0.3},\"confidence\":0}");
        Check(choice.Probabilities[0].Name == "β" && choice.Confidence.Present && choice.Confidence.Value == 0);
        foreach (string fixture in new[] { "{\"kind\":\"yes_no\",\"probability\":0}", "{\"kind\":\"tag\",\"probabilities\":{}}", "{\"kind\":\"score\",\"level\":\"a\",\"probabilities\":{\"a\":1}}", "{\"kind\":\"find\",\"pick\":\"a\",\"probabilities\":{\"a\":1}}" }) CompleteReaders.Atomic(fixture);
        var question = QuestionInput.QuestionFile("β.json");
        var source = InputSource.FromFiles(new FileSource(new[] { "β.png", "β.png" }, SourceUnit.ImageFile, 0));
        var controls = new CallControls(default, default, false, true);
        CompleteRequest[] requests = { Requests.Decide(question, source, controls), Requests.Choose(question, source, controls), Requests.Tag(question, source, controls), Requests.Score(question, source, controls), Requests.Filter(question, source, controls), Requests.Rank(question, source, controls), Requests.Find(question, source, controls), Requests.Annotate(question, source, controls), Requests.Recognize(question, source, controls), Requests.Relate(question, source, controls) };
        Function[] functions = { Function.Decide, Function.Choose, Function.Tag, Function.Score, Function.Filter, Function.Rank, Function.Find, Function.Annotate, Function.Recognize, Function.Relate };
        for (int i=0; i<requests.Length; i++) Check(requests[i].Function == functions[i]);
        Check(question.File.Present && !question.Question.Present && source.Files.Value.Paths.Count == 2);
        var span = new NameSpan(1, 3, Optional.Some<IReadOnlyList<Probability>>(Array.Empty<Probability>()), default);
        Check(span.Kinds.Present && !span.Edges.Present && span.End == 3);
        Content text = Questions.Decide(new Content(ContentKind.Text, "q.json", default)).Text;
        string textJson = JsonSerializer.Serialize(text);
        Check(JsonSerializer.Deserialize<Content>(textJson)!.Text == "q.json");
        using JsonDocument payload = JsonDocument.Parse("false");
        var authored = new Content(ContentKind.Json, "", payload.RootElement.Clone());
        string serialized = JsonSerializer.Serialize(authored);
        Content decoded = JsonSerializer.Deserialize<Content>(serialized)!;
        Check(decoded.Json.ValueKind == JsonValueKind.False);
        var identity = new ObservationIdentity(IdentityKind.Observation, Optional.Some(new ObservationId(new string('d', 64))), default);
        ObservationIdentity[] duplicates = { identity, identity };
        Check(duplicates.Length == 2 && duplicates[0].ObservationId.Value == duplicates[1].ObservationId.Value);
        var error = new CompleteError(5, "cancelled", false, default, Optional.Some(facts), Optional.Some<IReadOnlyList<Attempt>>(Array.Empty<Attempt>()));
        Check(error.Facts.Present && error.Attempts.Present && error.Attempts.Value.Count == 0);
        var failed = new MemberFailure(new FailureId(new string('b', 64)), MemberCause.MissingProbability);
        Check(failed.Cause == MemberCause.MissingProbability);
    }
}
