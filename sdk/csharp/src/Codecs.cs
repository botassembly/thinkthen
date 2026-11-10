using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Text.Json.Serialization;
namespace ThinkThen;

internal static class Json
{
    internal static ThinkThenException Abi(Exception? cause = null) => new(new(ErrorCode.UsageAbi, Contract.AbiMessage), cause);
    internal static void Check(int status, NativeError error)
    {
        try
        {
            if (status == 0) return;
            string? name = Contract.CodeName(error.Code);
            if (name is null || Marshal.PtrToStringUTF8(Native.CodeName(error.Code)) != name) throw Abi();
            throw new ThinkThenException(new((ErrorCode)error.Code, Encoding.UTF8.GetString(Native.Copy(error.Message)),
                error.HttpStatus == 0 ? null : error.HttpStatus));
        }
        finally { Native.ErrorFree(ref error); }
    }
    internal static ThinkThenException Conversion(Exception? cause, bool settings = false)
    {
        NativeError error = NativeError.Empty;
        int result = settings ? Native.SettingsConversionError(ref error) : Native.QuestionConversionError(ref error);
        try { Check(result, error); }
        catch (ThinkThenException failure) { return new(new(failure.ErrorCode, failure.Message, failure.HttpStatus), cause); }
        throw Abi();
    }
    internal static string HostText(string value, bool settings = false)
    {
        ArgumentNullException.ThrowIfNull(value);
        try { _ = new UTF8Encoding(false, true).GetByteCount(value); return value; }
        catch (Exception cause) when (cause is EncoderFallbackException or ArgumentNullException) { throw Conversion(cause, settings); }
    }
    internal static ErrorCode Code(string name)
    {
        foreach (ErrorCode code in Enum.GetValues<ErrorCode>()) if (Contract.CodeName((int)code) == name) return code;
        throw Abi();
    }
    internal static Backend Backend(string name)
    {
        foreach (Backend backend in Enum.GetValues<Backend>()) if (Contract.BackendName((int)backend) == name) return backend;
        throw Abi();
    }
    internal static Function Function(string name)
    {
        foreach (Function function in Enum.GetValues<Function>()) if (Contract.FunctionName((int)function) == name) return function;
        throw Abi();
    }
}
/// <summary>Looks up error families from the shared engine registry.</summary>
public static class ErrorCodes
{
    /// <summary>Gets this code's family from the shared engine.</summary>
    public static ErrorFamily Family(this ErrorCode code) => (Marshal.PtrToStringUTF8(Native.CodeFamily((int)code))) switch
    { "usage" => ErrorFamily.Usage, "record" => ErrorFamily.Record, "backend" => ErrorFamily.Backend,
      "local" => ErrorFamily.Local, "internal" => ErrorFamily.Internal, _ => throw Json.Abi() };
}
/// <summary>Serializes shared error codes by their stable wire name.</summary>
public sealed class ErrorCodeConverter : JsonConverter<ErrorCode>
{
    /// <summary>Serializes shared error codes by their stable wire name.</summary>
    public ErrorCodeConverter() { }
    /// <inheritdoc/>
    public override ErrorCode Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) => Json.Code(reader.GetString()!);
    /// <inheritdoc/>
    public override void Write(Utf8JsonWriter writer, ErrorCode value, JsonSerializerOptions options) =>
        writer.WriteStringValue(Contract.CodeName((int)value) ?? throw Json.Abi());
}
/// <summary>Serializes backend names using the engine registry.</summary>
public sealed class BackendConverter : JsonConverter<Backend>
{
    /// <summary>Serializes backend names using the engine registry.</summary>
    public BackendConverter() { }
    /// <inheritdoc/>
    public override Backend Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) => Json.Backend(reader.GetString()!);
    /// <inheritdoc/>
    public override void Write(Utf8JsonWriter writer, Backend value, JsonSerializerOptions options) =>
        writer.WriteStringValue(Contract.BackendName((int)value) ?? throw Json.Abi());
}
[JsonSourceGenerationOptions(PropertyNamingPolicy = JsonKnownNamingPolicy.SnakeCaseLower)]
[JsonSerializable(typeof(JsonElement))]
[JsonSerializable(typeof(Usage))]
internal partial class ContractJson : JsonSerializerContext { }

internal static class Codecs
{
    internal static Row<TAnswer,TDetail> Row<TAnswer,TDetail>(uint function, uint status, int errorCode, ulong position, double probability, byte[] id,
        byte[] answer, byte[] detail, byte[] source, byte[] error, string expectedFunction)
    {
        try
        {
            if (Contract.FunctionName(function) != expectedFunction) throw Json.Abi();
            if (answer.Length != 0 && Contract.StatusName(status) != "answered") throw Json.Abi();
            JsonObject raw = new() { ["function"] = expectedFunction, ["position"] = position,
                ["status"] = Contract.StatusName(status), ["source"] = JsonNode.Parse(source) };
            if (id.Length > 0) raw["id"] = JsonNode.Parse(id);
            if (answer.Length > 0) raw["answer"] = JsonNode.Parse(answer);
            if (detail.Length > 0) raw["detail"] = JsonNode.Parse(detail);
            if (error.Length > 0) raw["error"] = JsonNode.Parse(error);
            if (!double.IsNaN(probability)) raw["probability"] = probability;
            using var document = JsonDocument.Parse(raw.ToJsonString());
            JsonElement value = document.RootElement;
            ValidateOutcome(value);
            JsonElement route = value.GetProperty("source");
            foreach (var field in route.EnumerateObject())
                if (field.Name is not ("question_identity" or "backend" or "model" or "name")) throw Json.Abi();
            Source typedSource = new(route.GetProperty("question_identity").GetString()!,
                Json.Backend(route.GetProperty("backend").GetString()!), route.GetProperty("model").GetString()!,
                route.TryGetProperty("name", out var name) ? name.GetString() : null);
            RecordId? typedId = id.Length == 0 ? null : Id(value.GetProperty("id"));
            TDetail? typedDetail = detail.Length == 0 ? default : Detail<TDetail>(expectedFunction, value.GetProperty("detail"));
            var kind = (Function)function;
            Row<TAnswer,TDetail> result = status switch
            {
                1 => new Answered<TAnswer,TDetail>(kind, position, typedId, typedSource,
                    Answer<TAnswer>(expectedFunction, value.GetProperty("answer")), double.IsNaN(probability) ? null : probability, typedDetail),
                2 => new Unsure<TAnswer,TDetail>(kind, position, typedId, typedSource, probability, typedDetail),
                3 => new Failed<TAnswer,TDetail>(kind, position, typedId, typedSource, Error(value.GetProperty("error"))),
                _ => throw Json.Abi()
            };
            return result with { Canonical = value.Clone() };
        }
        catch (Exception cause) when (cause is JsonException or InvalidOperationException or KeyNotFoundException or FormatException or OverflowException)
        { throw Json.Abi(cause); }
    }
    private static RecordId Id(JsonElement value) => value.ValueKind switch
    {
        JsonValueKind.String => new StringId(value.GetString()!),
        JsonValueKind.Number when value.TryGetInt64(out long signed) => new SignedId(signed),
        JsonValueKind.Number when value.TryGetUInt64(out ulong unsigned) => new UnsignedId(unsigned),
        _ => throw Json.Abi()
    };
    private static RowError Error(JsonElement value) => new(Json.Code(value.GetProperty("code").GetString()!),
        value.GetProperty("message").GetString()!, value.TryGetProperty("http_status", out var status) ? status.GetInt32() : null);
    private static TextSpan Span(JsonElement value) => new(value.GetProperty("text").GetString()!,
        value.GetProperty("start").GetInt32(), value.GetProperty("end").GetInt32());
    private static Mention Mention(JsonElement value) => new(Span(value), value.GetProperty("kind").GetString()!);
    private static Entity Entity(JsonElement value) => new(Mention(value), value.GetProperty("probability").GetDouble());
    private static Relation Relation(JsonElement value) => new(Mention(value.GetProperty("source")),
        Mention(value.GetProperty("target")), value.GetProperty("relation").GetString()!, value.GetProperty("probability").GetDouble(),
        value.TryGetProperty("either", out var either) && either.GetBoolean());
    private static ValueMap<string,double> Map(JsonElement value) => value.EnumerateObject()
        .ToValueMap(pair => pair.Name, pair => pair.Value.GetDouble());
    private static BoundaryTag Boundary(string name) => name switch
    { "BEGIN" => BoundaryTag.Begin, "END" => BoundaryTag.End, "INSIDE" => BoundaryTag.Inside,
        "OUT" => BoundaryTag.Out, "SINGLE" => BoundaryTag.Single, _ => throw Json.Abi() };
    private static TAnswer Answer<TAnswer>(string function, JsonElement value)
    {
        object? answer = function switch
        {
            "decide" or "filter" => value.GetBoolean(),
            "choose" => value.GetString(),
            "tag" => value.EnumerateArray().Select(item => item.GetString()!).ToValueList(),
            "score" => value.GetDouble(),
            "rank" => value.GetInt64(),
            "find" => value.ValueKind == JsonValueKind.Null ? null : Span(value),
            "recognize" => value.EnumerateArray().Select(Entity).ToValueList(),
            "relate" => value.EnumerateArray().Select(Relation).ToValueList(),
            "annotate" => value.EnumerateObject().ToValueMap(pair => pair.Name, pair => Entry(pair.Value)),
            _ => throw Json.Abi()
        };
        return (TAnswer)answer!;
    }
    private static TDetail? Detail<TDetail>(string function, JsonElement value)
    {
        object detail = function switch
        {
            "choose" or "tag" => new LabelsDetail(Map(value.GetProperty("labels"))),
            "score" or "rank" => new LevelsDetail(value.GetProperty("levels").EnumerateArray().Select(item => item.GetDouble()).ToValueList()),
            "recognize" => new RecognizeDetail(value.GetProperty("pieces").EnumerateArray().Select(item =>
                new PieceDetail(Span(item), item.GetProperty("tags").EnumerateObject().ToValueMap(pair => Boundary(pair.Name), pair => pair.Value.GetDouble()))).ToValueList(),
                value.GetProperty("names").EnumerateArray().Select(item => new KindDetail(item.GetProperty("text").GetString()!, item.GetProperty("start").GetInt32(), item.GetProperty("end").GetInt32(), Map(item.GetProperty("kinds")))).ToValueList()),
            "relate" => new RelateDetail(value.GetProperty("names").EnumerateArray().Select(Entity).ToValueList(), value.GetProperty("pairs").EnumerateArray().Select(Relation).ToValueList()),
            _ => throw Json.Abi()
        };
        return (TDetail)detail;
    }
    private static EntryOutcome<TAnswer,TDetail> Outcome<TAnswer,TDetail>(string function, JsonElement value)
    {
        ValidateOutcome(value);
        double? probability = value.TryGetProperty("probability", out var p) ? p.GetDouble() : null;
        TDetail? detail = value.TryGetProperty("detail", out var d) ? Detail<TDetail>(function, d) : default;
        return value.GetProperty("status").GetString() switch
        {
            "answered" => new EntryAnswered<TAnswer,TDetail>(Answer<TAnswer>(function, value.GetProperty("answer")), probability, detail),
            "unsure" when probability is double number => new EntryUnsure<TAnswer,TDetail>(number, detail),
            "failed" => new EntryFailed<TAnswer,TDetail>(Error(value.GetProperty("error"))),
            _ => throw Json.Abi()
        };
    }
    private static void ValidateOutcome(JsonElement value)
    {
        bool Has(string field) => value.TryGetProperty(field, out _);
        bool valid = value.GetProperty("status").GetString() switch
        {
            "answered" => Has("answer") && !Has("error"),
            "unsure" => !Has("answer") && !Has("error") && Has("probability"),
            "failed" => !Has("answer") && !Has("probability") && !Has("detail") && Has("error"),
            _ => false
        };
        if (!valid) throw Json.Abi();
    }
    private static AnnotationEntry Entry(JsonElement value) => value.GetProperty("function").GetString() switch
    {
        "decide" => new DecideEntry(Outcome<bool,NoDetail>("decide", value)),
        "filter" => new FilterEntry(Outcome<bool,NoDetail>("filter", value)),
        "choose" => new ChooseEntry(Outcome<string,LabelsDetail>("choose", value)),
        "tag" => new TagEntry(Outcome<ValueList<string>,LabelsDetail>("tag", value)),
        "score" => new ScoreEntry(Outcome<double,LevelsDetail>("score", value)),
        "find" => new FindEntry(Outcome<TextSpan?,NoDetail>("find", value)),
        "recognize" => new RecognizeEntry(Outcome<ValueList<Entity>,RecognizeDetail>("recognize", value)),
        "relate" => new RelateEntry(Outcome<ValueList<Relation>,RelateDetail>("relate", value)),
        _ => throw Json.Abi()
    };
}
