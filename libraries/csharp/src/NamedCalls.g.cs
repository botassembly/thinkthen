// Generated from canonical RequestFunction; do not edit.
#nullable enable
using ThinkThen.Inputs;
namespace ThinkThen;
public sealed partial class Engine {
    public Task<OwnedCall> DecideAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options), null, cancellation);
    public Task<OwnedCall> DecideAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("decide", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> ChooseAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options), null, cancellation);
    public Task<OwnedCall> ChooseAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("choose", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> TagAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options), null, cancellation);
    public Task<OwnedCall> TagAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("tag", question, FeedInput, options), feed,
        cancellation);
    }
    public Task<OwnedCall> ScoreAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("score", question, input, options), null, cancellation);
    public Task<OwnedCall> ScoreAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("score", question, FeedInput, options), feed,
        cancellation);
    }
    public Task<OwnedCall> FilterAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options), null, cancellation);
    public Task<OwnedCall> FilterAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("filter", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> RankAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options), null, cancellation);
    public Task<OwnedCall> RankAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("rank", question, FeedInput, options), feed,
        cancellation);
    }
    public Task<OwnedCall> FindAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("find", question, input, options), null, cancellation);
    public Task<OwnedCall> FindAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("find", question, FeedInput, options), feed,
        cancellation);
    }
    public Task<OwnedCall> AnnotateAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options), null, cancellation);
    public Task<OwnedCall> AnnotateAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("annotate", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> RecognizeAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options), null, cancellation);
    public Task<OwnedCall> RecognizeAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(feed);
        return ExecuteSessionAsync(() => StartNamedSession("recognize", question, FeedInput, options),
        feed, cancellation);
    }
    public Task<OwnedCall> RelateAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options), null, cancellation);
    public Task<OwnedCall> RelateAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
    InputRequestOptions? options = null, CancellationToken cancellation = default) {
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
}
public sealed partial class Engine {
    public async Task<bool?> DecideValueAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    (await DecideAsync(question, input, options, cancellation).ConfigureAwait(false)).Decision();
    public async Task<string?> ChooseValueAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    (await ChooseAsync(question, input, options, cancellation).ConfigureAwait(false)).Choice();
    public async Task<double> ScoreValueAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    (await ScoreAsync(question, input, options, cancellation).ConfigureAwait(false)).Score();
    public async Task<IReadOnlyList<string>> TagValueAsync(InputRequestQuestion question, InputRequestInput input,
    InputRequestOptions? options = null, CancellationToken cancellation = default) =>
    (await TagAsync(question, input, options, cancellation).ConfigureAwait(false)).Tags();
}
