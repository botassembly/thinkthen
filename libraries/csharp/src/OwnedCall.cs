using ThinkThen.Inputs;
using ThinkThen.Results;
namespace ThinkThen;
/// <summary>Owned packets and the actual native terminal, in delivery order.</summary>
public sealed record OwnedCall(IReadOnlyList<SessionPacket> Packets, SessionPacketTerminal Terminal);
public sealed class SessionFailure : Exception
{
    public CallError Failure { get; }
    public OwnedCall Call { get; }
    internal SessionFailure(CallError failure, OwnedCall call) : base(failure.Error.Message) { Failure = failure; Call = call; }
}
public sealed partial class Engine
{
    public OwnedSession StartSession(InputRequest request)
    {
        ArgumentNullException.ThrowIfNull(request);
        byte[] bytes = request.ToBytes();
        return Live(_ => {
            NativeSession.Check(NativeSession.thinkthen_session_new_with_surface(ownedEngine, bytes, (nuint)bytes.Length, "csharp"u8.ToArray(), 6, out var pointer));
            return new OwnedSession(pointer);
        });
    }
    /// <summary>Run intake and output concurrently so neither bounded queue stalls the other.</summary>
    public async Task<OwnedCall> ExecuteAsync(InputRequest request,
        IAsyncEnumerable<InputRequestSessionDescriptor>? feed = null, CancellationToken cancellation = default)
    {
        cancellation.ThrowIfCancellationRequested();
        using var session = StartSession(request);
        using var stop = CancellationTokenSource.CreateLinkedTokenSource(cancellation);
        using var registration = cancellation.Register(session.Cancel);
        if (feed is null) session.Finish();
        Task producer = feed is null ? Task.CompletedTask : ProduceAsync();
        var packets = new List<SessionPacket>();
        SessionPacketTerminal? terminal = null;
        try
        {
            while (await session.ReadAsync(stop.Token).ConfigureAwait(false) is { } packet)
            {
                packets.Add(packet);
                if (packet is SessionPacketTerminal settled) terminal = settled;
            }
            stop.Cancel();
            if (producer.IsFaulted) await producer.ConfigureAwait(false);
            cancellation.ThrowIfCancellationRequested();
            var call = new OwnedCall(packets.AsReadOnly(), terminal ?? throw new InvalidOperationException("Native End has no terminal."));
            if (call.Terminal.Failure.State == PresenceState.Value) throw new SessionFailure(call.Terminal.Failure.Value, call);
            return call;
        }
        catch
        {
            if (producer.IsFaulted) await producer.ConfigureAwait(false);
            throw;
        }
        finally
        {
            stop.Cancel();
            session.Cancel();
            // Observe producer failures without delaying host cancellation for a foreign iterator.
            if (!producer.IsCompleted) _ = producer.ContinueWith(task => { _ = task.Exception; }, CancellationToken.None,
                TaskContinuationOptions.OnlyOnFaulted | TaskContinuationOptions.ExecuteSynchronously, TaskScheduler.Default);
        }
        async Task ProduceAsync()
        {
            try
            {
                await foreach (var descriptor in feed!.WithCancellation(stop.Token).ConfigureAwait(false))
                {
                    stop.Token.ThrowIfCancellationRequested();
                    if (!await session.PushAsync(descriptor, stop.Token).ConfigureAwait(false)) return;
                }
                session.Finish();
            }
            catch (IOException) when (!stop.IsCancellationRequested) { session.Finish(new InputRequestReaderFailureIo()); }
            catch (System.Text.DecoderFallbackException) when (!stop.IsCancellationRequested) { session.Finish(new InputRequestReaderFailureUtf8()); }
            catch (System.Text.Json.JsonException) when (!stop.IsCancellationRequested) { session.Finish(new InputRequestReaderFailureInvalidInput()); }
            catch (InvalidDataException) when (!stop.IsCancellationRequested) { session.Finish(new InputRequestReaderFailureInvalidInput()); }
            catch { session.Cancel(); stop.Cancel(); throw; }
        }
    }
    private static readonly InputRequestInput FeedInput = new InputRequestInputFeed { Name = "input" };
    private static InputPresence<InputRequestOptions> Controls(InputRequestOptions? options) =>
        options is null ? default : (InputPresence<InputRequestOptions>)options;
    private Task<OwnedCall> CallAsync(InputRequestCall call, CancellationToken cancellation,
        IAsyncEnumerable<InputRequestSessionDescriptor>? feed = null) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = call }, feed, cancellation);
    public Task<OwnedCall> DecideAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallDecide { Question = question, Input = input, Options = Controls(options) }, cancellation);
    public Task<OwnedCall> ChooseAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallChoose { Question = question, Input = input, Options = Controls(options) }, cancellation);
    public Task<OwnedCall> TagAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallTag { Question = question, Input = input, Options = Controls(options) }, cancellation);
    public Task<OwnedCall> ScoreAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallScore { Question = question, Input = input, Options = Controls(options) }, cancellation);
    public Task<OwnedCall> FilterAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallFilter { Question = question, Input = input, Options = Controls(options) }, cancellation);
    public Task<OwnedCall> RankAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallRank { Question = question, Input = input, Options = Controls(options) }, cancellation);
    public Task<OwnedCall> FindAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallFind { Question = question, Input = input, Options = Controls(options) }, cancellation);
    public Task<OwnedCall> AnnotateAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallAnnotate { Question = question, Input = input, Options = Controls(options) }, cancellation);
    public Task<OwnedCall> RecognizeAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallRecognize { Question = question, Input = input, Options = Controls(options) }, cancellation);
    public Task<OwnedCall> RelateAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => CallAsync(
        new InputRequestCallRelate { Question = question, Input = input, Options = Controls(options) }, cancellation);
    private Task<OwnedCall> FeedAsync(InputRequestCall call,
        IAsyncEnumerable<InputRequestSessionDescriptor> feed, CancellationToken cancellation) =>
        CallAsync(call, cancellation, feed ?? throw new ArgumentNullException(nameof(feed)));
    public Task<OwnedCall> DecideAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallDecide { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
    public Task<OwnedCall> ChooseAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallChoose { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
    public Task<OwnedCall> TagAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallTag { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
    public Task<OwnedCall> ScoreAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallScore { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
    public Task<OwnedCall> FilterAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallFilter { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
    public Task<OwnedCall> RankAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallRank { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
    public Task<OwnedCall> FindAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallFind { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
    public Task<OwnedCall> AnnotateAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallAnnotate { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
    public Task<OwnedCall> RecognizeAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallRecognize { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
    public Task<OwnedCall> RelateAsync(InputRequestQuestion question, IAsyncEnumerable<InputRequestSessionDescriptor> feed,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => FeedAsync(
        new InputRequestCallRelate { Question = question, Input = FeedInput, Options = Controls(options) }, feed, cancellation);
}
