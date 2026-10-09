// Generated from Rust result types; do not edit.
#nullable enable
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace ThinkThen.Results;

public enum PresenceState { Missing, Null, Value }

public readonly struct Presence<T>
{
    private readonly T? value;
    public PresenceState State { get; }
    private Presence(PresenceState state, T? value) { State = state; this.value = value; }
    public T Value => State == PresenceState.Value ? value! :
        throw new InvalidOperationException($"Member is {State}.");
    internal static Presence<T> Null => new(PresenceState.Null, default);
    internal static Presence<T> Present(T value) => new(PresenceState.Value, value);
}

public abstract class ResultObject
{
    private readonly JsonElement document;
    protected ResultObject(JsonElement document) { this.document = document.Clone(); }
    protected JsonElement Required(string name) => document.GetProperty(name);
    protected Presence<T> Optional<T>(string name, Func<JsonElement, T> read)
    {
        if (!document.TryGetProperty(name, out var member)) return default;
        return member.ValueKind == JsonValueKind.Null ? Presence<T>.Null : Presence<T>.Present(read(member));
    }
    // Cloning and plain conversion retain every JSON member, including unknown nested data.
    public JsonElement ToJson() => document.Clone();
    public JsonObject ToPlain() => JsonNode.Parse(document.GetRawText())!.AsObject();
    public string ToJsonString() => document.GetRawText();
}
public sealed class Attempt : ResultObject
{
    public Attempt(JsonElement document) : base(document) { }
    public ulong Ordinal => Required("ordinal").GetUInt64();
    public AttemptOutcome Outcome => new AttemptOutcome(Required("outcome").GetString()!);
    public Presence<string> RequestId => Optional<string>("request_id", member => member.GetString()!);
    public string RequestSha256 => Required("request_sha256").GetString()!;
    public string SdkRequestId => Required("sdk_request_id").GetString()!;
    public Presence<ulong> ServerMs => Optional<ulong>("server_ms", member => member.GetUInt64());
    public Presence<ushort> Status => Optional<ushort>("status", member => member.GetUInt16());
    public ulong WallMs => Required("wall_ms").GetUInt64();
}

public sealed class Facts : ResultObject
{
    public Facts(JsonElement document) : base(document) { }
    public Presence<IReadOnlyList<Attempt>> Attempts => Optional<IReadOnlyList<Attempt>>("attempts", member => Array.AsReadOnly(member.EnumerateArray().Select(item => new Attempt(item)).ToArray()));
    public ulong CacheAnswers => Required("cache_answers").GetUInt64();
    public string CallId => Required("call_id").GetString()!;
    public Presence<string> EstimatedCostUsd => Optional<string>("estimated_cost_usd", member => member.GetString()!);
    public Presence<bool> HeldModelMismatch => Optional<bool>("held_model_mismatch", member => member.GetBoolean());
    public Presence<ulong> InputTokens => Optional<ulong>("input_tokens", member => member.GetUInt64());
    public ulong LargestRequestBytes => Required("largest_request_bytes").GetUInt64();
    public Presence<ulong> LargestRequestEstimatedInputTokens => Optional<ulong>("largest_request_estimated_input_tokens", member => member.GetUInt64());
    public Presence<string> Model => Optional<string>("model", member => member.GetString()!);
    public Presence<ulong> OutputTokens => Optional<ulong>("output_tokens", member => member.GetUInt64());
    public ulong Records => Required("records").GetUInt64();
    public ulong RequestsSent => Required("requests_sent").GetUInt64();
    public double Seconds => Required("seconds").GetDouble();
    public string TokenEstimateMethod => Required("token_estimate_method").GetString()!;
    public Presence<PersistenceObservation> UsagePersistence => Optional<PersistenceObservation>("usage_persistence", member => new PersistenceObservation(member));
}

public sealed class PersistenceObservation : ResultObject
{
    public PersistenceObservation(JsonElement document) : base(document) { }
    public Presence<string> Advice => Optional<string>("advice", member => member.GetString()!);
    public string ObservedAt => Required("observed_at").GetString()!;
    public UsagePersistence State => new UsagePersistence(Required("state").GetString()!);
}

public readonly record struct UsagePersistence(string Value)
{
    public static UsagePersistence Disabled => new("disabled");
    public static UsagePersistence Pending => new("pending");
    public static UsagePersistence Written => new("written");
    public static UsagePersistence Failed => new("failed");
}

public readonly record struct AttemptOutcome(string Value)
{
    public static AttemptOutcome Ok => new("ok");
    public static AttemptOutcome Status => new("status");
    public static AttemptOutcome Transport => new("transport");
}

