using System.Collections.Concurrent;

namespace ThinkThen;

// This owner closes independently of a consumer paused after a yielded row.
internal sealed class CallState
{
    private readonly CallHandle handle;
    private readonly CancellationToken supplied;
    private readonly CancellationTokenSource stop;
    private readonly CancellationTokenRegistration cancellation;
    private readonly object closing = new();
    private readonly BlockingCollection<Action> commands = new(1);
    private readonly TaskCompletionSource finished = new(TaskCreationOptions.RunContinuationsAsynchronously);
    private readonly nint events;
    private Task? cleanup;
    private readonly Thread driver;
    internal CancellationToken Token { get; }
    internal CallState(CallHandle handle, CancellationToken supplied)
    {
        this.handle = handle;
        this.supplied = supplied;
        stop = CancellationTokenSource.CreateLinkedTokenSource(supplied);
        Token = stop.Token;
        events = Native.Events(handle);
        cancellation = Token.Register(() => { Native.Cancel(handle); Native.EventsWake(events); });
        driver = new Thread(Drive) { IsBackground = true, Name = "ThinkThen stream" };
        driver.Start();
    }
    private void Drive()
    {
        try { foreach (Action command in commands.GetConsumingEnumerable()) command(); }
        finally { finished.TrySetResult(); }
    }
    internal Task<T> Dispatch<T>(Func<CallHandle,T> action)
    {
        var result = new TaskCompletionSource<T>(TaskCreationOptions.RunContinuationsAsynchronously);
        lock (closing)
        {
            if (cleanup is not null) return Task.FromException<T>(Cancelled());
            commands.Add(() =>
            {
                try { result.SetResult(Invoke(action)); }
                catch (Exception error) { result.SetException(error); }
            });
        }
        return result.Task;
    }
    internal ulong Version() => Native.EventsVersion(events);
    internal async Task WaitForInput(Task<bool> input, ulong version)
    {
        Task<int> wait = Dispatch(_ => { Native.EventsWait(events, version); return 0; });
        await Task.WhenAny(input, wait).ConfigureAwait(false);
        // Join even if input failed or was cancelled before sending or closing.
        lock (closing) { if (cleanup is null) Native.EventsWake(events); }
        await wait.ConfigureAwait(false);
    }
    private T Invoke<T>(Func<CallHandle,T> action)
    {
        Token.ThrowIfCancellationRequested();
        return action(handle);
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
        commands.CompleteAdding();
        await finished.Task.ConfigureAwait(false);
        driver.Join();
        handle.Dispose();
        Native.EventsFree(events);
        commands.Dispose();
        stop.Dispose();
    }
}
