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

public sealed class CallError : ResultObject
{
    public CallError(JsonElement document) : base(document) { }
    public Error Error => new Error(Required("error"));
    public Presence<Facts> Facts => Optional<Facts>("facts", member => new Facts(member));
}

public sealed class Error : ResultObject
{
    public Error(JsonElement document) : base(document) { }
    public Presence<EstimatedInputDenial> EstimatedInputDenial => Optional<EstimatedInputDenial>("estimated_input_denial", member => global::ThinkThen.Results.EstimatedInputDenial.Read(member));
    public FailureKind Kind => new FailureKind(Required("kind").GetString()!);
    public string Message => Required("message").GetString()!;
    public bool Retryable => Required("retryable").GetBoolean();
    public Presence<SendBudgetDenial> SendBudgetDenial => Optional<SendBudgetDenial>("send_budget_denial", member => global::ThinkThen.Results.SendBudgetDenial.Read(member));
    public Stopped Stopped => new Stopped(Required("stopped"));
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
    public string Kind => Required("kind").GetString()!;
    public ulong Limit => Required("limit").GetUInt64();
}

public sealed class EstimatedInputDenialInitialRequest : EstimatedInputDenial
{
    public EstimatedInputDenialInitialRequest(JsonElement document) : base(document) { }
    public string Kind => Required("kind").GetString()!;
    public ulong Limit => Required("limit").GetUInt64();
}

public sealed class EstimatedInputDenialRetry : EstimatedInputDenial
{
    public EstimatedInputDenialRetry(JsonElement document) : base(document) { }
    public string Kind => Required("kind").GetString()!;
    public ushort LastStatus => Required("last_status").GetUInt16();
    public ulong Limit => Required("limit").GetUInt64();
}

public sealed class Facts : ResultObject
{
    public Facts(JsonElement document) : base(document) { }
    public Presence<IReadOnlyList<Attempt>> Attempts => Optional<IReadOnlyList<Attempt>>("attempts", member => Array.AsReadOnly(member.EnumerateArray().Select(item0 => new Attempt(item0)).ToArray()));
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
    public string FailureId => Required("failure_id").GetString()!;
}

public sealed class ObservationObservationId : Observation
{
    public ObservationObservationId(JsonElement document) : base(document) { }
    public string ObservationId => Required("observation_id").GetString()!;
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
    public string ObservedAt => Required("observed_at").GetString()!;
    public UsagePersistence State => new UsagePersistence(Required("state").GetString()!);
}

public sealed class QuestionSource : ResultObject
{
    public QuestionSource(JsonElement document) : base(document) { }
    public string AnsweredBy => Required("answered_by").GetString()!;
    public Presence<uint> BatchSize => Optional<uint>("batch_size", member => member.GetUInt32());
    public Origin Origin => new Origin(Required("origin").GetString()!);
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
    public RelationDirection Direction => new RelationDirection(Required("direction").GetString()!);
    public RelationMethod Method => new RelationMethod(Required("method").GetString()!);
    public IReadOnlyList<Observation> Observations => Array.AsReadOnly(Required("observations").EnumerateArray().Select(item0 => global::ThinkThen.Results.Observation.Read(item0)).ToArray());
    public Question Question => global::ThinkThen.Results.Question.Read(Required("question"));
    public IReadOnlyList<QuestionSource> QuestionSources => Array.AsReadOnly(Required("question_sources").EnumerateArray().Select(item0 => new QuestionSource(item0)).ToArray());
    public string Reads => Required("reads").GetString()!;
    public string Relation => Required("relation").GetString()!;
    public string Request => Required("request").GetString()!;
    public RelatedEntity Source => new RelatedEntity(Required("source"));
    public Presence<RelatedEntity> Target => Optional<RelatedEntity>("target", member => new RelatedEntity(member));
    public Threshold Threshold => global::ThinkThen.Results.Threshold.Read(Required("threshold"));
    public Presence<Usage> Usage => Optional<Usage>("usage", member => new Usage(member));
    public bool Accepted => Required("accepted").GetBoolean();
    public Answer Answer => global::ThinkThen.Results.Answer.Read(Required("answer"));
    public string AnswerId => Required("answer_id").GetString()!;
    public double Probability => Required("probability").GetDouble();
}

public sealed class RelationMemberFailureId : RelationMember
{
    public RelationMemberFailureId(JsonElement document) : base(document) { }
    public RelationDirection Direction => new RelationDirection(Required("direction").GetString()!);
    public RelationMethod Method => new RelationMethod(Required("method").GetString()!);
    public IReadOnlyList<Observation> Observations => Array.AsReadOnly(Required("observations").EnumerateArray().Select(item0 => global::ThinkThen.Results.Observation.Read(item0)).ToArray());
    public Question Question => global::ThinkThen.Results.Question.Read(Required("question"));
    public IReadOnlyList<QuestionSource> QuestionSources => Array.AsReadOnly(Required("question_sources").EnumerateArray().Select(item0 => new QuestionSource(item0)).ToArray());
    public string Reads => Required("reads").GetString()!;
    public string Relation => Required("relation").GetString()!;
    public string Request => Required("request").GetString()!;
    public RelatedEntity Source => new RelatedEntity(Required("source"));
    public Presence<RelatedEntity> Target => Optional<RelatedEntity>("target", member => new RelatedEntity(member));
    public Threshold Threshold => global::ThinkThen.Results.Threshold.Read(Required("threshold"));
    public Presence<Usage> Usage => Optional<Usage>("usage", member => new Usage(member));
    public Failure Failure => new Failure(Required("failure"));
    public string FailureId => Required("failure_id").GetString()!;
}

public readonly record struct RelationMethod(string Value)
{
    public static RelationMethod YesNo => new("yes_no");
    public static RelationMethod Choice => new("choice");
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
    public string Kind => Required("kind").GetString()!;
}

public sealed class SendBudgetDenialBeforeFirstSend : SendBudgetDenial
{
    public SendBudgetDenialBeforeFirstSend(JsonElement document) : base(document) { }
    public string Kind => Required("kind").GetString()!;
}

public sealed class SendBudgetDenialBeforeRetry : SendBudgetDenial
{
    public SendBudgetDenialBeforeRetry(JsonElement document) : base(document) { }
    public string Kind => Required("kind").GetString()!;
    public ushort LastStatus => Required("last_status").GetUInt16();
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
    public StopCause Cause => new StopCause(Required("cause").GetString()!);
    public bool Retryable => Required("retryable").GetBoolean();
    public Presence<ushort> Status => Optional<ushort>("status", member => member.GetUInt16());
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
    public string Kind => Required("kind").GetString()!;
    public string Pick => Required("pick").GetString()!;
    public IReadOnlyDictionary<string, double> Probabilities => new ReadOnlyDictionary<string, double>(Required("probabilities").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal));
}

public sealed class AnswerScore : Answer
{
    public AnswerScore(JsonElement document) : base(document) { }
    public Presence<double> Confidence => Optional<double>("confidence", member => member.GetDouble());
    public string Kind => Required("kind").GetString()!;
    public string Level => Required("level").GetString()!;
    public IReadOnlyDictionary<string, double> Probabilities => new ReadOnlyDictionary<string, double>(Required("probabilities").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal));
}

public sealed class AnswerTag : Answer
{
    public AnswerTag(JsonElement document) : base(document) { }
    public string Kind => Required("kind").GetString()!;
    public IReadOnlyDictionary<string, double> Probabilities => new ReadOnlyDictionary<string, double>(Required("probabilities").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal));
}

public sealed class AnswerYesNo : Answer
{
    public AnswerYesNo(JsonElement document) : base(document) { }
    public string Kind => Required("kind").GetString()!;
    public double Probability => Required("probability").GetDouble();
}

public readonly record struct AttemptOutcome(string Value)
{
    public static AttemptOutcome Ok => new("ok");
    public static AttemptOutcome Status => new("status");
    public static AttemptOutcome Transport => new("transport");
}

public sealed class Failure : ResultObject
{
    public Failure(JsonElement document) : base(document) { }
    public FailureCause Cause => new FailureCause(Required("cause").GetString()!);
    public string Kind => Required("kind").GetString()!;
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
    public string Kind => Required("kind").GetString()!;
    public string Pick => Required("pick").GetString()!;
    public IReadOnlyDictionary<string, double> Probabilities => new ReadOnlyDictionary<string, double>(Required("probabilities").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => entry0.Value.GetDouble(), StringComparer.Ordinal));
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

public sealed class QuestionChoose : Question
{
    public QuestionChoose(JsonElement document) : base(document) { }
    public IReadOnlyList<string> Options => Array.AsReadOnly(Required("options").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
    public JsonElement Text => Required("text").Clone();
    public string Verb => Required("verb").GetString()!;
}

public sealed class QuestionDecide : Question
{
    public QuestionDecide(JsonElement document) : base(document) { }
    public Presence<JsonElement> False => Optional<JsonElement>("false", member => member.Clone());
    public JsonElement Text => Required("text").Clone();
    public Presence<JsonElement> True => Optional<JsonElement>("true", member => member.Clone());
    public string Verb => Required("verb").GetString()!;
}

public sealed class QuestionScore : Question
{
    public QuestionScore(JsonElement document) : base(document) { }
    public IReadOnlyList<string> Levels => Array.AsReadOnly(Required("levels").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
    public JsonElement Text => Required("text").Clone();
    public string Verb => Required("verb").GetString()!;
}

public sealed class QuestionTag : Question
{
    public QuestionTag(JsonElement document) : base(document) { }
    public IReadOnlyList<string> Labels => Array.AsReadOnly(Required("labels").EnumerateArray().Select(item0 => item0.GetString()!).ToArray());
    public JsonElement Text => Required("text").Clone();
    public string Verb => Required("verb").GetString()!;
}

public sealed class RelatedEntity : ResultObject
{
    public RelatedEntity(JsonElement document) : base(document) { }
    public string Kind => Required("kind").GetString()!;
    public string Name => Required("name").GetString()!;
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

