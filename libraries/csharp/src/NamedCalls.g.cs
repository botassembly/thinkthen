// Generated from canonical RequestFunction; do not edit.
#nullable enable
using ThinkThen.Inputs;
using ThinkThen.Results;
using System.Text.Json;
namespace ThinkThen;
public sealed record CallResult<TValue, TProbability, TResults> : OwnedCall {
    public TValue Value {
        get;
    }
    public TProbability Probability {
        get;
    }
    public TResults Results {
        get;
    }
    public Facts Facts {
        get;
    }
    internal CallResult(OwnedCall call, TValue value, TProbability probability, TResults results) : base(call.Packets,
    call.Terminal) {
        Value = value;
        Probability = probability;
        Results = results;
        Facts = call.Terminal.Facts.State == PresenceState.Value ? call.Terminal.Facts.Value : throw new JsonException("Successful call has no facts.");
    }
}
public sealed partial class Engine {
    public async Task<CallResult<bool?, double?, IReadOnlyList<AtomicDecideValue>>> DecideAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectDecideScalar(call);
    }
    public async Task<CallResult<bool?, double?, IReadOnlyList<AtomicDecideValue>>> DecideAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectDecideScalar(call);
    }
    public async Task<CallResult<IReadOnlyList<bool?>, IReadOnlyList<double?>, IReadOnlyList<AtomicDecideValue>>> DecideAsync(RequestQuestion question,
    RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectDecide(call);
    }
    public async Task<CallResult<IReadOnlyList<bool?>, IReadOnlyList<double?>, IReadOnlyList<AtomicDecideValue>>> DecideAsync(RequestQuestion question,
    RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectDecide(call);
    }
    public async Task<CallResult<IReadOnlyList<bool?>, IReadOnlyList<double?>, IReadOnlyList<AtomicDecideValue>>> DecideAsync(RequestQuestion question,
    RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectDecide(call);
    }
    public async Task<CallResult<IReadOnlyList<bool?>, IReadOnlyList<double?>, IReadOnlyList<AtomicDecideValue>>> DecideAsync(RequestQuestion question,
    RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectDecide(call);
    }
    public async Task<CallResult<IReadOnlyList<bool?>, IReadOnlyList<double?>, IReadOnlyList<AtomicDecideValue>>> DecideAsync(RequestQuestion question,
    RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("decide", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectDecide(call);
    }
    public async Task<CallResult<IReadOnlyList<bool?>, IReadOnlyList<double?>, IReadOnlyList<AtomicDecideValue>>> DecideAsync(RequestQuestion question,
    IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("decide", question, FeedInput,
        options), input, cancellation).ConfigureAwait(false);
        return ProjectDecide(call);
    }
    private static CallResult<IReadOnlyList<bool?>, IReadOnlyList<double?>, IReadOnlyList<AtomicDecideValue>> ProjectDecide(OwnedCall call) {
        var receipts = new List<AtomicDecideValue>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketDecideAggregate aggregate) receipts.AddRange(aggregate.Value);
            else if (packet is SessionPacketDecideRow row) receipts.Add(row.Value);
        }
        var results = receipts.AsReadOnly();
        var values = new List<bool?>();
        foreach (var packet in call.Packets) if (packet is SessionPacketObservation {
            Value: SessionObservationRow {
                Value: SessionObservedRowJudgment {
                    Value: SessionJudgmentDecision judgment
                }
            }
        } observation && observation.Function.Value == "decide") values.Add(judgment.Value.State == PresenceState.Value ? judgment.Value.Value : null);
        return new(call, values.AsReadOnly(), Array.AsReadOnly(results.Select(row => (double?)((AnswerYesNo)row.Answer).Probability).ToArray()),
        results);
    }
    private static CallResult<bool?, double?, IReadOnlyList<AtomicDecideValue>> ProjectDecideScalar(OwnedCall call) {
        var result = ProjectDecide(call);
        return new(call, result.Value.Single(), result.Probability.Single(), result.Results);
    }
    public async Task<CallResult<string?, double?, IReadOnlyList<AtomicNullableString>>> ChooseAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectChooseScalar(call);
    }
    public async Task<CallResult<string?, double?, IReadOnlyList<AtomicNullableString>>> ChooseAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectChooseScalar(call);
    }
    public async Task<CallResult<IReadOnlyList<string?>, IReadOnlyList<double?>, IReadOnlyList<AtomicNullableString>>> ChooseAsync(RequestQuestion question,
    RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectChoose(call);
    }
    public async Task<CallResult<IReadOnlyList<string?>, IReadOnlyList<double?>, IReadOnlyList<AtomicNullableString>>> ChooseAsync(RequestQuestion question,
    RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectChoose(call);
    }
    public async Task<CallResult<IReadOnlyList<string?>, IReadOnlyList<double?>, IReadOnlyList<AtomicNullableString>>> ChooseAsync(RequestQuestion question,
    RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectChoose(call);
    }
    public async Task<CallResult<IReadOnlyList<string?>, IReadOnlyList<double?>, IReadOnlyList<AtomicNullableString>>> ChooseAsync(RequestQuestion question,
    RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectChoose(call);
    }
    public async Task<CallResult<IReadOnlyList<string?>, IReadOnlyList<double?>, IReadOnlyList<AtomicNullableString>>> ChooseAsync(RequestQuestion question,
    RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("choose", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectChoose(call);
    }
    public async Task<CallResult<IReadOnlyList<string?>, IReadOnlyList<double?>, IReadOnlyList<AtomicNullableString>>> ChooseAsync(RequestQuestion question,
    IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("choose", question, FeedInput,
        options), input, cancellation).ConfigureAwait(false);
        return ProjectChoose(call);
    }
    private static CallResult<IReadOnlyList<string?>, IReadOnlyList<double?>, IReadOnlyList<AtomicNullableString>> ProjectChoose(OwnedCall call) {
        var receipts = new List<AtomicNullableString>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketChooseAggregate aggregate) receipts.AddRange(aggregate.Value);
            else if (packet is SessionPacketChooseRow row) receipts.Add(row.Value);
        }
        var results = receipts.AsReadOnly();
        var values = new List<string?>();
        foreach (var packet in call.Packets) if (packet is SessionPacketObservation {
            Value: SessionObservationRow {
                Value: SessionObservedRowJudgment {
                    Value: SessionJudgmentChoice judgment
                }
            }
        } observation && observation.Function.Value == "choose") values.Add(judgment.Value.State == PresenceState.Value ? judgment.Value.Value : null);
        return new(call, values.AsReadOnly(), Array.AsReadOnly(results.Select(row => row.Answer is AnswerChoice odds ? (double?)odds.Probabilities[odds.Pick] : null).ToArray()),
        results);
    }
    private static CallResult<string?, double?, IReadOnlyList<AtomicNullableString>> ProjectChooseScalar(OwnedCall call) {
        var result = ProjectChoose(call);
        return new(call, result.Value.Single(), result.Probability.Single(), result.Results);
    }
    public async Task<CallResult<IReadOnlyList<string>, double?, IReadOnlyList<AtomicArrayOfString>>> TagAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectTagScalar(call);
    }
    public async Task<CallResult<IReadOnlyList<string>, double?, IReadOnlyList<AtomicArrayOfString>>> TagAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectTagScalar(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyList<string>>, double?, IReadOnlyList<AtomicArrayOfString>>> TagAsync(RequestQuestion question,
    RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectTag(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyList<string>>, double?, IReadOnlyList<AtomicArrayOfString>>> TagAsync(RequestQuestion question,
    RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectTag(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyList<string>>, double?, IReadOnlyList<AtomicArrayOfString>>> TagAsync(RequestQuestion question,
    RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectTag(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyList<string>>, double?, IReadOnlyList<AtomicArrayOfString>>> TagAsync(RequestQuestion question,
    RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectTag(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyList<string>>, double?, IReadOnlyList<AtomicArrayOfString>>> TagAsync(RequestQuestion question,
    RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("tag", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectTag(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyList<string>>, double?, IReadOnlyList<AtomicArrayOfString>>> TagAsync(RequestQuestion question,
    IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("tag", question, FeedInput, options),
        input, cancellation).ConfigureAwait(false);
        return ProjectTag(call);
    }
    private static CallResult<IReadOnlyList<IReadOnlyList<string>>, double?, IReadOnlyList<AtomicArrayOfString>> ProjectTag(OwnedCall call) {
        var receipts = new List<AtomicArrayOfString>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketTagAggregate aggregate) receipts.AddRange(aggregate.Value);
            else if (packet is SessionPacketTagRow row) receipts.Add(row.Value);
        }
        var results = receipts.AsReadOnly();
        var values = new List<IReadOnlyList<string>>();
        foreach (var packet in call.Packets) if (packet is SessionPacketObservation {
            Value: SessionObservationRow {
                Value: SessionObservedRowJudgment {
                    Value: SessionJudgmentTags judgment
                }
            }
        } observation && observation.Function.Value == "tag") values.Add(judgment.Value);
        return new(call, values.AsReadOnly(), (double?)null, results);
    }
    private static CallResult<IReadOnlyList<string>, double?, IReadOnlyList<AtomicArrayOfString>> ProjectTagScalar(OwnedCall call) {
        var result = ProjectTag(call);
        return new(call, result.Value.Single(), result.Probability, result.Results);
    }
    public async Task<CallResult<double, double?, IReadOnlyList<AtomicDouble>>> ScoreAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("score", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectScoreScalar(call);
    }
    public async Task<CallResult<double, double?, IReadOnlyList<AtomicDouble>>> ScoreAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("score", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectScoreScalar(call);
    }
    public async Task<CallResult<IReadOnlyList<double>, double?, IReadOnlyList<AtomicDouble>>> ScoreAsync(RequestQuestion question,
    RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("score", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectScore(call);
    }
    public async Task<CallResult<IReadOnlyList<double>, double?, IReadOnlyList<AtomicDouble>>> ScoreAsync(RequestQuestion question,
    RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("score", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectScore(call);
    }
    public async Task<CallResult<IReadOnlyList<double>, double?, IReadOnlyList<AtomicDouble>>> ScoreAsync(RequestQuestion question,
    RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("score", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectScore(call);
    }
    public async Task<CallResult<IReadOnlyList<double>, double?, IReadOnlyList<AtomicDouble>>> ScoreAsync(RequestQuestion question,
    RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("score", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectScore(call);
    }
    public async Task<CallResult<IReadOnlyList<double>, double?, IReadOnlyList<AtomicDouble>>> ScoreAsync(RequestQuestion question,
    RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("score", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectScore(call);
    }
    public async Task<CallResult<IReadOnlyList<double>, double?, IReadOnlyList<AtomicDouble>>> ScoreAsync(RequestQuestion question,
    IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("score", question, FeedInput, options),
        input, cancellation).ConfigureAwait(false);
        return ProjectScore(call);
    }
    private static CallResult<IReadOnlyList<double>, double?, IReadOnlyList<AtomicDouble>> ProjectScore(OwnedCall call) {
        var receipts = new List<AtomicDouble>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketScoreAggregate aggregate) receipts.AddRange(aggregate.Value);
            else if (packet is SessionPacketScoreRow row) receipts.Add(row.Value);
        }
        var results = receipts.AsReadOnly();
        var values = new List<double>();
        foreach (var packet in call.Packets) if (packet is SessionPacketObservation {
            Value: SessionObservationRow {
                Value: SessionObservedRowJudgment {
                    Value: SessionJudgmentScore judgment
                }
            }
        } observation && observation.Function.Value == "score") values.Add(judgment.Value);
        return new(call, values.AsReadOnly(), (double?)null, results);
    }
    private static CallResult<double, double?, IReadOnlyList<AtomicDouble>> ProjectScoreScalar(OwnedCall call) {
        var result = ProjectScore(call);
        return new(call, result.Value.Single(), result.Probability, result.Results);
    }
    public async Task<CallResult<IReadOnlyList<JsonElement>, IReadOnlyList<double?>, IReadOnlyList<AtomicBoolean>>> FilterAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFilter(call);
    }
    public async Task<CallResult<IReadOnlyList<JsonElement>, IReadOnlyList<double?>, IReadOnlyList<AtomicBoolean>>> FilterAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFilter(call);
    }
    public async Task<CallResult<IReadOnlyList<JsonElement>, IReadOnlyList<double?>, IReadOnlyList<AtomicBoolean>>> FilterAsync(RequestQuestion question,
    RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFilter(call);
    }
    public async Task<CallResult<IReadOnlyList<JsonElement>, IReadOnlyList<double?>, IReadOnlyList<AtomicBoolean>>> FilterAsync(RequestQuestion question,
    RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFilter(call);
    }
    public async Task<CallResult<IReadOnlyList<JsonElement>, IReadOnlyList<double?>, IReadOnlyList<AtomicBoolean>>> FilterAsync(RequestQuestion question,
    RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFilter(call);
    }
    public async Task<CallResult<IReadOnlyList<JsonElement>, IReadOnlyList<double?>, IReadOnlyList<AtomicBoolean>>> FilterAsync(RequestQuestion question,
    RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFilter(call);
    }
    public async Task<CallResult<IReadOnlyList<JsonElement>, IReadOnlyList<double?>, IReadOnlyList<AtomicBoolean>>> FilterAsync(RequestQuestion question,
    RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("filter", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFilter(call);
    }
    public async Task<CallResult<IReadOnlyList<JsonElement>, IReadOnlyList<double?>, IReadOnlyList<AtomicBoolean>>> FilterAsync(RequestQuestion question,
    IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("filter", question, FeedInput,
        options), input, cancellation).ConfigureAwait(false);
        return ProjectFilter(call);
    }
    private static CallResult<IReadOnlyList<JsonElement>, IReadOnlyList<double?>, IReadOnlyList<AtomicBoolean>> ProjectFilter(OwnedCall call) {
        var receipts = new List<AtomicBoolean>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketFilterAggregate aggregate) receipts.AddRange(aggregate.Value);
            else if (packet is SessionPacketFilterRow row) receipts.Add(row.Value);
        }
        var results = receipts.AsReadOnly();
        var values = results.Select(row => row.Input.State == PresenceState.Value ? row.Input.Value : JsonSerializer.SerializeToElement<object?>(null)).ToArray();
        return new(call, Array.AsReadOnly(values), Array.AsReadOnly(results.Select(row => (double?)((AnswerYesNo)row.Answer).Probability).ToArray()),
        results);
    }
    public async Task<CallResult<IReadOnlyList<ulong>, IReadOnlyList<double?>, IReadOnlyList<AtomicNonZeroUsize>>> RankAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRank(call);
    }
    public async Task<CallResult<IReadOnlyList<ulong>, IReadOnlyList<double?>, IReadOnlyList<AtomicNonZeroUsize>>> RankAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRank(call);
    }
    public async Task<CallResult<IReadOnlyList<ulong>, IReadOnlyList<double?>, IReadOnlyList<AtomicNonZeroUsize>>> RankAsync(RequestQuestion question,
    RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRank(call);
    }
    public async Task<CallResult<IReadOnlyList<ulong>, IReadOnlyList<double?>, IReadOnlyList<AtomicNonZeroUsize>>> RankAsync(RequestQuestion question,
    RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRank(call);
    }
    public async Task<CallResult<IReadOnlyList<ulong>, IReadOnlyList<double?>, IReadOnlyList<AtomicNonZeroUsize>>> RankAsync(RequestQuestion question,
    RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRank(call);
    }
    public async Task<CallResult<IReadOnlyList<ulong>, IReadOnlyList<double?>, IReadOnlyList<AtomicNonZeroUsize>>> RankAsync(RequestQuestion question,
    RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRank(call);
    }
    public async Task<CallResult<IReadOnlyList<ulong>, IReadOnlyList<double?>, IReadOnlyList<AtomicNonZeroUsize>>> RankAsync(RequestQuestion question,
    RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("rank", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRank(call);
    }
    public async Task<CallResult<IReadOnlyList<ulong>, IReadOnlyList<double?>, IReadOnlyList<AtomicNonZeroUsize>>> RankAsync(RequestQuestion question,
    IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("rank", question, FeedInput, options),
        input, cancellation).ConfigureAwait(false);
        return ProjectRank(call);
    }
    private static CallResult<IReadOnlyList<ulong>, IReadOnlyList<double?>, IReadOnlyList<AtomicNonZeroUsize>> ProjectRank(OwnedCall call) {
        var receipts = new List<AtomicNonZeroUsize>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketRankAggregate aggregate) receipts.AddRange(aggregate.Value);
        }
        var results = receipts.AsReadOnly();
        var values = results.Select(row => row.Index.Value).ToArray();
        return new(call, Array.AsReadOnly(values), Array.AsReadOnly(results.Select(row => (double?)((AnswerYesNo)row.Answer).Probability).ToArray()),
        results);
    }
    public async Task<CallResult<JsonElement?, double?, IReadOnlyList<Find>>> FindAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("find", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFind(call);
    }
    public async Task<CallResult<JsonElement?, double?, IReadOnlyList<Find>>> FindAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("find", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFind(call);
    }
    public async Task<CallResult<JsonElement?, double?, IReadOnlyList<Find>>> FindAsync(RequestQuestion question,
    RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("find", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFind(call);
    }
    public async Task<CallResult<JsonElement?, double?, IReadOnlyList<Find>>> FindAsync(RequestQuestion question,
    RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("find", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFind(call);
    }
    public async Task<CallResult<JsonElement?, double?, IReadOnlyList<Find>>> FindAsync(RequestQuestion question,
    RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("find", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFind(call);
    }
    public async Task<CallResult<JsonElement?, double?, IReadOnlyList<Find>>> FindAsync(RequestQuestion question,
    RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("find", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFind(call);
    }
    public async Task<CallResult<JsonElement?, double?, IReadOnlyList<Find>>> FindAsync(RequestQuestion question,
    RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("find", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectFind(call);
    }
    public async Task<CallResult<JsonElement?, double?, IReadOnlyList<Find>>> FindAsync(RequestQuestion question,
    IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("find", question, FeedInput, options),
        input, cancellation).ConfigureAwait(false);
        return ProjectFind(call);
    }
    private static CallResult<JsonElement?, double?, IReadOnlyList<Find>> ProjectFind(OwnedCall call) {
        var receipts = new List<Find>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketFindAggregate aggregate) receipts.Add(aggregate.Value);
        }
        var results = receipts.AsReadOnly();
        var values = results.Select(row => row.Value.State == PresenceState.Value ? row.Value.Value : (JsonElement?)null).ToArray();
        if (results.Count != 1) throw new JsonException("Missing aggregate result.");
        return new(call, values.Single(), results.Select(row => row.Answer is FindAnswer odds ? (double?)odds.Probabilities[odds.Pick] : null).Single(),
        results);
    }
    public async Task<CallResult<IReadOnlyDictionary<string, AnnotatedField>, double?, IReadOnlyList<Annotation>>> AnnotateAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectAnnotateScalar(call);
    }
    public async Task<CallResult<IReadOnlyDictionary<string, AnnotatedField>, double?, IReadOnlyList<Annotation>>> AnnotateAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectAnnotateScalar(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyDictionary<string, AnnotatedField>>, double?,
    IReadOnlyList<Annotation>>> AnnotateAsync(RequestQuestion question, RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectAnnotate(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyDictionary<string, AnnotatedField>>, double?,
    IReadOnlyList<Annotation>>> AnnotateAsync(RequestQuestion question, RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectAnnotate(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyDictionary<string, AnnotatedField>>, double?,
    IReadOnlyList<Annotation>>> AnnotateAsync(RequestQuestion question, RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectAnnotate(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyDictionary<string, AnnotatedField>>, double?,
    IReadOnlyList<Annotation>>> AnnotateAsync(RequestQuestion question, RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectAnnotate(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyDictionary<string, AnnotatedField>>, double?,
    IReadOnlyList<Annotation>>> AnnotateAsync(RequestQuestion question, RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("annotate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectAnnotate(call);
    }
    public async Task<CallResult<IReadOnlyList<IReadOnlyDictionary<string, AnnotatedField>>, double?,
    IReadOnlyList<Annotation>>> AnnotateAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("annotate", question, FeedInput,
        options), input, cancellation).ConfigureAwait(false);
        return ProjectAnnotate(call);
    }
    private static CallResult<IReadOnlyList<IReadOnlyDictionary<string, AnnotatedField>>, double?,
    IReadOnlyList<Annotation>> ProjectAnnotate(OwnedCall call) {
        var receipts = new List<Annotation>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketAnnotateAggregate aggregate) receipts.AddRange(aggregate.Value);
            else if (packet is SessionPacketAnnotateRow row) receipts.Add(row.Value);
        }
        var results = receipts.AsReadOnly();
        var values = results.Select(row => row.Value).ToArray();
        return new(call, Array.AsReadOnly(values), (double?)null, results);
    }
    private static CallResult<IReadOnlyDictionary<string, AnnotatedField>, double?, IReadOnlyList<Annotation>> ProjectAnnotateScalar(OwnedCall call) {
        var result = ProjectAnnotate(call);
        return new(call, result.Value.Single(), result.Probability, result.Results);
    }
    public async Task<CallResult<Recognize, IReadOnlyList<double>, IReadOnlyList<Recognition>>> RecognizeAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRecognizeScalar(call);
    }
    public async Task<CallResult<Recognize, IReadOnlyList<double>, IReadOnlyList<Recognition>>> RecognizeAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRecognizeScalar(call);
    }
    public async Task<CallResult<IReadOnlyList<Recognize>, IReadOnlyList<IReadOnlyList<double>>,
    IReadOnlyList<Recognition>>> RecognizeAsync(RequestQuestion question, RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRecognize(call);
    }
    public async Task<CallResult<IReadOnlyList<Recognize>, IReadOnlyList<IReadOnlyList<double>>,
    IReadOnlyList<Recognition>>> RecognizeAsync(RequestQuestion question, RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRecognize(call);
    }
    public async Task<CallResult<IReadOnlyList<Recognize>, IReadOnlyList<IReadOnlyList<double>>,
    IReadOnlyList<Recognition>>> RecognizeAsync(RequestQuestion question, RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRecognize(call);
    }
    public async Task<CallResult<IReadOnlyList<Recognize>, IReadOnlyList<IReadOnlyList<double>>,
    IReadOnlyList<Recognition>>> RecognizeAsync(RequestQuestion question, RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRecognize(call);
    }
    public async Task<CallResult<IReadOnlyList<Recognize>, IReadOnlyList<IReadOnlyList<double>>,
    IReadOnlyList<Recognition>>> RecognizeAsync(RequestQuestion question, RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("recognize", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRecognize(call);
    }
    public async Task<CallResult<IReadOnlyList<Recognize>, IReadOnlyList<IReadOnlyList<double>>,
    IReadOnlyList<Recognition>>> RecognizeAsync(RequestQuestion question, IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("recognize", question, FeedInput,
        options), input, cancellation).ConfigureAwait(false);
        return ProjectRecognize(call);
    }
    private static CallResult<IReadOnlyList<Recognize>, IReadOnlyList<IReadOnlyList<double>>, IReadOnlyList<Recognition>> ProjectRecognize(OwnedCall call) {
        var receipts = new List<Recognition>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketRecognizeAggregate aggregate) receipts.AddRange(aggregate.Value);
        }
        var results = receipts.AsReadOnly();
        var values = results.Select(row => row.Value).ToArray();
        return new(call, Array.AsReadOnly(values), Array.AsReadOnly(results.Select(row => RecognitionProbabilities(row)).ToArray()),
        results);
    }
    private static CallResult<Recognize, IReadOnlyList<double>, IReadOnlyList<Recognition>> ProjectRecognizeScalar(OwnedCall call) {
        var result = ProjectRecognize(call);
        return new(call, result.Value.Single(), result.Probability.Single(), result.Results);
    }
    public async Task<CallResult<IReadOnlyList<RelatedEntityEdge>, IReadOnlyList<double>, IReadOnlyList<Relation>>> RelateAsync(RequestQuestion question,
    RequestInputText input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRelate(call);
    }
    public async Task<CallResult<IReadOnlyList<RelatedEntityEdge>, IReadOnlyList<double>, IReadOnlyList<Relation>>> RelateAsync(RequestQuestion question,
    RequestInputJson input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRelate(call);
    }
    public async Task<CallResult<IReadOnlyList<RelatedEntityEdge>, IReadOnlyList<double>, IReadOnlyList<Relation>>> RelateAsync(RequestQuestion question,
    RequestInputRecords input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRelate(call);
    }
    public async Task<CallResult<IReadOnlyList<RelatedEntityEdge>, IReadOnlyList<double>, IReadOnlyList<Relation>>> RelateAsync(RequestQuestion question,
    RequestInputSource input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRelate(call);
    }
    public async Task<CallResult<IReadOnlyList<RelatedEntityEdge>, IReadOnlyList<double>, IReadOnlyList<Relation>>> RelateAsync(RequestQuestion question,
    RequestInputUnits input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRelate(call);
    }
    public async Task<CallResult<IReadOnlyList<RelatedEntityEdge>, IReadOnlyList<double>, IReadOnlyList<Relation>>> RelateAsync(RequestQuestion question,
    RequestInputEntities input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRelate(call);
    }
    public async Task<CallResult<IReadOnlyList<RelatedEntityEdge>, IReadOnlyList<double>, IReadOnlyList<Relation>>> RelateAsync(RequestQuestion question,
    RequestInputFeed input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("relate", question, input, options),
        null, cancellation).ConfigureAwait(false);
        return ProjectRelate(call);
    }
    public async Task<CallResult<IReadOnlyList<RelatedEntityEdge>, IReadOnlyList<double>, IReadOnlyList<Relation>>> RelateAsync(RequestQuestion question,
    IAsyncEnumerable<RequestSessionDescriptor> input,
    RequestOptions? options = null, CancellationToken cancellation = default) {
        ArgumentNullException.ThrowIfNull(input);
        var call = await ExecuteSessionAsync(() => StartNamedSession("relate", question, FeedInput,
        options), input, cancellation).ConfigureAwait(false);
        return ProjectRelate(call);
    }
    private static CallResult<IReadOnlyList<RelatedEntityEdge>, IReadOnlyList<double>, IReadOnlyList<Relation>> ProjectRelate(OwnedCall call) {
        var receipts = new List<Relation>();
        foreach (var packet in call.Packets) {
            if (packet is SessionPacketRelateAggregate aggregate) receipts.Add(aggregate.Value);
        }
        var results = receipts.AsReadOnly();
        var values = results.Select(row => row.Value).ToArray();
        if (results.Count != 1) throw new JsonException("Missing aggregate result.");
        return new(call, values.Single(), results.Select(row => Array.AsReadOnly(row.Value.Select(edge => edge.Probability).ToArray())).Single(),
        results);
    }
    private static IReadOnlyList<double> RecognitionProbabilities(Recognition row) {
        if (row.Value is RecognizeModeProposals boundary) return Array.AsReadOnly(boundary.Proposals.Select(proposal => proposal.Probability).ToArray());
        var entities = ((RecognizeEntities)row.Value).Entities;
        var answer = (RecognitionOddsRecognizeAnswer)row.Answer;
        return Array.AsReadOnly(entities.Select(entity => answer.Proposals.Single(proposal => proposal.Kept && proposal.Selected.State == PresenceState.Value && proposal.Selected.Value.Start == entity.Start && proposal.Selected.Value.End == entity.End && proposal.Kind.State == PresenceState.Value && proposal.Kind.Value == entity.Kind).SpanProbability).ToArray());
    }
}
