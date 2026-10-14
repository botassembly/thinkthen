using ThinkThen.Inputs;
using System.Runtime.InteropServices;
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
    public OwnedSession StartSession(Request request)
    {
        ArgumentNullException.ThrowIfNull(request);
        byte[] bytes = request.ToBytes();
        return Live(_ => {
            NativeSession.Check(NativeSession.thinkthen_session_new_with_surface(ownedEngine, bytes, (nuint)bytes.Length, "csharp"u8.ToArray(), 6, out var pointer));
            return new OwnedSession(pointer);
        });
    }
    /// <summary>Run intake and output concurrently so neither bounded queue stalls the other.</summary>
    public Task<OwnedCall> ExecuteAsync(Request request,
        IAsyncEnumerable<RequestSessionDescriptor>? feed = null, CancellationToken cancellation = default) =>
        ExecuteSessionAsync(() => StartSession(request), feed, cancellation);
    private async Task<OwnedCall> ExecuteSessionAsync(Func<OwnedSession> start,
        IAsyncEnumerable<RequestSessionDescriptor>? feed, CancellationToken cancellation)
    {
        cancellation.ThrowIfCancellationRequested();
        using var session = start();
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
            catch (IOException) when (!stop.IsCancellationRequested) { session.Finish(new RequestReaderFailureIo()); }
            catch (System.Text.DecoderFallbackException) when (!stop.IsCancellationRequested) { session.Finish(new RequestReaderFailureUtf8()); }
            catch (System.Text.Json.JsonException) when (!stop.IsCancellationRequested) { session.Finish(new RequestReaderFailureInvalidInput()); }
            catch (InvalidDataException) when (!stop.IsCancellationRequested) { session.Finish(new RequestReaderFailureInvalidInput()); }
            catch { session.Cancel(); stop.Cancel(); throw; }
        }
    }
    private static readonly RequestInput FeedInput = new RequestInputFeed { Name = "input" };
    private OwnedSession StartNamedSession(string function, RequestQuestion question,
        RequestInput input, RequestOptions? options)
    {
        byte[][] values = [Text(function), question?.ToBytes() ?? "null"u8.ToArray(),
            input?.ToBytes() ?? "null"u8.ToArray(), (options ?? new RequestOptions()).ToBytes(), "csharp"u8.ToArray()];
        var pinned = new GCHandle[values.Length];
        try
        {
            for (int i = 0; i < values.Length; i++) pinned[i] = GCHandle.Alloc(values[i], GCHandleType.Pinned);
            CompleteUtf8V1 View(int index) => new() { data = pinned[index].AddrOfPinnedObject(), len = (nuint)values[index].Length };
            var request = new NamedRequestV1 { function = View(0), question_json = View(1), input_json = View(2), options_json = View(3) };
            return Live(_ => {
                NativeSession.Check(NativeNamed.thinkthen_session_new_named_with_surface(ownedEngine, in request, View(4), out var pointer));
                return new OwnedSession(pointer);
            });
        }
        finally { foreach (var handle in pinned) if (handle.IsAllocated) handle.Free(); }
    }
}
