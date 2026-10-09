using System.Runtime.InteropServices;
using System.Text.Json;
using ThinkThen.Inputs;
using ThinkThen.Results;
namespace ThinkThen;
/// <summary>A bounded native session. Cancel retains the receiver for an explicit drain.</summary>
public sealed class OwnedSession : IDisposable
{
    private readonly SessionHandle handle;
    private readonly CancellationTokenSource disposed = new();
    private int reading, producing, closed;
    private const int WaitMilliseconds = 10;
    internal OwnedSession(IntPtr pointer) => handle = new(pointer);
    public void Cancel()
    {
        try { NativeSession.thinkthen_session_cancel(handle); }
        catch (ObjectDisposedException) { }
    }
    public void Finish() { ThrowIfDisposed(); NativeSession.Check(NativeSession.thinkthen_session_finish(handle, IntPtr.Zero, 0)); }
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
            while (true)
            {
                linked.Token.ThrowIfCancellationRequested();
                NativeSession.Check(NativeSession.thinkthen_session_try_read(handle, out uint status, out var pointer));
                if (status == 2) return null;
                if (status == 0)
                {
                    using var packet = new PacketHandle(pointer);
                    // Keep the owner pinned across both accessor and copy, not just P/Invoke.
                    bool pinned = false;
                    try
                    {
                        packet.DangerousAddRef(ref pinned);
                        NativeSession.Check(NativeSession.thinkthen_session_result_json(packet, out var json, out var length));
                        byte[] bytes = new byte[checked((int)length)];
                        Marshal.Copy(json, bytes, 0, bytes.Length);
                        using var document = JsonDocument.Parse(bytes);
                        return SessionPacket.Read(document.RootElement);
                    }
                    finally { if (pinned) packet.DangerousRelease(); }
                }
                if (status != 1) throw new InvalidOperationException("Invalid native read status.");
                await Task.Delay(WaitMilliseconds, linked.Token).ConfigureAwait(false);
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
            while (true)
            {
                linked.Token.ThrowIfCancellationRequested();
                NativeSession.Check(NativeSession.thinkthen_session_try_push(handle, bytes, (nuint)bytes.Length, out uint status));
                if (status == 0) return true;
                if (status == 2) return false;
                if (status != 1) throw new InvalidOperationException("Invalid native push status.");
                await Task.Delay(WaitMilliseconds, linked.Token).ConfigureAwait(false);
            }
        }
        finally { Volatile.Write(ref producing, 0); }
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
