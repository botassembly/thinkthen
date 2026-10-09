// Generated from Rust result types; do not edit.
#nullable enable
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
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
    protected JsonElement RequiredElement(string name) => document.GetProperty(name);
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
public sealed class Annotation : ResultObject
{
    public Annotation(JsonElement document) : base(document) { }
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public IReadOnlyDictionary<string, AnnotationMember> Answers => new ReadOnlyDictionary<string, AnnotationMember>(RequiredElement("answers").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => global::ThinkThen.Results.AnnotationMember.Read(entry0.Value), StringComparer.Ordinal));
    public Presence<string> File => Optional<string>("file", member => member.GetString()!);
    public Presence<ulong> FirstLine => Optional<ulong>("first_line", member => member.GetUInt64());
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public Presence<ulong> LastLine => Optional<ulong>("last_line", member => member.GetUInt64());
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Presence<Position> Position => Optional<Position>("position", member => new Position(member));
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
    public IReadOnlyDictionary<string, AnnotatedField> Value => new ReadOnlyDictionary<string, AnnotatedField>(RequiredElement("value").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => global::ThinkThen.Results.AnnotatedField.Read(entry0.Value), StringComparer.Ordinal));
}

public abstract class AnnotationMember : ResultObject
{
    protected AnnotationMember(JsonElement document) : base(document) { }
    public static AnnotationMember Read(JsonElement document)
    {
        AnnotationMember? result = null;
        if (document.TryGetProperty("answer_id", out var tagAnnotationMemberAnswerId))
        {
            if (!(tagAnnotationMemberAnswerId.ValueKind == JsonValueKind.String)) throw new JsonException("Invalid result identity kind.");
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnnotationMemberAnswerId(document);
        }
        if (document.TryGetProperty("failure_id", out var tagAnnotationMemberFailureId))
        {
            if (!(tagAnnotationMemberFailureId.ValueKind == JsonValueKind.String)) throw new JsonException("Invalid result identity kind.");
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnnotationMemberFailureId(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class AnnotationMemberAnswerId : AnnotationMember
{
    public AnnotationMemberAnswerId(JsonElement document) : base(document) { }
    public Answer Answer => global::ThinkThen.Results.Answer.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public IReadOnlyList<Observation> Observations => Array.AsReadOnly(RequiredElement("observations").EnumerateArray().Select(item0 => global::ThinkThen.Results.Observation.Read(item0)).ToArray());
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public IReadOnlyList<QuestionSource> QuestionSources => Array.AsReadOnly(RequiredElement("question_sources").EnumerateArray().Select(item0 => new QuestionSource(item0)).ToArray());
    public string Request => RequiredElement("request").GetString()!;
    public Presence<Threshold> Threshold => Optional<Threshold>("threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public Presence<Usage> Usage => Optional<Usage>("usage", member => new Usage(member));
    public Presence<Value> Value => Optional<Value>("value", member => global::ThinkThen.Results.Value.Read(member));
}

public sealed class AnnotationMemberFailureId : AnnotationMember
{
    public AnnotationMemberFailureId(JsonElement document) : base(document) { }
    public Failure Failure => new Failure(RequiredElement("failure"));
    public string FailureId => RequiredElement("failure_id").GetString()!;
    public IReadOnlyList<Observation> Observations => Array.AsReadOnly(RequiredElement("observations").EnumerateArray().Select(item0 => global::ThinkThen.Results.Observation.Read(item0)).ToArray());
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public IReadOnlyList<QuestionSource> QuestionSources => Array.AsReadOnly(RequiredElement("question_sources").EnumerateArray().Select(item0 => new QuestionSource(item0)).ToArray());
    public string Request => RequiredElement("request").GetString()!;
    public Presence<Threshold> Threshold => Optional<Threshold>("threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public Presence<Usage> Usage => Optional<Usage>("usage", member => new Usage(member));
}

public abstract class AnnotationValue : ResultObject
{
    protected AnnotationValue(JsonElement document) : base(document) { }
    public static AnnotationValue Read(JsonElement document)
    {
        AnnotationValue? result = null;
        if (document.TryGetProperty("kind", out var tagAnnotationValueDecision) && tagAnnotationValueDecision.ValueKind == JsonValueKind.String && tagAnnotationValueDecision.GetString() == "decision")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnnotationValueDecision(document);
        }
        if (document.TryGetProperty("kind", out var tagAnnotationValueChoice) && tagAnnotationValueChoice.ValueKind == JsonValueKind.String && tagAnnotationValueChoice.GetString() == "choice")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnnotationValueChoice(document);
        }
        if (document.TryGetProperty("kind", out var tagAnnotationValueScore) && tagAnnotationValueScore.ValueKind == JsonValueKind.String && tagAnnotationValueScore.GetString() == "score")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnnotationValueScore(document);
        }
        if (document.TryGetProperty("kind", out var tagAnnotationValueTags) && tagAnnotationValueTags.ValueKind == JsonValueKind.String && tagAnnotationValueTags.GetString() == "tags")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnnotationValueTags(document);
        }
        if (document.TryGetProperty("kind", out var tagAnnotationValueFailed) && tagAnnotationValueFailed.ValueKind == JsonValueKind.String && tagAnnotationValueFailed.GetString() == "failed")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnnotationValueFailed(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class AnnotationValueChoice : AnnotationValue
{
    public AnnotationValueChoice(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public Presence<string> Value => Optional<string>("value", member => member.GetString()!);
}

public sealed class AnnotationValueDecision : AnnotationValue
{
    public AnnotationValueDecision(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public Presence<bool> Value => Optional<bool>("value", member => member.GetBoolean());
}

public sealed class AnnotationValueFailed : AnnotationValue
{
    public AnnotationValueFailed(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public Failure Value => new Failure(RequiredElement("value"));
}

public sealed class AnnotationValueScore : AnnotationValue
{
    public AnnotationValueScore(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public double Value => RequiredElement("value").GetDouble();
}

public sealed class AnnotationValueTags : AnnotationValue
{
    public AnnotationValueTags(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<string> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
}

public sealed class Answers : ResultObject
{
    public Answers(JsonElement document) : base(document) { }
    public IReadOnlyList<RelationMember> Questions => Array.AsReadOnly(RequiredElement("questions").EnumerateArray().Select(item0 => global::ThinkThen.Results.RelationMember.Read(item0)).ToArray());
}

public sealed class AtomicArrayOfString : ResultObject
{
    public AtomicArrayOfString(JsonElement document) : base(document) { }
    public Answer Answer => global::ThinkThen.Results.Answer.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<IReadOnlyList<Image>> Images => Optional<IReadOnlyList<Image>>("images", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Image(item0)).ToArray()));
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public Presence<IReadOnlyList<RankMember>> Members => Optional<IReadOnlyList<RankMember>>("members", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new RankMember(item0)).ToArray()));
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public Presence<string> QuestionName => Optional<string>("question_name", member => member.GetString()!);
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
    public Presence<Threshold> Threshold => Optional<Threshold>("threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public IReadOnlyList<string> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
}

public sealed class AtomicDecideValue : ResultObject
{
    public AtomicDecideValue(JsonElement document) : base(document) { }
    public Answer Answer => global::ThinkThen.Results.Answer.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<IReadOnlyList<Image>> Images => Optional<IReadOnlyList<Image>>("images", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Image(item0)).ToArray()));
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public Presence<IReadOnlyList<RankMember>> Members => Optional<IReadOnlyList<RankMember>>("members", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new RankMember(item0)).ToArray()));
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public Presence<string> QuestionName => Optional<string>("question_name", member => member.GetString()!);
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
    public Presence<Threshold> Threshold => Optional<Threshold>("threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public JsonElement Value => RequiredElement("value").Clone();
}

public sealed class AtomicNonZeroUsize : ResultObject
{
    public AtomicNonZeroUsize(JsonElement document) : base(document) { }
    public Answer Answer => global::ThinkThen.Results.Answer.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<IReadOnlyList<Image>> Images => Optional<IReadOnlyList<Image>>("images", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Image(item0)).ToArray()));
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public Presence<IReadOnlyList<RankMember>> Members => Optional<IReadOnlyList<RankMember>>("members", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new RankMember(item0)).ToArray()));
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public Presence<string> QuestionName => Optional<string>("question_name", member => member.GetString()!);
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
    public Presence<Threshold> Threshold => Optional<Threshold>("threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public ulong Value => RequiredElement("value").GetUInt64();
}

public sealed class AtomicNullableString : ResultObject
{
    public AtomicNullableString(JsonElement document) : base(document) { }
    public Answer Answer => global::ThinkThen.Results.Answer.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<IReadOnlyList<Image>> Images => Optional<IReadOnlyList<Image>>("images", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Image(item0)).ToArray()));
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public Presence<IReadOnlyList<RankMember>> Members => Optional<IReadOnlyList<RankMember>>("members", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new RankMember(item0)).ToArray()));
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public Presence<string> QuestionName => Optional<string>("question_name", member => member.GetString()!);
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
    public Presence<Threshold> Threshold => Optional<Threshold>("threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public Presence<string> Value => Optional<string>("value", member => member.GetString()!);
}

public sealed class AtomicBoolean : ResultObject
{
    public AtomicBoolean(JsonElement document) : base(document) { }
    public Answer Answer => global::ThinkThen.Results.Answer.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<IReadOnlyList<Image>> Images => Optional<IReadOnlyList<Image>>("images", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Image(item0)).ToArray()));
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public Presence<IReadOnlyList<RankMember>> Members => Optional<IReadOnlyList<RankMember>>("members", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new RankMember(item0)).ToArray()));
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public Presence<string> QuestionName => Optional<string>("question_name", member => member.GetString()!);
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
    public Presence<Threshold> Threshold => Optional<Threshold>("threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public bool Value => RequiredElement("value").GetBoolean();
}

public sealed class AtomicDouble : ResultObject
{
    public AtomicDouble(JsonElement document) : base(document) { }
    public Answer Answer => global::ThinkThen.Results.Answer.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<IReadOnlyList<Image>> Images => Optional<IReadOnlyList<Image>>("images", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Image(item0)).ToArray()));
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public Presence<IReadOnlyList<RankMember>> Members => Optional<IReadOnlyList<RankMember>>("members", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new RankMember(item0)).ToArray()));
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public Presence<string> QuestionName => Optional<string>("question_name", member => member.GetString()!);
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
    public Presence<Threshold> Threshold => Optional<Threshold>("threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public double Value => RequiredElement("value").GetDouble();
}

public sealed class Attempt : ResultObject
{
    public Attempt(JsonElement document) : base(document) { }
    public ulong Ordinal => RequiredElement("ordinal").GetUInt64();
    public AttemptOutcome Outcome => new AttemptOutcome(RequiredElement("outcome").GetString()!);
    public Presence<string> RequestId => Optional<string>("request_id", member => member.GetString()!);
    public string RequestSha256 => RequiredElement("request_sha256").GetString()!;
    public string SdkRequestId => RequiredElement("sdk_request_id").GetString()!;
    public Presence<ulong> ServerMs => Optional<ulong>("server_ms", member => member.GetUInt64());
    public Presence<ushort> Status => Optional<ushort>("status", member => member.GetUInt16());
    public ulong WallMs => RequiredElement("wall_ms").GetUInt64();
}

public abstract class Batch
{
    private readonly JsonElement document;
    protected Batch(JsonElement document) { this.document = document.Clone(); }
    protected JsonElement Document => document;
    public JsonElement ToJson() => document.Clone();
    public JsonNode ToPlain() => JsonNode.Parse(document.GetRawText())!;
    public string ToJsonString() => document.GetRawText();
    public static Batch Read(JsonElement document) => document.ValueKind switch
    {
        JsonValueKind.Number => new BatchInteger(document),
        JsonValueKind.String => new BatchString(document),
        _ => throw new JsonException("Unknown primitive result variant.")
    };
}

public sealed class BatchInteger : Batch
{
    public BatchInteger(JsonElement document) : base(document) { }
    public ulong Value => Document.GetUInt64();
}

public sealed class BatchString : Batch
{
    public BatchString(JsonElement document) : base(document) { }
    public string Value => Document.GetString()!;
}

public readonly record struct BoundaryMode(string Value)
{
    public static BoundaryMode BoundaryOnly => new("boundary_only");
}

public sealed class BoundaryOdds : ResultObject
{
    public BoundaryOdds(JsonElement document) : base(document) { }
    public IReadOnlyList<PieceOdds> Pieces => Array.AsReadOnly(RequiredElement("pieces").EnumerateArray().Select(item0 => new PieceOdds(item0)).ToArray());
    public IReadOnlyList<BoundaryProposal> Proposals => Array.AsReadOnly(RequiredElement("proposals").EnumerateArray().Select(item0 => new BoundaryProposal(item0)).ToArray());
}

public sealed class BoundaryProposal : ResultObject
{
    public BoundaryProposal(JsonElement document) : base(document) { }
    public ulong End => RequiredElement("end").GetUInt64();
    public ulong Length => RequiredElement("length").GetUInt64();
    public double Probability => RequiredElement("probability").GetDouble();
    public ulong Start => RequiredElement("start").GetUInt64();
    public string Text => RequiredElement("text").GetString()!;
}

public sealed class CallError : ResultObject
{
    public CallError(JsonElement document) : base(document) { }
    public Error Error => new Error(RequiredElement("error"));
    public Presence<Facts> Facts => Optional<Facts>("facts", member => new Facts(member));
}

public sealed class EntityDocument : ResultObject
{
    public EntityDocument(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public string Name => RequiredElement("name").GetString()!;
}

public sealed class Error : ResultObject
{
    public Error(JsonElement document) : base(document) { }
    public Presence<EstimatedInputDenial> EstimatedInputDenial => Optional<EstimatedInputDenial>("estimated_input_denial", member => global::ThinkThen.Results.EstimatedInputDenial.Read(member));
    public FailureKind Kind => new FailureKind(RequiredElement("kind").GetString()!);
    public string Message => RequiredElement("message").GetString()!;
    public bool Retryable => RequiredElement("retryable").GetBoolean();
    public Presence<SendBudgetDenial> SendBudgetDenial => Optional<SendBudgetDenial>("send_budget_denial", member => global::ThinkThen.Results.SendBudgetDenial.Read(member));
    public Stopped Stopped => new Stopped(RequiredElement("stopped"));
}

public abstract class EstimatedInputDenial : ResultObject
{
    protected EstimatedInputDenial(JsonElement document) : base(document) { }
    public static EstimatedInputDenial Read(JsonElement document)
    {
        EstimatedInputDenial? result = null;
        if (document.TryGetProperty("kind", out var tagEstimatedInputDenialInitialRequest) && tagEstimatedInputDenialInitialRequest.ValueKind == JsonValueKind.String && tagEstimatedInputDenialInitialRequest.GetString() == "initial_request")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new EstimatedInputDenialInitialRequest(document);
        }
        if (document.TryGetProperty("kind", out var tagEstimatedInputDenialAdditionalRequest) && tagEstimatedInputDenialAdditionalRequest.ValueKind == JsonValueKind.String && tagEstimatedInputDenialAdditionalRequest.GetString() == "additional_request")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new EstimatedInputDenialAdditionalRequest(document);
        }
        if (document.TryGetProperty("kind", out var tagEstimatedInputDenialRetry) && tagEstimatedInputDenialRetry.ValueKind == JsonValueKind.String && tagEstimatedInputDenialRetry.GetString() == "retry")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new EstimatedInputDenialRetry(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class EstimatedInputDenialAdditionalRequest : EstimatedInputDenial
{
    public EstimatedInputDenialAdditionalRequest(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public ulong Limit => RequiredElement("limit").GetUInt64();
}

public sealed class EstimatedInputDenialInitialRequest : EstimatedInputDenial
{
    public EstimatedInputDenialInitialRequest(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public ulong Limit => RequiredElement("limit").GetUInt64();
}

public sealed class EstimatedInputDenialRetry : EstimatedInputDenial
{
    public EstimatedInputDenialRetry(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public ushort LastStatus => RequiredElement("last_status").GetUInt16();
    public ulong Limit => RequiredElement("limit").GetUInt64();
}

public sealed class Facts : ResultObject
{
    public Facts(JsonElement document) : base(document) { }
    public Presence<IReadOnlyList<Attempt>> Attempts => Optional<IReadOnlyList<Attempt>>("attempts", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Attempt(item0)).ToArray()));
    public ulong CacheAnswers => RequiredElement("cache_answers").GetUInt64();
    public string CallId => RequiredElement("call_id").GetString()!;
    public Presence<string> EstimatedCostUsd => Optional<string>("estimated_cost_usd", member => member.GetString()!);
    public Presence<bool> HeldModelMismatch => Optional<bool>("held_model_mismatch", member => member.GetBoolean());
    public Presence<ulong> InputTokens => Optional<ulong>("input_tokens", member => member.GetUInt64());
    public ulong LargestRequestBytes => RequiredElement("largest_request_bytes").GetUInt64();
    public Presence<ulong> LargestRequestEstimatedInputTokens => Optional<ulong>("largest_request_estimated_input_tokens", member => member.GetUInt64());
    public Presence<string> Model => Optional<string>("model", member => member.GetString()!);
    public Presence<ulong> OutputTokens => Optional<ulong>("output_tokens", member => member.GetUInt64());
    public ulong Records => RequiredElement("records").GetUInt64();
    public ulong RequestsSent => RequiredElement("requests_sent").GetUInt64();
    public double Seconds => RequiredElement("seconds").GetDouble();
    public string TokenEstimateMethod => RequiredElement("token_estimate_method").GetString()!;
    public Presence<PersistenceObservation> UsagePersistence => Optional<PersistenceObservation>("usage_persistence", member => new PersistenceObservation(member));
}

public sealed class Find : ResultObject
{
    public Find(JsonElement document) : base(document) { }
    public FindAnswer Answer => new FindAnswer(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<IReadOnlyList<FindCandidate>> Candidates => Optional<IReadOnlyList<FindCandidate>>("candidates", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new FindCandidate(item0)).ToArray()));
    public Presence<string> File => Optional<string>("file", member => member.GetString()!);
    public Presence<ulong> FirstLine => Optional<ulong>("first_line", member => member.GetUInt64());
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<ulong> LastLine => Optional<ulong>("last_line", member => member.GetUInt64());
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Presence<Position> Position => Optional<Position>("position", member => new Position(member));
    public Question2 Question => new Question2(RequiredElement("question"));
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<JsonElement> Threshold => Optional<JsonElement>("threshold", member => member.Clone());
    public Presence<JsonElement> Value => Optional<JsonElement>("value", member => member.Clone());
}

public sealed class FindCandidate : ResultObject
{
    public FindCandidate(JsonElement document) : base(document) { }
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public double Probability => RequiredElement("probability").GetDouble();
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
}

public sealed class Image : ResultObject
{
    public Image(JsonElement document) : base(document) { }
    public string Base64 => RequiredElement("base64").GetString()!;
    public uint Height => RequiredElement("height").GetUInt32();
    public ImageMedia Media => new ImageMedia(RequiredElement("media").GetString()!);
    public uint Width => RequiredElement("width").GetUInt32();
}

public readonly record struct ImageMedia(string Value)
{
    public static ImageMedia ImageJpeg => new("image/jpeg");
    public static ImageMedia ImagePng => new("image/png");
}

public abstract class InputDeclaration : ResultObject
{
    protected InputDeclaration(JsonElement document) : base(document) { }
    public static InputDeclaration Read(JsonElement document)
    {
        InputDeclaration? result = null;
        if (document.TryGetProperty("type", out var tagInputDeclarationString) && tagInputDeclarationString.ValueKind == JsonValueKind.String && tagInputDeclarationString.GetString() == "string")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new InputDeclarationString(document);
        }
        if (document.TryGetProperty("type", out var tagInputDeclarationObject) && tagInputDeclarationObject.ValueKind == JsonValueKind.String && tagInputDeclarationObject.GetString() == "object")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new InputDeclarationObject(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class InputDeclarationObject : InputDeclaration
{
    public InputDeclarationObject(JsonElement document) : base(document) { }
    public IReadOnlyDictionary<string, InputPropertyType> Properties => new ReadOnlyDictionary<string, InputPropertyType>(RequiredElement("properties").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => global::ThinkThen.Results.InputPropertyType.Read(entry0.Value), StringComparer.Ordinal));
    public Presence<IReadOnlyList<string>> Required => Optional<IReadOnlyList<string>>("required", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public ObjectType Type => new ObjectType(RequiredElement("type").GetString()!);
}

public sealed class InputDeclarationString : InputDeclaration
{
    public InputDeclarationString(JsonElement document) : base(document) { }
    public StringType Type => new StringType(RequiredElement("type").GetString()!);
}

public abstract class InputPropertyType : ResultObject
{
    protected InputPropertyType(JsonElement document) : base(document) { }
    public static InputPropertyType Read(JsonElement document)
    {
        InputPropertyType? result = null;
        if (document.TryGetProperty("type", out var tagInputPropertyTypeString) && tagInputPropertyTypeString.ValueKind == JsonValueKind.String && tagInputPropertyTypeString.GetString() == "string")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new InputPropertyTypeString(document);
        }
        if (document.TryGetProperty("type", out var tagInputPropertyTypeNumber) && tagInputPropertyTypeNumber.ValueKind == JsonValueKind.String && tagInputPropertyTypeNumber.GetString() == "number")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new InputPropertyTypeNumber(document);
        }
        if (document.TryGetProperty("type", out var tagInputPropertyTypeBoolean) && tagInputPropertyTypeBoolean.ValueKind == JsonValueKind.String && tagInputPropertyTypeBoolean.GetString() == "boolean")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new InputPropertyTypeBoolean(document);
        }
        if (document.TryGetProperty("type", out var tagInputPropertyTypeArray) && tagInputPropertyTypeArray.ValueKind == JsonValueKind.String && tagInputPropertyTypeArray.GetString() == "array")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new InputPropertyTypeArray(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class InputPropertyTypeArray : InputPropertyType
{
    public InputPropertyTypeArray(JsonElement document) : base(document) { }
    public StringRoot Items => new StringRoot(RequiredElement("items"));
    public string Type => RequiredElement("type").GetString()!;
}

public sealed class InputPropertyTypeBoolean : InputPropertyType
{
    public InputPropertyTypeBoolean(JsonElement document) : base(document) { }
    public string Type => RequiredElement("type").GetString()!;
}

public sealed class InputPropertyTypeNumber : InputPropertyType
{
    public InputPropertyTypeNumber(JsonElement document) : base(document) { }
    public string Type => RequiredElement("type").GetString()!;
}

public sealed class InputPropertyTypeString : InputPropertyType
{
    public InputPropertyTypeString(JsonElement document) : base(document) { }
    public string Type => RequiredElement("type").GetString()!;
}

public sealed class Label : ResultObject
{
    public Label(JsonElement document) : base(document) { }
    public Presence<JsonElement> Description => Optional<JsonElement>("description", member => member.Clone());
    public string Name => RequiredElement("name").GetString()!;
}

public sealed class Meta : ResultObject
{
    public Meta(JsonElement document) : base(document) { }
    public Presence<string> AnsweredBy => Optional<string>("answered_by", member => member.GetString()!);
    public Presence<IReadOnlyList<Attempt>> Attempts => Optional<IReadOnlyList<Attempt>>("attempts", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Attempt(item0)).ToArray()));
    public Presence<BatchSetting> BatchSetting => Optional<BatchSetting>("batch_setting", member => global::ThinkThen.Results.BatchSetting.Read(member));
    public Presence<BatchWarning> BatchWarning => Optional<BatchWarning>("batch_warning", member => new BatchWarning(member));
    public bool Cached => RequiredElement("cached").GetBoolean();
    public Presence<string> ContextSha256 => Optional<string>("context_sha256", member => member.GetString()!);
    public ulong FailedQuestions => RequiredElement("failed_questions").GetUInt64();
    public string Model => RequiredElement("model").GetString()!;
    public IReadOnlyList<Observation> Observations => Array.AsReadOnly(RequiredElement("observations").EnumerateArray().Select(item0 => global::ThinkThen.Results.Observation.Read(item0)).ToArray());
    public Presence<Origin> Origin => Optional<Origin>("origin", member => new Origin(member.GetString()!));
    public Presence<ProfileWarning> ProfileWarning => Optional<ProfileWarning>("profile_warning", member => new ProfileWarning(member));
    public Presence<string> QuestionSha256 => Optional<string>("question_sha256", member => member.GetString()!);
    public IReadOnlyList<QuestionSource> QuestionSources => Array.AsReadOnly(RequiredElement("question_sources").EnumerateArray().Select(item0 => new QuestionSource(item0)).ToArray());
    public Presence<string> QuestionsSha256 => Optional<string>("questions_sha256", member => member.GetString()!);
    public IReadOnlyList<string> Requests => Array.AsReadOnly(RequiredElement("requests").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
    public ulong RequestsSent => RequiredElement("requests_sent").GetUInt64();
    public string Tool => RequiredElement("tool").GetString()!;
    public string Url => RequiredElement("url").GetString()!;
    public Presence<Usage> Usage => Optional<Usage>("usage", member => new Usage(member));
}

public sealed class ObjectRoot : ResultObject
{
    public ObjectRoot(JsonElement document) : base(document) { }
    public IReadOnlyDictionary<string, InputPropertyType> Properties => new ReadOnlyDictionary<string, InputPropertyType>(RequiredElement("properties").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => global::ThinkThen.Results.InputPropertyType.Read(entry0.Value), StringComparer.Ordinal));
    public Presence<IReadOnlyList<string>> Required => Optional<IReadOnlyList<string>>("required", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public ObjectType Type => new ObjectType(RequiredElement("type").GetString()!);
}

public readonly record struct ObjectType(string Value)
{
    public static ObjectType Object => new("object");
}

public abstract class Observation : ResultObject
{
    protected Observation(JsonElement document) : base(document) { }
    public static Observation Read(JsonElement document)
    {
        Observation? result = null;
        if (document.TryGetProperty("observation_id", out var tagObservationObservationId))
        {
            if (!(tagObservationObservationId.ValueKind == JsonValueKind.String)) throw new JsonException("Invalid result identity kind.");
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new ObservationObservationId(document);
        }
        if (document.TryGetProperty("failure_id", out var tagObservationFailureId))
        {
            if (!(tagObservationFailureId.ValueKind == JsonValueKind.String)) throw new JsonException("Invalid result identity kind.");
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new ObservationFailureId(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class ObservationFailureId : Observation
{
    public ObservationFailureId(JsonElement document) : base(document) { }
    public string FailureId => RequiredElement("failure_id").GetString()!;
}

public sealed class ObservationObservationId : Observation
{
    public ObservationObservationId(JsonElement document) : base(document) { }
    public string ObservationId => RequiredElement("observation_id").GetString()!;
}

public readonly record struct Origin(string Value)
{
    public static Origin Live => new("live");
    public static Origin Cache => new("cache");
    public static Origin Replay => new("replay");
    public static Origin Proxy => new("proxy");
    public static Origin Memory => new("memory");
}

public sealed class PersistenceObservation : ResultObject
{
    public PersistenceObservation(JsonElement document) : base(document) { }
    public Presence<string> Advice => Optional<string>("advice", member => member.GetString()!);
    public string ObservedAt => RequiredElement("observed_at").GetString()!;
    public UsagePersistence State => new UsagePersistence(RequiredElement("state").GetString()!);
}

public sealed class PhysicalSource : ResultObject
{
    public PhysicalSource(JsonElement document) : base(document) { }
    public string File => RequiredElement("file").GetString()!;
    public Presence<ulong> FirstLine => Optional<ulong>("first_line", member => member.GetUInt64());
    public Presence<ulong> LastLine => Optional<ulong>("last_line", member => member.GetUInt64());
}

public sealed class Position : ResultObject
{
    public Position(JsonElement document) : base(document) { }
    public Presence<string> File => Optional<string>("file", member => member.GetString()!);
    public Presence<ulong> First => Optional<ulong>("first", member => member.GetUInt64());
    public Presence<IReadOnlyList<string>> Images => Optional<IReadOnlyList<string>>("images", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public Presence<ulong> Last => Optional<ulong>("last", member => member.GetUInt64());
}

public sealed class QuestionSource : ResultObject
{
    public QuestionSource(JsonElement document) : base(document) { }
    public string AnsweredBy => RequiredElement("answered_by").GetString()!;
    public Presence<uint> BatchSize => Optional<uint>("batch_size", member => member.GetUInt32());
    public Origin Origin => new Origin(RequiredElement("origin").GetString()!);
}

public sealed class RankMember : ResultObject
{
    public RankMember(JsonElement document) : base(document) { }
    public string Name => RequiredElement("name").GetString()!;
    public RankMemberResult Result => new RankMemberResult(RequiredElement("result"));
}

public sealed class RankMemberResult : ResultObject
{
    public RankMemberResult(JsonElement document) : base(document) { }
    public Answer Answer => global::ThinkThen.Results.Answer.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<IReadOnlyList<Image>> Images => Optional<IReadOnlyList<Image>>("images", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Image(item0)).ToArray()));
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
    public Presence<JsonElement> Threshold => Optional<JsonElement>("threshold", member => member.Clone());
    public ulong Value => RequiredElement("value").GetUInt64();
}

public abstract class Question : ResultObject
{
    protected Question(JsonElement document) : base(document) { }
    public static Question Read(JsonElement document)
    {
        Question? result = null;
        if (document.TryGetProperty("verb", out var tagQuestionDecide) && tagQuestionDecide.ValueKind == JsonValueKind.String && tagQuestionDecide.GetString() == "decide")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new QuestionDecide(document);
        }
        if (document.TryGetProperty("verb", out var tagQuestionChoose) && tagQuestionChoose.ValueKind == JsonValueKind.String && tagQuestionChoose.GetString() == "choose")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new QuestionChoose(document);
        }
        if (document.TryGetProperty("verb", out var tagQuestionTag) && tagQuestionTag.ValueKind == JsonValueKind.String && tagQuestionTag.GetString() == "tag")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new QuestionTag(document);
        }
        if (document.TryGetProperty("verb", out var tagQuestionScore) && tagQuestionScore.ValueKind == JsonValueKind.String && tagQuestionScore.GetString() == "score")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new QuestionScore(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class Question2 : ResultObject
{
    public Question2(JsonElement document) : base(document) { }
    public Presence<Batch> Batch => Optional<Batch>("batch", member => global::ThinkThen.Results.Batch.Read(member));
    public Presence<InputDeclaration> ContextSchema => Optional<InputDeclaration>("context_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<InputDeclaration> ItemSchema => Optional<InputDeclaration>("item_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<IReadOnlyList<Label>> LabelDetails => Optional<IReadOnlyList<Label>>("label_details", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Label(item0)).ToArray()));
    public Presence<string> Model => Optional<string>("model", member => member.GetString()!);
    public Presence<string> Name => Optional<string>("name", member => member.GetString()!);
    public bool None => RequiredElement("none").GetBoolean();
    public Presence<IReadOnlyList<string>> On => Optional<IReadOnlyList<string>>("on", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public Presence<string> Profile => Optional<string>("profile", member => member.GetString()!);
    public JsonElement Text => RequiredElement("text").Clone();
    public string Verb => RequiredElement("verb").GetString()!;
    public Presence<uint> WordingVersion => Optional<uint>("wording_version", member => member.GetUInt32());
}

public sealed class Question3 : ResultObject
{
    public Question3(JsonElement document) : base(document) { }
    public Presence<Batch> Batch => Optional<Batch>("batch", member => global::ThinkThen.Results.Batch.Read(member));
    public Presence<InputDeclaration> ContextSchema => Optional<InputDeclaration>("context_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<JsonElement> EntityDefinition => Optional<JsonElement>("entity_definition", member => member.Clone());
    public Presence<JsonElement> Instructions => Optional<JsonElement>("instructions", member => member.Clone());
    public Presence<InputDeclaration> ItemSchema => Optional<InputDeclaration>("item_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public IReadOnlyDictionary<string, JsonElement> Kinds => new ReadOnlyDictionary<string, JsonElement>(RequiredElement("kinds").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.Clone(), StringComparer.Ordinal));
    public Presence<IReadOnlyList<Label>> LabelDetails => Optional<IReadOnlyList<Label>>("label_details", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Label(item0)).ToArray()));
    public Presence<RecognitionMode> Mode => Optional<RecognitionMode>("mode", member => new RecognitionMode(member.GetString()!));
    public Presence<string> Model => Optional<string>("model", member => member.GetString()!);
    public Presence<string> Name => Optional<string>("name", member => member.GetString()!);
    public Presence<IReadOnlyList<string>> On => Optional<IReadOnlyList<string>>("on", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public Presence<string> Profile => Optional<string>("profile", member => member.GetString()!);
    public Presence<Threshold> RelationThreshold => Optional<Threshold>("relation_threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public Presence<IReadOnlyList<RelationRule>> Relations => Optional<IReadOnlyList<RelationRule>>("relations", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new RelationRule(item0)).ToArray()));
    public Presence<uint> SnippetPieces => Optional<uint>("snippet_pieces", member => member.GetUInt32());
    public Presence<RecognitionStageContext> StageContext => Optional<RecognitionStageContext>("stage_context", member => new RecognitionStageContext(member));
    public Threshold Threshold => global::ThinkThen.Results.Threshold.Read(RequiredElement("threshold"));
    public Verb Verb => new Verb(RequiredElement("verb").GetString()!);
    public Presence<uint> WordingVersion => Optional<uint>("wording_version", member => member.GetUInt32());
}

public sealed class Question4 : ResultObject
{
    public Question4(JsonElement document) : base(document) { }
    public Presence<Batch> Batch => Optional<Batch>("batch", member => global::ThinkThen.Results.Batch.Read(member));
    public Presence<InputDeclaration> ContextSchema => Optional<InputDeclaration>("context_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<RelateFields> Fields => Optional<RelateFields>("fields", member => new RelateFields(member));
    public Presence<InputDeclaration> ItemSchema => Optional<InputDeclaration>("item_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<IReadOnlyList<Label>> LabelDetails => Optional<IReadOnlyList<Label>>("label_details", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Label(item0)).ToArray()));
    public Presence<string> Model => Optional<string>("model", member => member.GetString()!);
    public Presence<string> Name => Optional<string>("name", member => member.GetString()!);
    public Presence<IReadOnlyList<string>> On => Optional<IReadOnlyList<string>>("on", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public Presence<string> Profile => Optional<string>("profile", member => member.GetString()!);
    public IReadOnlyList<RelationRule> Relations => Array.AsReadOnly(RequiredElement("relations").EnumerateArray().Select(item0 => new RelationRule(item0)).ToArray());
    public Threshold Threshold => global::ThinkThen.Results.Threshold.Read(RequiredElement("threshold"));
    public string Verb => RequiredElement("verb").GetString()!;
    public Presence<uint> WordingVersion => Optional<uint>("wording_version", member => member.GetUInt32());
}

public sealed class QuestionChoose : Question
{
    public QuestionChoose(JsonElement document) : base(document) { }
    public Presence<Batch> Batch => Optional<Batch>("batch", member => global::ThinkThen.Results.Batch.Read(member));
    public Presence<InputDeclaration> ContextSchema => Optional<InputDeclaration>("context_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<InputDeclaration> ItemSchema => Optional<InputDeclaration>("item_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<IReadOnlyList<Label>> LabelDetails => Optional<IReadOnlyList<Label>>("label_details", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Label(item0)).ToArray()));
    public Presence<string> Model => Optional<string>("model", member => member.GetString()!);
    public Presence<string> Name => Optional<string>("name", member => member.GetString()!);
    public Presence<IReadOnlyList<string>> On => Optional<IReadOnlyList<string>>("on", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public Presence<string> Profile => Optional<string>("profile", member => member.GetString()!);
    public Presence<uint> WordingVersion => Optional<uint>("wording_version", member => member.GetUInt32());
    public IReadOnlyList<string> Options => Array.AsReadOnly(RequiredElement("options").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
    public JsonElement Text => RequiredElement("text").Clone();
    public string Verb => RequiredElement("verb").GetString()!;
}

public sealed class QuestionDecide : Question
{
    public QuestionDecide(JsonElement document) : base(document) { }
    public Presence<Batch> Batch => Optional<Batch>("batch", member => global::ThinkThen.Results.Batch.Read(member));
    public Presence<InputDeclaration> ContextSchema => Optional<InputDeclaration>("context_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<InputDeclaration> ItemSchema => Optional<InputDeclaration>("item_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<IReadOnlyList<Label>> LabelDetails => Optional<IReadOnlyList<Label>>("label_details", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Label(item0)).ToArray()));
    public Presence<string> Model => Optional<string>("model", member => member.GetString()!);
    public Presence<string> Name => Optional<string>("name", member => member.GetString()!);
    public Presence<IReadOnlyList<string>> On => Optional<IReadOnlyList<string>>("on", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public Presence<string> Profile => Optional<string>("profile", member => member.GetString()!);
    public Presence<uint> WordingVersion => Optional<uint>("wording_version", member => member.GetUInt32());
    public Presence<JsonElement> False => Optional<JsonElement>("false", member => member.Clone());
    public JsonElement Text => RequiredElement("text").Clone();
    public Presence<JsonElement> True => Optional<JsonElement>("true", member => member.Clone());
    public string Verb => RequiredElement("verb").GetString()!;
}

public sealed class QuestionScore : Question
{
    public QuestionScore(JsonElement document) : base(document) { }
    public Presence<Batch> Batch => Optional<Batch>("batch", member => global::ThinkThen.Results.Batch.Read(member));
    public Presence<InputDeclaration> ContextSchema => Optional<InputDeclaration>("context_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<InputDeclaration> ItemSchema => Optional<InputDeclaration>("item_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<IReadOnlyList<Label>> LabelDetails => Optional<IReadOnlyList<Label>>("label_details", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Label(item0)).ToArray()));
    public Presence<string> Model => Optional<string>("model", member => member.GetString()!);
    public Presence<string> Name => Optional<string>("name", member => member.GetString()!);
    public Presence<IReadOnlyList<string>> On => Optional<IReadOnlyList<string>>("on", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public Presence<string> Profile => Optional<string>("profile", member => member.GetString()!);
    public Presence<uint> WordingVersion => Optional<uint>("wording_version", member => member.GetUInt32());
    public IReadOnlyList<string> Levels => Array.AsReadOnly(RequiredElement("levels").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
    public JsonElement Text => RequiredElement("text").Clone();
    public string Verb => RequiredElement("verb").GetString()!;
}

public sealed class QuestionTag : Question
{
    public QuestionTag(JsonElement document) : base(document) { }
    public Presence<Batch> Batch => Optional<Batch>("batch", member => global::ThinkThen.Results.Batch.Read(member));
    public Presence<InputDeclaration> ContextSchema => Optional<InputDeclaration>("context_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<InputDeclaration> ItemSchema => Optional<InputDeclaration>("item_schema", member => global::ThinkThen.Results.InputDeclaration.Read(member));
    public Presence<IReadOnlyList<Label>> LabelDetails => Optional<IReadOnlyList<Label>>("label_details", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Label(item0)).ToArray()));
    public Presence<string> Model => Optional<string>("model", member => member.GetString()!);
    public Presence<string> Name => Optional<string>("name", member => member.GetString()!);
    public Presence<IReadOnlyList<string>> On => Optional<IReadOnlyList<string>>("on", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()));
    public Presence<string> Profile => Optional<string>("profile", member => member.GetString()!);
    public Presence<uint> WordingVersion => Optional<uint>("wording_version", member => member.GetUInt32());
    public IReadOnlyList<string> Labels => Array.AsReadOnly(RequiredElement("labels").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
    public JsonElement Text => RequiredElement("text").Clone();
    public string Verb => RequiredElement("verb").GetString()!;
}

public sealed class Recognition : ResultObject
{
    public Recognition(JsonElement document) : base(document) { }
    public RecognitionOdds Answer => global::ThinkThen.Results.RecognitionOdds.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<string> File => Optional<string>("file", member => member.GetString()!);
    public Presence<ulong> FirstLine => Optional<ulong>("first_line", member => member.GetUInt64());
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public Presence<ulong> LastLine => Optional<ulong>("last_line", member => member.GetUInt64());
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Presence<Position> Position => Optional<Position>("position", member => new Position(member));
    public Question3 Question => new Question3(RequiredElement("question"));
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public Presence<PhysicalSource> Source => Optional<PhysicalSource>("source", member => new PhysicalSource(member));
    public Recognize Value => global::ThinkThen.Results.Recognize.Read(RequiredElement("value"));
}

public sealed class RecognitionEdgeDocument : ResultObject
{
    public RecognitionEdgeDocument(JsonElement document) : base(document) { }
    public bool Either => RequiredElement("either").GetBoolean();
    public double Probability => RequiredElement("probability").GetDouble();
    public string Relation => RequiredElement("relation").GetString()!;
    public Entity Source => new Entity(RequiredElement("source"));
    public Entity Target => new Entity(RequiredElement("target"));
}

public readonly record struct RecognitionMode(string Value)
{
    public static RecognitionMode Whole => new("whole");
    public static RecognitionMode BoundaryOnly => new("boundary_only");
}

public abstract class RecognitionOdds : ResultObject
{
    protected RecognitionOdds(JsonElement document) : base(document) { }
    public static RecognitionOdds Read(JsonElement document)
    {
        RecognitionOdds? result = null;
        if (document.TryGetProperty("names", out var tagRecognitionOddsFieldsNamesPairsPiecesProposals0) && (tagRecognitionOddsFieldsNamesPairsPiecesProposals0.ValueKind == JsonValueKind.Array) && document.TryGetProperty("pairs", out var tagRecognitionOddsFieldsNamesPairsPiecesProposals1) && (tagRecognitionOddsFieldsNamesPairsPiecesProposals1.ValueKind == JsonValueKind.Array) && document.TryGetProperty("pieces", out var tagRecognitionOddsFieldsNamesPairsPiecesProposals2) && (tagRecognitionOddsFieldsNamesPairsPiecesProposals2.ValueKind == JsonValueKind.Array) && document.TryGetProperty("proposals", out var tagRecognitionOddsFieldsNamesPairsPiecesProposals3) && (tagRecognitionOddsFieldsNamesPairsPiecesProposals3.ValueKind == JsonValueKind.Array))
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new RecognitionOddsFieldsNamesPairsPiecesProposals(document);
        }
        if (document.TryGetProperty("pieces", out var tagRecognitionOddsFieldsPiecesProposals0) && (tagRecognitionOddsFieldsPiecesProposals0.ValueKind == JsonValueKind.Array) && document.TryGetProperty("proposals", out var tagRecognitionOddsFieldsPiecesProposals1) && (tagRecognitionOddsFieldsPiecesProposals1.ValueKind == JsonValueKind.Array) && !document.TryGetProperty("names", out _) && !document.TryGetProperty("pairs", out _))
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new RecognitionOddsFieldsPiecesProposals(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class RecognitionOddsFieldsNamesPairsPiecesProposals : RecognitionOdds
{
    public RecognitionOddsFieldsNamesPairsPiecesProposals(JsonElement document) : base(document) { }
    public IReadOnlyList<NameOdds> Names => Array.AsReadOnly(RequiredElement("names").EnumerateArray().Select(item0 => new NameOdds(item0)).ToArray());
    public IReadOnlyList<PairOdds> Pairs => Array.AsReadOnly(RequiredElement("pairs").EnumerateArray().Select(item0 => new PairOdds(item0)).ToArray());
    public IReadOnlyList<PieceOdds> Pieces => Array.AsReadOnly(RequiredElement("pieces").EnumerateArray().Select(item0 => new PieceOdds(item0)).ToArray());
    public IReadOnlyList<RecognitionProposal> Proposals => Array.AsReadOnly(RequiredElement("proposals").EnumerateArray().Select(item0 => new RecognitionProposal(item0)).ToArray());
}

public sealed class RecognitionOddsFieldsPiecesProposals : RecognitionOdds
{
    public RecognitionOddsFieldsPiecesProposals(JsonElement document) : base(document) { }
    public IReadOnlyList<PieceOdds> Pieces => Array.AsReadOnly(RequiredElement("pieces").EnumerateArray().Select(item0 => new PieceOdds(item0)).ToArray());
    public IReadOnlyList<BoundaryProposal> Proposals => Array.AsReadOnly(RequiredElement("proposals").EnumerateArray().Select(item0 => new BoundaryProposal(item0)).ToArray());
}

public sealed class RecognitionProposal : ResultObject
{
    public RecognitionProposal(JsonElement document) : base(document) { }
    public ulong End => RequiredElement("end").GetUInt64();
    public bool Kept => RequiredElement("kept").GetBoolean();
    public Presence<string> Kind => Optional<string>("kind", member => member.GetString()!);
    public Presence<Place> Selected => Optional<Place>("selected", member => new Place(member));
    public double SpanProbability => RequiredElement("span_probability").GetDouble();
    public ulong Start => RequiredElement("start").GetUInt64();
    public Presence<double> Strength => Optional<double>("strength", member => member.GetDouble());
}

public sealed class RecognitionStageContext : ResultObject
{
    public RecognitionStageContext(JsonElement document) : base(document) { }
    public Presence<string> Boundary => Optional<string>("boundary", member => member.GetString()!);
    public Presence<string> KindEdge => Optional<string>("kind_edge", member => member.GetString()!);
    public Presence<string> Relation => Optional<string>("relation", member => member.GetString()!);
}

public sealed class Relation : ResultObject
{
    public Relation(JsonElement document) : base(document) { }
    public Answers Answer => new Answers(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public Presence<string> File => Optional<string>("file", member => member.GetString()!);
    public Presence<ulong> FirstLine => Optional<ulong>("first_line", member => member.GetUInt64());
    public Presence<ulong> Index => Optional<ulong>("index", member => member.GetUInt64());
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public Presence<ulong> LastLine => Optional<ulong>("last_line", member => member.GetUInt64());
    public Meta Meta => new Meta(RequiredElement("meta"));
    public Presence<Position> Position => Optional<Position>("position", member => new Position(member));
    public Question4 Question => new Question4(RequiredElement("question"));
    public Version Schema => new Version(RequiredElement("schema").GetString()!);
    public IReadOnlyList<RelatedEntityEdge> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new RelatedEntityEdge(item0)).ToArray());
}

public readonly record struct RelationDirection(string Value)
{
    public static RelationDirection SourceToTarget => new("source_to_target");
    public static RelationDirection Either => new("either");
}

public abstract class RelationMember : ResultObject
{
    protected RelationMember(JsonElement document) : base(document) { }
    public static RelationMember Read(JsonElement document)
    {
        RelationMember? result = null;
        if (document.TryGetProperty("answer_id", out var tagRelationMemberAnswerId))
        {
            if (!(tagRelationMemberAnswerId.ValueKind == JsonValueKind.String)) throw new JsonException("Invalid result identity kind.");
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new RelationMemberAnswerId(document);
        }
        if (document.TryGetProperty("failure_id", out var tagRelationMemberFailureId))
        {
            if (!(tagRelationMemberFailureId.ValueKind == JsonValueKind.String)) throw new JsonException("Invalid result identity kind.");
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new RelationMemberFailureId(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class RelationMemberAnswerId : RelationMember
{
    public RelationMemberAnswerId(JsonElement document) : base(document) { }
    public RelationDirection Direction => new RelationDirection(RequiredElement("direction").GetString()!);
    public RelationMethod Method => new RelationMethod(RequiredElement("method").GetString()!);
    public IReadOnlyList<Observation> Observations => Array.AsReadOnly(RequiredElement("observations").EnumerateArray().Select(item0 => global::ThinkThen.Results.Observation.Read(item0)).ToArray());
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public IReadOnlyList<QuestionSource> QuestionSources => Array.AsReadOnly(RequiredElement("question_sources").EnumerateArray().Select(item0 => new QuestionSource(item0)).ToArray());
    public string Reads => RequiredElement("reads").GetString()!;
    public string Relation => RequiredElement("relation").GetString()!;
    public string Request => RequiredElement("request").GetString()!;
    public RelatedEntity Source => new RelatedEntity(RequiredElement("source"));
    public Presence<RelatedEntity> Target => Optional<RelatedEntity>("target", member => new RelatedEntity(member));
    public Threshold Threshold => global::ThinkThen.Results.Threshold.Read(RequiredElement("threshold"));
    public Presence<Usage> Usage => Optional<Usage>("usage", member => new Usage(member));
    public bool Accepted => RequiredElement("accepted").GetBoolean();
    public Answer Answer => global::ThinkThen.Results.Answer.Read(RequiredElement("answer"));
    public string AnswerId => RequiredElement("answer_id").GetString()!;
    public double Probability => RequiredElement("probability").GetDouble();
}

public sealed class RelationMemberFailureId : RelationMember
{
    public RelationMemberFailureId(JsonElement document) : base(document) { }
    public RelationDirection Direction => new RelationDirection(RequiredElement("direction").GetString()!);
    public RelationMethod Method => new RelationMethod(RequiredElement("method").GetString()!);
    public IReadOnlyList<Observation> Observations => Array.AsReadOnly(RequiredElement("observations").EnumerateArray().Select(item0 => global::ThinkThen.Results.Observation.Read(item0)).ToArray());
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public IReadOnlyList<QuestionSource> QuestionSources => Array.AsReadOnly(RequiredElement("question_sources").EnumerateArray().Select(item0 => new QuestionSource(item0)).ToArray());
    public string Reads => RequiredElement("reads").GetString()!;
    public string Relation => RequiredElement("relation").GetString()!;
    public string Request => RequiredElement("request").GetString()!;
    public RelatedEntity Source => new RelatedEntity(RequiredElement("source"));
    public Presence<RelatedEntity> Target => Optional<RelatedEntity>("target", member => new RelatedEntity(member));
    public Threshold Threshold => global::ThinkThen.Results.Threshold.Read(RequiredElement("threshold"));
    public Presence<Usage> Usage => Optional<Usage>("usage", member => new Usage(member));
    public Failure Failure => new Failure(RequiredElement("failure"));
    public string FailureId => RequiredElement("failure_id").GetString()!;
}

public readonly record struct RelationMethod(string Value)
{
    public static RelationMethod YesNo => new("yes_no");
    public static RelationMethod Choice => new("choice");
}

public readonly record struct RequestFunction(string Value)
{
    public static RequestFunction Decide => new("decide");
    public static RequestFunction Choose => new("choose");
    public static RequestFunction Tag => new("tag");
    public static RequestFunction Score => new("score");
    public static RequestFunction Filter => new("filter");
    public static RequestFunction Rank => new("rank");
    public static RequestFunction Find => new("find");
    public static RequestFunction Annotate => new("annotate");
    public static RequestFunction Recognize => new("recognize");
    public static RequestFunction Relate => new("relate");
}

public abstract class SendBudgetDenial : ResultObject
{
    protected SendBudgetDenial(JsonElement document) : base(document) { }
    public static SendBudgetDenial Read(JsonElement document)
    {
        SendBudgetDenial? result = null;
        if (document.TryGetProperty("kind", out var tagSendBudgetDenialBeforeFirstSend) && tagSendBudgetDenialBeforeFirstSend.ValueKind == JsonValueKind.String && tagSendBudgetDenialBeforeFirstSend.GetString() == "before_first_send")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SendBudgetDenialBeforeFirstSend(document);
        }
        if (document.TryGetProperty("kind", out var tagSendBudgetDenialBeforeAdditionalSend) && tagSendBudgetDenialBeforeAdditionalSend.ValueKind == JsonValueKind.String && tagSendBudgetDenialBeforeAdditionalSend.GetString() == "before_additional_send")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SendBudgetDenialBeforeAdditionalSend(document);
        }
        if (document.TryGetProperty("kind", out var tagSendBudgetDenialBeforeRetry) && tagSendBudgetDenialBeforeRetry.ValueKind == JsonValueKind.String && tagSendBudgetDenialBeforeRetry.GetString() == "before_retry")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SendBudgetDenialBeforeRetry(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class SendBudgetDenialBeforeAdditionalSend : SendBudgetDenial
{
    public SendBudgetDenialBeforeAdditionalSend(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
}

public sealed class SendBudgetDenialBeforeFirstSend : SendBudgetDenial
{
    public SendBudgetDenialBeforeFirstSend(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
}

public sealed class SendBudgetDenialBeforeRetry : SendBudgetDenial
{
    public SendBudgetDenialBeforeRetry(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public ushort LastStatus => RequiredElement("last_status").GetUInt16();
}

public readonly record struct StopCause(string Value)
{
    public static StopCause Usage => new("usage");
    public static StopCause Local => new("local");
    public static StopCause NoKey => new("no_key");
    public static StopCause Transport => new("transport");
    public static StopCause Status => new("status");
    public static StopCause TooLarge => new("too_large");
    public static StopCause Reply => new("reply");
    public static StopCause Backend => new("backend");
    public static StopCause Cancelled => new("cancelled");
    public static StopCause Deadline => new("deadline");
    public static StopCause Defect => new("defect");
}

public sealed class Stopped : ResultObject
{
    public Stopped(JsonElement document) : base(document) { }
    public Presence<ulong> At => Optional<ulong>("at", member => member.GetUInt64());
    public StopCause Cause => new StopCause(RequiredElement("cause").GetString()!);
    public bool Retryable => RequiredElement("retryable").GetBoolean();
    public Presence<ushort> Status => Optional<ushort>("status", member => member.GetUInt16());
}

public sealed class StringRoot : ResultObject
{
    public StringRoot(JsonElement document) : base(document) { }
    public StringType Type => new StringType(RequiredElement("type").GetString()!);
}

public readonly record struct StringType(string Value)
{
    public static StringType String => new("string");
}

public sealed class Usage : ResultObject
{
    public Usage(JsonElement document) : base(document) { }
    public Presence<ulong> InputTokens => Optional<ulong>("input_tokens", member => member.GetUInt64());
    public Presence<ulong> OutputTokens => Optional<ulong>("output_tokens", member => member.GetUInt64());
}

public readonly record struct UsagePersistence(string Value)
{
    public static UsagePersistence Disabled => new("disabled");
    public static UsagePersistence Pending => new("pending");
    public static UsagePersistence Written => new("written");
    public static UsagePersistence Failed => new("failed");
}

public readonly record struct Verb(string Value)
{
    public static Verb Recognize => new("recognize");
}

public readonly record struct Version(string Value)
{
    public static Version ThinkthenResult2 => new("thinkthen.result/2");
}

public abstract class AnnotatedField
{
    private readonly JsonElement document;
    protected AnnotatedField(JsonElement document) { this.document = document.Clone(); }
    protected JsonElement Document => document;
    public JsonElement ToJson() => document.Clone();
    public JsonNode ToPlain() => JsonNode.Parse(document.GetRawText())!;
    public string ToJsonString() => document.GetRawText();
    public static AnnotatedField Read(JsonElement document) => document.ValueKind switch
    {
        JsonValueKind.True => new AnnotatedFieldBoolean(document),
        JsonValueKind.False => new AnnotatedFieldBoolean(document),
        JsonValueKind.Null => new AnnotatedFieldNull(document),
        JsonValueKind.String => new AnnotatedFieldString(document),
        JsonValueKind.Array => new AnnotatedFieldArray(document),
        JsonValueKind.Number => new AnnotatedFieldNumber(document),
        JsonValueKind.Object => new AnnotatedFieldObject(document),
        _ => throw new JsonException("Unknown primitive result variant.")
    };
}

public sealed class AnnotatedFieldBoolean : AnnotatedField
{
    public AnnotatedFieldBoolean(JsonElement document) : base(document) { }
    public bool Value => Document.GetBoolean();
}

public sealed class AnnotatedFieldNull : AnnotatedField
{
    public AnnotatedFieldNull(JsonElement document) : base(document) { }
    public JsonElement Value => Document.Clone();
}

public sealed class AnnotatedFieldString : AnnotatedField
{
    public AnnotatedFieldString(JsonElement document) : base(document) { }
    public string Value => Document.GetString()!;
}

public sealed class AnnotatedFieldArray : AnnotatedField
{
    public AnnotatedFieldArray(JsonElement document) : base(document) { }
    public IReadOnlyList<string> Value => Array.AsReadOnly(Document.EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
}

public sealed class AnnotatedFieldNumber : AnnotatedField
{
    public AnnotatedFieldNumber(JsonElement document) : base(document) { }
    public double Value => Document.GetDouble();
}

public sealed class AnnotatedFieldObject : AnnotatedField
{
    public AnnotatedFieldObject(JsonElement document) : base(document) { }
    public Failed Value => new Failed(Document);
}

public abstract class Answer : ResultObject
{
    protected Answer(JsonElement document) : base(document) { }
    public static Answer Read(JsonElement document)
    {
        Answer? result = null;
        if (document.TryGetProperty("kind", out var tagAnswerYesNo) && tagAnswerYesNo.ValueKind == JsonValueKind.String && tagAnswerYesNo.GetString() == "yes_no")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnswerYesNo(document);
        }
        if (document.TryGetProperty("kind", out var tagAnswerChoice) && tagAnswerChoice.ValueKind == JsonValueKind.String && tagAnswerChoice.GetString() == "choice")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnswerChoice(document);
        }
        if (document.TryGetProperty("kind", out var tagAnswerTag) && tagAnswerTag.ValueKind == JsonValueKind.String && tagAnswerTag.GetString() == "tag")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnswerTag(document);
        }
        if (document.TryGetProperty("kind", out var tagAnswerScore) && tagAnswerScore.ValueKind == JsonValueKind.String && tagAnswerScore.GetString() == "score")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new AnswerScore(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class AnswerChoice : Answer
{
    public AnswerChoice(JsonElement document) : base(document) { }
    public Presence<double> Confidence => Optional<double>("confidence", member => member.GetDouble());
    public string Kind => RequiredElement("kind").GetString()!;
    public string Pick => RequiredElement("pick").GetString()!;
    public IReadOnlyDictionary<string, double> Probabilities => new ReadOnlyDictionary<string, double>(RequiredElement("probabilities").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal));
}

public sealed class AnswerScore : Answer
{
    public AnswerScore(JsonElement document) : base(document) { }
    public Presence<double> Confidence => Optional<double>("confidence", member => member.GetDouble());
    public string Kind => RequiredElement("kind").GetString()!;
    public string Level => RequiredElement("level").GetString()!;
    public IReadOnlyDictionary<string, double> Probabilities => new ReadOnlyDictionary<string, double>(RequiredElement("probabilities").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal));
}

public sealed class AnswerTag : Answer
{
    public AnswerTag(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyDictionary<string, double> Probabilities => new ReadOnlyDictionary<string, double>(RequiredElement("probabilities").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal));
}

public sealed class AnswerYesNo : Answer
{
    public AnswerYesNo(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public double Probability => RequiredElement("probability").GetDouble();
}

public readonly record struct AttemptOutcome(string Value)
{
    public static AttemptOutcome Ok => new("ok");
    public static AttemptOutcome Status => new("status");
    public static AttemptOutcome Transport => new("transport");
}

public abstract class BatchSetting
{
    private readonly JsonElement document;
    protected BatchSetting(JsonElement document) { this.document = document.Clone(); }
    protected JsonElement Document => document;
    public JsonElement ToJson() => document.Clone();
    public JsonNode ToPlain() => JsonNode.Parse(document.GetRawText())!;
    public string ToJsonString() => document.GetRawText();
    public static BatchSetting Read(JsonElement document) => document.ValueKind switch
    {
        JsonValueKind.Number => new BatchSettingInteger(document),
        JsonValueKind.String => new BatchSettingString(document),
        _ => throw new JsonException("Unknown primitive result variant.")
    };
}

public sealed class BatchSettingInteger : BatchSetting
{
    public BatchSettingInteger(JsonElement document) : base(document) { }
    public ulong Value => Document.GetUInt64();
}

public sealed class BatchSettingString : BatchSetting
{
    public BatchSettingString(JsonElement document) : base(document) { }
    public string Value => Document.GetString()!;
}

public sealed class BatchWarning : ResultObject
{
    public BatchWarning(JsonElement document) : base(document) { }
    public BatchSetting Running => global::ThinkThen.Results.BatchSetting.Read(RequiredElement("running"));
    public BatchSetting TunedFor => global::ThinkThen.Results.BatchSetting.Read(RequiredElement("tuned_for"));
}

public sealed class Entity : ResultObject
{
    public Entity(JsonElement document) : base(document) { }
    public ulong End => RequiredElement("end").GetUInt64();
    public Presence<string> File => Optional<string>("file", member => member.GetString()!);
    public Presence<ulong> FirstLine => Optional<ulong>("first_line", member => member.GetUInt64());
    public string Kind => RequiredElement("kind").GetString()!;
    public Presence<ulong> LastLine => Optional<ulong>("last_line", member => member.GetUInt64());
    public ulong Length => RequiredElement("length").GetUInt64();
    public ulong Start => RequiredElement("start").GetUInt64();
    public double Strength => RequiredElement("strength").GetDouble();
    public string Text => RequiredElement("text").GetString()!;
}

public sealed class EntityEdge : ResultObject
{
    public EntityEdge(JsonElement document) : base(document) { }
    public Presence<bool> Either => Optional<bool>("either", member => member.GetBoolean());
    public double Probability => RequiredElement("probability").GetDouble();
    public string Relation => RequiredElement("relation").GetString()!;
    public Entity Source => new Entity(RequiredElement("source"));
    public Entity Target => new Entity(RequiredElement("target"));
}

public sealed class Failed : ResultObject
{
    public Failed(JsonElement document) : base(document) { }
    public Failure FailedValue => new Failure(RequiredElement("failed"));
}

public sealed class Failure : ResultObject
{
    public Failure(JsonElement document) : base(document) { }
    public FailureCause Cause => new FailureCause(RequiredElement("cause").GetString()!);
    public string Kind => RequiredElement("kind").GetString()!;
}

public readonly record struct FailureCause(string Value)
{
    public static FailureCause MissingAnswer => new("missing_answer");
    public static FailureCause WrongKind => new("wrong_kind");
    public static FailureCause MissingProbability => new("missing_probability");
    public static FailureCause InvalidProbability => new("invalid_probability");
    public static FailureCause InvalidDistribution => new("invalid_distribution");
    public static FailureCause UnexpectedProbability => new("unexpected_probability");
}

public readonly record struct FailureKind(string Value)
{
    public static FailureKind Usage => new("usage");
    public static FailureKind Backend => new("backend");
    public static FailureKind Local => new("local");
    public static FailureKind Cancelled => new("cancelled");
    public static FailureKind Deadline => new("deadline");
    public static FailureKind Defect => new("defect");
}

public sealed class FindAnswer : ResultObject
{
    public FindAnswer(JsonElement document) : base(document) { }
    public Presence<double> Confidence => Optional<double>("confidence", member => member.GetDouble());
    public string Kind => RequiredElement("kind").GetString()!;
    public string Pick => RequiredElement("pick").GetString()!;
    public IReadOnlyDictionary<string, double> Probabilities => new ReadOnlyDictionary<string, double>(RequiredElement("probabilities").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal));
}

public sealed class NameOdds : ResultObject
{
    public NameOdds(JsonElement document) : base(document) { }
    public Presence<IReadOnlyDictionary<string, double>> Edges => Optional<IReadOnlyDictionary<string, double>>("edges", member => new ReadOnlyDictionary<string, double>(member.EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal)));
    public ulong End => RequiredElement("end").GetUInt64();
    public Presence<IReadOnlyDictionary<string, double>> Kinds => Optional<IReadOnlyDictionary<string, double>>("kinds", member => new ReadOnlyDictionary<string, double>(member.EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal)));
    public ulong Start => RequiredElement("start").GetUInt64();
}

public sealed class PairOdds : ResultObject
{
    public PairOdds(JsonElement document) : base(document) { }
    public double Probability => RequiredElement("probability").GetDouble();
    public string Relation => RequiredElement("relation").GetString()!;
    public Place Source => new Place(RequiredElement("source"));
    public Place Target => new Place(RequiredElement("target"));
}

public sealed class PieceOdds : ResultObject
{
    public PieceOdds(JsonElement document) : base(document) { }
    public ulong End => RequiredElement("end").GetUInt64();
    public ulong Start => RequiredElement("start").GetUInt64();
    public IReadOnlyDictionary<string, double> Tags => new ReadOnlyDictionary<string, double>(RequiredElement("tags").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal));
}

public sealed class Place : ResultObject
{
    public Place(JsonElement document) : base(document) { }
    public ulong End => RequiredElement("end").GetUInt64();
    public ulong Start => RequiredElement("start").GetUInt64();
}

public sealed class ProfileWarning : ResultObject
{
    public ProfileWarning(JsonElement document) : base(document) { }
    public string Running => RequiredElement("running").GetString()!;
    public string TunedFor => RequiredElement("tuned_for").GetString()!;
}

public abstract class Recognize : ResultObject
{
    protected Recognize(JsonElement document) : base(document) { }
    public static Recognize Read(JsonElement document)
    {
        Recognize? result = null;
        if (document.TryGetProperty("entities", out var tagRecognizeFieldsEntities0) && (tagRecognizeFieldsEntities0.ValueKind == JsonValueKind.Array) && !document.TryGetProperty("mode", out _) && !document.TryGetProperty("proposals", out _))
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new RecognizeFieldsEntities(document);
        }
        if (document.TryGetProperty("mode", out var tagRecognizeFieldsModeProposals0) && (tagRecognizeFieldsModeProposals0.ValueKind == JsonValueKind.String) && document.TryGetProperty("proposals", out var tagRecognizeFieldsModeProposals1) && (tagRecognizeFieldsModeProposals1.ValueKind == JsonValueKind.Array) && !document.TryGetProperty("entities", out _))
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new RecognizeFieldsModeProposals(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class RecognizeAnswer : ResultObject
{
    public RecognizeAnswer(JsonElement document) : base(document) { }
    public IReadOnlyList<NameOdds> Names => Array.AsReadOnly(RequiredElement("names").EnumerateArray().Select(item0 => new NameOdds(item0)).ToArray());
    public IReadOnlyList<PairOdds> Pairs => Array.AsReadOnly(RequiredElement("pairs").EnumerateArray().Select(item0 => new PairOdds(item0)).ToArray());
    public IReadOnlyList<PieceOdds> Pieces => Array.AsReadOnly(RequiredElement("pieces").EnumerateArray().Select(item0 => new PieceOdds(item0)).ToArray());
    public IReadOnlyList<RecognitionProposal> Proposals => Array.AsReadOnly(RequiredElement("proposals").EnumerateArray().Select(item0 => new RecognitionProposal(item0)).ToArray());
}

public sealed class RecognizeFieldsEntities : Recognize
{
    public RecognizeFieldsEntities(JsonElement document) : base(document) { }
    public IReadOnlyList<Entity> Entities => Array.AsReadOnly(RequiredElement("entities").EnumerateArray().Select(item0 => new Entity(item0)).ToArray());
    public Presence<IReadOnlyList<EntityEdge>> Relations => Optional<IReadOnlyList<EntityEdge>>("relations", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new EntityEdge(item0)).ToArray()));
}

public sealed class RecognizeFieldsModeProposals : Recognize
{
    public RecognizeFieldsModeProposals(JsonElement document) : base(document) { }
    public BoundaryMode Mode => new BoundaryMode(RequiredElement("mode").GetString()!);
    public IReadOnlyList<BoundaryProposal> Proposals => Array.AsReadOnly(RequiredElement("proposals").EnumerateArray().Select(item0 => new BoundaryProposal(item0)).ToArray());
}

public sealed class RelateFields : ResultObject
{
    public RelateFields(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public string Name => RequiredElement("name").GetString()!;
}

public sealed class RelatedEntity : ResultObject
{
    public RelatedEntity(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public string Name => RequiredElement("name").GetString()!;
}

public sealed class RelatedEntityEdge : ResultObject
{
    public RelatedEntityEdge(JsonElement document) : base(document) { }
    public Presence<bool> Either => Optional<bool>("either", member => member.GetBoolean());
    public double Probability => RequiredElement("probability").GetDouble();
    public string Relation => RequiredElement("relation").GetString()!;
    public RelatedEntityEdgePropertiesSource Source => global::ThinkThen.Results.RelatedEntityEdgePropertiesSource.Read(RequiredElement("source"));
    public RelatedEntityEdgePropertiesSource Target => global::ThinkThen.Results.RelatedEntityEdgePropertiesSource.Read(RequiredElement("target"));
}

public abstract class RelatedEntityEdgePropertiesSource : ResultObject
{
    protected RelatedEntityEdgePropertiesSource(JsonElement document) : base(document) { }
    public static RelatedEntityEdgePropertiesSource Read(JsonElement document)
    {
        RelatedEntityEdgePropertiesSource? result = null;
        if (document.TryGetProperty("kind", out var tagRelatedEntityEdgePropertiesSourceFieldsKindName0) && (tagRelatedEntityEdgePropertiesSourceFieldsKindName0.ValueKind == JsonValueKind.String) && document.TryGetProperty("name", out var tagRelatedEntityEdgePropertiesSourceFieldsKindName1) && (tagRelatedEntityEdgePropertiesSourceFieldsKindName1.ValueKind == JsonValueKind.String) && !document.TryGetProperty("file", out _) && !document.TryGetProperty("ordinal", out _) && !document.TryGetProperty("record", out _))
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new RelatedEntityEdgePropertiesSourceFieldsKindName(document);
        }
        if (document.TryGetProperty("file", out var tagRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord0) && (tagRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord0.ValueKind == JsonValueKind.String) && document.TryGetProperty("kind", out var tagRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord1) && (tagRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord1.ValueKind == JsonValueKind.String) && document.TryGetProperty("name", out var tagRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord2) && (tagRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord2.ValueKind == JsonValueKind.String) && document.TryGetProperty("ordinal", out var tagRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord3) && (tagRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord3.ValueKind == JsonValueKind.Number) && document.TryGetProperty("record", out var tagRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord4) && (true))
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord : RelatedEntityEdgePropertiesSource
{
    public RelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord(JsonElement document) : base(document) { }
    public Presence<string> File => Optional<string>("file", member => member.GetString()!);
    public Presence<ulong> FirstLine => Optional<ulong>("first_line", member => member.GetUInt64());
    public string Kind => RequiredElement("kind").GetString()!;
    public Presence<ulong> LastLine => Optional<ulong>("last_line", member => member.GetUInt64());
    public string Name => RequiredElement("name").GetString()!;
    public ulong Ordinal => RequiredElement("ordinal").GetUInt64();
    public JsonElement Record => RequiredElement("record").Clone();
}

public sealed class RelatedEntityEdgePropertiesSourceFieldsKindName : RelatedEntityEdgePropertiesSource
{
    public RelatedEntityEdgePropertiesSourceFieldsKindName(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public string Name => RequiredElement("name").GetString()!;
}

public sealed class RelationRule : ResultObject
{
    public RelationRule(JsonElement document) : base(document) { }
    public bool Either => RequiredElement("either").GetBoolean();
    public string Name => RequiredElement("name").GetString()!;
    public string Reads => RequiredElement("reads").GetString()!;
    public Presence<bool> Single => Optional<bool>("single", member => member.GetBoolean());
    public string Source => RequiredElement("source").GetString()!;
    public string Target => RequiredElement("target").GetString()!;
}

public sealed class SessionAnnotation : ResultObject
{
    public SessionAnnotation(JsonElement document) : base(document) { }
    public string Name => RequiredElement("name").GetString()!;
    public AnnotationValue Value => global::ThinkThen.Results.AnnotationValue.Read(RequiredElement("value"));
}

public abstract class SessionJudgment : ResultObject
{
    protected SessionJudgment(JsonElement document) : base(document) { }
    public static SessionJudgment Read(JsonElement document)
    {
        SessionJudgment? result = null;
        if (document.TryGetProperty("kind", out var tagSessionJudgmentDecision) && tagSessionJudgmentDecision.ValueKind == JsonValueKind.String && tagSessionJudgmentDecision.GetString() == "decision")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionJudgmentDecision(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionJudgmentChoice) && tagSessionJudgmentChoice.ValueKind == JsonValueKind.String && tagSessionJudgmentChoice.GetString() == "choice")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionJudgmentChoice(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionJudgmentScore) && tagSessionJudgmentScore.ValueKind == JsonValueKind.String && tagSessionJudgmentScore.GetString() == "score")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionJudgmentScore(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionJudgmentTags) && tagSessionJudgmentTags.ValueKind == JsonValueKind.String && tagSessionJudgmentTags.GetString() == "tags")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionJudgmentTags(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class SessionJudgmentChoice : SessionJudgment
{
    public SessionJudgmentChoice(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public Presence<string> Value => Optional<string>("value", member => member.GetString()!);
}

public sealed class SessionJudgmentDecision : SessionJudgment
{
    public SessionJudgmentDecision(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public Presence<bool> Value => Optional<bool>("value", member => member.GetBoolean());
}

public sealed class SessionJudgmentScore : SessionJudgment
{
    public SessionJudgmentScore(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public double Value => RequiredElement("value").GetDouble();
}

public sealed class SessionJudgmentTags : SessionJudgment
{
    public SessionJudgmentTags(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<string> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
}

public sealed class SessionNamedProbability : ResultObject
{
    public SessionNamedProbability(JsonElement document) : base(document) { }
    public string Name => RequiredElement("name").GetString()!;
    public double Probability => RequiredElement("probability").GetDouble();
}

public abstract class SessionObservation : ResultObject
{
    protected SessionObservation(JsonElement document) : base(document) { }
    public static SessionObservation Read(JsonElement document)
    {
        SessionObservation? result = null;
        if (document.TryGetProperty("kind", out var tagSessionObservationQuestion) && tagSessionObservationQuestion.ValueKind == JsonValueKind.String && tagSessionObservationQuestion.GetString() == "question")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionObservationQuestion(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionObservationRow) && tagSessionObservationRow.ValueKind == JsonValueKind.String && tagSessionObservationRow.GetString() == "row")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionObservationRow(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class SessionObservationQuestion : SessionObservation
{
    public SessionObservationQuestion(JsonElement document) : base(document) { }
    public SessionQuestionDetail Detail => new SessionQuestionDetail(RequiredElement("detail"));
    public ulong Index => RequiredElement("index").GetUInt64();
    public string Kind => RequiredElement("kind").GetString()!;
    public Presence<string> Member => Optional<string>("member", member => member.GetString()!);
    public ulong Position => RequiredElement("position").GetUInt64();
    public Presence<string> Stage => Optional<string>("stage", member => member.GetString()!);
}

public sealed class SessionObservationRow : SessionObservation
{
    public SessionObservationRow(JsonElement document) : base(document) { }
    public ulong Index => RequiredElement("index").GetUInt64();
    public string Kind => RequiredElement("kind").GetString()!;
    public SessionObservedRow Value => global::ThinkThen.Results.SessionObservedRow.Read(RequiredElement("value"));
}

public abstract class SessionObservedRow : ResultObject
{
    protected SessionObservedRow(JsonElement document) : base(document) { }
    public static SessionObservedRow Read(JsonElement document)
    {
        SessionObservedRow? result = null;
        if (document.TryGetProperty("kind", out var tagSessionObservedRowJudgment) && tagSessionObservedRowJudgment.ValueKind == JsonValueKind.String && tagSessionObservedRowJudgment.GetString() == "judgment")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionObservedRowJudgment(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionObservedRowAnnotated) && tagSessionObservedRowAnnotated.ValueKind == JsonValueKind.String && tagSessionObservedRowAnnotated.GetString() == "annotated")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionObservedRowAnnotated(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionObservedRowRecognized) && tagSessionObservedRowRecognized.ValueKind == JsonValueKind.String && tagSessionObservedRowRecognized.GetString() == "recognized")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionObservedRowRecognized(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionObservedRowFind) && tagSessionObservedRowFind.ValueKind == JsonValueKind.String && tagSessionObservedRowFind.GetString() == "find")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionObservedRowFind(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionObservedRowRelations) && tagSessionObservedRowRelations.ValueKind == JsonValueKind.String && tagSessionObservedRowRelations.GetString() == "relations")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionObservedRowRelations(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class SessionObservedRowAnnotated : SessionObservedRow
{
    public SessionObservedRowAnnotated(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<SessionAnnotation> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new SessionAnnotation(item0)).ToArray());
}

public sealed class SessionObservedRowFind : SessionObservedRow
{
    public SessionObservedRowFind(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public Presence<ulong> Value => Optional<ulong>("value", member => member.GetUInt64());
}

public sealed class SessionObservedRowJudgment : SessionObservedRow
{
    public SessionObservedRowJudgment(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public SessionJudgment Value => global::ThinkThen.Results.SessionJudgment.Read(RequiredElement("value"));
}

public sealed class SessionObservedRowRecognized : SessionObservedRow
{
    public SessionObservedRowRecognized(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public SessionRecognition Value => new SessionRecognition(RequiredElement("value"));
}

public sealed class SessionObservedRowRelations : SessionObservedRow
{
    public SessionObservedRowRelations(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<SessionRelationEdge> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new SessionRelationEdge(item0)).ToArray());
}

public abstract class SessionPacket : ResultObject
{
    protected SessionPacket(JsonElement document) : base(document) { }
    public static SessionPacket Read(JsonElement document)
    {
        SessionPacket? result = null;
        if (document.TryGetProperty("function", out var tagSessionPacketDecideRow0) && tagSessionPacketDecideRow0.ValueKind == JsonValueKind.String && tagSessionPacketDecideRow0.GetString() == "decide" && document.TryGetProperty("kind", out var tagSessionPacketDecideRow1) && tagSessionPacketDecideRow1.ValueKind == JsonValueKind.String && tagSessionPacketDecideRow1.GetString() == "row")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketDecideRow(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketChooseRow0) && tagSessionPacketChooseRow0.ValueKind == JsonValueKind.String && tagSessionPacketChooseRow0.GetString() == "choose" && document.TryGetProperty("kind", out var tagSessionPacketChooseRow1) && tagSessionPacketChooseRow1.ValueKind == JsonValueKind.String && tagSessionPacketChooseRow1.GetString() == "row")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketChooseRow(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketTagRow0) && tagSessionPacketTagRow0.ValueKind == JsonValueKind.String && tagSessionPacketTagRow0.GetString() == "tag" && document.TryGetProperty("kind", out var tagSessionPacketTagRow1) && tagSessionPacketTagRow1.ValueKind == JsonValueKind.String && tagSessionPacketTagRow1.GetString() == "row")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketTagRow(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketScoreRow0) && tagSessionPacketScoreRow0.ValueKind == JsonValueKind.String && tagSessionPacketScoreRow0.GetString() == "score" && document.TryGetProperty("kind", out var tagSessionPacketScoreRow1) && tagSessionPacketScoreRow1.ValueKind == JsonValueKind.String && tagSessionPacketScoreRow1.GetString() == "row")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketScoreRow(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketFilterRow0) && tagSessionPacketFilterRow0.ValueKind == JsonValueKind.String && tagSessionPacketFilterRow0.GetString() == "filter" && document.TryGetProperty("kind", out var tagSessionPacketFilterRow1) && tagSessionPacketFilterRow1.ValueKind == JsonValueKind.String && tagSessionPacketFilterRow1.GetString() == "row")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketFilterRow(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketAnnotateRow0) && tagSessionPacketAnnotateRow0.ValueKind == JsonValueKind.String && tagSessionPacketAnnotateRow0.GetString() == "annotate" && document.TryGetProperty("kind", out var tagSessionPacketAnnotateRow1) && tagSessionPacketAnnotateRow1.ValueKind == JsonValueKind.String && tagSessionPacketAnnotateRow1.GetString() == "row")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketAnnotateRow(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketDecideAggregate0) && tagSessionPacketDecideAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketDecideAggregate0.GetString() == "decide" && document.TryGetProperty("kind", out var tagSessionPacketDecideAggregate1) && tagSessionPacketDecideAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketDecideAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketDecideAggregate(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketChooseAggregate0) && tagSessionPacketChooseAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketChooseAggregate0.GetString() == "choose" && document.TryGetProperty("kind", out var tagSessionPacketChooseAggregate1) && tagSessionPacketChooseAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketChooseAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketChooseAggregate(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketTagAggregate0) && tagSessionPacketTagAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketTagAggregate0.GetString() == "tag" && document.TryGetProperty("kind", out var tagSessionPacketTagAggregate1) && tagSessionPacketTagAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketTagAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketTagAggregate(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketScoreAggregate0) && tagSessionPacketScoreAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketScoreAggregate0.GetString() == "score" && document.TryGetProperty("kind", out var tagSessionPacketScoreAggregate1) && tagSessionPacketScoreAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketScoreAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketScoreAggregate(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketFilterAggregate0) && tagSessionPacketFilterAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketFilterAggregate0.GetString() == "filter" && document.TryGetProperty("kind", out var tagSessionPacketFilterAggregate1) && tagSessionPacketFilterAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketFilterAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketFilterAggregate(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketRankAggregate0) && tagSessionPacketRankAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketRankAggregate0.GetString() == "rank" && document.TryGetProperty("kind", out var tagSessionPacketRankAggregate1) && tagSessionPacketRankAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketRankAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketRankAggregate(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketFindAggregate0) && tagSessionPacketFindAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketFindAggregate0.GetString() == "find" && document.TryGetProperty("kind", out var tagSessionPacketFindAggregate1) && tagSessionPacketFindAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketFindAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketFindAggregate(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketAnnotateAggregate0) && tagSessionPacketAnnotateAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketAnnotateAggregate0.GetString() == "annotate" && document.TryGetProperty("kind", out var tagSessionPacketAnnotateAggregate1) && tagSessionPacketAnnotateAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketAnnotateAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketAnnotateAggregate(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketRecognizeAggregate0) && tagSessionPacketRecognizeAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketRecognizeAggregate0.GetString() == "recognize" && document.TryGetProperty("kind", out var tagSessionPacketRecognizeAggregate1) && tagSessionPacketRecognizeAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketRecognizeAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketRecognizeAggregate(document);
        }
        if (document.TryGetProperty("function", out var tagSessionPacketRelateAggregate0) && tagSessionPacketRelateAggregate0.ValueKind == JsonValueKind.String && tagSessionPacketRelateAggregate0.GetString() == "relate" && document.TryGetProperty("kind", out var tagSessionPacketRelateAggregate1) && tagSessionPacketRelateAggregate1.ValueKind == JsonValueKind.String && tagSessionPacketRelateAggregate1.GetString() == "aggregate")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketRelateAggregate(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionPacketObservation0) && tagSessionPacketObservation0.ValueKind == JsonValueKind.String && tagSessionPacketObservation0.GetString() == "observation")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketObservation(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionPacketTerminal0) && tagSessionPacketTerminal0.ValueKind == JsonValueKind.String && tagSessionPacketTerminal0.GetString() == "terminal")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionPacketTerminal(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class SessionPacketAnnotateAggregate : SessionPacket
{
    public SessionPacketAnnotateAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<Annotation> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new Annotation(item0)).ToArray());
}

public sealed class SessionPacketAnnotateRow : SessionPacket
{
    public SessionPacketAnnotateRow(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public Annotation Value => new Annotation(RequiredElement("value"));
}

public sealed class SessionPacketChooseAggregate : SessionPacket
{
    public SessionPacketChooseAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<AtomicNullableString> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new AtomicNullableString(item0)).ToArray());
}

public sealed class SessionPacketChooseRow : SessionPacket
{
    public SessionPacketChooseRow(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public AtomicNullableString Value => new AtomicNullableString(RequiredElement("value"));
}

public sealed class SessionPacketDecideAggregate : SessionPacket
{
    public SessionPacketDecideAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<AtomicDecideValue> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new AtomicDecideValue(item0)).ToArray());
}

public sealed class SessionPacketDecideRow : SessionPacket
{
    public SessionPacketDecideRow(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public AtomicDecideValue Value => new AtomicDecideValue(RequiredElement("value"));
}

public sealed class SessionPacketFilterAggregate : SessionPacket
{
    public SessionPacketFilterAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<AtomicBoolean> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new AtomicBoolean(item0)).ToArray());
}

public sealed class SessionPacketFilterRow : SessionPacket
{
    public SessionPacketFilterRow(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public AtomicBoolean Value => new AtomicBoolean(RequiredElement("value"));
}

public sealed class SessionPacketFindAggregate : SessionPacket
{
    public SessionPacketFindAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public Find Value => new Find(RequiredElement("value"));
}

public sealed class SessionPacketObservation : SessionPacket
{
    public SessionPacketObservation(JsonElement document) : base(document) { }
    public RequestFunction Function => new RequestFunction(RequiredElement("function").GetString()!);
    public string Kind => RequiredElement("kind").GetString()!;
    public SessionObservation Value => global::ThinkThen.Results.SessionObservation.Read(RequiredElement("value"));
}

public sealed class SessionPacketRankAggregate : SessionPacket
{
    public SessionPacketRankAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<AtomicNonZeroUsize> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new AtomicNonZeroUsize(item0)).ToArray());
}

public sealed class SessionPacketRecognizeAggregate : SessionPacket
{
    public SessionPacketRecognizeAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<Recognition> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new Recognition(item0)).ToArray());
}

public sealed class SessionPacketRelateAggregate : SessionPacket
{
    public SessionPacketRelateAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public Relation Value => new Relation(RequiredElement("value"));
}

public sealed class SessionPacketScoreAggregate : SessionPacket
{
    public SessionPacketScoreAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<AtomicDouble> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new AtomicDouble(item0)).ToArray());
}

public sealed class SessionPacketScoreRow : SessionPacket
{
    public SessionPacketScoreRow(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public AtomicDouble Value => new AtomicDouble(RequiredElement("value"));
}

public sealed class SessionPacketTagAggregate : SessionPacket
{
    public SessionPacketTagAggregate(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<AtomicArrayOfString> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new AtomicArrayOfString(item0)).ToArray());
}

public sealed class SessionPacketTagRow : SessionPacket
{
    public SessionPacketTagRow(JsonElement document) : base(document) { }
    public string Function => RequiredElement("function").GetString()!;
    public string Kind => RequiredElement("kind").GetString()!;
    public AtomicArrayOfString Value => new AtomicArrayOfString(RequiredElement("value"));
}

public sealed class SessionPacketTerminal : SessionPacket
{
    public SessionPacketTerminal(JsonElement document) : base(document) { }
    public Presence<Facts> Facts => Optional<Facts>("facts", member => new Facts(member));
    public Presence<CallError> Failure => Optional<CallError>("failure", member => new CallError(member));
    public string Kind => RequiredElement("kind").GetString()!;
}

public abstract class SessionProbabilities : ResultObject
{
    protected SessionProbabilities(JsonElement document) : base(document) { }
    public static SessionProbabilities Read(JsonElement document)
    {
        SessionProbabilities? result = null;
        if (document.TryGetProperty("kind", out var tagSessionProbabilitiesYesNo) && tagSessionProbabilitiesYesNo.ValueKind == JsonValueKind.String && tagSessionProbabilitiesYesNo.GetString() == "yes_no")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionProbabilitiesYesNo(document);
        }
        if (document.TryGetProperty("kind", out var tagSessionProbabilitiesNamed) && tagSessionProbabilitiesNamed.ValueKind == JsonValueKind.String && tagSessionProbabilitiesNamed.GetString() == "named")
        {
            if (result is not null) throw new JsonException("Ambiguous result variant.");
            result = new SessionProbabilitiesNamed(document);
        }
        return result ?? throw new JsonException("Unknown result variant.");
    }
}

public sealed class SessionProbabilitiesNamed : SessionProbabilities
{
    public SessionProbabilitiesNamed(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public IReadOnlyList<SessionNamedProbability> Value => Array.AsReadOnly(RequiredElement("value").EnumerateArray().Select(item0 => new SessionNamedProbability(item0)).ToArray());
}

public sealed class SessionProbabilitiesYesNo : SessionProbabilities
{
    public SessionProbabilitiesYesNo(JsonElement document) : base(document) { }
    public string Kind => RequiredElement("kind").GetString()!;
    public double Value => RequiredElement("value").GetDouble();
}

public sealed class SessionQuestionDetail : ResultObject
{
    public SessionQuestionDetail(JsonElement document) : base(document) { }
    public Presence<string> AnswerId => Optional<string>("answer_id", member => member.GetString()!);
    public bool Cached => RequiredElement("cached").GetBoolean();
    public Presence<double> Confidence => Optional<double>("confidence", member => member.GetDouble());
    public ulong FailedQuestions => RequiredElement("failed_questions").GetUInt64();
    public Presence<Failure> Failure => Optional<Failure>("failure", member => new Failure(member));
    public Presence<string> FailureId => Optional<string>("failure_id", member => member.GetString()!);
    public Presence<JsonElement> Input => Optional<JsonElement>("input", member => member.Clone());
    public IReadOnlyList<JsonElement> Inputs => Array.AsReadOnly(RequiredElement("inputs").EnumerateArray().Select(item0 => item0.Clone()).ToArray());
    public string Model => RequiredElement("model").GetString()!;
    public IReadOnlyList<Observation> Observations => Array.AsReadOnly(RequiredElement("observations").EnumerateArray().Select(item0 => global::ThinkThen.Results.Observation.Read(item0)).ToArray());
    public Presence<SessionProbabilities> Probabilities => Optional<SessionProbabilities>("probabilities", member => global::ThinkThen.Results.SessionProbabilities.Read(member));
    public Question Question => global::ThinkThen.Results.Question.Read(RequiredElement("question"));
    public string QuestionSha256 => RequiredElement("question_sha256").GetString()!;
    public IReadOnlyList<QuestionSource> QuestionSources => Array.AsReadOnly(RequiredElement("question_sources").EnumerateArray().Select(item0 => new QuestionSource(item0)).ToArray());
    public Presence<string> RawPick => Optional<string>("raw_pick", member => member.GetString()!);
    public Presence<Usage> ReportedUsage => Optional<Usage>("reported_usage", member => new Usage(member));
    public IReadOnlyList<string> Requests => Array.AsReadOnly(RequiredElement("requests").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
    public ulong RequestsSent => RequiredElement("requests_sent").GetUInt64();
    public Presence<Threshold> Threshold => Optional<Threshold>("threshold", member => global::ThinkThen.Results.Threshold.Read(member));
    public string Url => RequiredElement("url").GetString()!;
    public Presence<TokenUsage> Usage => Optional<TokenUsage>("usage", member => new TokenUsage(member));
    public Presence<Value> Value => Optional<Value>("value", member => global::ThinkThen.Results.Value.Read(member));
}

public sealed class SessionRecognition : ResultObject
{
    public SessionRecognition(JsonElement document) : base(document) { }
    public IReadOnlyList<Entity> Entities => Array.AsReadOnly(RequiredElement("entities").EnumerateArray().Select(item0 => new Entity(item0)).ToArray());
    public RecognitionMode Mode => new RecognitionMode(RequiredElement("mode").GetString()!);
    public Presence<IReadOnlyList<BoundaryProposal>> Proposals => Optional<IReadOnlyList<BoundaryProposal>>("proposals", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new BoundaryProposal(item0)).ToArray()));
    public Presence<IReadOnlyList<RecognitionEdgeDocument>> Relations => Optional<IReadOnlyList<RecognitionEdgeDocument>>("relations", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new RecognitionEdgeDocument(item0)).ToArray()));
}

public sealed class SessionRelationEdge : ResultObject
{
    public SessionRelationEdge(JsonElement document) : base(document) { }
    public bool Either => RequiredElement("either").GetBoolean();
    public double Probability => RequiredElement("probability").GetDouble();
    public string Relation => RequiredElement("relation").GetString()!;
    public EntityDocument Source => new EntityDocument(RequiredElement("source"));
    public EntityDocument Target => new EntityDocument(RequiredElement("target"));
}

public sealed class SourceRelationEndpoint : ResultObject
{
    public SourceRelationEndpoint(JsonElement document) : base(document) { }
    public Presence<string> File => Optional<string>("file", member => member.GetString()!);
    public Presence<ulong> FirstLine => Optional<ulong>("first_line", member => member.GetUInt64());
    public string Kind => RequiredElement("kind").GetString()!;
    public Presence<ulong> LastLine => Optional<ulong>("last_line", member => member.GetUInt64());
    public string Name => RequiredElement("name").GetString()!;
    public ulong Ordinal => RequiredElement("ordinal").GetUInt64();
    public JsonElement Record => RequiredElement("record").Clone();
}

public abstract class Threshold
{
    private readonly JsonElement document;
    protected Threshold(JsonElement document) { this.document = document.Clone(); }
    protected JsonElement Document => document;
    public JsonElement ToJson() => document.Clone();
    public JsonNode ToPlain() => JsonNode.Parse(document.GetRawText())!;
    public string ToJsonString() => document.GetRawText();
    public static Threshold Read(JsonElement document) => document.ValueKind switch
    {
        JsonValueKind.Number => new ThresholdNumber(document),
        JsonValueKind.String => new ThresholdString(document),
        _ => throw new JsonException("Unknown primitive result variant.")
    };
}

public sealed class ThresholdNumber : Threshold
{
    public ThresholdNumber(JsonElement document) : base(document) { }
    public double Value => Document.GetDouble();
}

public sealed class ThresholdString : Threshold
{
    public ThresholdString(JsonElement document) : base(document) { }
    public string Value => Document.GetString()!;
}

public sealed class TokenUsage : ResultObject
{
    public TokenUsage(JsonElement document) : base(document) { }
    public ulong InputTokens => RequiredElement("input_tokens").GetUInt64();
    public ulong OutputTokens => RequiredElement("output_tokens").GetUInt64();
}

public abstract class Value
{
    private readonly JsonElement document;
    protected Value(JsonElement document) { this.document = document.Clone(); }
    protected JsonElement Document => document;
    public JsonElement ToJson() => document.Clone();
    public JsonNode ToPlain() => JsonNode.Parse(document.GetRawText())!;
    public string ToJsonString() => document.GetRawText();
    public static Value Read(JsonElement document) => document.ValueKind switch
    {
        JsonValueKind.True => new ValueBoolean(document),
        JsonValueKind.False => new ValueBoolean(document),
        JsonValueKind.Null => new ValueNull(document),
        JsonValueKind.String => new ValueString(document),
        JsonValueKind.Array => new ValueArray(document),
        JsonValueKind.Number => new ValueNumber(document),
        _ => throw new JsonException("Unknown primitive result variant.")
    };
}

public sealed class ValueBoolean : Value
{
    public ValueBoolean(JsonElement document) : base(document) { }
    public bool Value => Document.GetBoolean();
}

public sealed class ValueNull : Value
{
    public ValueNull(JsonElement document) : base(document) { }
    public JsonElement Value => Document.Clone();
}

public sealed class ValueString : Value
{
    public ValueString(JsonElement document) : base(document) { }
    public string Value => Document.GetString()!;
}

public sealed class ValueArray : Value
{
    public ValueArray(JsonElement document) : base(document) { }
    public IReadOnlyList<string> Value => Array.AsReadOnly(Document.EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
}

public sealed class ValueNumber : Value
{
    public ValueNumber(JsonElement document) : base(document) { }
    public double Value => Document.GetDouble();
}

