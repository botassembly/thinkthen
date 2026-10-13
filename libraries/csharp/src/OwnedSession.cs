using ThinkThen.Inputs;
using ThinkThen.Results;
namespace ThinkThen;
/// <summary>A bounded native session. Cancel retains the receiver for an explicit drain.</summary>
public sealed class OwnedSession : IDisposable
{
    private readonly SessionHandle handle;
    private readonly CancellationTokenSource disposed = new();
    private int reading, producing, closed, cancelled;
    internal OwnedSession(IntPtr pointer) => handle = new(pointer);
    public void Cancel()
    {
        Volatile.Write(ref cancelled, 1);
        try { NativeSession.thinkthen_session_cancel(handle); }
        catch (ObjectDisposedException) { }
    }
    public void Finish(InputRequestReaderFailure? failure = null)
    {
        ThrowIfDisposed();
        byte[]? bytes = failure?.ToBytes();
        NativeSession.Check(NativeSession.thinkthen_session_finish(handle, bytes, (nuint)(bytes?.Length ?? 0)));
    }
    private void ThrowIfDisposed() { if (Volatile.Read(ref closed) != 0) throw new ObjectDisposedException(nameof(OwnedSession)); }
    /// <summary>Return null only after native End. A fresh token can drain after Cancel.</summary>
    public async Task<SessionPacket?> ReadAsync(CancellationToken cancellation = default)
    {
        ThrowIfDisposed();
        if (Interlocked.Exchange(ref reading, 1) != 0) throw new InvalidOperationException("A session allows one reader.");
        try
        {
            using var linked = CancellationTokenSource.CreateLinkedTokenSource(cancellation, disposed.Token);
            using var registration = cancellation.Register(Cancel);
            linked.Token.ThrowIfCancellationRequested();
            try
            {
                SessionPacket? result = await Task.Factory.StartNew(() => ReadPacket(linked.Token),
                    CancellationToken.None, TaskCreationOptions.LongRunning, TaskScheduler.Default).ConfigureAwait(false);
                linked.Token.ThrowIfCancellationRequested();
                return result;
            }
            catch (Failure) when (linked.IsCancellationRequested)
            {
                linked.Token.ThrowIfCancellationRequested();
                throw;
            }
        }
        finally { Volatile.Write(ref reading, 0); }
    }
    /// <summary>Retry the same descriptor after Full; false means stop advancing the producer.</summary>
    public async Task<bool> PushAsync(InputRequestSessionDescriptor descriptor, CancellationToken cancellation = default)
    {
        ArgumentNullException.ThrowIfNull(descriptor);
        ThrowIfDisposed();
        if (Interlocked.Exchange(ref producing, 1) != 0) throw new InvalidOperationException("A session allows one producer.");
        try
        {
            using var linked = CancellationTokenSource.CreateLinkedTokenSource(cancellation, disposed.Token);
            using var registration = cancellation.Register(Cancel);
            byte[] bytes = descriptor.ToBytes();
            linked.Token.ThrowIfCancellationRequested();
            try
            {
                bool accepted = await Task.Factory.StartNew(() =>
                {
                    NativeSession.Check(NativeWait.thinkthen_session_push(handle, bytes, (nuint)bytes.Length, out uint status));
                    return status switch { 0 => true, 2 => false, _ => throw new InvalidOperationException("Invalid native push status.") };
                }, CancellationToken.None, TaskCreationOptions.LongRunning, TaskScheduler.Default).ConfigureAwait(false);
                linked.Token.ThrowIfCancellationRequested();
                return accepted;
            }
            catch (Failure) when (linked.IsCancellationRequested)
            {
                linked.Token.ThrowIfCancellationRequested();
                throw;
            }
        }
        finally { Volatile.Write(ref producing, 0); }
    }
    private SessionPacket? ReadPacket(CancellationToken cancellation)
    {
        while (true)
        {
            uint interest = Volatile.Read(ref cancelled) != 0 ? NativeWait.Settlement : NativeWait.Result;
            uint status;
            IntPtr pointer;
            try
            {
                // Native diagnostics belong to this worker thread.
                NativeSession.Check(NativeWait.thinkthen_session_next_packet(handle, interest, out status, out pointer));
            }
            catch (Failure failure) when (failure.Kind == FailureKind.Cancelled && Volatile.Read(ref cancelled) != 0)
            {
                cancellation.ThrowIfCancellationRequested();
                continue;
            }
            if (status == NativeWait.ReadEnd) return null;
            if (status != NativeWait.ReadResult) throw new InvalidOperationException("Invalid native read status.");
            using var packet = new PacketHandle(pointer);
            bool pinned = false;
            try
            {
                packet.DangerousAddRef(ref pinned);
                NativeSession.Check(NativeSession.thinkthen_session_result_json(packet, out var json, out var length));
                return SessionPacket.Read(Engine.ReadJson(json, length));
            }
            finally { if (pinned) packet.DangerousRelease(); }
        }
    }
    public void Dispose()
    {
        if (Interlocked.Exchange(ref closed, 1) != 0) return;
        disposed.Cancel();
        Cancel();
        handle.Dispose();
        // Active operations retain linked registrations until their finally blocks run.
    }
}
