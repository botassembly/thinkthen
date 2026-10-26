using System.Text.Json;
using System.Text.Json.Nodes;
using System.Text.Json.Serialization;
namespace ThinkThen;

public sealed class ThinkThenOptionsConverter : JsonConverter<ThinkThenOptions>
{
    public ThinkThenOptionsConverter() { }
    public override ThinkThenOptions Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options)
    {
        using JsonDocument document = JsonDocument.ParseValue(ref reader);
        JsonElement value = document.RootElement;
        byte[] bytes = System.Text.Encoding.UTF8.GetBytes(value.GetRawText());
        NativeError error = NativeError.Empty;
        int result = Native.SettingsFromJson(bytes, (nuint)bytes.Length, out nint settings, ref error);
        try { Json.Check(result, error); } finally { if (settings != 0) Native.SettingsFree(settings); }
        var admitted = new ThinkThenOptions();
        if (value.TryGetProperty("backend", out var backend)) admitted.Backend = Json.Backend(backend.GetString()!);
        if (value.TryGetProperty("model", out var model) && model.ValueKind != JsonValueKind.Null) admitted.Model = model.GetString();
        if (value.TryGetProperty("url", out var url) && url.ValueKind != JsonValueKind.Null)
        {
            try { admitted.Url = new Uri(url.GetString()!); }
            catch (UriFormatException cause) { throw Json.Conversion(cause, settings: true); }
        }
        if (value.TryGetProperty("api_key", out var key) && key.ValueKind != JsonValueKind.Null) admitted.ApiKey = ApiKey.From(key.GetString()!);
        if (value.TryGetProperty("batch", out var batch) && batch.ValueKind != JsonValueKind.Null) admitted.Batch = Integer(batch);
        if (value.TryGetProperty("retry", out var retry) && retry.ValueKind != JsonValueKind.Null)
            admitted.Retry = new RetryOptions
            {
                MaxRetries = retry.TryGetProperty("max_retries", out var retries) ? Integer(retries) : null,
                InitialBackoff = Duration(retry, "initial_backoff_ms"),
                AttemptTimeout = Duration(retry, "attempt_timeout_ms")
            };
        return admitted;
    }
    private static int Integer(JsonElement value)
    {
        try { return value.GetInt32(); }
        catch (FormatException cause) { throw Json.Conversion(cause, settings: true); }
    }
    private static TimeSpan? Duration(JsonElement value, string field)
    {
        if (!value.TryGetProperty(field, out var milliseconds)) return null;
        try { return TimeSpan.FromTicks(checked(milliseconds.GetInt64() * TimeSpan.TicksPerMillisecond)); }
        catch (Exception cause) when (cause is OverflowException or FormatException)
        { throw Json.Conversion(cause, settings: true); }
    }
    public override void Write(Utf8JsonWriter writer, ThinkThenOptions value, JsonSerializerOptions options) => Node(value, false).WriteTo(writer);
    internal static JsonObject Node(ThinkThenOptions options, bool includeKey)
    {
        JsonObject settings = new() { ["backend"] = Contract.BackendName((int)options.Backend) };
        if (Contract.BackendName((int)options.Backend) is null) settings["backend"] = (int)options.Backend;
        if (options.Model is string model) settings["model"] = Json.HostText(model, true);
        if (options.Url is Uri url) settings["url"] = Json.HostText(url.OriginalString, true);
        if (options.Batch is int batch) settings["batch"] = batch;
        if (includeKey && options.ApiKey is ApiKey key) settings["api_key"] = Json.HostText(key.Value, true);
        if (options.Retry is RetryOptions retry)
        {
            JsonObject policy = new();
            if (retry.MaxRetries is int retries) policy["max_retries"] = retries;
            if (retry.InitialBackoff is TimeSpan initial) policy["initial_backoff_ms"] = Milliseconds(initial);
            if (retry.AttemptTimeout is TimeSpan attempt) policy["attempt_timeout_ms"] = Milliseconds(attempt);
            settings["retry"] = policy;
        }
        return settings;
    }
    private static long Milliseconds(TimeSpan value)
    {
        if (value.Ticks % TimeSpan.TicksPerMillisecond != 0) throw Json.Conversion(null, settings: true);
        return value.Ticks / TimeSpan.TicksPerMillisecond;
    }
}
