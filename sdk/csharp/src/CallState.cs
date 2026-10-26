namespace ThinkThen;

// This owner closes independently of a consumer paused after a yielded row.
internal sealed class CallState
{
    private readonly CallHandle handle;
    private readonly CancellationToken supplied;
    private readonly CancellationTokenSource stop;
    private readonly CancellationTokenRegistration cancellation;
    private readonly object native = new();
    private readonly object closing = new();
    private Task? cleanup;
    private bool closed;
    internal CancellationToken Token { get; }
    internal CallState(CallHandle handle, CancellationToken supplied)
    {
        this.handle = handle;
        this.supplied = supplied;
        stop = CancellationTokenSource.CreateLinkedTokenSource(supplied);
        Token = stop.Token;
        cancellation = Token.Register(() => Native.Cancel(handle));
    }
    internal T Invoke<T>(Func<CallHandle,T> action)
    {
        lock (native)
        {
            Token.ThrowIfCancellationRequested();
            if (closed) throw Cancelled();
            return action(handle);
        }
    }
    internal OperationCanceledException Cancelled() => new(supplied.IsCancellationRequested ? supplied : Token);
    internal Task Close()
    {
        lock (closing) return cleanup ??= CloseCore();
    }
    private async Task CloseCore()
    {
        await Task.Yield();
        stop.Cancel();
        cancellation.Dispose();
        await Task.Run(() => { lock (native) { closed = true; handle.Dispose(); } }).ConfigureAwait(false);
        stop.Dispose();
    }
}
