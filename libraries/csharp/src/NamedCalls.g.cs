// Generated from canonical RequestFunction; do not edit.
#nullable enable
using ThinkThen.Inputs;
using ThinkThen.Results;
using System.Text.Json;
namespace ThinkThen;
public sealed partial class Engine {
    public Task<OwnedCall> DecideAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options), null, cancellation);
    public Task<OwnedCall> DecideAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("decide", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> ChooseAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options), null, cancellation);
    public Task<OwnedCall> ChooseAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("choose", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> TagAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options), null, cancellation);
    public Task<OwnedCall> TagAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("tag", question, FeedInput, options), feed,
        cancellation);
    }
    public Task<OwnedCall> ScoreAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("score", question, input, options), null, cancellation);
    public Task<OwnedCall> ScoreAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("score", question, FeedInput, options), feed,
        cancellation);
    }
    public Task<OwnedCall> FilterAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options), null, cancellation);
    public Task<OwnedCall> FilterAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("filter", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> RankAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options), null, cancellation);
    public Task<OwnedCall> RankAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("rank", question, FeedInput, options), feed,
        cancellation);
    }
    public Task<OwnedCall> FindAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("find", question, input, options), null, cancellation);
    public Task<OwnedCall> FindAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("find", question, FeedInput, options), feed,
        cancellation);
    }
    public Task<OwnedCall> AnnotateAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options), null, cancellation);
    public Task<OwnedCall> AnnotateAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("annotate", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> RecognizeAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options), null, cancellation);
    public Task<OwnedCall> RecognizeAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("recognize", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> RelateAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options), null, cancellation);
    public Task<OwnedCall> RelateAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> feed,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("relate", question, FeedInput, options),
        feed, cancellation);
    }
}
public sealed class AnswerCardinalityFailure : InvalidOperationException {
    public OwnedCall Call {
        get;
    }
    internal AnswerCardinalityFailure(string function, OwnedCall call) : base("Expected exactly one " + function + " judgment.") {
        Call = call;
    }
}
public static class AnswerAccessors {
    private static void Successful(OwnedCall call) {
        ArgumentNullException.ThrowIfNull(call);
        if (call.Terminal is null || call.Terminal.Kind != "terminal") throw new AnswerCardinalityFailure("terminal",
        call);
        if (call.Terminal.Failure.State == ThinkThen.Results.PresenceState.Null) throw new System.Text.Json.JsonException("Terminal failure must be an object.");
        if (call.Terminal.Failure.State == ThinkThen.Results.PresenceState.Value) throw new SessionFailure(call.Terminal.Failure.Value,
        call);
    }
    public static bool? Decision(this OwnedCall call) {
        Successful(call);
        var values = new List<bool?>();
        foreach (var packet in call.Packets) if (packet is ThinkThen.Results.SessionPacketObservation {
            Value: ThinkThen.Results.SessionObservationRow {
                Value: ThinkThen.Results.SessionObservedRowJudgment {
                    Value: ThinkThen.Results.SessionJudgmentDecision judgment
                }
            }
        } && ((ThinkThen.Results.SessionPacketObservation)packet).Function.Value == "decide") values.Add(judgment.Value.State == ThinkThen.Results.PresenceState.Null ? null : judgment.Value.Value);
        if (values.Count != 1) throw new AnswerCardinalityFailure("decide", call);
        return values[0];
    }
    public static string? Choice(this OwnedCall call) {
        Successful(call);
        var values = new List<string?>();
        foreach (var packet in call.Packets) if (packet is ThinkThen.Results.SessionPacketObservation {
            Value: ThinkThen.Results.SessionObservationRow {
                Value: ThinkThen.Results.SessionObservedRowJudgment {
                    Value: ThinkThen.Results.SessionJudgmentChoice judgment
                }
            }
        } && ((ThinkThen.Results.SessionPacketObservation)packet).Function.Value == "choose") values.Add(judgment.Value.State == ThinkThen.Results.PresenceState.Null ? null : judgment.Value.Value);
        if (values.Count != 1) throw new AnswerCardinalityFailure("choose", call);
        return values[0];
    }
    public static double Score(this OwnedCall call) {
        Successful(call);
        var values = new List<double>();
        foreach (var packet in call.Packets) if (packet is ThinkThen.Results.SessionPacketObservation {
            Value: ThinkThen.Results.SessionObservationRow {
                Value: ThinkThen.Results.SessionObservedRowJudgment {
                    Value: ThinkThen.Results.SessionJudgmentScore judgment
                }
            }
        } && ((ThinkThen.Results.SessionPacketObservation)packet).Function.Value == "score") values.Add(judgment.Value);
        if (values.Count != 1) throw new AnswerCardinalityFailure("score", call);
        return values[0];
    }
    public static IReadOnlyList<string> Tags(this OwnedCall call) {
        Successful(call);
        var values = new List<IReadOnlyList<string>>();
        foreach (var packet in call.Packets) if (packet is ThinkThen.Results.SessionPacketObservation {
            Value: ThinkThen.Results.SessionObservationRow {
                Value: ThinkThen.Results.SessionObservedRowJudgment {
                    Value: ThinkThen.Results.SessionJudgmentTags judgment
                }
            }
        } && ((ThinkThen.Results.SessionPacketObservation)packet).Function.Value == "tag") values.Add(judgment.Value);
        if (values.Count != 1) throw new AnswerCardinalityFailure("tag", call);
        return values[0];
    }
    public static IReadOnlyList<JsonElement> Filter(this OwnedCall call) {
        Successful(call);
        var values = new List<AtomicBoolean>();
        foreach (var packet in call.Packets) {
            if (packet is ThinkThen.Results.SessionPacketFilterAggregate aggregate) values.AddRange(aggregate.Value);
            else if (packet is ThinkThen.Results.SessionPacketFilterRow row) values.Add(row.Value);
        }
        return Array.AsReadOnly(values.Select(value => value.Input.State == ThinkThen.Results.PresenceState.Null ? System.Text.Json.JsonSerializer.SerializeToElement<object?>(null) : value.Input.Value).ToArray());
    }
    public static IReadOnlyList<ulong> Rank(this OwnedCall call) {
        Successful(call);
        var values = new List<AtomicNonZeroUsize>();
        foreach (var packet in call.Packets) {
            if (packet is ThinkThen.Results.SessionPacketRankAggregate aggregate) values.AddRange(aggregate.Value);
        }
        return Array.AsReadOnly(values.Select(value => value.Index.Value).ToArray());
    }
    public static Find Find(this OwnedCall call) {
        Successful(call);
        var values = call.Packets.OfType<ThinkThen.Results.SessionPacketFindAggregate>().Select(packet => packet.Value).ToArray();
        if (values.Length != 1) throw new AnswerCardinalityFailure("find", call);
        return values[0];
    }
    public static IReadOnlyList<Annotation> Annotate(this OwnedCall call) {
        Successful(call);
        var values = new List<Annotation>();
        foreach (var packet in call.Packets) {
            if (packet is ThinkThen.Results.SessionPacketAnnotateAggregate aggregate) values.AddRange(aggregate.Value);
            else if (packet is ThinkThen.Results.SessionPacketAnnotateRow row) values.Add(row.Value);
        }
        return values.AsReadOnly();
    }
    public static IReadOnlyList<Recognition> Recognize(this OwnedCall call) {
        Successful(call);
        var values = new List<Recognition>();
        foreach (var packet in call.Packets) {
            if (packet is ThinkThen.Results.SessionPacketRecognizeAggregate aggregate) values.AddRange(aggregate.Value);
        }
        return values.AsReadOnly();
    }
    public static Relation Relate(this OwnedCall call) {
        Successful(call);
        var values = call.Packets.OfType<ThinkThen.Results.SessionPacketRelateAggregate>().Select(packet => packet.Value).ToArray();
        if (values.Length != 1) throw new AnswerCardinalityFailure("relate", call);
        return values[0];
    }
}
public sealed partial class Engine {
    public async Task<bool?> DecideValueAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await DecideAsync(question, input, options, cancellation).ConfigureAwait(false)).Decision();
    public async Task<string?> ChooseValueAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await ChooseAsync(question, input, options, cancellation).ConfigureAwait(false)).Choice();
    public async Task<double> ScoreValueAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await ScoreAsync(question, input, options, cancellation).ConfigureAwait(false)).Score();
    public async Task<IReadOnlyList<string>> TagValueAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await TagAsync(question, input, options, cancellation).ConfigureAwait(false)).Tags();
    public async Task<IReadOnlyList<JsonElement>> FilterValueAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await FilterAsync(question, input, options, cancellation).ConfigureAwait(false)).Filter();
    public async Task<IReadOnlyList<ulong>> RankValueAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await RankAsync(question, input, options, cancellation).ConfigureAwait(false)).Rank();
    public async Task<Find> FindValueAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await FindAsync(question, input, options, cancellation).ConfigureAwait(false)).Find();
    public async Task<IReadOnlyList<Annotation>> AnnotateValueAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await AnnotateAsync(question, input, options, cancellation).ConfigureAwait(false)).Annotate();
    public async Task<IReadOnlyList<Recognition>> RecognizeValueAsync(RequestQuestion question,
    RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await RecognizeAsync(question, input, options, cancellation).ConfigureAwait(false)).Recognize();
    public async Task<Relation> RelateValueAsync(RequestQuestion question, RequestInput input,
    RequestOptions? options = null, CancellationToken cancellation = default) =>
    (await RelateAsync(question, input, options, cancellation).ConfigureAwait(false)).Relate();
}
