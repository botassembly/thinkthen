using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace ThinkThen;

public sealed record ClientSettings
{
    public string? Mode { get; init; }
    public string? Adapter { get; init; }
    public string? Url { get; init; }
    public string? KeySource { get; init; }
    public string? Model { get; init; }
    public double? Batch { get; init; }
}
public record CallOptions
{
    public Threshold? Threshold { get; init; }
    public string? On { get; init; }
}
public sealed record RankOptions : CallOptions
{
    public double? Top { get; init; }
    public IReadOnlyList<string>? Levels { get; init; }
}
public readonly record struct Threshold
{
    internal object Value { get; }
    private Threshold(object value) => Value = value;
    public static implicit operator Threshold(double value) => new(value);
    public static implicit operator Threshold(string value) => new(value);
}
public sealed class Record
{
    public bool IsText { get; }
    public string? TextValue { get; }
    public JsonElement? JsonValue { get; }
    private Record(string value) { IsText = true; TextValue = value; }
    private Record(JsonElement value) => JsonValue = value.Clone();
    public static Record Text(string value) => new(value);
    public static Record Json(JsonElement value) => new(value);
    public static implicit operator Record(string value) => Text(value);
    internal byte[] Bytes => Encoding.UTF8.GetBytes(JsonValue!.Value.GetRawText());
}
public sealed class Question
{
    internal JsonElement Value { get; }
    private Question(object value) => Value = JsonSerializer.SerializeToElement(value, Json.Options);
    public static Question Inline(string text) => new(new { type = "inline", value = text });
    public static Question Kinds(params string[] kinds) => new(new { type = "inline", value = kinds });
    public static Question Relations(IReadOnlyList<RelationRule> rules) => new(new { type = "inline", value = rules });
    public static Question Toolbox(JsonElement value, string? name = null) => new(new { type = "toolbox", value, name });
    public static Question Reference(string name) => new(new { type = "reference", name });
    public static implicit operator Question(string text) => Inline(text);
}
public sealed record RelationRule(string Name, string Source, string Target, string? Reads = null, bool? Either = null);
public enum RowStatus { Answered, Unsure, Failed }
public sealed record ThinkThenError(string Code, string Message);
public sealed class ThinkThenException : Exception
{
    public string Code { get; }
    internal ThinkThenException(ThinkThenError error) : base(error.Message) => Code = error.Code;
}
public sealed record TextSpan(string Text, int Start, int End);
public sealed record RecognizedName(string Text, int Start, int End, string Kind, double P);
public sealed record RelationName(string Text, int Start, int End, string Kind);
public sealed record RelationEdge(RelationName Source, RelationName Target, string Relation, double P, bool? Either);

public sealed class ResultRow<TAnswer>
{
    public JsonElement Id { get; }
    public RowStatus Status { get; }
    public TAnswer? Answer { get; }
    public double? P { get; }
    public ThinkThenError? Error { get; }
    private readonly Lazy<JsonElement?> detail;
    public JsonElement? Detail => detail.Value;
    internal ResultRow(byte[] fields, byte[] rawDetail)
    {
        using var document = JsonDocument.Parse(fields);
        var row = document.RootElement;
        Id = row.GetProperty("id").Clone();
        Status = Enum.Parse<RowStatus>(row.GetProperty("status").GetString()!, true);
        Answer = row.TryGetProperty("answer", out var answer) && answer.ValueKind != JsonValueKind.Null
            ? answer.Deserialize<TAnswer>(Json.Options) : default;
        P = row.TryGetProperty("p", out var p) ? p.GetDouble() : null;
        Error = row.TryGetProperty("error", out var error) ? error.Deserialize<ThinkThenError>(Json.Options) : null;
        detail = new(() => rawDetail.Length == 0 ? null : Json.Parse(rawDetail));
    }
}

internal static class Json
{
    internal static readonly JsonSerializerOptions Options = CreateOptions();
    private static JsonSerializerOptions CreateOptions()
    {
        var options = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
            PropertyNameCaseInsensitive = true,
            DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
        };
        options.Converters.Add(new ReadOnlyListConverter());
        return options;
    }
    internal static JsonElement Parse(byte[] bytes)
    {
        using var document = JsonDocument.Parse(bytes);
        return document.RootElement.Clone();
    }
    internal static void Check(int status, Buffer error)
    {
        try
        {
            if (status != 0)
                throw new ThinkThenException(JsonSerializer.Deserialize<ThinkThenError>(Native.Copy(error), Options)!);
        }
        finally { Native.BufferFree(ref error); }
    }
    private sealed class ReadOnlyListConverter : JsonConverterFactory
    {
        public override bool CanConvert(Type type) => type.IsGenericType && type.GetGenericTypeDefinition() == typeof(IReadOnlyList<>);
        public override JsonConverter CreateConverter(Type type, JsonSerializerOptions options) =>
            (JsonConverter)Activator.CreateInstance(typeof(ListConverter<>).MakeGenericType(type.GetGenericArguments()))!;
    }
    private sealed class ListConverter<T> : JsonConverter<IReadOnlyList<T>>
    {
        public override IReadOnlyList<T> Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) =>
            JsonSerializer.Deserialize<List<T>>(ref reader, options)!.AsReadOnly();
        public override void Write(Utf8JsonWriter writer, IReadOnlyList<T> value, JsonSerializerOptions options)
        {
            writer.WriteStartArray();
            foreach (T item in value) JsonSerializer.Serialize(writer, item, options);
            writer.WriteEndArray();
        }
    }
}
