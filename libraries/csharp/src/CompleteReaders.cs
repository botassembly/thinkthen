using System.Text.Json;
namespace ThinkThen;

/// <summary>Serialized-field readers; compatibility replies are never promoted to complete results.</summary>
public static class CompleteReaders
{
    private static JsonElement Required(JsonElement fields, string name)
    {
        JsonElement value = fields.GetProperty(name);
        if (value.ValueKind == JsonValueKind.Null) throw new JsonException("missing " + name);
        return value;
    }
    private static string Text(JsonElement value) => value.GetString() ?? throw new JsonException("expected string");
    private static Optional<T> Option<T>(JsonElement f, string name, Func<JsonElement, T> read) =>
        f.TryGetProperty(name, out _) ? Optional.Some(read(Required(f, name))) : default;
    private static double Probability(JsonElement value)
    {
        double n = value.GetDouble();
        if (!double.IsFinite(n) || n < 0 || n > 1) throw new JsonException("invalid probability");
        return n;
    }
    /// <summary>Requires an actual result/2 call ID; legacy facts refuse.</summary>
    public static CallFacts Facts(string json)
    {
        using JsonDocument document = JsonDocument.Parse(json);
        JsonElement f = document.RootElement;
        Optional<string> cost = Option(f, "estimated_cost_usd", Text);
        if (cost.Present && !System.Text.RegularExpressions.Regex.IsMatch(cost.Value, @"\A[0-9]+\.[0-9]{6}\z"))
            throw new JsonException("invalid exact cost");
        double seconds = Required(f, "seconds").GetDouble();
        if (!double.IsFinite(seconds) || seconds < 0) throw new JsonException("invalid seconds");
        return new(new CallId(Text(Required(f, "call_id"))), Required(f, "cache_answers").GetUInt64(), cost,
            Option(f, "input_tokens", v => v.GetUInt64()), Option(f, "model", Text),
            Option(f, "output_tokens", v => v.GetUInt64()), Required(f, "records").GetUInt64(),
            Required(f, "requests_sent").GetUInt64(), seconds, Option(f, "command_ms", v => v.GetUInt64()));
    }
    public static IReadOnlyList<Probability> Probabilities(string json)
    {
        using JsonDocument document = JsonDocument.Parse(json);
        return Probabilities(document.RootElement);
    }
    private static IReadOnlyList<Probability> Probabilities(JsonElement fields)
    {
        var result = new List<Probability>(); var seen = new HashSet<string>();
        foreach (JsonProperty entry in fields.EnumerateObject())
        {
            if (!seen.Add(entry.Name)) throw new JsonException("duplicate probability name");
            result.Add(new(entry.Name, Probability(entry.Value)));
        }
        return result.AsReadOnly();
    }
    public static AtomicAnswer Atomic(string json)
    {
        using JsonDocument document = JsonDocument.Parse(json);
        JsonElement f = document.RootElement;
        AtomicKind kind = Text(Required(f, "kind")) switch {
            "yes_no" => AtomicKind.YesNo, "choice" => AtomicKind.Choice, "tag" => AtomicKind.Tag,
            "score" => AtomicKind.Score, "find" => AtomicKind.Find, _ => throw new JsonException("unknown atomic answer kind") };
        if (kind == AtomicKind.YesNo) return new(kind, Optional.Some(Probability(Required(f, "probability"))), default, default, Array.Empty<Probability>(), default);
        Optional<string> pick = kind is AtomicKind.Choice or AtomicKind.Find ? Optional.Some(Text(Required(f, "pick"))) : default;
        Optional<string> level = kind == AtomicKind.Score ? Optional.Some(Text(Required(f, "level"))) : default;
        Optional<double> confidence = kind != AtomicKind.Tag ? Option(f, "confidence", Probability) : default;
        return new(kind, default, pick, level, Probabilities(Required(f, "probabilities")), confidence);
    }
}
