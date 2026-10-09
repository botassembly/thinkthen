using ThinkThen.Inputs;
using ThinkThen.Results;
namespace ThinkThen;
/// <summary>Owned packets and the actual native terminal, in delivery order.</summary>
public sealed record OwnedCall(IReadOnlyList<SessionPacket> Packets, SessionPacketTerminal Terminal);
public sealed class SessionFailure : Exception
{
    public CallError Failure { get; }
    public OwnedCall Call { get; }
    internal SessionFailure(CallError failure, OwnedCall call) : base("Native execution failed.") { Failure = failure; Call = call; }
}
public sealed partial class Engine
{
    public OwnedSession StartSession(InputRequest request)
    {
        ArgumentNullException.ThrowIfNull(request);
        byte[] bytes = request.ToBytes();
        return Live(_ => {
            NativeSession.Check(NativeSession.thinkthen_session_new(ownedEngine, bytes, (nuint)bytes.Length, out var pointer));
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
            await producer.ConfigureAwait(false);
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
            catch { session.Cancel(); stop.Cancel(); throw; }
        }
    }
    public Task<OwnedCall> DecideAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallDecide { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
    public Task<OwnedCall> ChooseAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallChoose { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
    public Task<OwnedCall> TagAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallTag { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
    public Task<OwnedCall> ScoreAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallScore { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
    public Task<OwnedCall> FilterAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallFilter { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
    public Task<OwnedCall> RankAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallRank { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
    public Task<OwnedCall> FindAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallFind { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
    public Task<OwnedCall> AnnotateAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallAnnotate { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
    public Task<OwnedCall> RecognizeAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallRecognize { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
    public Task<OwnedCall> RelateAsync(InputRequestQuestion question, InputRequestInput input,
        InputRequestOptions? options = null, CancellationToken cancellation = default) => ExecuteAsync(
        new InputRequest { Schema = new InputRequestVersionAlternative0(), Call = new InputRequestCallRelate { Question = question, Input = input,
            Options = options is null ? default : (InputPresence<InputRequestOptions>)options } }, cancellation: cancellation);
}
